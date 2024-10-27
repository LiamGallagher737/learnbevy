// From: https://github.com/rust-lang/rust-playground/blob/02674fc4043fea162164f54e5c8918aa63f68c4f/ui/frontend/editor/MonacoEditorCore.tsx

export const languageExtension = {
    id: "rust-plus",
};

export const languageConfig = {
    comments: {
        lineComment: "//",
        blockComment: ["/*", "*/"],
    },
    brackets: [
        ["{", "}"],
        ["[", "]"],
        ["(", ")"],
    ],
    autoClosingPairs: [
        { open: "[", close: "]" },
        { open: "{", close: "}" },
        { open: "(", close: ")" },
        { open: '"', close: '"', notIn: ["string"] },
    ],
    surroundingPairs: [
        { open: "{", close: "}" },
        { open: "[", close: "]" },
        { open: "(", close: ")" },
        { open: '"', close: '"' },
        { open: "'", close: "'" },
    ],
    folding: {
        markers: {
            start: new RegExp("^\\s*#pragma\\s+region\\b"),
            end: new RegExp("^\\s*#pragma\\s+endregion\\b"),
        },
    },
};

export const grammar = {
    keywords: [
        "as", "break", "const", "crate", "enum", "extern", "false", "fn", "impl", "in", "let", "mod", "move", 
        "mut", "pub", "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type", 
        "unsafe", "use", "where", "macro_rules",
    ],
    controlFlowKeywords: ["continue", "else", "for", "if", "while", "loop", "match"],
    typeKeywords: [
        "Self", "m32", "m64", "m128", "f80", "f16", "f128", "int", "uint", "float", "char", "bool", 
        "u8", "u16", "u32", "u64", "f32", "f64", "i8", "i16", "i32", "i64", "str", "Option", "Either", 
        "c_float", "c_double", "c_void", "FILE", "fpos_t", "DIR", "dirent", "c_char", "c_schar", "c_uchar", 
        "c_short", "c_ushort", "c_int", "c_uint", "c_long", "c_ulong", "size_t", "ptrdiff_t", "clock_t", 
        "time_t", "c_longlong", "c_ulonglong", "intptr_t", "uintptr_t", "off_t", "dev_t", "ino_t", 
        "pid_t", "mode_t", "ssize_t",
    ],
    operators: [
        "=", ">", "<", "!", "~", "?", ":", "==", "<=", ">=", "!=", "&&", "||", "++", "--", "+", "-", "*", "/", 
        "&", "|", "^", "%", "<<", ">>", ">>>", "+=", "-=", "*=", "/=", "&=", "|=", "^=", "%=", "<<=", ">>=", ">>>=",
    ],
    symbols: /[=><!~?:&|+\-*\/\^%]+/,
    escapes: /\\(?:[abfnrtv\\"']|x[0-9A-Fa-f]{1,4}|u[0-9A-Fa-f]{4}|U[0-9A-Fa-f]{8})/,
    tokenizer: {
        root: [
            [/r"/, { token: "string.quote", next: "@rawstring0" }],
            [/r(#+)"/, { token: "string.quote", next: "@rawstring1.$1" }],
            [
                /[a-z_$][\w$]*/,
                {
                    cases: {
                        "@typeKeywords": "type.identifier",
                        "@keywords": {
                            cases: {
                                fn: { token: "keyword", next: "@func_decl" },
                                "@default": "keyword",
                            },
                        },
                        "@controlFlowKeywords": "keyword.control",
                        "@default": "variable",
                    },
                },
            ],
            [/[A-Z][\w\$]*/, "type.identifier"],
            { include: "@whitespace" },
            [/[{}()\[\]]/, "@brackets"],
            [/[<>](?!@symbols)/, "@brackets"],
            [
                /@symbols/,
                {
                    cases: {
                        "@operators": "operator",
                        "@default": "",
                    },
                },
            ],
            [/@\s*[a-zA-Z_\$][\w\$]*/, { token: "annotation", log: "annotation token: $0" }],
            [/\d*\.\d+([eE][\-+]?\d+)?/, "number.float"],
            [/0[xX][0-9a-fA-F]+/, "number.hex"],
            [/\d+/, "number"],
            [/[;,.]/, "delimiter"],
            [/"([^"\\]|\\.)*$/, "string.invalid"], // non-teminated string
            [/"/, { token: "string.quote", bracket: "@open", next: "@string" }],
            [/'[^\\']'/, "string"],
            [/(')(@escapes)(')/, ["string", "string.escape", "string"]],
            [/'/, "string.invalid"],
        ],

        comment: [
            [/[^\/*]+/, "comment"],
            [/\/\*/, "comment", "@push"],
            ["\\*/", "comment", "@pop"],
            [/[\/*]/, "comment"],
        ],

        rawstring0: [
            [/[^"]+/, "string"],
            [/"/, { token: "string.quote", next: "@pop" }],
        ],
        rawstring1: [
            [
                /"(#+)/,
                {
                    cases: {
                        "$1==$S2": { token: "string.quote", next: "@pop" },
                        "@default": { token: "string" },
                    },
                },
            ],
            [/./, "string"],
        ],
        string: [
            [/[^\\"]+/, "string"],
            [/@escapes/, "string.escape"],
            [/\\./, "string.escape.invalid"],
            [/"/, { token: "string.quote", bracket: "@close", next: "@pop" }],
        ],
        whitespace: [
            [/[ \t\r\n]+/, "white"],
            [/\/\*/, "comment", "@comment"],
            [/\/\/.*$/, "comment"],
        ],
        func_decl: [[/[a-z_$][\w$]*/, "support.function", "@pop"]],
    },
};

export const themeVsDarkPlus = {
    base: "vs-dark",
    inherit: true,
    colors: {
        "editor.background": "#1c1917",
    },
    rules: [
        { token: "keyword.control", foreground: "C586C0" },
        { token: "string.escape", foreground: "D7BA7D" },
        { token: "keyword.controlFlow", foreground: "C586C0" },
        { token: "variable", foreground: "9CDCFE" },
        { token: "parameter", foreground: "9CDCFE" },
        { token: "property", foreground: "9CDCFE" },
        { token: "support.function", foreground: "DCDCAA" },
        { token: "function", foreground: "DCDCAA" },
        { token: "member", foreground: "4FC1FF" },
        { token: "variable.constant", foreground: "4FC1FF" },
        { token: "macro", foreground: "569CD6" },
        { token: "typeParameter", foreground: "4EC9B0" },
        { token: "interface", foreground: "4EC9B0" },
        { token: "namespace", foreground: "4EC9B0" },
        { token: "variable.mutable", fontStyle: "underline" },
        { token: "parameter.mutable", fontStyle: "underline" },
        { token: "comment", foreground: "737373" },
    ],
};

