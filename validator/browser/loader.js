/**
 * AISP WASM Loader
 * Minimal JavaScript interface for browser integration (<1KB minified)
 */

const AISP = {
    _instance: null,
    _memory: null,
    _allocPtr: 0x1000,

    /**
     * Initialize AISP kernel
     * @param {string} wasmUrl - URL to aisp.wasm file
     * @returns {Promise<number>} 0 on success
     */
    async init(wasmUrl = '/aisp.wasm') {
        const response = await fetch(wasmUrl);
        const bytes = await response.arrayBuffer();

        const { instance } = await WebAssembly.instantiate(bytes, {
            env: {
                // Host memory allocator (optional)
                host_alloc: (size, align) => {
                    const aligned = (this._allocPtr + align - 1) & ~(align - 1);
                    this._allocPtr = aligned + size;
                    return aligned;
                }
            }
        });

        this._instance = instance.exports;
        this._memory = new Uint8Array(instance.exports.memory.buffer);

        return this._instance.aisp_init();
    },

    /**
     * Validate AISP document
     * @param {string} source - AISP source code
     * @returns {object} Validation result
     */
    validate(source) {
        if (!this._instance) {
            throw new Error('AISP not initialized. Call init() first.');
        }

        const encoder = new TextEncoder();
        const bytes = encoder.encode(source);

        // Write to parse buffer at offset 0x1000
        const ptr = 0x1000;
        if (bytes.length > 1024) {
            return { valid: false, error: 'Document too large (max 1KB)' };
        }

        // Refresh memory view in case it grew
        this._memory = new Uint8Array(this._instance.memory.buffer);
        this._memory.set(bytes, ptr);

        // Parse document
        const docId = this._instance.aisp_parse(ptr, bytes.length);
        if (docId < 0) {
            return {
                valid: false,
                error: `Parse error at offset ${this._instance.aisp_error_offset()}`,
                errorCode: this._instance.aisp_error_code()
            };
        }

        // Validate
        const result = this._instance.aisp_validate(docId);

        return {
            valid: result === 0,
            tier: this._getTierSymbol(this._instance.aisp_tier(docId)),
            tierValue: this._instance.aisp_tier(docId),
            delta: this._instance.aisp_density(docId),
            ambiguity: this._instance.aisp_ambig(docId),
            errorCode: result
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
    }
};

// Export for different environments
if (typeof module !== 'undefined' && module.exports) {
    module.exports = AISP;
} else if (typeof window !== 'undefined') {
    window.AISP = AISP;
}

export default AISP;
