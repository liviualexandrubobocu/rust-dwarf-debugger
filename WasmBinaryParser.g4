parser grammar WasmBinaryParser;

options { tokenVocab=WasmBinaryLexer; }

numtype
    : NUMTYPE_I32    # NumtypeI32
    | NUMTYPE_I64    # NumtypeI64
    | NUMTYPE_F32    # NumtypeF32
    | NUMTYPE_F64    # NumtypeF64
    ;

limits
    : LIMITS_TYPE0 U32                  # LimitsMin
    | LIMITS_TYPE1 U32 U32              # LimitsMinMax
    ;

vec
    : U32 element+=elementType*         # Vector
      {
        if ($element.size() != Integer.parseInt($U32.text)) {
          throw new RuntimeException("Vector length does not match the specified count.");
        }
      }
    ;

elementType
    : /* parser rule for 'B' */
    ;


// Parser rules

// Byte: matches any single byte (0x00 to 0xFF)
byte
    : BYTE
    ;

// Unsigned integer (uN): LEB128-encoded unsigned integer of N bits
uN
    : lebUnsigned[N]       # UnsignedInteger
    ;

// Signed integer (sN): LEB128-encoded signed integer of N bits
sN
    : lebSigned[N]         # SignedInteger
    ;

// Uninterpreted integer (iN): sN interpreted as iN
iN
    : sN                   # UninterpretedInteger
    ;

// Floating-point value (fN): N-bit IEEE 754 floating-point number
fN
    : BYTE_SEQUENCE[N / 8] # FloatingPointValue
    ;

// Name: UTF-8 encoded string
name
    : vec(byte)            # Name
      {
        // Convert byte vector to UTF-8 string
        String utf8String = bytesToUtf8($byteList);
        // Handle invalid UTF-8 encoding if necessary
      }
    ;

// Vector: generic vector of elements
vec[elementType]
    : u32 elements+=elementType*
      {
        if ($elements.size() != $u32.value) {
          throw new RuntimeException("Vector length does not match the specified count.");
        }
      }
    ;

// u32: 32-bit unsigned integer (LEB128-encoded)
u32
    : lebUnsigned[32]      # U32
    ;

// Lexer rules

// BYTE: Matches any single byte (0x00 to 0xFF)
BYTE
    : .                    // Matches any single byte
    ;

// Lexer modes and helper rules for LEB128 encoding

// LEB128-encoded unsigned integer
fragment lebUnsigned[int N]
    :   { int result = 0; int shift = 0; }
        (   lebByteUnsigned[$result, $shift]
        )+
        {
            // Set the value property
            $value = result;
        }
    ;

// LEB128-encoded signed integer
fragment lebSigned[int N]
    :   { int result = 0; int shift = 0; }
        (   lebByteSigned[$result, $shift]
        )+
        {
            // Set the value property
            $value = result;
        }
    ;

// Helper lexer rules for LEB128 bytes

fragment lebByteUnsigned[int result, int shift]
    :   BYTE
        {
            int byteValue = (int) $BYTE.text.charAt(0) & 0xFF;
            result |= (byteValue & 0x7F) << shift;
            shift += 7;
            if ((byteValue & 0x80) == 0) {
                $channel = HIDDEN; // Consume the byte
                $value = result;
            }
        }
    ;

fragment lebByteSigned[int result, int shift]
    :   BYTE
        {
            int byteValue = (int) $BYTE.text.charAt(0) & 0xFF;
            result |= (byteValue & 0x7F) << shift;
            shift += 7;
            boolean isLastByte = (byteValue & 0x80) == 0;
            if (isLastByte) {
                // Sign extend if negative
                if ((byteValue & 0x40) != 0 && shift < N) {
                    result |= - (1 << shift);
                }
                $channel = HIDDEN; // Consume the byte
                $value = result;
            }
        }
    ;

// BYTE_SEQUENCE: Matches a sequence of N bytes
fragment BYTE_SEQUENCE[int count]
    :   (BYTE){count}
    ;

// Channels

channels { HIDDEN }

// Tokens


