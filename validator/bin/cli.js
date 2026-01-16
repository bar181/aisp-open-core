#!/usr/bin/env node
/**
 * AISP Validator CLI
 * Usage: aisp-validator <command> [file] [options]
 */

import AISP, { SUPPORTED_EXTENSIONS, SIZE_LIMITS } from '../src/index.js';
import { readFile, stat } from 'fs/promises';
import { existsSync } from 'fs';
import { extname, basename } from 'path';

const VERSION = '0.3.0';

const HELP = `
AISP Validator v${VERSION} - Validate AISP 5.1 documents

Usage:
  aisp-validator <command> [file] [options]
  aisp-validator [file]              Shorthand for validate

Commands:
  validate <file>   Validate an AISP document
  tier <file>       Get quality tier (⊘ to ◊⁺⁺)
  density <file>    Get density score (δ)
  debug <file>      Show detailed density breakdown
  long <file>       Long-format output with full details
  help              Show this help message

Options:
  --long, -l        Long-format output (detailed)
  --json, -j        JSON output
  --max-size <n>    Maximum document size in KB (default: 64)
  --strict          Force WASM validation (max 1KB)

Supported File Types:
  ${SUPPORTED_EXTENSIONS.join(', ')}

Quality Tiers:
  ◊⁺⁺  Platinum   δ ≥ 0.75
  ◊⁺   Gold       δ ≥ 0.60
  ◊    Silver     δ ≥ 0.40
  ◊⁻   Bronze     δ ≥ 0.20
  ⊘    Reject     δ < 0.20

Density Formulas:
  δ (semantic) = (blockScore × 0.4) + (bindingScore × 0.6)
  ρ (pure)     = |AISP_symbols| ÷ |non_ws_tokens|

  blockScore: Required blocks present (⟦Ω⟧, ⟦Σ⟧, ⟦Γ⟧, ⟦Λ⟧, ⟦Ε⟧)
  bindingScore: Semantic operators (≜, ≔, ∀, ∃, λ, ⇒, ∈, etc.)
  pureDensity: Ratio of AISP symbols to total tokens

Examples:
  npx aisp-validator validate spec.aisp
  npx aisp-validator tier spec.aisp
  npx aisp-validator debug spec.aisp
  npx aisp-validator long spec.md --max-size 128
  npx aisp-validator validate spec.aisp --json
`;

/**
 * Parse CLI arguments
 */
function parseArgs(args) {
    const result = {
        command: 'validate',
        file: null,
        options: {
            long: false,
            json: false,
            strict: false,
            maxSize: SIZE_LIMITS.DEFAULT_MAX / 1024
        }
    };

    const commands = ['validate', 'tier', 'density', 'debug', 'long', 'help'];
    let i = 0;

    while (i < args.length) {
        const arg = args[i];

        if (arg === '--long' || arg === '-l') {
            result.options.long = true;
        } else if (arg === '--json' || arg === '-j') {
            result.options.json = true;
        } else if (arg === '--strict') {
            result.options.strict = true;
        } else if (arg === '--max-size' && args[i + 1]) {
            result.options.maxSize = parseInt(args[++i], 10) || 64;
        } else if (commands.includes(arg)) {
            result.command = arg;
        } else if (!arg.startsWith('-') && !result.file) {
            result.file = arg;
        }
        i++;
    }

    // 'long' command implies long format
    if (result.command === 'long') {
        result.options.long = true;
        result.command = 'validate';
    }

    return result;
}

/**
 * Check if file extension is supported
 */
function isExtensionSupported(file) {
    const ext = extname(file).toLowerCase();
    // Also allow files without extension (assume AISP)
    return ext === '' || SUPPORTED_EXTENSIONS.includes(ext);
}

/**
 * Format long output
 */
function formatLong(file, result, debug, fileSize) {
    const lines = [];
    lines.push(`\n${'═'.repeat(60)}`);
    lines.push(`AISP Document Validation Report`);
    lines.push(`${'═'.repeat(60)}`);
    lines.push(`\nFile: ${basename(file)}`);
    lines.push(`Size: ${(fileSize / 1024).toFixed(2)} KB`);
    lines.push(`Mode: ${result.mode || 'wasm'}`);
    lines.push(`\n${'─'.repeat(40)}`);
    lines.push(`RESULT: ${result.valid ? '✓ VALID' : '✗ INVALID'}`);
    lines.push(`${'─'.repeat(40)}`);
    lines.push(`\nQuality Tier: ${result.tier} ${result.tierName}`);
    lines.push(`\nMetrics:`);
    lines.push(`  Semantic Density (δ): ${result.delta.toFixed(4)}`);
    lines.push(`  Pure Density (ρ):     ${result.pureDensity.toFixed(4)}`);
    lines.push(`  Ambiguity:            ${result.ambiguity.toFixed(4)}`);

    if (debug) {
        lines.push(`\n${'─'.repeat(40)}`);
        lines.push(`Semantic Score Breakdown:`);
        lines.push(`  Block Score:   ${(debug.blockScore * 100).toFixed(1)}% (weight: 40%)`);
        lines.push(`  Binding Score: ${(debug.bindingScore * 100).toFixed(1)}% (weight: 60%)`);
        lines.push(`\nBlocks Found: ${debug.breakdown.blocksFound}/${debug.breakdown.blocksRequired}`);
        lines.push(`  Required: ⟦Ω⟧, ⟦Σ⟧, ⟦Γ⟧, ⟦Λ⟧, ⟦Ε⟧`);
        lines.push(`\nSemantic Operators: ${debug.breakdown.totalBindings}`);
        lines.push(`  ≜ definitions:   ${debug.breakdown.definitions}`);
        lines.push(`  ≔ assignments:   ${debug.breakdown.assignments}`);
        lines.push(`  ∀∃ quantifiers:  ${debug.breakdown.quantifiers}`);
        lines.push(`  λ lambdas:       ${debug.breakdown.lambdas}`);
        lines.push(`  ⇒⇔ implications: ${debug.breakdown.implications}`);
        lines.push(`  ∈⊆∩∪ set ops:    ${debug.breakdown.setOps}`);
        lines.push(`\nPure Density:`);
        lines.push(`  AISP Symbols: ${debug.breakdown.symbolCount}`);
        lines.push(`  Total Tokens: ${debug.breakdown.tokenCount}`);
    }

    lines.push(`\n${'═'.repeat(60)}`);
    return lines.join('\n');
}

async function main() {
    const args = process.argv.slice(2);

    if (args.length === 0 || args[0] === 'help' || args[0] === '--help' || args[0] === '-h') {
        console.log(HELP);
        process.exit(0);
    }

    if (args[0] === '--version' || args[0] === '-v') {
        console.log(`aisp-validator v${VERSION}`);
        process.exit(0);
    }

    const { command, file, options } = parseArgs(args);

    if (!file) {
        console.error('Error: No file specified');
        console.log('Usage: aisp-validator <command> <file> [options]');
        process.exit(1);
    }

    if (!existsSync(file)) {
        console.error(`Error: File not found: ${file}`);
        process.exit(1);
    }

    // Check file extension
    if (!isExtensionSupported(file)) {
        console.warn(`Warning: File extension not in supported list: ${SUPPORTED_EXTENSIONS.join(', ')}`);
        console.warn('Continuing with validation...\n');
    }

    try {
        // Initialize with max size option
        await AISP.init({ maxDocSize: options.maxSize * 1024 });

        const content = await readFile(file, 'utf-8');
        const fileStats = await stat(file);
        const fileSize = fileStats.size;

        switch (command) {
            case 'validate': {
                const result = AISP.validate(content, { strict: options.strict });
                const debug = options.long ? AISP.debug(content) : null;

                // JSON output
                if (options.json) {
                    const output = {
                        file: basename(file),
                        fileSize,
                        ...result,
                        ...(debug ? { breakdown: debug.breakdown } : {})
                    };
                    console.log(JSON.stringify(output, null, 2));
                    process.exit(result.valid ? 0 : 1);
                }

                // Long format output
                if (options.long) {
                    console.log(formatLong(file, result, debug, fileSize));
                    process.exit(result.valid ? 0 : 1);
                }

                // Standard output
                if (result.valid) {
                    console.log(`✓ VALID`);
                    console.log(`  Tier: ${result.tier} ${result.tierName}`);
                    console.log(`  Semantic (δ): ${result.delta.toFixed(3)}`);
                    console.log(`  Pure (ρ):     ${result.pureDensity.toFixed(3)}`);
                    console.log(`  Ambiguity:    ${result.ambiguity.toFixed(3)}`);
                    if (result.mode) console.log(`  Mode:         ${result.mode}`);
                    process.exit(0);
                } else {
                    console.log(`✗ INVALID`);
                    console.log(`  Error: ${result.error || `Error code ${result.errorCode}`}`);
                    if (result.tier) {
                        console.log(`  Tier: ${result.tier} ${result.tierName}`);
                        console.log(`  Semantic (δ): ${result.delta.toFixed(3)}`);
                        console.log(`  Pure (ρ):     ${result.pureDensity.toFixed(3)}`);
                    }
                    process.exit(1);
                }
                break;
            }

            case 'tier': {
                const result = AISP.validate(content);
                if (options.json) {
                    console.log(JSON.stringify({ tier: result.tier, tierName: result.tierName, tierValue: result.tierValue }));
                } else {
                    console.log(`${result.tier} ${result.tierName}`);
                }
                break;
            }

            case 'density': {
                const result = AISP.validate(content);
                if (options.json) {
                    console.log(JSON.stringify({ delta: result.delta, pureDensity: result.pureDensity }));
                } else {
                    console.log(result.delta.toFixed(4));
                }
                break;
            }

            case 'debug': {
                const debug = AISP.debug(content);

                if (options.json) {
                    console.log(JSON.stringify(debug, null, 2));
                    break;
                }

                console.log(`\nAISP Density Debug`);
                console.log(`==================`);
                console.log(`\nFile: ${basename(file)}`);
                console.log(`Size: ${(fileSize / 1024).toFixed(2)} KB`);
                console.log(`\nTier: ${debug.tier} ${debug.tierName}`);
                console.log(`Semantic (δ): ${debug.delta.toFixed(3)}`);
                console.log(`Pure (ρ):     ${debug.pureDensity.toFixed(3)}`);
                console.log(`\nSemantic Score Breakdown:`);
                console.log(`  Block Score:   ${(debug.blockScore * 100).toFixed(1)}% (weight: 40%)`);
                console.log(`  Binding Score: ${(debug.bindingScore * 100).toFixed(1)}% (weight: 60%)`);
                console.log(`\nBlocks Found: ${debug.breakdown.blocksFound}/${debug.breakdown.blocksRequired}`);
                console.log(`  Required: ⟦Ω⟧, ⟦Σ⟧, ⟦Γ⟧, ⟦Λ⟧, ⟦Ε⟧`);
                console.log(`\nSemantic Operators: ${debug.breakdown.totalBindings}`);
                console.log(`  ≜ definitions:  ${debug.breakdown.definitions}`);
                console.log(`  ≔ assignments:  ${debug.breakdown.assignments}`);
                console.log(`  ∀∃ quantifiers: ${debug.breakdown.quantifiers}`);
                console.log(`  λ lambdas:      ${debug.breakdown.lambdas}`);
                console.log(`  ⇒⇔ implications: ${debug.breakdown.implications}`);
                console.log(`  ∈⊆∩∪ set ops:   ${debug.breakdown.setOps}`);
                console.log(`\nPure Density Breakdown:`);
                console.log(`  AISP Symbols: ${debug.breakdown.symbolCount}`);
                console.log(`  Total Tokens: ${debug.breakdown.tokenCount}`);
                console.log(`\nFormulas:`);
                console.log(`  δ = (${debug.blockScore.toFixed(2)} × 0.4) + (${debug.bindingScore.toFixed(2)} × 0.6) = ${debug.delta.toFixed(3)}`);
                console.log(`  ρ = ${debug.breakdown.symbolCount} ÷ ${debug.breakdown.tokenCount} = ${debug.pureDensity.toFixed(3)}`);
                break;
            }
        }
    } catch (err) {
        console.error(`Error: ${err.message}`);
        process.exit(1);
    }
}

main();
