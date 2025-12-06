// Use this with ImHex to reverse engineer Radium files

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
    u8 unknown[12];
    u32 w1;
    u32 h1;
    Identifier second2;
    u32 w2;
    u32 h2;
    u8 unknown3[12];
    u32 l; // l == w1*h1
    u8 unknown4[8];
    u8 data[w1*h1] [[sealed, hex::visualize("image", this)]];
};

struct FontPart{
    Identifier id;
    u8 unknown[59];
};

struct Font{
    Identifier id;
    u8 unknown[12];
    String name;
    u16 x;
    u64 y;
    u16 chars[y];
    u8 unknown2[12];
    Identifier id2;
    u8 unknown3[27];
    Identifier id3;
    u8 unknown4[45];
    Identifier id4;
    u8 unknown5[0x000201CE - 0x000001AC];
    FontPart parts[127];
    u8 unknown6[2];
    Identifier id132;
    u8 unknown7[27];
    Identifier id133;
    u8 unknown8[45];
    Identifier id134;
    u8 unknown9[0x000201CE - 0x000001AC];
    FontPart parts2[102];
    //u8 unknown10[2];
    Identifier id237;
    u8 unknown10[45];
    Identifier id238;
    u8 unknown11[0x00063AF8 - 0x00043AD6];
    FontPart parts3[23];
    Identifier id262;
    u8 unknown12[69];
    Identifier id002;
    String type;
    Identifier id263;
    u8 unknown13[58];
    String type2;
    u8 unknown13a[29]; 
    Identifier id003; 
    String type3;
    Identifier id264;
    u8 unknown14[24];
    Identifier id265;
    u8 unknown15[135];
    Identifier id266;
    u8 unknown16[128];
    Identifier id267;
    
};

struct Radium {
    Header header;
    Identifier first;
    String type;
    Font font;
    //Bitmap bitmap;
    // not a whole bitmap
    //Bitmap bitmap2;
    //Identifier next;
    //String type2;
};



Radium radium @ 0x00;