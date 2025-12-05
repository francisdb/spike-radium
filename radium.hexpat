// Use this with ImHex to reverse engineer Radium files
// tested with lcd/auto_loaded/9db420cfd5ec648153ce2e0f17dfcbc3a6b43df6/scene.radium

#pragma description stern radium file
#pragma endian little

struct Header {
    u8 header[13];
};

struct String {
    u64 string_size;
    char array[string_size];
};

struct Identifier {
    u16 id; // increments
    padding[1];
    u8 end; // always 0x80
};

struct Bitmap {
    Identifier id;
    u8 unknown2[12];
    u32 w1;
    u32 h1;
    Identifier second2;
    u32 w2;
    u32 h2;
    u8 unknown3[12];
    u32 l; // l == w1*h1
    u8 unknown4[8];
    u8 data[w1*h1];
};

struct Radium {
    Header header;
    Identifier first;
    String type;
    Bitmap bitmap;
    // not a whole bitmap
    Bitmap bitmap2;
    Identifier next;
    String type2;
};



Radium radium @ 0x00;
