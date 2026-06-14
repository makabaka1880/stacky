/* tslint:disable */
/* eslint-disable */

/**
 * A Stacky program that can be executed incrementally.
 *
 * JS usage:
 * ```js
 * const prog = new Program("(begin (push a) (push b) (squish pair 2))");
 * while (!prog.done()) {
 *     prog.step();
 *     console.log(prog.stack_len(), prog.peek(0));
 * }
 * const tree = prog.run_all(); // one-shot alternative
 * ```
 */
export class Program {
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Parse a Stacky source program. Throws a JS error on parse failure.
     */
    constructor(source: string);
    /**
     * Peek at the stack from the top: depth 0 = top, depth 1 = one below, etc.
     * Returns the tree as a JS object, or `undefined` if depth is out of bounds.
     */
    peek(depth: number): any;
    /**
     * Run the program to completion and return the top-of-stack tree as a JS object.
     * Throws on runtime errors or if the stack is empty when the program finishes.
     */
    run_all(): any;
    /**
     * Number of items currently on the stack.
     */
    stack_len(): number;
    /**
     * Execute a single statement. Returns `true` when the program is done,
     * `false` when more steps remain. Throws on runtime errors.
     */
    step(): boolean;
    /**
     * Has the program exhausted all statements?
     */
    readonly done: boolean;
}

/**
 * Convenience: parse source, run to completion, return the result tree as a debug string.
 * Prefer the `Program` class for interactive use.
 */
export function get_top(source: string): string;
