/**
 * Z3 Integration Tests for AISP Validator
 */

import { describe, it } from 'node:test';
import assert from 'node:assert';
import AISP from '../src/index.js';
import { AispZ3Translator, validateWithZ3 } from '../src/z3-bridge.js';

describe('Z3 Integration', () => {
    describe('AispZ3Translator', () => {
        it('should parse universal quantifier with iff', () => {
            const translator = new AispZ3Translator();
            const constructs = translator.parseRulesBlock('∀move:ValidMove(board,pos)⇔board[pos]=Empty');
            
            assert.strictEqual(constructs.length, 1);
            assert.strictEqual(constructs[0].type, 'forall');
            assert.strictEqual(constructs[0].variable, 'move');
            assert.strictEqual(constructs[0].leftSide, 'ValidMove(board,pos)');
            assert.strictEqual(constructs[0].rightSide, 'board[pos]=Empty');
        });

        it('should parse existential quantifier', () => {
            const translator = new AispZ3Translator();
            const constructs = translator.parseRulesBlock('∀win:WinCondition⇔∃line∈Lines:∀c∈line:c=player');
            
            assert.strictEqual(constructs.length, 1);
            assert.strictEqual(constructs[0].type, 'forall');
            assert.ok(constructs[0].rightSide.includes('∃line∈Lines'));
        });

        it('should parse definitions', () => {
            const translator = new AispZ3Translator();
            const constructs = translator.parseRulesBlock('Lines≜{rows,cols,diags}');
            
            assert.strictEqual(constructs.length, 1);
            assert.strictEqual(constructs[0].type, 'definition');
            assert.strictEqual(constructs[0].name, 'Lines');
            assert.strictEqual(constructs[0].definition, '{rows,cols,diags}');
        });

        it('should generate valid SMT-LIB', () => {
            const translator = new AispZ3Translator();
            const constructs = [
                {
                    type: 'forall',
                    variable: 'x',
                    domain: 'Nat',
                    leftSide: 'P(x)',
                    rightSide: 'Q(x)'
                }
            ];
            
            const smtlib = translator.generateSMTLib(constructs);
            
            assert.ok(smtlib.includes('(declare-sort'));
            assert.ok(smtlib.includes('(assert (forall'));
            assert.ok(smtlib.includes('(check-sat)'));
            assert.ok(smtlib.includes('(exit)'));
        });

        it('should convert AISP predicates to Z3 format', () => {
            const translator = new AispZ3Translator();
            
            assert.strictEqual(
                translator.convertPredicate('board[pos]=Empty'),
                '(= (select board pos) Empty)'
            );
            
            assert.strictEqual(
                translator.convertPredicate('x∈S'),
                '(member x S)'
            );
            
            assert.strictEqual(
                translator.convertPredicate('A⊆B'),
                '(subset A B)'
            );
        });
    });

    describe('AISP Validator with Z3', () => {
        const testDoc = `𝔸1.0.test@2026-01-16
γ≔test

⟦Ω:Meta⟧{
  ∀D∈AISP:Ambig(D)<0.02
}

⟦Σ:Types⟧{
  Player≜{X,O}
  Cell≜{Empty,X,O}
}

⟦Γ:Rules⟧{
  ∀move:ValidMove(board,pos)⇔board[pos]=Empty
  Lines≜{rows,cols,diags}
}

⟦Λ:Funcs⟧{
  makeMove≜λ(board,pos,player).board[pos]←player
}

⟦Ε⟧⟨δ≜0.75;τ≜◊⁺⁺⟩`;

        it('should validate normally without Z3', async () => {
            await AISP.init();
            const result = AISP.validate(testDoc);
            
            assert.strictEqual(result.valid, true);
            assert.strictEqual(result.tier, '◊⁺⁺');
            assert.ok(result.delta >= 0.75);
        });

        it('should provide Z3 option info in sync validation', async () => {
            await AISP.init();
            const result = AISP.validate(testDoc, { z3: true });
            
            assert.strictEqual(result.valid, true);
            assert.ok(result.z3);
            assert.strictEqual(result.z3.available, false);
            assert.ok(result.z3.message.includes('validateAsync'));
        });

        it('should attempt Z3 validation in async mode', async () => {
            await AISP.init();
            const result = await AISP.validateAsync(testDoc, { z3: true });
            
            assert.strictEqual(result.valid, true);
            assert.ok(result.z3);
            
            // Z3 may not be installed, but we should get a result
            if (result.z3.available) {
                assert.ok(typeof result.z3.satisfiable === 'boolean');
                assert.ok(typeof result.z3.constructs === 'number');
            } else {
                assert.ok(result.z3.error || result.z3.warning);
            }
        });

        it('should generate SMT-LIB in debug mode', async () => {
            const rulesContent = `
  ∀move:ValidMove(board,pos)⇔board[pos]=Empty
  Lines≜{rows,cols,diags}
            `.trim();
            
            try {
                const result = await validateWithZ3(`⟦Γ:Rules⟧{${rulesContent}}`, { debug: true });
                
                if (result.smtlib) {
                    assert.ok(result.smtlib.includes('(declare-sort'));
                    assert.ok(result.smtlib.includes('(assert'));
                }
            } catch (error) {
                // Z3 might not be available, which is fine for this test
                assert.ok(error.message.includes('Z3'));
            }
        });
    });

    describe('Edge Cases', () => {
        it('should handle documents without rules blocks', async () => {
            const docWithoutRules = `𝔸1.0.test@2026-01-16
⟦Ω:Meta⟧{ Test≜true }
⟦Σ:Types⟧{ T≜ℕ }
⟦Λ:Funcs⟧{ f≜λx.x }
⟦Ε⟧⟨δ≜0.5;τ≜◊⟩`;

            const result = await validateWithZ3(docWithoutRules);
            assert.strictEqual(result.valid, false);
            assert.ok(result.error.includes('No rules block'));
        });

        it('should handle empty rules blocks', async () => {
            const result = await validateWithZ3('⟦Γ:Rules⟧{}');
            assert.strictEqual(result.valid, true);
            assert.strictEqual(result.constructs, 0);
            assert.ok(result.warning.includes('No formal constructs'));
        });

        it('should handle malformed rules', () => {
            const translator = new AispZ3Translator();
            const constructs = translator.parseRulesBlock('invalid syntax here');
            assert.strictEqual(constructs.length, 0);
        });
    });
});