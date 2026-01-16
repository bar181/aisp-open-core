/**
 * AISP WASM Kernel - C/C++ Header
 * For chip integration (ESP32, RP2040, etc.)
 */

#ifndef AISP_H
#define AISP_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/* ============================================================================
 * Core API
 * ============================================================================ */

/**
 * Initialize AISP kernel
 * Must be called before any other functions.
 * @return 0 on success, <0 on error
 */
int32_t aisp_init(void);

/**
 * Parse AISP document from memory
 * @param ptr Pointer to UTF-8 encoded AISP source
 * @param len Length of source in bytes
 * @return Document ID (0) on success, <0 on error
 */
int32_t aisp_parse(const uint8_t* ptr, uint32_t len);

/**
 * Validate parsed document
 * @param doc_id Document ID from aisp_parse
 * @return 0 if valid, <0 on error
 */
int32_t aisp_validate(int32_t doc_id);

/**
 * Get quality tier
 * @param doc_id Document ID
 * @return Tier value (0-4)
 */
int32_t aisp_tier(int32_t doc_id);

/**
 * Get ambiguity score
 * @param doc_id Document ID
 * @return Ambiguity [0.0, 1.0]
 */
float aisp_ambig(int32_t doc_id);

/**
 * Get density score (δ)
 * @param doc_id Document ID
 * @return Density [0.0, 1.0]
 */
float aisp_density(int32_t doc_id);

/* ============================================================================
 * Error Handling
 * ============================================================================ */

/**
 * Get last error code
 * @return Error code (0 = no error)
 */
int32_t aisp_error_code(void);

/**
 * Get error offset in input
 * @return Byte offset where error occurred
 */
uint32_t aisp_error_offset(void);

/* ============================================================================
 * Constants
 * ============================================================================ */

/* Quality Tiers */
#define AISP_TIER_REJECT    0  /* ⊘: δ < 0.20 */
#define AISP_TIER_BRONZE    1  /* ◊⁻: δ ≥ 0.20 */
#define AISP_TIER_SILVER    2  /* ◊: δ ≥ 0.40 */
#define AISP_TIER_GOLD      3  /* ◊⁺: δ ≥ 0.60 */
#define AISP_TIER_PLATINUM  4  /* ◊⁺⁺: δ ≥ 0.75 */

/* Error Codes */
#define AISP_OK             0   /* Success */
#define AISP_ERR_PARSE     -1   /* Parse error */
#define AISP_ERR_TYPE      -2   /* Type error */
#define AISP_ERR_AMBIG     -3   /* Ambiguity too high */
#define AISP_ERR_MEMORY    -4   /* Memory error */
#define AISP_ERR_OVERFLOW  -5   /* Buffer overflow */

/* Binding States (Δ⊗λ) */
#define AISP_BIND_CRASH     0   /* ⊥: Logic conflict */
#define AISP_BIND_NULL      1   /* ∅: Socket mismatch */
#define AISP_BIND_ADAPT     2   /* λ: Type mismatch */
#define AISP_BIND_ZERO      3   /* ⊤: Full compatibility */

/* Limits */
#define AISP_MAX_DOC_SIZE   1024  /* Maximum document size in bytes */
#define AISP_MAX_TERMS      128   /* Maximum unique terms */
#define AISP_MAX_DEPTH      32    /* Maximum context depth */

#ifdef __cplusplus
}
#endif

#endif /* AISP_H */
