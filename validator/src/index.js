/**
 * AISP Validator - Node.js Entry Point
 * Validates AISP 5.1 documents using the WASM kernel
 */

import { readFile } from 'fs/promises';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

/**
 * AISP Symbol Set (Σ_512 subset for density calculation)
 * Formal symbols that define AISP semantic content
 */
const AISP_SYMBOLS = [
    // Block delimiters
    '⟦', '⟧',
    // Operators
    '≜', '≔', '≡', '≢',
    // Quantifiers
    '∀', '∃',
    // Lambda
    'λ',
    // Logic
    '⇒', '⇔', '→', '↔', '∧', '∨', '¬', '⊕',
    // Sets
    '∈', '∉', '⊆', '⊇', '∩', '∪', '∅', '𝒫',
    // Relations
    '≤', '≥', '<', '>',
    // Types
    'ℕ', 'ℤ', 'ℝ', '𝔹', '𝕊',
    // Document
    '𝔸',
    // Tier symbols
    '◊', '⊘',
    // Angle brackets (tuples)
    '⟨', '⟩',
    // Greek letters (commonly used)
    'α', 'β', 'γ', 'δ', 'ε', 'φ', 'τ', 'ρ', 'Ω', 'Σ', 'Γ', 'Λ', 'Ε', 'Θ', 'Χ', 'Δ', 'Π'
];

/**
 * Calculate pure density: |AISP_symbols| ÷ |non_ws_tokens|
 * @param {string} text - AISP source
 * @returns {object} { pureDensity, symbolCount, tokenCount }
 */
function calculatePureDensity(text) {
    // Count AISP symbols
    let symbolCount = 0;
    for (const symbol of AISP_SYMBOLS) {
        const matches = text.split(symbol).length - 1;
        symbolCount += matches;
    }

    // Count non-whitespace tokens (split on whitespace, filter empty)
    const tokens = text.split(/\s+/).filter(t => t.length > 0);
    const tokenCount = tokens.length;

    // Pure density: ratio of AISP symbols to tokens
    const pureDensity = tokenCount > 0 ? symbolCount / tokenCount : 0;

    return {
        pureDensity,
        symbolCount,
        tokenCount
    };
}

/**
 * Calculate semantic density using Block Coverage + Binding Density
 * δ = (blockScore × 0.4) + (bindingScore × 0.6)
 * @param {string} text - AISP source
 * @returns {object} { delta, blockScore, bindingScore, breakdown, pureDensity }
 */
function calculateSemanticDensity(text) {
    // Required blocks present?
    const blocks = ['⟦Ω', '⟦Σ', '⟦Γ', '⟦Λ', '⟦Ε'];
    const blocksFound = blocks.filter(b => text.includes(b));
    const blockScore = blocksFound.length / blocks.length;

    // Binding density (semantic operators)
    const definitions = (text.match(/≜/g) || []).length;
    const assignments = (text.match(/≔/g) || []).length;
    const quantifiers = (text.match(/[∀∃]/g) || []).length;
    const lambdas = (text.match(/λ/g) || []).length;
    const implications = (text.match(/[⇒⇔→↔]/g) || []).length;
    const setOps = (text.match(/[∈⊆∩∪∅]/g) || []).length;

    const totalBindings = definitions + assignments + quantifiers + lambdas + implications + setOps;
    const bindingScore = Math.min(1, totalBindings / 20);

    // Combined score (semantic delta)
    const delta = (blockScore * 0.4) + (bindingScore * 0.6);

    // Pure density (symbol/token ratio)
    const pureResult = calculatePureDensity(text);

    return {
        delta,
        blockScore,
        bindingScore,
        pureDensity: pureResult.pureDensity,
        breakdown: {
            blocksFound: blocksFound.length,
            blocksRequired: blocks.length,
            definitions,
            assignments,
            quantifiers,
            lambdas,
            implications,
            setOps,
            totalBindings,
            symbolCount: pureResult.symbolCount,
            tokenCount: pureResult.tokenCount
        }
    };
}

/**
 * Get tier from delta value
 * @param {number} delta
 * @returns {object} { tier, tierValue, tierName }
 */
function getTierFromDelta(delta) {
    if (delta >= 0.75) return { tier: '◊⁺⁺', tierValue: 4, tierName: 'Platinum' };
    if (delta >= 0.60) return { tier: '◊⁺', tierValue: 3, tierName: 'Gold' };
    if (delta >= 0.40) return { tier: '◊', tierValue: 2, tierName: 'Silver' };
    if (delta >= 0.20) return { tier: '◊⁻', tierValue: 1, tierName: 'Bronze' };
    return { tier: '⊘', tierValue: 0, tierName: 'Reject' };
}

/**
 * Supported file extensions for AISP documents
 */
const SUPPORTED_EXTENSIONS = ['.aisp', '.md', '.txt', '.spec', '.aisp5'];

/**
 * Default and max document size limits
 */
const SIZE_LIMITS = {
    DEFAULT_MAX: 64 * 1024,  // 64KB default
    ABSOLUTE_MAX: 1024 * 1024, // 1MB absolute max
    WASM_MAX: 1024  // WASM kernel limit
};

const AISP = {
    _instance: null,
    _memory: null,
    _allocPtr: 0x1000,
    _initialized: false,
    _maxDocSize: SIZE_LIMITS.DEFAULT_MAX,

    /**
     * Initialize AISP kernel
     * @param {string|object} [options] - Path to aisp.wasm or options object
     * @param {string} [options.wasmPath] - Path to aisp.wasm
     * @param {number} [options.maxDocSize] - Maximum document size in bytes (default: 64KB)
     * @returns {Promise<number>} 0 on success
     */
    async init(options) {
        if (this._initialized) return 0;

        // Handle both string (legacy) and object (new) options
        let wasmPath;
        if (typeof options === 'string') {
            wasmPath = options;
        } else if (options && typeof options === 'object') {
            wasmPath = options.wasmPath;
            if (options.maxDocSize) {
                this._maxDocSize = Math.min(options.maxDocSize, SIZE_LIMITS.ABSOLUTE_MAX);
            }
        }

        const path = wasmPath || join(__dirname, '..', 'wasm', 'aisp.wasm');
        const bytes = await readFile(path);

        const { instance } = await WebAssembly.instantiate(bytes, {
            env: {
                host_alloc: (size, align) => {
                    const aligned = (this._allocPtr + align - 1) & ~(align - 1);
                    this._allocPtr = aligned + size;
                    return aligned;
                }
            }
        });

        this._instance = instance.exports;
        this._memory = new Uint8Array(instance.exports.memory.buffer);
        this._initialized = true;

        return this._instance.aisp_init();
    },

    /**
     * Set maximum document size
     * @param {number} size - Maximum size in bytes
     */
    setMaxDocSize(size) {
        this._maxDocSize = Math.min(size, SIZE_LIMITS.ABSOLUTE_MAX);
    },

    /**
     * Validate AISP document
     * @param {string} source - AISP source code
     * @param {object} [options] - Validation options
     * @param {boolean} [options.strict] - Use strict WASM validation (limited to 1KB)
     * @returns {object} Validation result
     */
    validate(source, options = {}) {
        if (!this._instance) {
            throw new Error('AISP not initialized. Call init() first.');
        }

        const encoder = new TextEncoder();
        const bytes = encoder.encode(source);

        // Check size against configurable limit
        if (bytes.length > this._maxDocSize) {
            return {
                valid: false,
                error: `Document too large (${bytes.length} bytes, max ${this._maxDocSize} bytes)`,
                errorCode: -4
            };
        }

        // Use semantic density calculation (works for all sizes)
        const densityResult = calculateSemanticDensity(source);
        const tierResult = getTierFromDelta(densityResult.delta);

        // For documents within WASM limit, use WASM validation
        if (bytes.length <= SIZE_LIMITS.WASM_MAX && options.strict !== false) {
            const ptr = 0x1000;
            this._memory = new Uint8Array(this._instance.memory.buffer);
            this._memory.set(bytes, ptr);

            const docId = this._instance.aisp_parse(ptr, bytes.length);
            if (docId < 0) {
                return {
                    valid: false,
                    error: `Parse error at offset ${this._instance.aisp_error_offset()}`,
                    errorCode: this._instance.aisp_error_code(),
                    tier: tierResult.tier,
                    tierValue: tierResult.tierValue,
                    tierName: tierResult.tierName,
                    delta: densityResult.delta,
                    pureDensity: densityResult.pureDensity
                };
            }

            const parseResult = this._instance.aisp_validate(docId);

            return {
                valid: parseResult === 0,
                tier: tierResult.tier,
                tierValue: tierResult.tierValue,
                tierName: tierResult.tierName,
                delta: densityResult.delta,
                pureDensity: densityResult.pureDensity,
                ambiguity: this._instance.aisp_ambig(docId),
                errorCode: parseResult,
                mode: 'wasm'
            };
        }

        // For larger documents, use pure JS validation
        const jsValidation = this._validatePureJS(source, densityResult);

        return {
            valid: jsValidation.valid,
            tier: tierResult.tier,
            tierValue: tierResult.tierValue,
            tierName: tierResult.tierName,
            delta: densityResult.delta,
            pureDensity: densityResult.pureDensity,
            ambiguity: jsValidation.ambiguity,
            errorCode: jsValidation.valid ? 0 : -3,
            mode: 'js',
            docSize: bytes.length
        };
    },

    /**
     * Pure JavaScript validation for larger documents
     * @private
     */
    _validatePureJS(source, densityResult) {
        // Check for AISP header
        if (!source.trim().startsWith('𝔸')) {
            return { valid: false, ambiguity: 1.0, error: 'Missing AISP header (𝔸)' };
        }

        // Check for required blocks
        const requiredBlocks = ['⟦Ω', '⟦Σ', '⟦Γ', '⟦Λ', '⟦Ε'];
        const missingBlocks = requiredBlocks.filter(b => !source.includes(b));

        if (missingBlocks.length > 0) {
            return {
                valid: false,
                ambiguity: 0.5,
                error: `Missing required blocks: ${missingBlocks.join(', ')}`
            };
        }

        // Valid if all required blocks present and density >= 0.20 (Bronze tier minimum)
        const valid = densityResult.delta >= 0.20;
        const ambiguity = valid ? 0.01 : 0.5;

        return { valid, ambiguity };
    },

    /**
     * Get detailed density breakdown for debugging
     * @param {string} source - AISP source
     * @returns {object} Detailed breakdown
     */
    debug(source) {
        const densityResult = calculateSemanticDensity(source);
        const tierResult = getTierFromDelta(densityResult.delta);

        return {
            ...tierResult,
            delta: densityResult.delta,
            pureDensity: densityResult.pureDensity,
            blockScore: densityResult.blockScore,
            bindingScore: densityResult.bindingScore,
            breakdown: densityResult.breakdown
        };
    },

    /**
     * Get tier symbol from numeric value
     * @private
     */
    _getTierSymbol(tier) {
        const symbols = ['⊘', '◊⁻', '◊', '◊⁺', '◊⁺⁺'];
        return symbols[tier] || '⊘';
    },

    /**
     * Quick validation check
     * @param {string} source - AISP source
     * @returns {boolean} true if valid
     */
    isValid(source) {
        return this.validate(source).valid;
    },

    /**
     * Get density score
     * @param {string} source - AISP source
     * @returns {number} density δ [0, 1]
     */
    getDensity(source) {
        return this.validate(source).delta;
    },

    /**
     * Get quality tier
     * @param {string} source - AISP source
     * @returns {string} tier symbol
     */
    getTier(source) {
        return this.validate(source).tier;
    },

    /**
     * Validate file
     * @param {string} filePath - Path to AISP file
     * @returns {Promise<object>} Validation result
     */
    async validateFile(filePath) {
        const content = await readFile(filePath, 'utf-8');
        return this.validate(content);
    },

    /**
     * Debug file - show density breakdown
     * @param {string} filePath - Path to AISP file
     * @returns {Promise<object>} Debug breakdown
     */
    async debugFile(filePath) {
        const content = await readFile(filePath, 'utf-8');
        return this.debug(content);
    }
};

export default AISP;
export const { init, validate, isValid, getDensity, getTier, validateFile, debug, debugFile, setMaxDocSize } = AISP;
export { calculateSemanticDensity, calculatePureDensity, getTierFromDelta, AISP_SYMBOLS, SUPPORTED_EXTENSIONS, SIZE_LIMITS };
