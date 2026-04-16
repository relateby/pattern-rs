// Fallback type declarations for the wasm-pack generated modules.
// The actual files are generated at build time by `npm run build:wasm`
// and `npm run build:wasm:node`.

declare module "../wasm/pattern_wasm.js" {
  /**
   * Gram namespace for JSON-based parsing and serialization.
   */
  export class Gram {
    private constructor()
    free(): void
    [Symbol.dispose](): void
    static parseToJson(gram: string): string
    static stringifyFromJson(json: string): string
    static validate(gram: string): Array<any>
  }

  /**
   * Result of parsing gram notation
   */
  export class ParseResult {
    private constructor()
    free(): void
    [Symbol.dispose](): void
    readonly identifiers: string[]
    readonly pattern_count: number
  }

  export function parse_gram(input: string): ParseResult
  export function parse_to_ast(input: string): any
  export function round_trip(input: string): string
  export function validate_gram(input: string): boolean
  export function version(): string
}

declare module "../wasm-node/pattern_wasm.js" {
  /**
   * Gram namespace for JSON-based parsing and serialization.
   */
  export class Gram {
    private constructor()
    free(): void
    [Symbol.dispose](): void
    static parseToJson(gram: string): string
    static stringifyFromJson(json: string): string
    static validate(gram: string): Array<any>
  }

  /**
   * Result of parsing gram notation
   */
  export class ParseResult {
    private constructor()
    free(): void
    [Symbol.dispose](): void
    readonly identifiers: string[]
    readonly pattern_count: number
  }

  export function parse_gram(input: string): ParseResult
  export function parse_to_ast(input: string): any
  export function round_trip(input: string): string
  export function validate_gram(input: string): boolean
  export function version(): string
}
