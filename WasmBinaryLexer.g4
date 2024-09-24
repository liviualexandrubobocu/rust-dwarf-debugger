lexer grammar WasmBinaryLexer;

// Existing tokens for numtype
NUMTYPE_I32      : '\x7F';
NUMTYPE_I64      : '\x7E';
NUMTYPE_F32      : '\x7D';
NUMTYPE_F64      : '\x7C';

// New tokens for vectype
VECTYPE_V128     : '\x7B';

// New tokens for reftype
REFTYPE_FUNCREF     : '\x70';
REFTYPE_EXTERNREF   : '\x6F';

// BYTE: Matches any single byte (0x00 to 0xFF)
BYTE
    : .
    ;
