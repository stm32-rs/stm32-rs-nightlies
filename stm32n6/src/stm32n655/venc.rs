#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    swreg0: SWREG0,
    swreg1: SWREG1,
    swreg2: SWREG2,
    swreg3: SWREG3,
    _reserved4: [u8; 0x04],
    swreg5: SWREG5,
    swreg6: SWREG6,
    swreg7: SWREG7,
    swreg8: SWREG8,
    swreg9: SWREG9,
    swreg10: SWREG10,
    swreg11: SWREG11,
    swreg12: SWREG12,
    swreg13: SWREG13,
    swreg14: SWREG14,
    swreg15: SWREG15,
    swreg16: SWREG16,
    swreg17: SWREG17,
    swreg18: SWREG18,
    swreg19: SWREG19,
    swreg20: SWREG20,
    swreg21: SWREG21,
    swreg22: SWREG22,
    swreg23: SWREG23,
    swreg24: SWREG24,
    swreg25: SWREG25,
    swreg26: SWREG26,
    swreg27: SWREG27,
    swreg28: SWREG28,
    swreg29: SWREG29,
    swreg30: SWREG30,
    swreg31: SWREG31,
    swreg32: SWREG32,
    swreg33: SWREG33,
    swreg34: SWREG34,
    swreg35: SWREG35,
    swreg36: SWREG36,
    swreg37: SWREG37,
    swreg38: SWREG38,
    swreg39: SWREG39,
    swreg40: SWREG40,
    swreg41: SWREG41,
    swreg42: SWREG42,
    swreg43: SWREG43,
    swreg44: SWREG44,
    swreg45: SWREG45,
    swreg46: SWREG46,
    swreg47: SWREG47,
    swreg48: SWREG48,
    swreg49: SWREG49,
    swreg50: SWREG50,
    swreg51: SWREG51,
    swreg52: SWREG52,
    swreg53: SWREG53,
    swreg54: SWREG54,
    swreg55: SWREG55,
    swreg56: SWREG56,
    swreg57: SWREG57,
    swreg58: SWREG58,
    swreg59: SWREG59,
    swreg60: SWREG60,
    swreg61: SWREG61,
    swreg62: SWREG62,
    swreg63: SWREG63,
    swreg64: SWREG64,
    swreg65: SWREG65,
    swreg66: SWREG66,
    swreg67: SWREG67,
    swreg68: SWREG68,
    swreg69: SWREG69,
    swreg70: SWREG70,
    swreg71: SWREG71,
    swreg72: SWREG72,
    swreg73: SWREG73,
    swreg74: SWREG74,
    swreg75: SWREG75,
    swreg76: SWREG76,
    swreg77: SWREG77,
    swreg78: SWREG78,
    swreg79: SWREG79,
    swreg80: SWREG80,
    swreg81: SWREG81,
    swreg82: SWREG82,
    swreg83: SWREG83,
    swreg84: SWREG84,
    swreg85: SWREG85,
    swreg86: SWREG86,
    swreg87: SWREG87,
    swreg88: SWREG88,
    swreg89: SWREG89,
    swreg90: SWREG90,
    swreg91: SWREG91,
    swreg92: SWREG92,
    swreg93: SWREG93,
    swreg94: SWREG94,
    swreg95: SWREG95,
    swreg96: SWREG96,
    swreg97: SWREG97,
    swreg98: SWREG98,
    swreg99: SWREG99,
    swreg100: SWREG100,
    swreg101: SWREG101,
    swreg102: SWREG102,
    swreg103: SWREG103,
    swreg104: SWREG104,
    swreg105: SWREG105,
    swreg106: SWREG106,
    swreg107: SWREG107,
    swreg108: SWREG108,
    swreg109: SWREG109,
    swreg110: SWREG110,
    swreg111: SWREG111,
    swreg112: SWREG112,
    swreg113: SWREG113,
    swreg114: SWREG114,
    swreg115: SWREG115,
    swreg116: SWREG116,
    swreg117: SWREG117,
    swreg118: SWREG118,
    swreg119: SWREG119,
    swreg120: SWREG120,
    swreg121: SWREG121,
    swreg122: SWREG122,
    swreg123: SWREG123,
    swreg124: SWREG124,
    swreg125: SWREG125,
    swreg126: SWREG126,
    swreg127: SWREG127,
    swreg128: SWREG128,
    swreg129: SWREG129,
    swreg130: SWREG130,
    swreg131: SWREG131,
    swreg132: SWREG132,
    swreg133: SWREG133,
    swreg134: SWREG134,
    swreg135: SWREG135,
    swreg136: SWREG136,
    swreg137: SWREG137,
    swreg138: SWREG138,
    swreg139: SWREG139,
    swreg140: SWREG140,
    swreg141: SWREG141,
    swreg142: SWREG142,
    swreg143: SWREG143,
    swreg144: SWREG144,
    swreg145: SWREG145,
    swreg146: SWREG146,
    swreg147: SWREG147,
    swreg148: SWREG148,
    swreg149: SWREG149,
    swreg150: SWREG150,
    swreg151: SWREG151,
    swreg152: SWREG152,
    swreg153: SWREG153,
    swreg154: SWREG154,
    swreg155: SWREG155,
    swreg156: SWREG156,
    swreg157: SWREG157,
    swreg158: SWREG158,
    swreg159: SWREG159,
    _reserved159: [u8; 0x011c],
    swreg231: SWREG231,
    swreg232: SWREG232,
    swreg233: SWREG233,
    _reserved162: [u8; 0x08],
    swreg236: SWREG236,
    swreg237: SWREG237,
    swreg238: SWREG238,
    _reserved165: [u8; 0x44],
    swreg256: SWREG256,
    swreg257: SWREG257,
    swreg258: SWREG258,
    swreg259: SWREG259,
    swreg260: SWREG260,
    swreg261: SWREG261,
    swreg262: SWREG262,
    swreg263: SWREG263,
    swreg264: SWREG264,
    swreg265: SWREG265,
    swreg266: SWREG266,
    swreg267: SWREG267,
    swreg268: SWREG268,
    swreg269: SWREG269,
    swreg270: SWREG270,
    swreg271: SWREG271,
    swreg272: SWREG272,
    swreg273: SWREG273,
    swreg274: SWREG274,
    swreg275: SWREG275,
    swreg276: SWREG276,
    swreg277: SWREG277,
    swreg278: SWREG278,
    swreg279: SWREG279,
    swreg280: SWREG280,
    swreg281: SWREG281,
    swreg282: SWREG282,
    swreg283: SWREG283,
    swreg284: SWREG284,
    swreg285: SWREG285,
    swreg286: SWREG286,
    swreg287: SWREG287,
    swreg288: SWREG288,
    swreg289: SWREG289,
    swreg290: SWREG290,
    swreg291: SWREG291,
    _reserved201: [u8; 0x08],
    swreg294: SWREG294,
    swreg295: SWREG295,
    swreg296: SWREG296,
    swreg297: SWREG297,
    swreg298: SWREG298,
    swreg299: SWREG299,
    swreg300: SWREG300,
    swreg301: SWREG301,
    swreg302: SWREG302,
    swreg303: SWREG303,
    swreg304: SWREG304,
    swreg305: SWREG305,
    swreg306: SWREG306,
    swreg307: SWREG307,
    swreg308: SWREG308,
    swreg309: SWREG309,
    swreg310: SWREG310,
    swreg311: SWREG311,
    swreg312: SWREG312,
    swreg313: SWREG313,
    swreg314: SWREG314,
    swreg315: SWREG315,
    swreg316: SWREG316,
    swreg317: SWREG317,
    swreg318: SWREG318,
    swreg319: SWREG319,
    swreg320: SWREG320,
    swreg321: SWREG321,
    swreg322: SWREG322,
    swreg323: SWREG323,
    swreg324: SWREG324,
    swreg325: SWREG325,
    swreg326: SWREG326,
    swreg327: SWREG327,
    swreg328: SWREG328,
    swreg329: SWREG329,
    swreg330: SWREG330,
    swreg331: SWREG331,
    swreg332: SWREG332,
    swreg333: SWREG333,
    swreg334: SWREG334,
    swreg335: SWREG335,
    swreg336: SWREG336,
    swreg337: SWREG337,
    swreg338: SWREG338,
    swreg339: SWREG339,
    swreg340: SWREG340,
    swreg341: SWREG341,
    swreg342: SWREG342,
    swreg343: SWREG343,
    swreg344: SWREG344,
    swreg345: SWREG345,
    swreg346: SWREG346,
    swreg347: SWREG347,
    swreg348: SWREG348,
    swreg349: SWREG349,
    swreg350: SWREG350,
    swreg351: SWREG351,
    swreg352: SWREG352,
    swreg353: SWREG353,
    swreg354: SWREG354,
    swreg355: SWREG355,
    swreg356: SWREG356,
    swreg357: SWREG357,
    swreg358: SWREG358,
    swreg359: SWREG359,
    swreg360: SWREG360,
    swreg361: SWREG361,
    swreg362: SWREG362,
    swreg363: SWREG363,
    swreg364: SWREG364,
    swreg365: SWREG365,
    swreg366: SWREG366,
    swreg367: SWREG367,
    swreg368: SWREG368,
    swreg369: SWREG369,
    swreg370: SWREG370,
    swreg371: SWREG371,
    swreg372: SWREG372,
    swreg373: SWREG373,
    swreg374: SWREG374,
    swreg375: SWREG375,
    swreg376: SWREG376,
    swreg377: SWREG377,
    swreg378: SWREG378,
    swreg379: SWREG379,
    swreg380: SWREG380,
    swreg381: SWREG381,
    swreg382: SWREG382,
    swreg383: SWREG383,
    swreg384: SWREG384,
    swreg385: SWREG385,
    swreg386: SWREG386,
    swreg387: SWREG387,
    swreg388: SWREG388,
    swreg389: SWREG389,
    swreg390: SWREG390,
    swreg391: SWREG391,
    swreg392: SWREG392,
    swreg393: SWREG393,
    swreg394: SWREG394,
    swreg395: SWREG395,
    swreg396: SWREG396,
    swreg397: SWREG397,
    swreg398: SWREG398,
    swreg399: SWREG399,
    swreg400: SWREG400,
    swreg401: SWREG401,
    swreg402: SWREG402,
    swreg403: SWREG403,
    swreg404: SWREG404,
    swreg405: SWREG405,
    swreg406: SWREG406,
    swreg407: SWREG407,
    swreg408: SWREG408,
    swreg409: SWREG409,
    swreg410: SWREG410,
    swreg411: SWREG411,
    swreg412: SWREG412,
    swreg413: SWREG413,
    swreg414: SWREG414,
    swreg415: SWREG415,
    swreg416: SWREG416,
    swreg417: SWREG417,
    swreg418: SWREG418,
    swreg419: SWREG419,
    swreg420: SWREG420,
    swreg421: SWREG421,
    swreg422: SWREG422,
    swreg423: SWREG423,
    swreg424: SWREG424,
    swreg425: SWREG425,
    swreg426: SWREG426,
    swreg427: SWREG427,
    swreg428: SWREG428,
    swreg429: SWREG429,
    swreg430: SWREG430,
    swreg431: SWREG431,
    swreg432: SWREG432,
    swreg433: SWREG433,
    swreg434: SWREG434,
    swreg435: SWREG435,
    swreg436: SWREG436,
    swreg437: SWREG437,
    swreg438: SWREG438,
    swreg439: SWREG439,
    swreg440: SWREG440,
    swreg441: SWREG441,
    swreg442: SWREG442,
    swreg443: SWREG443,
    swreg444: SWREG444,
    _reserved352: [u8; 0x10],
    swreg449: SWREG449,
    _reserved353: [u8; 0xbc],
    swreg497: SWREG497,
    swreg498: SWREG498,
}
impl RegisterBlock {
    ///0x00 - VENC ID register
    #[inline(always)]
    pub const fn swreg0(&self) -> &SWREG0 {
        &self.swreg0
    }
    ///0x04 - VENC interrupt register
    #[inline(always)]
    pub const fn swreg1(&self) -> &SWREG1 {
        &self.swreg1
    }
    ///0x08 - VENC bus interface configuration register
    #[inline(always)]
    pub const fn swreg2(&self) -> &SWREG2 {
        &self.swreg2
    }
    ///0x0c - VENC device configuration register
    #[inline(always)]
    pub const fn swreg3(&self) -> &SWREG3 {
        &self.swreg3
    }
    ///0x14 - VENC base address for output stream data register
    #[inline(always)]
    pub const fn swreg5(&self) -> &SWREG5 {
        &self.swreg5
    }
    ///0x18 - VENC base address for output control data register
    #[inline(always)]
    pub const fn swreg6(&self) -> &SWREG6 {
        &self.swreg6
    }
    ///0x1c - VENC base address for reference luma register
    #[inline(always)]
    pub const fn swreg7(&self) -> &SWREG7 {
        &self.swreg7
    }
    ///0x20 - VENC base address for reference chroma register
    #[inline(always)]
    pub const fn swreg8(&self) -> &SWREG8 {
        &self.swreg8
    }
    ///0x24 - VENC base address for reconstructed luma register
    #[inline(always)]
    pub const fn swreg9(&self) -> &SWREG9 {
        &self.swreg9
    }
    ///0x28 - VENC base address for reconstructed chroma register
    #[inline(always)]
    pub const fn swreg10(&self) -> &SWREG10 {
        &self.swreg10
    }
    ///0x2c - VENC base address for input picture luma register
    #[inline(always)]
    pub const fn swreg11(&self) -> &SWREG11 {
        &self.swreg11
    }
    ///0x30 - VENC base address for input picture cb register
    #[inline(always)]
    pub const fn swreg12(&self) -> &SWREG12 {
        &self.swreg12
    }
    ///0x34 - VENC base address for input picture cr register
    #[inline(always)]
    pub const fn swreg13(&self) -> &SWREG13 {
        &self.swreg13
    }
    ///0x38 - VENC encoder control register 0
    #[inline(always)]
    pub const fn swreg14(&self) -> &SWREG14 {
        &self.swreg14
    }
    ///0x3c - VENC encoder control register 1
    #[inline(always)]
    pub const fn swreg15(&self) -> &SWREG15 {
        &self.swreg15
    }
    ///0x40 - VENC encoder control register 2
    #[inline(always)]
    pub const fn swreg16(&self) -> &SWREG16 {
        &self.swreg16
    }
    ///0x44 - VENC encoder control register 3
    #[inline(always)]
    pub const fn swreg17(&self) -> &SWREG17 {
        &self.swreg17
    }
    ///0x48 - VENC encoder control register 4
    #[inline(always)]
    pub const fn swreg18(&self) -> &SWREG18 {
        &self.swreg18
    }
    ///0x4c - VENC encoder control register 5
    #[inline(always)]
    pub const fn swreg19(&self) -> &SWREG19 {
        &self.swreg19
    }
    ///0x50 - VENC encoder control register 6
    #[inline(always)]
    pub const fn swreg20(&self) -> &SWREG20 {
        &self.swreg20
    }
    ///0x54 - VENC encoder control register 7
    #[inline(always)]
    pub const fn swreg21(&self) -> &SWREG21 {
        &self.swreg21
    }
    ///0x58 - VENC stream header remainder MSB bits register
    #[inline(always)]
    pub const fn swreg22(&self) -> &SWREG22 {
        &self.swreg22
    }
    ///0x5c - VENC stream header remainder LSB bits register
    #[inline(always)]
    pub const fn swreg23(&self) -> &SWREG23 {
        &self.swreg23
    }
    ///0x60 - VENC stream buffer limit/output stream size register
    #[inline(always)]
    pub const fn swreg24(&self) -> &SWREG24 {
        &self.swreg24
    }
    ///0x64 - VENC encoder control register 8
    #[inline(always)]
    pub const fn swreg25(&self) -> &SWREG25 {
        &self.swreg25
    }
    ///0x68 - VENC intra-slice bitmap register
    #[inline(always)]
    pub const fn swreg26(&self) -> &SWREG26 {
        &self.swreg26
    }
    ///0x6c - VENC encoder control register 9
    #[inline(always)]
    pub const fn swreg27(&self) -> &SWREG27 {
        &self.swreg27
    }
    ///0x70 - VENC encoder control register 10
    #[inline(always)]
    pub const fn swreg28(&self) -> &SWREG28 {
        &self.swreg28
    }
    ///0x74 - VENC encoder control register 11
    #[inline(always)]
    pub const fn swreg29(&self) -> &SWREG29 {
        &self.swreg29
    }
    ///0x78 - VENC encoder control register 12
    #[inline(always)]
    pub const fn swreg30(&self) -> &SWREG30 {
        &self.swreg30
    }
    ///0x7c - VENC encoder control register 13
    #[inline(always)]
    pub const fn swreg31(&self) -> &SWREG31 {
        &self.swreg31
    }
    ///0x80 - VENC encoder control register 14
    #[inline(always)]
    pub const fn swreg32(&self) -> &SWREG32 {
        &self.swreg32
    }
    ///0x84 - VENC encoder control register 15
    #[inline(always)]
    pub const fn swreg33(&self) -> &SWREG33 {
        &self.swreg33
    }
    ///0x88 - VENC encoder control register 16
    #[inline(always)]
    pub const fn swreg34(&self) -> &SWREG34 {
        &self.swreg34
    }
    ///0x8c - VENC H.264 checkpoint word error 5-6/encoder control register 17
    #[inline(always)]
    pub const fn swreg35(&self) -> &SWREG35 {
        &self.swreg35
    }
    ///0x90 - VENC H.264 checkpoint delta QP 1-8/encoder control register 18
    #[inline(always)]
    pub const fn swreg36(&self) -> &SWREG36 {
        &self.swreg36
    }
    ///0x94 - VENC encoder control register 19, stream start offset
    #[inline(always)]
    pub const fn swreg37(&self) -> &SWREG37 {
        &self.swreg37
    }
    ///0x98 - VENC macroblock count output register
    #[inline(always)]
    pub const fn swreg38(&self) -> &SWREG38 {
        &self.swreg38
    }
    ///0x9c - VENC base address for next pic luminance register
    #[inline(always)]
    pub const fn swreg39(&self) -> &SWREG39 {
        &self.swreg39
    }
    ///0xa0 - VENC stabilization mode control register
    #[inline(always)]
    pub const fn swreg40(&self) -> &SWREG40 {
        &self.swreg40
    }
    ///0xa4 - VENC stabilization motion sum div8 output register
    #[inline(always)]
    pub const fn swreg41(&self) -> &SWREG41 {
        &self.swreg41
    }
    ///0xa8 - VENC stabilization GMV output, matrix 1, up-left position output register
    #[inline(always)]
    pub const fn swreg42(&self) -> &SWREG42 {
        &self.swreg42
    }
    ///0xac - VENC stabilization GMV output, matrix 2, up position output register
    #[inline(always)]
    pub const fn swreg43(&self) -> &SWREG43 {
        &self.swreg43
    }
    ///0xb0 - VENC stabilization matrix 3, up-right position output register
    #[inline(always)]
    pub const fn swreg44(&self) -> &SWREG44 {
        &self.swreg44
    }
    ///0xb4 - VENC stabilization matrix 4, left position output register
    #[inline(always)]
    pub const fn swreg45(&self) -> &SWREG45 {
        &self.swreg45
    }
    ///0xb8 - VENC stabilization matrix 5, GMV position output register
    #[inline(always)]
    pub const fn swreg46(&self) -> &SWREG46 {
        &self.swreg46
    }
    ///0xbc - VENC stabilization matrix 6, right position output register
    #[inline(always)]
    pub const fn swreg47(&self) -> &SWREG47 {
        &self.swreg47
    }
    ///0xc0 - VENC stabilization matrix 7, down-left position output register
    #[inline(always)]
    pub const fn swreg48(&self) -> &SWREG48 {
        &self.swreg48
    }
    ///0xc4 - VENC stabilization matrix 8, down position output register
    #[inline(always)]
    pub const fn swreg49(&self) -> &SWREG49 {
        &self.swreg49
    }
    ///0xc8 - VENC stabilization matrix 9, down-right position output register
    #[inline(always)]
    pub const fn swreg50(&self) -> &SWREG50 {
        &self.swreg50
    }
    ///0xcc - VENC base address for cabac context tables H264 register
    #[inline(always)]
    pub const fn swreg51(&self) -> &SWREG51 {
        &self.swreg51
    }
    ///0xd0 - VENC base address for MV output writing register
    #[inline(always)]
    pub const fn swreg52(&self) -> &SWREG52 {
        &self.swreg52
    }
    ///0xd4 - VENC RGB to YUV conversion coefficient A - B register
    #[inline(always)]
    pub const fn swreg53(&self) -> &SWREG53 {
        &self.swreg53
    }
    ///0xd8 - VENC RGB to YUV conversion coefficient C - E register
    #[inline(always)]
    pub const fn swreg54(&self) -> &SWREG54 {
        &self.swreg54
    }
    ///0xdc - VENC RGB to YUV conversion coefficient F, RGB mask MSB bit position register
    #[inline(always)]
    pub const fn swreg55(&self) -> &SWREG55 {
        &self.swreg55
    }
    ///0xe0 - VENC intra area register
    #[inline(always)]
    pub const fn swreg56(&self) -> &SWREG56 {
        &self.swreg56
    }
    ///0xe4 - VENC CIR intra mb position register
    #[inline(always)]
    pub const fn swreg57(&self) -> &SWREG57 {
        &self.swreg57
    }
    ///0xe8 - VENC intra slice bitmap for slices 0..31/base address for 1st DCT partition register
    #[inline(always)]
    pub const fn swreg58(&self) -> &SWREG58 {
        &self.swreg58
    }
    ///0xec - VENC intra slice bitmap for slices 32..63/base address for 2nd DCT partition register
    #[inline(always)]
    pub const fn swreg59(&self) -> &SWREG59 {
        &self.swreg59
    }
    ///0xf0 - VENC 1st ROI area register
    #[inline(always)]
    pub const fn swreg60(&self) -> &SWREG60 {
        &self.swreg60
    }
    ///0xf4 - VENC 2nd ROI area register
    #[inline(always)]
    pub const fn swreg61(&self) -> &SWREG61 {
        &self.swreg61
    }
    ///0xf8 - VENC ROI area delta QP, MV register
    #[inline(always)]
    pub const fn swreg62(&self) -> &SWREG62 {
        &self.swreg62
    }
    ///0xfc - VENC synthesis configuration register encoder 0 register
    #[inline(always)]
    pub const fn swreg63(&self) -> &SWREG63 {
        &self.swreg63
    }
    ///0x100 - VENC JPEG luma quantization 1/intra 16x16 mode 0-1 penalty register
    #[inline(always)]
    pub const fn swreg64(&self) -> &SWREG64 {
        &self.swreg64
    }
    ///0x104 - VENC JPEG luma quantization 2/intra 16x16 mode 2-3 penalty register
    #[inline(always)]
    pub const fn swreg65(&self) -> &SWREG65 {
        &self.swreg65
    }
    ///0x108 - VENC JPEG luma quantization 3/intra 4x4 mode 0-1 penalty register
    #[inline(always)]
    pub const fn swreg66(&self) -> &SWREG66 {
        &self.swreg66
    }
    ///0x10c - VENC JPEG luma quantization 4/intra 4x4 mode 2-3 penalty register
    #[inline(always)]
    pub const fn swreg67(&self) -> &SWREG67 {
        &self.swreg67
    }
    ///0x110 - VENC JPEG luma quantization 5/intra 4x4 mode 4-5 penalty register
    #[inline(always)]
    pub const fn swreg68(&self) -> &SWREG68 {
        &self.swreg68
    }
    ///0x114 - VENC JPEG luma quantization 6/intra 4x4 mode 6-7 penalty register
    #[inline(always)]
    pub const fn swreg69(&self) -> &SWREG69 {
        &self.swreg69
    }
    ///0x118 - VENC JPEG luma quantization 7/intra 4x4 mode 8-9 penalty register
    #[inline(always)]
    pub const fn swreg70(&self) -> &SWREG70 {
        &self.swreg70
    }
    ///0x11c - VENC JPEG luma quantization 8/base address for segmentation map register
    #[inline(always)]
    pub const fn swreg71(&self) -> &SWREG71 {
        &self.swreg71
    }
    ///0x120 - VENC JPEG luma quantization 9/segment1 parameter register
    #[inline(always)]
    pub const fn swreg72(&self) -> &SWREG72 {
        &self.swreg72
    }
    ///0x124 - VENC JPEG luma quantization 10/segment1 parameter register
    #[inline(always)]
    pub const fn swreg73(&self) -> &SWREG73 {
        &self.swreg73
    }
    ///0x128 - VENC JPEG luma quantization 11/segment1 parameter register
    #[inline(always)]
    pub const fn swreg74(&self) -> &SWREG74 {
        &self.swreg74
    }
    ///0x12c - VENC JPEG luma quantization 12/segment1 parameter register
    #[inline(always)]
    pub const fn swreg75(&self) -> &SWREG75 {
        &self.swreg75
    }
    ///0x130 - VENC JPEG luma quantization 13/segment1 parameter register
    #[inline(always)]
    pub const fn swreg76(&self) -> &SWREG76 {
        &self.swreg76
    }
    ///0x134 - VENC JPEG luma quantization 14/segment1 parameter register
    #[inline(always)]
    pub const fn swreg77(&self) -> &SWREG77 {
        &self.swreg77
    }
    ///0x138 - VENC JPEG luma quantization 15/segment1 parameter register
    #[inline(always)]
    pub const fn swreg78(&self) -> &SWREG78 {
        &self.swreg78
    }
    ///0x13c - VENC JPEG luma quantization 16/segment2 parameter register
    #[inline(always)]
    pub const fn swreg79(&self) -> &SWREG79 {
        &self.swreg79
    }
    ///0x140 - VENC JPEG chroma quantization 1/segment2 parameter register
    #[inline(always)]
    pub const fn swreg80(&self) -> &SWREG80 {
        &self.swreg80
    }
    ///0x144 - VENC JPEG chroma quantization 2/segment2 parameter register
    #[inline(always)]
    pub const fn swreg81(&self) -> &SWREG81 {
        &self.swreg81
    }
    ///0x148 - VENC JPEG chroma quantization 3/segment2 parameter register
    #[inline(always)]
    pub const fn swreg82(&self) -> &SWREG82 {
        &self.swreg82
    }
    ///0x14c - VENC JPEG chroma quantization 4/segment2 parameter register
    #[inline(always)]
    pub const fn swreg83(&self) -> &SWREG83 {
        &self.swreg83
    }
    ///0x150 - VENC JPEG chroma quantization 5/segment2 parameter register
    #[inline(always)]
    pub const fn swreg84(&self) -> &SWREG84 {
        &self.swreg84
    }
    ///0x154 - VENC JPEG chroma quantization 6/segment2 parameter register
    #[inline(always)]
    pub const fn swreg85(&self) -> &SWREG85 {
        &self.swreg85
    }
    ///0x158 - VENC JPEG chroma quantization 7/segment2 parameter register
    #[inline(always)]
    pub const fn swreg86(&self) -> &SWREG86 {
        &self.swreg86
    }
    ///0x15c - VENC JPEG chroma quantization 8/segment2 parameter register
    #[inline(always)]
    pub const fn swreg87(&self) -> &SWREG87 {
        &self.swreg87
    }
    ///0x160 - VENC JPEG chroma quantization 9/segment3 parameter register
    #[inline(always)]
    pub const fn swreg88(&self) -> &SWREG88 {
        &self.swreg88
    }
    ///0x164 - VENC JPEG chroma quantization 10/segment3 parameter register
    #[inline(always)]
    pub const fn swreg89(&self) -> &SWREG89 {
        &self.swreg89
    }
    ///0x168 - VENC JPEG chroma quantization 11/segment3 parameter register
    #[inline(always)]
    pub const fn swreg90(&self) -> &SWREG90 {
        &self.swreg90
    }
    ///0x16c - VENC JPEG chroma quantization 12/segment3 parameter register
    #[inline(always)]
    pub const fn swreg91(&self) -> &SWREG91 {
        &self.swreg91
    }
    ///0x170 - VENC JPEG chroma quantization 13/segment3 parameter register
    #[inline(always)]
    pub const fn swreg92(&self) -> &SWREG92 {
        &self.swreg92
    }
    ///0x174 - VENC JPEG chroma quantization 14/segment3 parameter register
    #[inline(always)]
    pub const fn swreg93(&self) -> &SWREG93 {
        &self.swreg93
    }
    ///0x178 - VENC JPEG chroma quantization 15/segment3 parameter register
    #[inline(always)]
    pub const fn swreg94(&self) -> &SWREG94 {
        &self.swreg94
    }
    ///0x17c - VENC JPEG chroma quantization 16/segment3 parameter register
    #[inline(always)]
    pub const fn swreg95(&self) -> &SWREG95 {
        &self.swreg95
    }
    ///0x180 - VENC DMV 4p/1p penalty values 0-3 register
    #[inline(always)]
    pub const fn swreg96(&self) -> &SWREG96 {
        &self.swreg96
    }
    ///0x184 - VENC DMV 4p/1p penalty values 4-7 register
    #[inline(always)]
    pub const fn swreg97(&self) -> &SWREG97 {
        &self.swreg97
    }
    ///0x188 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg98(&self) -> &SWREG98 {
        &self.swreg98
    }
    ///0x18c - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg99(&self) -> &SWREG99 {
        &self.swreg99
    }
    ///0x190 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg100(&self) -> &SWREG100 {
        &self.swreg100
    }
    ///0x194 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg101(&self) -> &SWREG101 {
        &self.swreg101
    }
    ///0x198 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg102(&self) -> &SWREG102 {
        &self.swreg102
    }
    ///0x19c - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg103(&self) -> &SWREG103 {
        &self.swreg103
    }
    ///0x1a0 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg104(&self) -> &SWREG104 {
        &self.swreg104
    }
    ///0x1a4 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg105(&self) -> &SWREG105 {
        &self.swreg105
    }
    ///0x1a8 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg106(&self) -> &SWREG106 {
        &self.swreg106
    }
    ///0x1ac - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg107(&self) -> &SWREG107 {
        &self.swreg107
    }
    ///0x1b0 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg108(&self) -> &SWREG108 {
        &self.swreg108
    }
    ///0x1b4 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg109(&self) -> &SWREG109 {
        &self.swreg109
    }
    ///0x1b8 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg110(&self) -> &SWREG110 {
        &self.swreg110
    }
    ///0x1bc - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg111(&self) -> &SWREG111 {
        &self.swreg111
    }
    ///0x1c0 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg112(&self) -> &SWREG112 {
        &self.swreg112
    }
    ///0x1c4 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg113(&self) -> &SWREG113 {
        &self.swreg113
    }
    ///0x1c8 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg114(&self) -> &SWREG114 {
        &self.swreg114
    }
    ///0x1cc - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg115(&self) -> &SWREG115 {
        &self.swreg115
    }
    ///0x1d0 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg116(&self) -> &SWREG116 {
        &self.swreg116
    }
    ///0x1d4 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg117(&self) -> &SWREG117 {
        &self.swreg117
    }
    ///0x1d8 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg118(&self) -> &SWREG118 {
        &self.swreg118
    }
    ///0x1dc - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg119(&self) -> &SWREG119 {
        &self.swreg119
    }
    ///0x1e0 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg120(&self) -> &SWREG120 {
        &self.swreg120
    }
    ///0x1e4 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg121(&self) -> &SWREG121 {
        &self.swreg121
    }
    ///0x1e8 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg122(&self) -> &SWREG122 {
        &self.swreg122
    }
    ///0x1ec - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg123(&self) -> &SWREG123 {
        &self.swreg123
    }
    ///0x1f0 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg124(&self) -> &SWREG124 {
        &self.swreg124
    }
    ///0x1f4 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg125(&self) -> &SWREG125 {
        &self.swreg125
    }
    ///0x1f8 - VENC DMV 4p/1p penalty values register
    #[inline(always)]
    pub const fn swreg126(&self) -> &SWREG126 {
        &self.swreg126
    }
    ///0x1fc - VENC DMV 4p/1p penalty values 124-127 register
    #[inline(always)]
    pub const fn swreg127(&self) -> &SWREG127 {
        &self.swreg127
    }
    ///0x200 - VENC DMV qpel penalty values 0-3 register
    #[inline(always)]
    pub const fn swreg128(&self) -> &SWREG128 {
        &self.swreg128
    }
    ///0x204 - VENC DMV qpel penalty values 4-7 register
    #[inline(always)]
    pub const fn swreg129(&self) -> &SWREG129 {
        &self.swreg129
    }
    ///0x208 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg130(&self) -> &SWREG130 {
        &self.swreg130
    }
    ///0x20c - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg131(&self) -> &SWREG131 {
        &self.swreg131
    }
    ///0x210 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg132(&self) -> &SWREG132 {
        &self.swreg132
    }
    ///0x214 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg133(&self) -> &SWREG133 {
        &self.swreg133
    }
    ///0x218 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg134(&self) -> &SWREG134 {
        &self.swreg134
    }
    ///0x21c - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg135(&self) -> &SWREG135 {
        &self.swreg135
    }
    ///0x220 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg136(&self) -> &SWREG136 {
        &self.swreg136
    }
    ///0x224 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg137(&self) -> &SWREG137 {
        &self.swreg137
    }
    ///0x228 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg138(&self) -> &SWREG138 {
        &self.swreg138
    }
    ///0x22c - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg139(&self) -> &SWREG139 {
        &self.swreg139
    }
    ///0x230 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg140(&self) -> &SWREG140 {
        &self.swreg140
    }
    ///0x234 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg141(&self) -> &SWREG141 {
        &self.swreg141
    }
    ///0x238 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg142(&self) -> &SWREG142 {
        &self.swreg142
    }
    ///0x23c - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg143(&self) -> &SWREG143 {
        &self.swreg143
    }
    ///0x240 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg144(&self) -> &SWREG144 {
        &self.swreg144
    }
    ///0x244 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg145(&self) -> &SWREG145 {
        &self.swreg145
    }
    ///0x248 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg146(&self) -> &SWREG146 {
        &self.swreg146
    }
    ///0x24c - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg147(&self) -> &SWREG147 {
        &self.swreg147
    }
    ///0x250 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg148(&self) -> &SWREG148 {
        &self.swreg148
    }
    ///0x254 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg149(&self) -> &SWREG149 {
        &self.swreg149
    }
    ///0x258 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg150(&self) -> &SWREG150 {
        &self.swreg150
    }
    ///0x25c - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg151(&self) -> &SWREG151 {
        &self.swreg151
    }
    ///0x260 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg152(&self) -> &SWREG152 {
        &self.swreg152
    }
    ///0x264 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg153(&self) -> &SWREG153 {
        &self.swreg153
    }
    ///0x268 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg154(&self) -> &SWREG154 {
        &self.swreg154
    }
    ///0x26c - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg155(&self) -> &SWREG155 {
        &self.swreg155
    }
    ///0x270 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg156(&self) -> &SWREG156 {
        &self.swreg156
    }
    ///0x274 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg157(&self) -> &SWREG157 {
        &self.swreg157
    }
    ///0x278 - VENC DMV qpel penalty values register
    #[inline(always)]
    pub const fn swreg158(&self) -> &SWREG158 {
        &self.swreg158
    }
    ///0x27c - VENC DMV qpel penalty values 124-127 register
    #[inline(always)]
    pub const fn swreg159(&self) -> &SWREG159 {
        &self.swreg159
    }
    ///0x39c - VENC base address for output of down-scaled encoder image in YUYV 4:2:2 format register
    #[inline(always)]
    pub const fn swreg231(&self) -> &SWREG231 {
        &self.swreg231
    }
    ///0x3a0 - VENC scaling control register
    #[inline(always)]
    pub const fn swreg232(&self) -> &SWREG232 {
        &self.swreg232
    }
    ///0x3a4 - VENC scaling control register
    #[inline(always)]
    pub const fn swreg233(&self) -> &SWREG233 {
        &self.swreg233
    }
    ///0x3b0 - VENC squared error output calculated for 13x13 pixels per macroblock register
    #[inline(always)]
    pub const fn swreg236(&self) -> &SWREG236 {
        &self.swreg236
    }
    ///0x3b4 - VENC MAD 2 control and output register
    #[inline(always)]
    pub const fn swreg237(&self) -> &SWREG237 {
        &self.swreg237
    }
    ///0x3b8 - VENC MAD 3 control and output register
    #[inline(always)]
    pub const fn swreg238(&self) -> &SWREG238 {
        &self.swreg238
    }
    ///0x400 - VENC segment 1: intra 16x16 mode 0-2 penalty register
    #[inline(always)]
    pub const fn swreg256(&self) -> &SWREG256 {
        &self.swreg256
    }
    ///0x404 - VENC segment 1: intra 16x16 mode 3, intra 4x4 0-1 penalty register
    #[inline(always)]
    pub const fn swreg257(&self) -> &SWREG257 {
        &self.swreg257
    }
    ///0x408 - VENC segment 1: intra 4x4 mode 2-4 penalty register
    #[inline(always)]
    pub const fn swreg258(&self) -> &SWREG258 {
        &self.swreg258
    }
    ///0x40c - VENC segment 1: intra 4x4 mode 5-7 penalty register
    #[inline(always)]
    pub const fn swreg259(&self) -> &SWREG259 {
        &self.swreg259
    }
    ///0x410 - VENC segment 1: intra 4x4 mode 8-9 penalty, previous mode favor for H.264 register
    #[inline(always)]
    pub const fn swreg260(&self) -> &SWREG260 {
        &self.swreg260
    }
    ///0x414 - VENC segment 1: bit cost of inter type, intra 16x16 mode favor register
    #[inline(always)]
    pub const fn swreg261(&self) -> &SWREG261 {
        &self.swreg261
    }
    ///0x418 - VENC segment 1: inter MB mode favor, skip mode penalty, penalty value for 2nd reference frame register
    #[inline(always)]
    pub const fn swreg262(&self) -> &SWREG262 {
        &self.swreg262
    }
    ///0x41c - VENC segment 1: penalty value register
    #[inline(always)]
    pub const fn swreg263(&self) -> &SWREG263 {
        &self.swreg263
    }
    ///0x420 - VENC segment 1: penalty value register
    #[inline(always)]
    pub const fn swreg264(&self) -> &SWREG264 {
        &self.swreg264
    }
    ///0x424 - VENC segment 1: deadzone rate multiplier for plane 0-1 register
    #[inline(always)]
    pub const fn swreg265(&self) -> &SWREG265 {
        &self.swreg265
    }
    ///0x428 - VENC segment 1: deadzone rate multiplier for plane 2-3 register
    #[inline(always)]
    pub const fn swreg266(&self) -> &SWREG266 {
        &self.swreg266
    }
    ///0x42c - VENC segment 1: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register
    #[inline(always)]
    pub const fn swreg267(&self) -> &SWREG267 {
        &self.swreg267
    }
    ///0x430 - VENC segment 2: intra 16x16 mode 0-2 penalty register
    #[inline(always)]
    pub const fn swreg268(&self) -> &SWREG268 {
        &self.swreg268
    }
    ///0x434 - VENC segment 2: intra 16x16 mode 3 penalty, intra 4x4 mode 0-1 penalty register
    #[inline(always)]
    pub const fn swreg269(&self) -> &SWREG269 {
        &self.swreg269
    }
    ///0x438 - VENC segment 2: intra 4x4 mode 2-4 penalty register
    #[inline(always)]
    pub const fn swreg270(&self) -> &SWREG270 {
        &self.swreg270
    }
    ///0x43c - VENC segment 2: intra 4x4 mode 5-7 penalty register
    #[inline(always)]
    pub const fn swreg271(&self) -> &SWREG271 {
        &self.swreg271
    }
    ///0x440 - VENC segment 2: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 register
    #[inline(always)]
    pub const fn swreg272(&self) -> &SWREG272 {
        &self.swreg272
    }
    ///0x444 - VENC segment 2: bit cost of inter type, intra 16x16 mode favor register
    #[inline(always)]
    pub const fn swreg273(&self) -> &SWREG273 {
        &self.swreg273
    }
    ///0x448 - VENC segment 2: inter MB mode favor, skip mode penalty, penalty value register
    #[inline(always)]
    pub const fn swreg274(&self) -> &SWREG274 {
        &self.swreg274
    }
    ///0x44c - VENC segment 2: penalty value register
    #[inline(always)]
    pub const fn swreg275(&self) -> &SWREG275 {
        &self.swreg275
    }
    ///0x450 - VENC segment 2: penalty value register
    #[inline(always)]
    pub const fn swreg276(&self) -> &SWREG276 {
        &self.swreg276
    }
    ///0x454 - VENC segment 2: deadzone rate multiplier for plane 0-1 register
    #[inline(always)]
    pub const fn swreg277(&self) -> &SWREG277 {
        &self.swreg277
    }
    ///0x458 - VENC segment 2: deadzone rate multiplier for plane 2-3 register
    #[inline(always)]
    pub const fn swreg278(&self) -> &SWREG278 {
        &self.swreg278
    }
    ///0x45c - VENC segment 2: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register
    #[inline(always)]
    pub const fn swreg279(&self) -> &SWREG279 {
        &self.swreg279
    }
    ///0x460 - VENC segment 3: intra 16x16 mode 0-2 penalty register
    #[inline(always)]
    pub const fn swreg280(&self) -> &SWREG280 {
        &self.swreg280
    }
    ///0x464 - VENC segment 3: intra 16x16 mode 3 penalty, intra 4x4 mode 0-1 penalty register
    #[inline(always)]
    pub const fn swreg281(&self) -> &SWREG281 {
        &self.swreg281
    }
    ///0x468 - VENC segment 3: intra 4x4 mode 2-4 penalty register
    #[inline(always)]
    pub const fn swreg282(&self) -> &SWREG282 {
        &self.swreg282
    }
    ///0x46c - VENC segment 3: intra 4x4 mode 5-7 penalty register
    #[inline(always)]
    pub const fn swreg283(&self) -> &SWREG283 {
        &self.swreg283
    }
    ///0x470 - VENC segment 3: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 register
    #[inline(always)]
    pub const fn swreg284(&self) -> &SWREG284 {
        &self.swreg284
    }
    ///0x474 - VENC segment 3: bit cost of inter type, intra 16x16 mode favor register
    #[inline(always)]
    pub const fn swreg285(&self) -> &SWREG285 {
        &self.swreg285
    }
    ///0x478 - VENC segment 3: inter MB mode favor in intra/inter selection, inter MB mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg286(&self) -> &SWREG286 {
        &self.swreg286
    }
    ///0x47c - VENC segment 3: penalty value register
    #[inline(always)]
    pub const fn swreg287(&self) -> &SWREG287 {
        &self.swreg287
    }
    ///0x480 - VENC segment 3: penalty value register
    #[inline(always)]
    pub const fn swreg288(&self) -> &SWREG288 {
        &self.swreg288
    }
    ///0x484 - VENC segment 3: deadzone rate multiplier for plane 0-1 register
    #[inline(always)]
    pub const fn swreg289(&self) -> &SWREG289 {
        &self.swreg289
    }
    ///0x488 - VENC segment 3: deadzone rate multiplier for plane 2-3 register
    #[inline(always)]
    pub const fn swreg290(&self) -> &SWREG290 {
        &self.swreg290
    }
    ///0x48c - VENC segment 3: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register
    #[inline(always)]
    pub const fn swreg291(&self) -> &SWREG291 {
        &self.swreg291
    }
    ///0x498 - VENC Mb boost register
    #[inline(always)]
    pub const fn swreg294(&self) -> &SWREG294 {
        &self.swreg294
    }
    ///0x49c - VENC variance control, Pskop conding mode register
    #[inline(always)]
    pub const fn swreg295(&self) -> &SWREG295 {
        &self.swreg295
    }
    ///0x4a0 - VENC synthesis configuration register encoder 1 read only register
    #[inline(always)]
    pub const fn swreg296(&self) -> &SWREG296 {
        &self.swreg296
    }
    ///0x4a4 - VENC MBRC control register
    #[inline(always)]
    pub const fn swreg297(&self) -> &SWREG297 {
        &self.swreg297
    }
    ///0x4a8 - VENC segment 4: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg298(&self) -> &SWREG298 {
        &self.swreg298
    }
    ///0x4ac - VENC segment 4: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg299(&self) -> &SWREG299 {
        &self.swreg299
    }
    ///0x4b0 - VENC segment 4: penalty value register
    #[inline(always)]
    pub const fn swreg300(&self) -> &SWREG300 {
        &self.swreg300
    }
    ///0x4b4 - VENC segment 4: penalty value register
    #[inline(always)]
    pub const fn swreg301(&self) -> &SWREG301 {
        &self.swreg301
    }
    ///0x4b8 - VENC segment 5: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg302(&self) -> &SWREG302 {
        &self.swreg302
    }
    ///0x4bc - VENC segment 5: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg303(&self) -> &SWREG303 {
        &self.swreg303
    }
    ///0x4c0 - VENC segment 5: penalty value register
    #[inline(always)]
    pub const fn swreg304(&self) -> &SWREG304 {
        &self.swreg304
    }
    ///0x4c4 - VENC segment 5: penalty value register
    #[inline(always)]
    pub const fn swreg305(&self) -> &SWREG305 {
        &self.swreg305
    }
    ///0x4c8 - VENC segment 6: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg306(&self) -> &SWREG306 {
        &self.swreg306
    }
    ///0x4cc - VENC segment 6: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg307(&self) -> &SWREG307 {
        &self.swreg307
    }
    ///0x4d0 - VENC segment 6: penalty value register
    #[inline(always)]
    pub const fn swreg308(&self) -> &SWREG308 {
        &self.swreg308
    }
    ///0x4d4 - VENC segment 6: penalty value register
    #[inline(always)]
    pub const fn swreg309(&self) -> &SWREG309 {
        &self.swreg309
    }
    ///0x4d8 - VENC segment 7: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg310(&self) -> &SWREG310 {
        &self.swreg310
    }
    ///0x4dc - VENC segment 7: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg311(&self) -> &SWREG311 {
        &self.swreg311
    }
    ///0x4e0 - VENC segment 7: penalty value register
    #[inline(always)]
    pub const fn swreg312(&self) -> &SWREG312 {
        &self.swreg312
    }
    ///0x4e4 - VENC segment 7: penalty value register
    #[inline(always)]
    pub const fn swreg313(&self) -> &SWREG313 {
        &self.swreg313
    }
    ///0x4e8 - VENC segment 8: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg314(&self) -> &SWREG314 {
        &self.swreg314
    }
    ///0x4ec - VENC segment 8: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg315(&self) -> &SWREG315 {
        &self.swreg315
    }
    ///0x4f0 - VENC segment 8: penalty value register
    #[inline(always)]
    pub const fn swreg316(&self) -> &SWREG316 {
        &self.swreg316
    }
    ///0x4f4 - VENC segment 8: penalty value register
    #[inline(always)]
    pub const fn swreg317(&self) -> &SWREG317 {
        &self.swreg317
    }
    ///0x4f8 - VENC segment 9: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg318(&self) -> &SWREG318 {
        &self.swreg318
    }
    ///0x4fc - VENC segment 9: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg319(&self) -> &SWREG319 {
        &self.swreg319
    }
    ///0x500 - VENC segment 9: penalty value register
    #[inline(always)]
    pub const fn swreg320(&self) -> &SWREG320 {
        &self.swreg320
    }
    ///0x504 - VENC segment 9: penalty value register
    #[inline(always)]
    pub const fn swreg321(&self) -> &SWREG321 {
        &self.swreg321
    }
    ///0x508 - VENC segment 10: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg322(&self) -> &SWREG322 {
        &self.swreg322
    }
    ///0x50c - VENC segment 10: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg323(&self) -> &SWREG323 {
        &self.swreg323
    }
    ///0x510 - VENC segment 10: penalty value register
    #[inline(always)]
    pub const fn swreg324(&self) -> &SWREG324 {
        &self.swreg324
    }
    ///0x514 - VENC segment 10: penalty value register
    #[inline(always)]
    pub const fn swreg325(&self) -> &SWREG325 {
        &self.swreg325
    }
    ///0x518 - VENC segment 11: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg326(&self) -> &SWREG326 {
        &self.swreg326
    }
    ///0x51c - VENC segment 11: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg327(&self) -> &SWREG327 {
        &self.swreg327
    }
    ///0x520 - VENC segment 11: penalty value register
    #[inline(always)]
    pub const fn swreg328(&self) -> &SWREG328 {
        &self.swreg328
    }
    ///0x524 - VENC segment 11: penalty value register
    #[inline(always)]
    pub const fn swreg329(&self) -> &SWREG329 {
        &self.swreg329
    }
    ///0x528 - VENC segment 12: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg330(&self) -> &SWREG330 {
        &self.swreg330
    }
    ///0x52c - VENC segment 12: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg331(&self) -> &SWREG331 {
        &self.swreg331
    }
    ///0x530 - VENC segment 12: penalty value register
    #[inline(always)]
    pub const fn swreg332(&self) -> &SWREG332 {
        &self.swreg332
    }
    ///0x534 - VENC segment 12: penalty value register
    #[inline(always)]
    pub const fn swreg333(&self) -> &SWREG333 {
        &self.swreg333
    }
    ///0x538 - VENC segment 13: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg334(&self) -> &SWREG334 {
        &self.swreg334
    }
    ///0x53c - VENC segment 13: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg335(&self) -> &SWREG335 {
        &self.swreg335
    }
    ///0x540 - VENC segment 13: penalty value register
    #[inline(always)]
    pub const fn swreg336(&self) -> &SWREG336 {
        &self.swreg336
    }
    ///0x544 - VENC segment 13: penalty value register
    #[inline(always)]
    pub const fn swreg337(&self) -> &SWREG337 {
        &self.swreg337
    }
    ///0x548 - VENC segment 14: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg338(&self) -> &SWREG338 {
        &self.swreg338
    }
    ///0x54c - VENC segment 14: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg339(&self) -> &SWREG339 {
        &self.swreg339
    }
    ///0x550 - VENC segment 14: penalty value register
    #[inline(always)]
    pub const fn swreg340(&self) -> &SWREG340 {
        &self.swreg340
    }
    ///0x554 - VENC segment 14: penalty value register
    #[inline(always)]
    pub const fn swreg341(&self) -> &SWREG341 {
        &self.swreg341
    }
    ///0x558 - VENC segment 15: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg342(&self) -> &SWREG342 {
        &self.swreg342
    }
    ///0x55c - VENC segment 15: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg343(&self) -> &SWREG343 {
        &self.swreg343
    }
    ///0x560 - VENC segment 15: penalty value register
    #[inline(always)]
    pub const fn swreg344(&self) -> &SWREG344 {
        &self.swreg344
    }
    ///0x564 - VENC segment 15: penalty value register
    #[inline(always)]
    pub const fn swreg345(&self) -> &SWREG345 {
        &self.swreg345
    }
    ///0x568 - VENC segment 16: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg346(&self) -> &SWREG346 {
        &self.swreg346
    }
    ///0x56c - VENC segment 16: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg347(&self) -> &SWREG347 {
        &self.swreg347
    }
    ///0x570 - VENC segment 16: penalty value register
    #[inline(always)]
    pub const fn swreg348(&self) -> &SWREG348 {
        &self.swreg348
    }
    ///0x574 - VENC segment 16: penalty value register
    #[inline(always)]
    pub const fn swreg349(&self) -> &SWREG349 {
        &self.swreg349
    }
    ///0x578 - VENC segment 17: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg350(&self) -> &SWREG350 {
        &self.swreg350
    }
    ///0x57c - VENC segment 17: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg351(&self) -> &SWREG351 {
        &self.swreg351
    }
    ///0x580 - VENC segment 17: penalty value register
    #[inline(always)]
    pub const fn swreg352(&self) -> &SWREG352 {
        &self.swreg352
    }
    ///0x584 - VENC segment 17: penalty value register
    #[inline(always)]
    pub const fn swreg353(&self) -> &SWREG353 {
        &self.swreg353
    }
    ///0x588 - VENC segment 18: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg354(&self) -> &SWREG354 {
        &self.swreg354
    }
    ///0x58c - VENC segment 18: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg355(&self) -> &SWREG355 {
        &self.swreg355
    }
    ///0x590 - VENC segment 18: penalty value register
    #[inline(always)]
    pub const fn swreg356(&self) -> &SWREG356 {
        &self.swreg356
    }
    ///0x594 - VENC segment 18: penalty value register
    #[inline(always)]
    pub const fn swreg357(&self) -> &SWREG357 {
        &self.swreg357
    }
    ///0x598 - VENC segment 19: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg358(&self) -> &SWREG358 {
        &self.swreg358
    }
    ///0x59c - VENC segment 19: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg359(&self) -> &SWREG359 {
        &self.swreg359
    }
    ///0x5a0 - VENC segment 19: penalty value register
    #[inline(always)]
    pub const fn swreg360(&self) -> &SWREG360 {
        &self.swreg360
    }
    ///0x5a4 - VENC segment 19: penalty value register
    #[inline(always)]
    pub const fn swreg361(&self) -> &SWREG361 {
        &self.swreg361
    }
    ///0x5a8 - VENC segment 20: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg362(&self) -> &SWREG362 {
        &self.swreg362
    }
    ///0x5ac - VENC segment 20: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg363(&self) -> &SWREG363 {
        &self.swreg363
    }
    ///0x5b0 - VENC segment 20: penalty value register
    #[inline(always)]
    pub const fn swreg364(&self) -> &SWREG364 {
        &self.swreg364
    }
    ///0x5b4 - VENC segment 20: penalty value register
    #[inline(always)]
    pub const fn swreg365(&self) -> &SWREG365 {
        &self.swreg365
    }
    ///0x5b8 - VENC segment 21: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg366(&self) -> &SWREG366 {
        &self.swreg366
    }
    ///0x5bc - VENC segment 21: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg367(&self) -> &SWREG367 {
        &self.swreg367
    }
    ///0x5c0 - VENC segment 21: penalty value register
    #[inline(always)]
    pub const fn swreg368(&self) -> &SWREG368 {
        &self.swreg368
    }
    ///0x5c4 - VENC segment 21: penalty value register
    #[inline(always)]
    pub const fn swreg369(&self) -> &SWREG369 {
        &self.swreg369
    }
    ///0x5c8 - VENC segment 22: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg370(&self) -> &SWREG370 {
        &self.swreg370
    }
    ///0x5cc - VENC segment 22: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg371(&self) -> &SWREG371 {
        &self.swreg371
    }
    ///0x5d0 - VENC segment 22: penalty value register
    #[inline(always)]
    pub const fn swreg372(&self) -> &SWREG372 {
        &self.swreg372
    }
    ///0x5d4 - VENC segment 22: penalty value register
    #[inline(always)]
    pub const fn swreg373(&self) -> &SWREG373 {
        &self.swreg373
    }
    ///0x5d8 - VENC segment 23: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg374(&self) -> &SWREG374 {
        &self.swreg374
    }
    ///0x5dc - VENC segment 23: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg375(&self) -> &SWREG375 {
        &self.swreg375
    }
    ///0x5e0 - VENC segment 23: penalty value register
    #[inline(always)]
    pub const fn swreg376(&self) -> &SWREG376 {
        &self.swreg376
    }
    ///0x5e4 - VENC segment 23: penalty value register
    #[inline(always)]
    pub const fn swreg377(&self) -> &SWREG377 {
        &self.swreg377
    }
    ///0x5e8 - VENC segment 24: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg378(&self) -> &SWREG378 {
        &self.swreg378
    }
    ///0x5ec - VENC segment 24: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg379(&self) -> &SWREG379 {
        &self.swreg379
    }
    ///0x5f0 - VENC segment 24: penalty value register
    #[inline(always)]
    pub const fn swreg380(&self) -> &SWREG380 {
        &self.swreg380
    }
    ///0x5f4 - VENC segment 24: penalty value register
    #[inline(always)]
    pub const fn swreg381(&self) -> &SWREG381 {
        &self.swreg381
    }
    ///0x5f8 - VENC segment 25: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg382(&self) -> &SWREG382 {
        &self.swreg382
    }
    ///0x5fc - VENC segment 25: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg383(&self) -> &SWREG383 {
        &self.swreg383
    }
    ///0x600 - VENC segment 25: penalty value register
    #[inline(always)]
    pub const fn swreg384(&self) -> &SWREG384 {
        &self.swreg384
    }
    ///0x604 - VENC segment 25: penalty value register
    #[inline(always)]
    pub const fn swreg385(&self) -> &SWREG385 {
        &self.swreg385
    }
    ///0x608 - VENC segment 26: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg386(&self) -> &SWREG386 {
        &self.swreg386
    }
    ///0x60c - VENC segment 26: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg387(&self) -> &SWREG387 {
        &self.swreg387
    }
    ///0x610 - VENC segment 26: penalty value register
    #[inline(always)]
    pub const fn swreg388(&self) -> &SWREG388 {
        &self.swreg388
    }
    ///0x614 - VENC segment 26: penalty value register
    #[inline(always)]
    pub const fn swreg389(&self) -> &SWREG389 {
        &self.swreg389
    }
    ///0x618 - VENC segment 27: intra 4x4 previous mode favor, intra 16x16mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg390(&self) -> &SWREG390 {
        &self.swreg390
    }
    ///0x61c - VENC segment 27: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg391(&self) -> &SWREG391 {
        &self.swreg391
    }
    ///0x620 - VENC segment 27: penalty value register
    #[inline(always)]
    pub const fn swreg392(&self) -> &SWREG392 {
        &self.swreg392
    }
    ///0x624 - VENC segment 27: penalty value register
    #[inline(always)]
    pub const fn swreg393(&self) -> &SWREG393 {
        &self.swreg393
    }
    ///0x628 - VENC segment 28: intra 4x4 previous mode favor, intra 16x16mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg394(&self) -> &SWREG394 {
        &self.swreg394
    }
    ///0x62c - VENC segment 28: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg395(&self) -> &SWREG395 {
        &self.swreg395
    }
    ///0x630 - VENC segment 28: penalty value register
    #[inline(always)]
    pub const fn swreg396(&self) -> &SWREG396 {
        &self.swreg396
    }
    ///0x634 - VENC segment 28: penalty value register
    #[inline(always)]
    pub const fn swreg397(&self) -> &SWREG397 {
        &self.swreg397
    }
    ///0x638 - VENC segment 29: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg398(&self) -> &SWREG398 {
        &self.swreg398
    }
    ///0x63c - VENC segment 29: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg399(&self) -> &SWREG399 {
        &self.swreg399
    }
    ///0x640 - VENC segment 29: penalty value register
    #[inline(always)]
    pub const fn swreg400(&self) -> &SWREG400 {
        &self.swreg400
    }
    ///0x644 - VENC segment 29: penalty value register
    #[inline(always)]
    pub const fn swreg401(&self) -> &SWREG401 {
        &self.swreg401
    }
    ///0x648 - VENC segment 30: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg402(&self) -> &SWREG402 {
        &self.swreg402
    }
    ///0x64c - VENC segment 30: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg403(&self) -> &SWREG403 {
        &self.swreg403
    }
    ///0x650 - VENC segment 30: penalty value register
    #[inline(always)]
    pub const fn swreg404(&self) -> &SWREG404 {
        &self.swreg404
    }
    ///0x654 - VENC segment 30: penalty value register
    #[inline(always)]
    pub const fn swreg405(&self) -> &SWREG405 {
        &self.swreg405
    }
    ///0x658 - VENC segment 31: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
    #[inline(always)]
    pub const fn swreg406(&self) -> &SWREG406 {
        &self.swreg406
    }
    ///0x65c - VENC segment 31: skip mode penalty, inter MB mode favor register
    #[inline(always)]
    pub const fn swreg407(&self) -> &SWREG407 {
        &self.swreg407
    }
    ///0x660 - VENC segment 31: penalty value register
    #[inline(always)]
    pub const fn swreg408(&self) -> &SWREG408 {
        &self.swreg408
    }
    ///0x664 - VENC segment 31: penalty value register
    #[inline(always)]
    pub const fn swreg409(&self) -> &SWREG409 {
        &self.swreg409
    }
    ///0x668 - VENC MBRC control, QP, offset, enable register
    #[inline(always)]
    pub const fn swreg410(&self) -> &SWREG410 {
        &self.swreg410
    }
    ///0x66c - VENC gain of MB QP delta. 8.8 format register
    #[inline(always)]
    pub const fn swreg411(&self) -> &SWREG411 {
        &self.swreg411
    }
    ///0x670 - VENC average of MB complexity register
    #[inline(always)]
    pub const fn swreg412(&self) -> &SWREG412 {
        &self.swreg412
    }
    ///0x674 - VENC reference compression control register
    #[inline(always)]
    pub const fn swreg413(&self) -> &SWREG413 {
        &self.swreg413
    }
    ///0x678 - VENC base address for reference luma register
    #[inline(always)]
    pub const fn swreg414(&self) -> &SWREG414 {
        &self.swreg414
    }
    ///0x67c - VENC base address for reference chroma register
    #[inline(always)]
    pub const fn swreg415(&self) -> &SWREG415 {
        &self.swreg415
    }
    ///0x680 - VENC base address for reconstructed luma register
    #[inline(always)]
    pub const fn swreg416(&self) -> &SWREG416 {
        &self.swreg416
    }
    ///0x684 - VENC base address for reconstructed chroma register
    #[inline(always)]
    pub const fn swreg417(&self) -> &SWREG417 {
        &self.swreg417
    }
    ///0x688 - VENC base address for second reference luma register
    #[inline(always)]
    pub const fn swreg418(&self) -> &SWREG418 {
        &self.swreg418
    }
    ///0x68c - VENC base address for second reference chroma register
    #[inline(always)]
    pub const fn swreg419(&self) -> &SWREG419 {
        &self.swreg419
    }
    ///0x690 - VENC limit of chroma RFC buffer register
    #[inline(always)]
    pub const fn swreg420(&self) -> &SWREG420 {
        &self.swreg420
    }
    ///0x694 - VENC reorder control register
    #[inline(always)]
    pub const fn swreg421(&self) -> &SWREG421 {
        &self.swreg421
    }
    ///0x698 - VENC AXI read ID register
    #[inline(always)]
    pub const fn swreg422(&self) -> &SWREG422 {
        &self.swreg422
    }
    ///0x69c - VENC base address MSB for reference luma compression table register
    #[inline(always)]
    pub const fn swreg423(&self) -> &SWREG423 {
        &self.swreg423
    }
    ///0x6a0 - VENC base address MSB for reference chroma compression table register
    #[inline(always)]
    pub const fn swreg424(&self) -> &SWREG424 {
        &self.swreg424
    }
    ///0x6a4 - VENC base address MSB for reconstructed luma compression table register
    #[inline(always)]
    pub const fn swreg425(&self) -> &SWREG425 {
        &self.swreg425
    }
    ///0x6a8 - VENC base address for reconstructed chroma compression table register
    #[inline(always)]
    pub const fn swreg426(&self) -> &SWREG426 {
        &self.swreg426
    }
    ///0x6ac - VENC base address MSB for second reference luma compression table register
    #[inline(always)]
    pub const fn swreg427(&self) -> &SWREG427 {
        &self.swreg427
    }
    ///0x6b0 - VENC base address MSB for second reference chroma compression table register
    #[inline(always)]
    pub const fn swreg428(&self) -> &SWREG428 {
        &self.swreg428
    }
    ///0x6b4 - VENC high 32 bits of base address for output stream data register
    #[inline(always)]
    pub const fn swreg429(&self) -> &SWREG429 {
        &self.swreg429
    }
    ///0x6b8 - VENC high 32 bits of base address for output control data register
    #[inline(always)]
    pub const fn swreg430(&self) -> &SWREG430 {
        &self.swreg430
    }
    ///0x6bc - VENC high 32 bits of base address for reference luma register
    #[inline(always)]
    pub const fn swreg431(&self) -> &SWREG431 {
        &self.swreg431
    }
    ///0x6c0 - VENC high 32 bits of base address for reference chroma register
    #[inline(always)]
    pub const fn swreg432(&self) -> &SWREG432 {
        &self.swreg432
    }
    ///0x6c4 - VENC high 32 bits of base address for reconstructed luma register
    #[inline(always)]
    pub const fn swreg433(&self) -> &SWREG433 {
        &self.swreg433
    }
    ///0x6c8 - VENC high 32 bits of base address for reconstructed chroma register
    #[inline(always)]
    pub const fn swreg434(&self) -> &SWREG434 {
        &self.swreg434
    }
    ///0x6cc - VENC high 32 bits of base address for input picture luma register
    #[inline(always)]
    pub const fn swreg435(&self) -> &SWREG435 {
        &self.swreg435
    }
    ///0x6d0 - VENC high 32 bits of base address for input picture cb register
    #[inline(always)]
    pub const fn swreg436(&self) -> &SWREG436 {
        &self.swreg436
    }
    ///0x6d4 - VENC high 32 bits of base address for input picture cr register
    #[inline(always)]
    pub const fn swreg437(&self) -> &SWREG437 {
        &self.swreg437
    }
    ///0x6d8 - VENC high 32 bits of base address for second reference luma register
    #[inline(always)]
    pub const fn swreg438(&self) -> &SWREG438 {
        &self.swreg438
    }
    ///0x6dc - VENC high 32 bits of base address for second reference chroma register
    #[inline(always)]
    pub const fn swreg439(&self) -> &SWREG439 {
        &self.swreg439
    }
    ///0x6e0 - VENC high 32 bits of H264 secondary ref pic base register
    #[inline(always)]
    pub const fn swreg440(&self) -> &SWREG440 {
        &self.swreg440
    }
    ///0x6e4 - VENC high 32 bits of H264 secondary ref pic base register
    #[inline(always)]
    pub const fn swreg441(&self) -> &SWREG441 {
        &self.swreg441
    }
    ///0x6e8 - VENC high 32 bits of base address for next pic luminance register
    #[inline(always)]
    pub const fn swreg442(&self) -> &SWREG442 {
        &self.swreg442
    }
    ///0x6ec - VENC high 32 bits of base address for cabac context tables H264 register
    #[inline(always)]
    pub const fn swreg443(&self) -> &SWREG443 {
        &self.swreg443
    }
    ///0x6f0 - VENC high 32 bits of base address for MV output writing register
    #[inline(always)]
    pub const fn swreg444(&self) -> &SWREG444 {
        &self.swreg444
    }
    ///0x704 - VENC high 32 bits of base address for output of down-scaled encoder image in YUYV 4:2:2 format register
    #[inline(always)]
    pub const fn swreg449(&self) -> &SWREG449 {
        &self.swreg449
    }
    ///0x7c4 - VENC low-latency control register
    #[inline(always)]
    pub const fn swreg497(&self) -> &SWREG497 {
        &self.swreg497
    }
    ///0x7c8 - VENC encoder line buffer offset register
    #[inline(always)]
    pub const fn swreg498(&self) -> &SWREG498 {
        &self.swreg498
    }
}
/**SWREG0 (rw) register accessor: VENC ID register

You can [`read`](crate::Reg::read) this register and get [`swreg0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG0)

For information about available fields see [`mod@swreg0`] module*/
pub type SWREG0 = crate::Reg<swreg0::SWREG0rs>;
///VENC ID register
pub mod swreg0;
/**SWREG1 (rw) register accessor: VENC interrupt register

You can [`read`](crate::Reg::read) this register and get [`swreg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG1)

For information about available fields see [`mod@swreg1`] module*/
pub type SWREG1 = crate::Reg<swreg1::SWREG1rs>;
///VENC interrupt register
pub mod swreg1;
/**SWREG2 (rw) register accessor: VENC bus interface configuration register

You can [`read`](crate::Reg::read) this register and get [`swreg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG2)

For information about available fields see [`mod@swreg2`] module*/
pub type SWREG2 = crate::Reg<swreg2::SWREG2rs>;
///VENC bus interface configuration register
pub mod swreg2;
/**SWREG3 (rw) register accessor: VENC device configuration register

You can [`read`](crate::Reg::read) this register and get [`swreg3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG3)

For information about available fields see [`mod@swreg3`] module*/
pub type SWREG3 = crate::Reg<swreg3::SWREG3rs>;
///VENC device configuration register
pub mod swreg3;
/**SWREG5 (rw) register accessor: VENC base address for output stream data register

You can [`read`](crate::Reg::read) this register and get [`swreg5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG5)

For information about available fields see [`mod@swreg5`] module*/
pub type SWREG5 = crate::Reg<swreg5::SWREG5rs>;
///VENC base address for output stream data register
pub mod swreg5;
/**SWREG6 (rw) register accessor: VENC base address for output control data register

You can [`read`](crate::Reg::read) this register and get [`swreg6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG6)

For information about available fields see [`mod@swreg6`] module*/
pub type SWREG6 = crate::Reg<swreg6::SWREG6rs>;
///VENC base address for output control data register
pub mod swreg6;
/**SWREG7 (rw) register accessor: VENC base address for reference luma register

You can [`read`](crate::Reg::read) this register and get [`swreg7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG7)

For information about available fields see [`mod@swreg7`] module*/
pub type SWREG7 = crate::Reg<swreg7::SWREG7rs>;
///VENC base address for reference luma register
pub mod swreg7;
/**SWREG8 (rw) register accessor: VENC base address for reference chroma register

You can [`read`](crate::Reg::read) this register and get [`swreg8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG8)

For information about available fields see [`mod@swreg8`] module*/
pub type SWREG8 = crate::Reg<swreg8::SWREG8rs>;
///VENC base address for reference chroma register
pub mod swreg8;
/**SWREG9 (rw) register accessor: VENC base address for reconstructed luma register

You can [`read`](crate::Reg::read) this register and get [`swreg9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG9)

For information about available fields see [`mod@swreg9`] module*/
pub type SWREG9 = crate::Reg<swreg9::SWREG9rs>;
///VENC base address for reconstructed luma register
pub mod swreg9;
/**SWREG10 (rw) register accessor: VENC base address for reconstructed chroma register

You can [`read`](crate::Reg::read) this register and get [`swreg10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG10)

For information about available fields see [`mod@swreg10`] module*/
pub type SWREG10 = crate::Reg<swreg10::SWREG10rs>;
///VENC base address for reconstructed chroma register
pub mod swreg10;
/**SWREG11 (rw) register accessor: VENC base address for input picture luma register

You can [`read`](crate::Reg::read) this register and get [`swreg11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG11)

For information about available fields see [`mod@swreg11`] module*/
pub type SWREG11 = crate::Reg<swreg11::SWREG11rs>;
///VENC base address for input picture luma register
pub mod swreg11;
/**SWREG12 (rw) register accessor: VENC base address for input picture cb register

You can [`read`](crate::Reg::read) this register and get [`swreg12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG12)

For information about available fields see [`mod@swreg12`] module*/
pub type SWREG12 = crate::Reg<swreg12::SWREG12rs>;
///VENC base address for input picture cb register
pub mod swreg12;
/**SWREG13 (rw) register accessor: VENC base address for input picture cr register

You can [`read`](crate::Reg::read) this register and get [`swreg13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG13)

For information about available fields see [`mod@swreg13`] module*/
pub type SWREG13 = crate::Reg<swreg13::SWREG13rs>;
///VENC base address for input picture cr register
pub mod swreg13;
/**SWREG14 (rw) register accessor: VENC encoder control register 0

You can [`read`](crate::Reg::read) this register and get [`swreg14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG14)

For information about available fields see [`mod@swreg14`] module*/
pub type SWREG14 = crate::Reg<swreg14::SWREG14rs>;
///VENC encoder control register 0
pub mod swreg14;
/**SWREG15 (rw) register accessor: VENC encoder control register 1

You can [`read`](crate::Reg::read) this register and get [`swreg15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG15)

For information about available fields see [`mod@swreg15`] module*/
pub type SWREG15 = crate::Reg<swreg15::SWREG15rs>;
///VENC encoder control register 1
pub mod swreg15;
/**SWREG16 (rw) register accessor: VENC encoder control register 2

You can [`read`](crate::Reg::read) this register and get [`swreg16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG16)

For information about available fields see [`mod@swreg16`] module*/
pub type SWREG16 = crate::Reg<swreg16::SWREG16rs>;
///VENC encoder control register 2
pub mod swreg16;
/**SWREG17 (rw) register accessor: VENC encoder control register 3

You can [`read`](crate::Reg::read) this register and get [`swreg17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG17)

For information about available fields see [`mod@swreg17`] module*/
pub type SWREG17 = crate::Reg<swreg17::SWREG17rs>;
///VENC encoder control register 3
pub mod swreg17;
/**SWREG18 (rw) register accessor: VENC encoder control register 4

You can [`read`](crate::Reg::read) this register and get [`swreg18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG18)

For information about available fields see [`mod@swreg18`] module*/
pub type SWREG18 = crate::Reg<swreg18::SWREG18rs>;
///VENC encoder control register 4
pub mod swreg18;
/**SWREG19 (rw) register accessor: VENC encoder control register 5

You can [`read`](crate::Reg::read) this register and get [`swreg19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG19)

For information about available fields see [`mod@swreg19`] module*/
pub type SWREG19 = crate::Reg<swreg19::SWREG19rs>;
///VENC encoder control register 5
pub mod swreg19;
/**SWREG20 (rw) register accessor: VENC encoder control register 6

You can [`read`](crate::Reg::read) this register and get [`swreg20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG20)

For information about available fields see [`mod@swreg20`] module*/
pub type SWREG20 = crate::Reg<swreg20::SWREG20rs>;
///VENC encoder control register 6
pub mod swreg20;
/**SWREG21 (rw) register accessor: VENC encoder control register 7

You can [`read`](crate::Reg::read) this register and get [`swreg21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG21)

For information about available fields see [`mod@swreg21`] module*/
pub type SWREG21 = crate::Reg<swreg21::SWREG21rs>;
///VENC encoder control register 7
pub mod swreg21;
/**SWREG22 (rw) register accessor: VENC stream header remainder MSB bits register

You can [`read`](crate::Reg::read) this register and get [`swreg22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG22)

For information about available fields see [`mod@swreg22`] module*/
pub type SWREG22 = crate::Reg<swreg22::SWREG22rs>;
///VENC stream header remainder MSB bits register
pub mod swreg22;
/**SWREG23 (rw) register accessor: VENC stream header remainder LSB bits register

You can [`read`](crate::Reg::read) this register and get [`swreg23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG23)

For information about available fields see [`mod@swreg23`] module*/
pub type SWREG23 = crate::Reg<swreg23::SWREG23rs>;
///VENC stream header remainder LSB bits register
pub mod swreg23;
/**SWREG24 (rw) register accessor: VENC stream buffer limit/output stream size register

You can [`read`](crate::Reg::read) this register and get [`swreg24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG24)

For information about available fields see [`mod@swreg24`] module*/
pub type SWREG24 = crate::Reg<swreg24::SWREG24rs>;
///VENC stream buffer limit/output stream size register
pub mod swreg24;
/**SWREG25 (rw) register accessor: VENC encoder control register 8

You can [`read`](crate::Reg::read) this register and get [`swreg25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG25)

For information about available fields see [`mod@swreg25`] module*/
pub type SWREG25 = crate::Reg<swreg25::SWREG25rs>;
///VENC encoder control register 8
pub mod swreg25;
/**SWREG26 (rw) register accessor: VENC intra-slice bitmap register

You can [`read`](crate::Reg::read) this register and get [`swreg26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG26)

For information about available fields see [`mod@swreg26`] module*/
pub type SWREG26 = crate::Reg<swreg26::SWREG26rs>;
///VENC intra-slice bitmap register
pub mod swreg26;
/**SWREG27 (rw) register accessor: VENC encoder control register 9

You can [`read`](crate::Reg::read) this register and get [`swreg27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG27)

For information about available fields see [`mod@swreg27`] module*/
pub type SWREG27 = crate::Reg<swreg27::SWREG27rs>;
///VENC encoder control register 9
pub mod swreg27;
/**SWREG28 (rw) register accessor: VENC encoder control register 10

You can [`read`](crate::Reg::read) this register and get [`swreg28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG28)

For information about available fields see [`mod@swreg28`] module*/
pub type SWREG28 = crate::Reg<swreg28::SWREG28rs>;
///VENC encoder control register 10
pub mod swreg28;
/**SWREG29 (rw) register accessor: VENC encoder control register 11

You can [`read`](crate::Reg::read) this register and get [`swreg29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG29)

For information about available fields see [`mod@swreg29`] module*/
pub type SWREG29 = crate::Reg<swreg29::SWREG29rs>;
///VENC encoder control register 11
pub mod swreg29;
/**SWREG30 (rw) register accessor: VENC encoder control register 12

You can [`read`](crate::Reg::read) this register and get [`swreg30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG30)

For information about available fields see [`mod@swreg30`] module*/
pub type SWREG30 = crate::Reg<swreg30::SWREG30rs>;
///VENC encoder control register 12
pub mod swreg30;
/**SWREG31 (rw) register accessor: VENC encoder control register 13

You can [`read`](crate::Reg::read) this register and get [`swreg31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG31)

For information about available fields see [`mod@swreg31`] module*/
pub type SWREG31 = crate::Reg<swreg31::SWREG31rs>;
///VENC encoder control register 13
pub mod swreg31;
/**SWREG32 (rw) register accessor: VENC encoder control register 14

You can [`read`](crate::Reg::read) this register and get [`swreg32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG32)

For information about available fields see [`mod@swreg32`] module*/
pub type SWREG32 = crate::Reg<swreg32::SWREG32rs>;
///VENC encoder control register 14
pub mod swreg32;
/**SWREG33 (rw) register accessor: VENC encoder control register 15

You can [`read`](crate::Reg::read) this register and get [`swreg33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG33)

For information about available fields see [`mod@swreg33`] module*/
pub type SWREG33 = crate::Reg<swreg33::SWREG33rs>;
///VENC encoder control register 15
pub mod swreg33;
/**SWREG34 (rw) register accessor: VENC encoder control register 16

You can [`read`](crate::Reg::read) this register and get [`swreg34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG34)

For information about available fields see [`mod@swreg34`] module*/
pub type SWREG34 = crate::Reg<swreg34::SWREG34rs>;
///VENC encoder control register 16
pub mod swreg34;
/**SWREG35 (rw) register accessor: VENC H.264 checkpoint word error 5-6/encoder control register 17

You can [`read`](crate::Reg::read) this register and get [`swreg35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG35)

For information about available fields see [`mod@swreg35`] module*/
pub type SWREG35 = crate::Reg<swreg35::SWREG35rs>;
///VENC H.264 checkpoint word error 5-6/encoder control register 17
pub mod swreg35;
/**SWREG36 (rw) register accessor: VENC H.264 checkpoint delta QP 1-8/encoder control register 18

You can [`read`](crate::Reg::read) this register and get [`swreg36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG36)

For information about available fields see [`mod@swreg36`] module*/
pub type SWREG36 = crate::Reg<swreg36::SWREG36rs>;
///VENC H.264 checkpoint delta QP 1-8/encoder control register 18
pub mod swreg36;
/**SWREG37 (rw) register accessor: VENC encoder control register 19, stream start offset

You can [`read`](crate::Reg::read) this register and get [`swreg37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG37)

For information about available fields see [`mod@swreg37`] module*/
pub type SWREG37 = crate::Reg<swreg37::SWREG37rs>;
///VENC encoder control register 19, stream start offset
pub mod swreg37;
/**SWREG38 (rw) register accessor: VENC macroblock count output register

You can [`read`](crate::Reg::read) this register and get [`swreg38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG38)

For information about available fields see [`mod@swreg38`] module*/
pub type SWREG38 = crate::Reg<swreg38::SWREG38rs>;
///VENC macroblock count output register
pub mod swreg38;
/**SWREG39 (rw) register accessor: VENC base address for next pic luminance register

You can [`read`](crate::Reg::read) this register and get [`swreg39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG39)

For information about available fields see [`mod@swreg39`] module*/
pub type SWREG39 = crate::Reg<swreg39::SWREG39rs>;
///VENC base address for next pic luminance register
pub mod swreg39;
/**SWREG40 (rw) register accessor: VENC stabilization mode control register

You can [`read`](crate::Reg::read) this register and get [`swreg40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG40)

For information about available fields see [`mod@swreg40`] module*/
pub type SWREG40 = crate::Reg<swreg40::SWREG40rs>;
///VENC stabilization mode control register
pub mod swreg40;
/**SWREG41 (rw) register accessor: VENC stabilization motion sum div8 output register

You can [`read`](crate::Reg::read) this register and get [`swreg41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG41)

For information about available fields see [`mod@swreg41`] module*/
pub type SWREG41 = crate::Reg<swreg41::SWREG41rs>;
///VENC stabilization motion sum div8 output register
pub mod swreg41;
/**SWREG42 (rw) register accessor: VENC stabilization GMV output, matrix 1, up-left position output register

You can [`read`](crate::Reg::read) this register and get [`swreg42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG42)

For information about available fields see [`mod@swreg42`] module*/
pub type SWREG42 = crate::Reg<swreg42::SWREG42rs>;
///VENC stabilization GMV output, matrix 1, up-left position output register
pub mod swreg42;
/**SWREG43 (rw) register accessor: VENC stabilization GMV output, matrix 2, up position output register

You can [`read`](crate::Reg::read) this register and get [`swreg43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG43)

For information about available fields see [`mod@swreg43`] module*/
pub type SWREG43 = crate::Reg<swreg43::SWREG43rs>;
///VENC stabilization GMV output, matrix 2, up position output register
pub mod swreg43;
/**SWREG44 (rw) register accessor: VENC stabilization matrix 3, up-right position output register

You can [`read`](crate::Reg::read) this register and get [`swreg44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG44)

For information about available fields see [`mod@swreg44`] module*/
pub type SWREG44 = crate::Reg<swreg44::SWREG44rs>;
///VENC stabilization matrix 3, up-right position output register
pub mod swreg44;
/**SWREG45 (rw) register accessor: VENC stabilization matrix 4, left position output register

You can [`read`](crate::Reg::read) this register and get [`swreg45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG45)

For information about available fields see [`mod@swreg45`] module*/
pub type SWREG45 = crate::Reg<swreg45::SWREG45rs>;
///VENC stabilization matrix 4, left position output register
pub mod swreg45;
/**SWREG46 (rw) register accessor: VENC stabilization matrix 5, GMV position output register

You can [`read`](crate::Reg::read) this register and get [`swreg46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG46)

For information about available fields see [`mod@swreg46`] module*/
pub type SWREG46 = crate::Reg<swreg46::SWREG46rs>;
///VENC stabilization matrix 5, GMV position output register
pub mod swreg46;
/**SWREG47 (rw) register accessor: VENC stabilization matrix 6, right position output register

You can [`read`](crate::Reg::read) this register and get [`swreg47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG47)

For information about available fields see [`mod@swreg47`] module*/
pub type SWREG47 = crate::Reg<swreg47::SWREG47rs>;
///VENC stabilization matrix 6, right position output register
pub mod swreg47;
/**SWREG48 (rw) register accessor: VENC stabilization matrix 7, down-left position output register

You can [`read`](crate::Reg::read) this register and get [`swreg48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG48)

For information about available fields see [`mod@swreg48`] module*/
pub type SWREG48 = crate::Reg<swreg48::SWREG48rs>;
///VENC stabilization matrix 7, down-left position output register
pub mod swreg48;
/**SWREG49 (rw) register accessor: VENC stabilization matrix 8, down position output register

You can [`read`](crate::Reg::read) this register and get [`swreg49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG49)

For information about available fields see [`mod@swreg49`] module*/
pub type SWREG49 = crate::Reg<swreg49::SWREG49rs>;
///VENC stabilization matrix 8, down position output register
pub mod swreg49;
/**SWREG50 (rw) register accessor: VENC stabilization matrix 9, down-right position output register

You can [`read`](crate::Reg::read) this register and get [`swreg50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG50)

For information about available fields see [`mod@swreg50`] module*/
pub type SWREG50 = crate::Reg<swreg50::SWREG50rs>;
///VENC stabilization matrix 9, down-right position output register
pub mod swreg50;
/**SWREG51 (rw) register accessor: VENC base address for cabac context tables H264 register

You can [`read`](crate::Reg::read) this register and get [`swreg51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG51)

For information about available fields see [`mod@swreg51`] module*/
pub type SWREG51 = crate::Reg<swreg51::SWREG51rs>;
///VENC base address for cabac context tables H264 register
pub mod swreg51;
/**SWREG52 (rw) register accessor: VENC base address for MV output writing register

You can [`read`](crate::Reg::read) this register and get [`swreg52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG52)

For information about available fields see [`mod@swreg52`] module*/
pub type SWREG52 = crate::Reg<swreg52::SWREG52rs>;
///VENC base address for MV output writing register
pub mod swreg52;
/**SWREG53 (rw) register accessor: VENC RGB to YUV conversion coefficient A - B register

You can [`read`](crate::Reg::read) this register and get [`swreg53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG53)

For information about available fields see [`mod@swreg53`] module*/
pub type SWREG53 = crate::Reg<swreg53::SWREG53rs>;
///VENC RGB to YUV conversion coefficient A - B register
pub mod swreg53;
/**SWREG54 (rw) register accessor: VENC RGB to YUV conversion coefficient C - E register

You can [`read`](crate::Reg::read) this register and get [`swreg54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG54)

For information about available fields see [`mod@swreg54`] module*/
pub type SWREG54 = crate::Reg<swreg54::SWREG54rs>;
///VENC RGB to YUV conversion coefficient C - E register
pub mod swreg54;
/**SWREG55 (rw) register accessor: VENC RGB to YUV conversion coefficient F, RGB mask MSB bit position register

You can [`read`](crate::Reg::read) this register and get [`swreg55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG55)

For information about available fields see [`mod@swreg55`] module*/
pub type SWREG55 = crate::Reg<swreg55::SWREG55rs>;
///VENC RGB to YUV conversion coefficient F, RGB mask MSB bit position register
pub mod swreg55;
/**SWREG56 (rw) register accessor: VENC intra area register

You can [`read`](crate::Reg::read) this register and get [`swreg56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG56)

For information about available fields see [`mod@swreg56`] module*/
pub type SWREG56 = crate::Reg<swreg56::SWREG56rs>;
///VENC intra area register
pub mod swreg56;
/**SWREG57 (rw) register accessor: VENC CIR intra mb position register

You can [`read`](crate::Reg::read) this register and get [`swreg57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG57)

For information about available fields see [`mod@swreg57`] module*/
pub type SWREG57 = crate::Reg<swreg57::SWREG57rs>;
///VENC CIR intra mb position register
pub mod swreg57;
/**SWREG58 (rw) register accessor: VENC intra slice bitmap for slices 0..31/base address for 1st DCT partition register

You can [`read`](crate::Reg::read) this register and get [`swreg58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG58)

For information about available fields see [`mod@swreg58`] module*/
pub type SWREG58 = crate::Reg<swreg58::SWREG58rs>;
///VENC intra slice bitmap for slices 0..31/base address for 1st DCT partition register
pub mod swreg58;
/**SWREG59 (rw) register accessor: VENC intra slice bitmap for slices 32..63/base address for 2nd DCT partition register

You can [`read`](crate::Reg::read) this register and get [`swreg59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG59)

For information about available fields see [`mod@swreg59`] module*/
pub type SWREG59 = crate::Reg<swreg59::SWREG59rs>;
///VENC intra slice bitmap for slices 32..63/base address for 2nd DCT partition register
pub mod swreg59;
/**SWREG60 (rw) register accessor: VENC 1st ROI area register

You can [`read`](crate::Reg::read) this register and get [`swreg60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG60)

For information about available fields see [`mod@swreg60`] module*/
pub type SWREG60 = crate::Reg<swreg60::SWREG60rs>;
///VENC 1st ROI area register
pub mod swreg60;
/**SWREG61 (rw) register accessor: VENC 2nd ROI area register

You can [`read`](crate::Reg::read) this register and get [`swreg61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG61)

For information about available fields see [`mod@swreg61`] module*/
pub type SWREG61 = crate::Reg<swreg61::SWREG61rs>;
///VENC 2nd ROI area register
pub mod swreg61;
/**SWREG62 (rw) register accessor: VENC ROI area delta QP, MV register

You can [`read`](crate::Reg::read) this register and get [`swreg62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG62)

For information about available fields see [`mod@swreg62`] module*/
pub type SWREG62 = crate::Reg<swreg62::SWREG62rs>;
///VENC ROI area delta QP, MV register
pub mod swreg62;
/**SWREG63 (r) register accessor: VENC synthesis configuration register encoder 0 register

You can [`read`](crate::Reg::read) this register and get [`swreg63::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG63)

For information about available fields see [`mod@swreg63`] module*/
pub type SWREG63 = crate::Reg<swreg63::SWREG63rs>;
///VENC synthesis configuration register encoder 0 register
pub mod swreg63;
/**SWREG64 (rw) register accessor: VENC JPEG luma quantization 1/intra 16x16 mode 0-1 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG64)

For information about available fields see [`mod@swreg64`] module*/
pub type SWREG64 = crate::Reg<swreg64::SWREG64rs>;
///VENC JPEG luma quantization 1/intra 16x16 mode 0-1 penalty register
pub mod swreg64;
/**SWREG65 (rw) register accessor: VENC JPEG luma quantization 2/intra 16x16 mode 2-3 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG65)

For information about available fields see [`mod@swreg65`] module*/
pub type SWREG65 = crate::Reg<swreg65::SWREG65rs>;
///VENC JPEG luma quantization 2/intra 16x16 mode 2-3 penalty register
pub mod swreg65;
/**SWREG66 (rw) register accessor: VENC JPEG luma quantization 3/intra 4x4 mode 0-1 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG66)

For information about available fields see [`mod@swreg66`] module*/
pub type SWREG66 = crate::Reg<swreg66::SWREG66rs>;
///VENC JPEG luma quantization 3/intra 4x4 mode 0-1 penalty register
pub mod swreg66;
/**SWREG67 (rw) register accessor: VENC JPEG luma quantization 4/intra 4x4 mode 2-3 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG67)

For information about available fields see [`mod@swreg67`] module*/
pub type SWREG67 = crate::Reg<swreg67::SWREG67rs>;
///VENC JPEG luma quantization 4/intra 4x4 mode 2-3 penalty register
pub mod swreg67;
/**SWREG68 (rw) register accessor: VENC JPEG luma quantization 5/intra 4x4 mode 4-5 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG68)

For information about available fields see [`mod@swreg68`] module*/
pub type SWREG68 = crate::Reg<swreg68::SWREG68rs>;
///VENC JPEG luma quantization 5/intra 4x4 mode 4-5 penalty register
pub mod swreg68;
/**SWREG69 (rw) register accessor: VENC JPEG luma quantization 6/intra 4x4 mode 6-7 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG69)

For information about available fields see [`mod@swreg69`] module*/
pub type SWREG69 = crate::Reg<swreg69::SWREG69rs>;
///VENC JPEG luma quantization 6/intra 4x4 mode 6-7 penalty register
pub mod swreg69;
/**SWREG70 (rw) register accessor: VENC JPEG luma quantization 7/intra 4x4 mode 8-9 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG70)

For information about available fields see [`mod@swreg70`] module*/
pub type SWREG70 = crate::Reg<swreg70::SWREG70rs>;
///VENC JPEG luma quantization 7/intra 4x4 mode 8-9 penalty register
pub mod swreg70;
/**SWREG71 (rw) register accessor: VENC JPEG luma quantization 8/base address for segmentation map register

You can [`read`](crate::Reg::read) this register and get [`swreg71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG71)

For information about available fields see [`mod@swreg71`] module*/
pub type SWREG71 = crate::Reg<swreg71::SWREG71rs>;
///VENC JPEG luma quantization 8/base address for segmentation map register
pub mod swreg71;
/**SWREG72 (rw) register accessor: VENC JPEG luma quantization 9/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG72)

For information about available fields see [`mod@swreg72`] module*/
pub type SWREG72 = crate::Reg<swreg72::SWREG72rs>;
///VENC JPEG luma quantization 9/segment1 parameter register
pub mod swreg72;
/**SWREG73 (rw) register accessor: VENC JPEG luma quantization 10/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG73)

For information about available fields see [`mod@swreg73`] module*/
pub type SWREG73 = crate::Reg<swreg73::SWREG73rs>;
///VENC JPEG luma quantization 10/segment1 parameter register
pub mod swreg73;
/**SWREG74 (rw) register accessor: VENC JPEG luma quantization 11/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG74)

For information about available fields see [`mod@swreg74`] module*/
pub type SWREG74 = crate::Reg<swreg74::SWREG74rs>;
///VENC JPEG luma quantization 11/segment1 parameter register
pub mod swreg74;
/**SWREG75 (rw) register accessor: VENC JPEG luma quantization 12/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG75)

For information about available fields see [`mod@swreg75`] module*/
pub type SWREG75 = crate::Reg<swreg75::SWREG75rs>;
///VENC JPEG luma quantization 12/segment1 parameter register
pub mod swreg75;
/**SWREG76 (rw) register accessor: VENC JPEG luma quantization 13/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG76)

For information about available fields see [`mod@swreg76`] module*/
pub type SWREG76 = crate::Reg<swreg76::SWREG76rs>;
///VENC JPEG luma quantization 13/segment1 parameter register
pub mod swreg76;
/**SWREG77 (rw) register accessor: VENC JPEG luma quantization 14/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG77)

For information about available fields see [`mod@swreg77`] module*/
pub type SWREG77 = crate::Reg<swreg77::SWREG77rs>;
///VENC JPEG luma quantization 14/segment1 parameter register
pub mod swreg77;
/**SWREG78 (rw) register accessor: VENC JPEG luma quantization 15/segment1 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG78)

For information about available fields see [`mod@swreg78`] module*/
pub type SWREG78 = crate::Reg<swreg78::SWREG78rs>;
///VENC JPEG luma quantization 15/segment1 parameter register
pub mod swreg78;
/**SWREG79 (rw) register accessor: VENC JPEG luma quantization 16/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG79)

For information about available fields see [`mod@swreg79`] module*/
pub type SWREG79 = crate::Reg<swreg79::SWREG79rs>;
///VENC JPEG luma quantization 16/segment2 parameter register
pub mod swreg79;
/**SWREG80 (rw) register accessor: VENC JPEG chroma quantization 1/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG80)

For information about available fields see [`mod@swreg80`] module*/
pub type SWREG80 = crate::Reg<swreg80::SWREG80rs>;
///VENC JPEG chroma quantization 1/segment2 parameter register
pub mod swreg80;
/**SWREG81 (rw) register accessor: VENC JPEG chroma quantization 2/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG81)

For information about available fields see [`mod@swreg81`] module*/
pub type SWREG81 = crate::Reg<swreg81::SWREG81rs>;
///VENC JPEG chroma quantization 2/segment2 parameter register
pub mod swreg81;
/**SWREG82 (rw) register accessor: VENC JPEG chroma quantization 3/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG82)

For information about available fields see [`mod@swreg82`] module*/
pub type SWREG82 = crate::Reg<swreg82::SWREG82rs>;
///VENC JPEG chroma quantization 3/segment2 parameter register
pub mod swreg82;
/**SWREG83 (rw) register accessor: VENC JPEG chroma quantization 4/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG83)

For information about available fields see [`mod@swreg83`] module*/
pub type SWREG83 = crate::Reg<swreg83::SWREG83rs>;
///VENC JPEG chroma quantization 4/segment2 parameter register
pub mod swreg83;
/**SWREG84 (rw) register accessor: VENC JPEG chroma quantization 5/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG84)

For information about available fields see [`mod@swreg84`] module*/
pub type SWREG84 = crate::Reg<swreg84::SWREG84rs>;
///VENC JPEG chroma quantization 5/segment2 parameter register
pub mod swreg84;
/**SWREG85 (rw) register accessor: VENC JPEG chroma quantization 6/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG85)

For information about available fields see [`mod@swreg85`] module*/
pub type SWREG85 = crate::Reg<swreg85::SWREG85rs>;
///VENC JPEG chroma quantization 6/segment2 parameter register
pub mod swreg85;
/**SWREG86 (rw) register accessor: VENC JPEG chroma quantization 7/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG86)

For information about available fields see [`mod@swreg86`] module*/
pub type SWREG86 = crate::Reg<swreg86::SWREG86rs>;
///VENC JPEG chroma quantization 7/segment2 parameter register
pub mod swreg86;
/**SWREG87 (rw) register accessor: VENC JPEG chroma quantization 8/segment2 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG87)

For information about available fields see [`mod@swreg87`] module*/
pub type SWREG87 = crate::Reg<swreg87::SWREG87rs>;
///VENC JPEG chroma quantization 8/segment2 parameter register
pub mod swreg87;
/**SWREG88 (rw) register accessor: VENC JPEG chroma quantization 9/segment3 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG88)

For information about available fields see [`mod@swreg88`] module*/
pub type SWREG88 = crate::Reg<swreg88::SWREG88rs>;
///VENC JPEG chroma quantization 9/segment3 parameter register
pub mod swreg88;
/**SWREG89 (rw) register accessor: VENC JPEG chroma quantization 10/segment3 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG89)

For information about available fields see [`mod@swreg89`] module*/
pub type SWREG89 = crate::Reg<swreg89::SWREG89rs>;
///VENC JPEG chroma quantization 10/segment3 parameter register
pub mod swreg89;
/**SWREG90 (rw) register accessor: VENC JPEG chroma quantization 11/segment3 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG90)

For information about available fields see [`mod@swreg90`] module*/
pub type SWREG90 = crate::Reg<swreg90::SWREG90rs>;
///VENC JPEG chroma quantization 11/segment3 parameter register
pub mod swreg90;
/**SWREG91 (rw) register accessor: VENC JPEG chroma quantization 12/segment3 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG91)

For information about available fields see [`mod@swreg91`] module*/
pub type SWREG91 = crate::Reg<swreg91::SWREG91rs>;
///VENC JPEG chroma quantization 12/segment3 parameter register
pub mod swreg91;
/**SWREG92 (rw) register accessor: VENC JPEG chroma quantization 13/segment3 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG92)

For information about available fields see [`mod@swreg92`] module*/
pub type SWREG92 = crate::Reg<swreg92::SWREG92rs>;
///VENC JPEG chroma quantization 13/segment3 parameter register
pub mod swreg92;
/**SWREG93 (rw) register accessor: VENC JPEG chroma quantization 14/segment3 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG93)

For information about available fields see [`mod@swreg93`] module*/
pub type SWREG93 = crate::Reg<swreg93::SWREG93rs>;
///VENC JPEG chroma quantization 14/segment3 parameter register
pub mod swreg93;
/**SWREG94 (rw) register accessor: VENC JPEG chroma quantization 15/segment3 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG94)

For information about available fields see [`mod@swreg94`] module*/
pub type SWREG94 = crate::Reg<swreg94::SWREG94rs>;
///VENC JPEG chroma quantization 15/segment3 parameter register
pub mod swreg94;
/**SWREG95 (rw) register accessor: VENC JPEG chroma quantization 16/segment3 parameter register

You can [`read`](crate::Reg::read) this register and get [`swreg95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG95)

For information about available fields see [`mod@swreg95`] module*/
pub type SWREG95 = crate::Reg<swreg95::SWREG95rs>;
///VENC JPEG chroma quantization 16/segment3 parameter register
pub mod swreg95;
/**SWREG96 (w) register accessor: VENC DMV 4p/1p penalty values 0-3 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg96::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG96)

For information about available fields see [`mod@swreg96`] module*/
pub type SWREG96 = crate::Reg<swreg96::SWREG96rs>;
///VENC DMV 4p/1p penalty values 0-3 register
pub mod swreg96;
/**SWREG97 (w) register accessor: VENC DMV 4p/1p penalty values 4-7 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg97::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG97)

For information about available fields see [`mod@swreg97`] module*/
pub type SWREG97 = crate::Reg<swreg97::SWREG97rs>;
///VENC DMV 4p/1p penalty values 4-7 register
pub mod swreg97;
/**SWREG98 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg98::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG98)

For information about available fields see [`mod@swreg98`] module*/
pub type SWREG98 = crate::Reg<swreg98::SWREG98rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg98;
/**SWREG99 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg99::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG99)

For information about available fields see [`mod@swreg99`] module*/
pub type SWREG99 = crate::Reg<swreg99::SWREG99rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg99;
/**SWREG100 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg100::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG100)

For information about available fields see [`mod@swreg100`] module*/
pub type SWREG100 = crate::Reg<swreg100::SWREG100rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg100;
/**SWREG101 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg101::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG101)

For information about available fields see [`mod@swreg101`] module*/
pub type SWREG101 = crate::Reg<swreg101::SWREG101rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg101;
/**SWREG102 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg102::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG102)

For information about available fields see [`mod@swreg102`] module*/
pub type SWREG102 = crate::Reg<swreg102::SWREG102rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg102;
/**SWREG103 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg103::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG103)

For information about available fields see [`mod@swreg103`] module*/
pub type SWREG103 = crate::Reg<swreg103::SWREG103rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg103;
/**SWREG104 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg104::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG104)

For information about available fields see [`mod@swreg104`] module*/
pub type SWREG104 = crate::Reg<swreg104::SWREG104rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg104;
/**SWREG105 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg105::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG105)

For information about available fields see [`mod@swreg105`] module*/
pub type SWREG105 = crate::Reg<swreg105::SWREG105rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg105;
/**SWREG106 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg106::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG106)

For information about available fields see [`mod@swreg106`] module*/
pub type SWREG106 = crate::Reg<swreg106::SWREG106rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg106;
/**SWREG107 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg107::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG107)

For information about available fields see [`mod@swreg107`] module*/
pub type SWREG107 = crate::Reg<swreg107::SWREG107rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg107;
/**SWREG108 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg108::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG108)

For information about available fields see [`mod@swreg108`] module*/
pub type SWREG108 = crate::Reg<swreg108::SWREG108rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg108;
/**SWREG109 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg109::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG109)

For information about available fields see [`mod@swreg109`] module*/
pub type SWREG109 = crate::Reg<swreg109::SWREG109rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg109;
/**SWREG110 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg110::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG110)

For information about available fields see [`mod@swreg110`] module*/
pub type SWREG110 = crate::Reg<swreg110::SWREG110rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg110;
/**SWREG111 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg111::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG111)

For information about available fields see [`mod@swreg111`] module*/
pub type SWREG111 = crate::Reg<swreg111::SWREG111rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg111;
/**SWREG112 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg112::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG112)

For information about available fields see [`mod@swreg112`] module*/
pub type SWREG112 = crate::Reg<swreg112::SWREG112rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg112;
/**SWREG113 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg113::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG113)

For information about available fields see [`mod@swreg113`] module*/
pub type SWREG113 = crate::Reg<swreg113::SWREG113rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg113;
/**SWREG114 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg114::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG114)

For information about available fields see [`mod@swreg114`] module*/
pub type SWREG114 = crate::Reg<swreg114::SWREG114rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg114;
/**SWREG115 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg115::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG115)

For information about available fields see [`mod@swreg115`] module*/
pub type SWREG115 = crate::Reg<swreg115::SWREG115rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg115;
/**SWREG116 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg116::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG116)

For information about available fields see [`mod@swreg116`] module*/
pub type SWREG116 = crate::Reg<swreg116::SWREG116rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg116;
/**SWREG117 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg117::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG117)

For information about available fields see [`mod@swreg117`] module*/
pub type SWREG117 = crate::Reg<swreg117::SWREG117rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg117;
/**SWREG118 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg118::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG118)

For information about available fields see [`mod@swreg118`] module*/
pub type SWREG118 = crate::Reg<swreg118::SWREG118rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg118;
/**SWREG119 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg119::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG119)

For information about available fields see [`mod@swreg119`] module*/
pub type SWREG119 = crate::Reg<swreg119::SWREG119rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg119;
/**SWREG120 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg120::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG120)

For information about available fields see [`mod@swreg120`] module*/
pub type SWREG120 = crate::Reg<swreg120::SWREG120rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg120;
/**SWREG121 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg121::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG121)

For information about available fields see [`mod@swreg121`] module*/
pub type SWREG121 = crate::Reg<swreg121::SWREG121rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg121;
/**SWREG122 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg122::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG122)

For information about available fields see [`mod@swreg122`] module*/
pub type SWREG122 = crate::Reg<swreg122::SWREG122rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg122;
/**SWREG123 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg123::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG123)

For information about available fields see [`mod@swreg123`] module*/
pub type SWREG123 = crate::Reg<swreg123::SWREG123rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg123;
/**SWREG124 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg124::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG124)

For information about available fields see [`mod@swreg124`] module*/
pub type SWREG124 = crate::Reg<swreg124::SWREG124rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg124;
/**SWREG125 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg125::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG125)

For information about available fields see [`mod@swreg125`] module*/
pub type SWREG125 = crate::Reg<swreg125::SWREG125rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg125;
/**SWREG126 (w) register accessor: VENC DMV 4p/1p penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg126::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG126)

For information about available fields see [`mod@swreg126`] module*/
pub type SWREG126 = crate::Reg<swreg126::SWREG126rs>;
///VENC DMV 4p/1p penalty values register
pub mod swreg126;
/**SWREG127 (w) register accessor: VENC DMV 4p/1p penalty values 124-127 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg127::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG127)

For information about available fields see [`mod@swreg127`] module*/
pub type SWREG127 = crate::Reg<swreg127::SWREG127rs>;
///VENC DMV 4p/1p penalty values 124-127 register
pub mod swreg127;
/**SWREG128 (w) register accessor: VENC DMV qpel penalty values 0-3 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg128::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG128)

For information about available fields see [`mod@swreg128`] module*/
pub type SWREG128 = crate::Reg<swreg128::SWREG128rs>;
///VENC DMV qpel penalty values 0-3 register
pub mod swreg128;
/**SWREG129 (w) register accessor: VENC DMV qpel penalty values 4-7 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg129::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG129)

For information about available fields see [`mod@swreg129`] module*/
pub type SWREG129 = crate::Reg<swreg129::SWREG129rs>;
///VENC DMV qpel penalty values 4-7 register
pub mod swreg129;
/**SWREG130 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg130::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG130)

For information about available fields see [`mod@swreg130`] module*/
pub type SWREG130 = crate::Reg<swreg130::SWREG130rs>;
///VENC DMV qpel penalty values register
pub mod swreg130;
/**SWREG131 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg131::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG131)

For information about available fields see [`mod@swreg131`] module*/
pub type SWREG131 = crate::Reg<swreg131::SWREG131rs>;
///VENC DMV qpel penalty values register
pub mod swreg131;
/**SWREG132 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg132::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG132)

For information about available fields see [`mod@swreg132`] module*/
pub type SWREG132 = crate::Reg<swreg132::SWREG132rs>;
///VENC DMV qpel penalty values register
pub mod swreg132;
/**SWREG133 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg133::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG133)

For information about available fields see [`mod@swreg133`] module*/
pub type SWREG133 = crate::Reg<swreg133::SWREG133rs>;
///VENC DMV qpel penalty values register
pub mod swreg133;
/**SWREG134 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg134::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG134)

For information about available fields see [`mod@swreg134`] module*/
pub type SWREG134 = crate::Reg<swreg134::SWREG134rs>;
///VENC DMV qpel penalty values register
pub mod swreg134;
/**SWREG135 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg135::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG135)

For information about available fields see [`mod@swreg135`] module*/
pub type SWREG135 = crate::Reg<swreg135::SWREG135rs>;
///VENC DMV qpel penalty values register
pub mod swreg135;
/**SWREG136 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg136::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG136)

For information about available fields see [`mod@swreg136`] module*/
pub type SWREG136 = crate::Reg<swreg136::SWREG136rs>;
///VENC DMV qpel penalty values register
pub mod swreg136;
/**SWREG137 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg137::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG137)

For information about available fields see [`mod@swreg137`] module*/
pub type SWREG137 = crate::Reg<swreg137::SWREG137rs>;
///VENC DMV qpel penalty values register
pub mod swreg137;
/**SWREG138 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg138::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG138)

For information about available fields see [`mod@swreg138`] module*/
pub type SWREG138 = crate::Reg<swreg138::SWREG138rs>;
///VENC DMV qpel penalty values register
pub mod swreg138;
/**SWREG139 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg139::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG139)

For information about available fields see [`mod@swreg139`] module*/
pub type SWREG139 = crate::Reg<swreg139::SWREG139rs>;
///VENC DMV qpel penalty values register
pub mod swreg139;
/**SWREG140 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg140::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG140)

For information about available fields see [`mod@swreg140`] module*/
pub type SWREG140 = crate::Reg<swreg140::SWREG140rs>;
///VENC DMV qpel penalty values register
pub mod swreg140;
/**SWREG141 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg141::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG141)

For information about available fields see [`mod@swreg141`] module*/
pub type SWREG141 = crate::Reg<swreg141::SWREG141rs>;
///VENC DMV qpel penalty values register
pub mod swreg141;
/**SWREG142 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg142::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG142)

For information about available fields see [`mod@swreg142`] module*/
pub type SWREG142 = crate::Reg<swreg142::SWREG142rs>;
///VENC DMV qpel penalty values register
pub mod swreg142;
/**SWREG143 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg143::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG143)

For information about available fields see [`mod@swreg143`] module*/
pub type SWREG143 = crate::Reg<swreg143::SWREG143rs>;
///VENC DMV qpel penalty values register
pub mod swreg143;
/**SWREG144 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg144::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG144)

For information about available fields see [`mod@swreg144`] module*/
pub type SWREG144 = crate::Reg<swreg144::SWREG144rs>;
///VENC DMV qpel penalty values register
pub mod swreg144;
/**SWREG145 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg145::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG145)

For information about available fields see [`mod@swreg145`] module*/
pub type SWREG145 = crate::Reg<swreg145::SWREG145rs>;
///VENC DMV qpel penalty values register
pub mod swreg145;
/**SWREG146 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg146::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG146)

For information about available fields see [`mod@swreg146`] module*/
pub type SWREG146 = crate::Reg<swreg146::SWREG146rs>;
///VENC DMV qpel penalty values register
pub mod swreg146;
/**SWREG147 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg147::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG147)

For information about available fields see [`mod@swreg147`] module*/
pub type SWREG147 = crate::Reg<swreg147::SWREG147rs>;
///VENC DMV qpel penalty values register
pub mod swreg147;
/**SWREG148 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg148::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG148)

For information about available fields see [`mod@swreg148`] module*/
pub type SWREG148 = crate::Reg<swreg148::SWREG148rs>;
///VENC DMV qpel penalty values register
pub mod swreg148;
/**SWREG149 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg149::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG149)

For information about available fields see [`mod@swreg149`] module*/
pub type SWREG149 = crate::Reg<swreg149::SWREG149rs>;
///VENC DMV qpel penalty values register
pub mod swreg149;
/**SWREG150 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg150::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG150)

For information about available fields see [`mod@swreg150`] module*/
pub type SWREG150 = crate::Reg<swreg150::SWREG150rs>;
///VENC DMV qpel penalty values register
pub mod swreg150;
/**SWREG151 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg151::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG151)

For information about available fields see [`mod@swreg151`] module*/
pub type SWREG151 = crate::Reg<swreg151::SWREG151rs>;
///VENC DMV qpel penalty values register
pub mod swreg151;
/**SWREG152 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg152::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG152)

For information about available fields see [`mod@swreg152`] module*/
pub type SWREG152 = crate::Reg<swreg152::SWREG152rs>;
///VENC DMV qpel penalty values register
pub mod swreg152;
/**SWREG153 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg153::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG153)

For information about available fields see [`mod@swreg153`] module*/
pub type SWREG153 = crate::Reg<swreg153::SWREG153rs>;
///VENC DMV qpel penalty values register
pub mod swreg153;
/**SWREG154 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg154::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG154)

For information about available fields see [`mod@swreg154`] module*/
pub type SWREG154 = crate::Reg<swreg154::SWREG154rs>;
///VENC DMV qpel penalty values register
pub mod swreg154;
/**SWREG155 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg155::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG155)

For information about available fields see [`mod@swreg155`] module*/
pub type SWREG155 = crate::Reg<swreg155::SWREG155rs>;
///VENC DMV qpel penalty values register
pub mod swreg155;
/**SWREG156 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg156::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG156)

For information about available fields see [`mod@swreg156`] module*/
pub type SWREG156 = crate::Reg<swreg156::SWREG156rs>;
///VENC DMV qpel penalty values register
pub mod swreg156;
/**SWREG157 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg157::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG157)

For information about available fields see [`mod@swreg157`] module*/
pub type SWREG157 = crate::Reg<swreg157::SWREG157rs>;
///VENC DMV qpel penalty values register
pub mod swreg157;
/**SWREG158 (w) register accessor: VENC DMV qpel penalty values register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg158::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG158)

For information about available fields see [`mod@swreg158`] module*/
pub type SWREG158 = crate::Reg<swreg158::SWREG158rs>;
///VENC DMV qpel penalty values register
pub mod swreg158;
/**SWREG159 (w) register accessor: VENC DMV qpel penalty values 124-127 register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg159::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG159)

For information about available fields see [`mod@swreg159`] module*/
pub type SWREG159 = crate::Reg<swreg159::SWREG159rs>;
///VENC DMV qpel penalty values 124-127 register
pub mod swreg159;
/**SWREG231 (rw) register accessor: VENC base address for output of down-scaled encoder image in YUYV 4:2:2 format register

You can [`read`](crate::Reg::read) this register and get [`swreg231::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg231::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG231)

For information about available fields see [`mod@swreg231`] module*/
pub type SWREG231 = crate::Reg<swreg231::SWREG231rs>;
///VENC base address for output of down-scaled encoder image in YUYV 4:2:2 format register
pub mod swreg231;
/**SWREG232 (rw) register accessor: VENC scaling control register

You can [`read`](crate::Reg::read) this register and get [`swreg232::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg232::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG232)

For information about available fields see [`mod@swreg232`] module*/
pub type SWREG232 = crate::Reg<swreg232::SWREG232rs>;
///VENC scaling control register
pub mod swreg232;
/**SWREG233 (rw) register accessor: VENC scaling control register

You can [`read`](crate::Reg::read) this register and get [`swreg233::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg233::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG233)

For information about available fields see [`mod@swreg233`] module*/
pub type SWREG233 = crate::Reg<swreg233::SWREG233rs>;
///VENC scaling control register
pub mod swreg233;
/**SWREG236 (rw) register accessor: VENC squared error output calculated for 13x13 pixels per macroblock register

You can [`read`](crate::Reg::read) this register and get [`swreg236::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg236::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG236)

For information about available fields see [`mod@swreg236`] module*/
pub type SWREG236 = crate::Reg<swreg236::SWREG236rs>;
///VENC squared error output calculated for 13x13 pixels per macroblock register
pub mod swreg236;
/**SWREG237 (rw) register accessor: VENC MAD 2 control and output register

You can [`read`](crate::Reg::read) this register and get [`swreg237::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg237::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG237)

For information about available fields see [`mod@swreg237`] module*/
pub type SWREG237 = crate::Reg<swreg237::SWREG237rs>;
///VENC MAD 2 control and output register
pub mod swreg237;
/**SWREG238 (rw) register accessor: VENC MAD 3 control and output register

You can [`read`](crate::Reg::read) this register and get [`swreg238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG238)

For information about available fields see [`mod@swreg238`] module*/
pub type SWREG238 = crate::Reg<swreg238::SWREG238rs>;
///VENC MAD 3 control and output register
pub mod swreg238;
/**SWREG256 (rw) register accessor: VENC segment 1: intra 16x16 mode 0-2 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg256::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg256::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG256)

For information about available fields see [`mod@swreg256`] module*/
pub type SWREG256 = crate::Reg<swreg256::SWREG256rs>;
///VENC segment 1: intra 16x16 mode 0-2 penalty register
pub mod swreg256;
/**SWREG257 (rw) register accessor: VENC segment 1: intra 16x16 mode 3, intra 4x4 0-1 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg257::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg257::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG257)

For information about available fields see [`mod@swreg257`] module*/
pub type SWREG257 = crate::Reg<swreg257::SWREG257rs>;
///VENC segment 1: intra 16x16 mode 3, intra 4x4 0-1 penalty register
pub mod swreg257;
/**SWREG258 (rw) register accessor: VENC segment 1: intra 4x4 mode 2-4 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg258::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg258::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG258)

For information about available fields see [`mod@swreg258`] module*/
pub type SWREG258 = crate::Reg<swreg258::SWREG258rs>;
///VENC segment 1: intra 4x4 mode 2-4 penalty register
pub mod swreg258;
/**SWREG259 (rw) register accessor: VENC segment 1: intra 4x4 mode 5-7 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg259::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg259::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG259)

For information about available fields see [`mod@swreg259`] module*/
pub type SWREG259 = crate::Reg<swreg259::SWREG259rs>;
///VENC segment 1: intra 4x4 mode 5-7 penalty register
pub mod swreg259;
/**SWREG260 (rw) register accessor: VENC segment 1: intra 4x4 mode 8-9 penalty, previous mode favor for H.264 register

You can [`read`](crate::Reg::read) this register and get [`swreg260::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg260::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG260)

For information about available fields see [`mod@swreg260`] module*/
pub type SWREG260 = crate::Reg<swreg260::SWREG260rs>;
///VENC segment 1: intra 4x4 mode 8-9 penalty, previous mode favor for H.264 register
pub mod swreg260;
/**SWREG261 (rw) register accessor: VENC segment 1: bit cost of inter type, intra 16x16 mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg261::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg261::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG261)

For information about available fields see [`mod@swreg261`] module*/
pub type SWREG261 = crate::Reg<swreg261::SWREG261rs>;
///VENC segment 1: bit cost of inter type, intra 16x16 mode favor register
pub mod swreg261;
/**SWREG262 (rw) register accessor: VENC segment 1: inter MB mode favor, skip mode penalty, penalty value for 2nd reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg262::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg262::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG262)

For information about available fields see [`mod@swreg262`] module*/
pub type SWREG262 = crate::Reg<swreg262::SWREG262rs>;
///VENC segment 1: inter MB mode favor, skip mode penalty, penalty value for 2nd reference frame register
pub mod swreg262;
/**SWREG263 (rw) register accessor: VENC segment 1: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg263::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg263::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG263)

For information about available fields see [`mod@swreg263`] module*/
pub type SWREG263 = crate::Reg<swreg263::SWREG263rs>;
///VENC segment 1: penalty value register
pub mod swreg263;
/**SWREG264 (rw) register accessor: VENC segment 1: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg264::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg264::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG264)

For information about available fields see [`mod@swreg264`] module*/
pub type SWREG264 = crate::Reg<swreg264::SWREG264rs>;
///VENC segment 1: penalty value register
pub mod swreg264;
/**SWREG265 (rw) register accessor: VENC segment 1: deadzone rate multiplier for plane 0-1 register

You can [`read`](crate::Reg::read) this register and get [`swreg265::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg265::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG265)

For information about available fields see [`mod@swreg265`] module*/
pub type SWREG265 = crate::Reg<swreg265::SWREG265rs>;
///VENC segment 1: deadzone rate multiplier for plane 0-1 register
pub mod swreg265;
/**SWREG266 (rw) register accessor: VENC segment 1: deadzone rate multiplier for plane 2-3 register

You can [`read`](crate::Reg::read) this register and get [`swreg266::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg266::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG266)

For information about available fields see [`mod@swreg266`] module*/
pub type SWREG266 = crate::Reg<swreg266::SWREG266rs>;
///VENC segment 1: deadzone rate multiplier for plane 2-3 register
pub mod swreg266;
/**SWREG267 (rw) register accessor: VENC segment 1: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register

You can [`read`](crate::Reg::read) this register and get [`swreg267::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg267::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG267)

For information about available fields see [`mod@swreg267`] module*/
pub type SWREG267 = crate::Reg<swreg267::SWREG267rs>;
///VENC segment 1: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register
pub mod swreg267;
/**SWREG268 (rw) register accessor: VENC segment 2: intra 16x16 mode 0-2 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg268::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg268::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG268)

For information about available fields see [`mod@swreg268`] module*/
pub type SWREG268 = crate::Reg<swreg268::SWREG268rs>;
///VENC segment 2: intra 16x16 mode 0-2 penalty register
pub mod swreg268;
/**SWREG269 (rw) register accessor: VENC segment 2: intra 16x16 mode 3 penalty, intra 4x4 mode 0-1 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg269::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg269::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG269)

For information about available fields see [`mod@swreg269`] module*/
pub type SWREG269 = crate::Reg<swreg269::SWREG269rs>;
///VENC segment 2: intra 16x16 mode 3 penalty, intra 4x4 mode 0-1 penalty register
pub mod swreg269;
/**SWREG270 (rw) register accessor: VENC segment 2: intra 4x4 mode 2-4 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg270::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg270::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG270)

For information about available fields see [`mod@swreg270`] module*/
pub type SWREG270 = crate::Reg<swreg270::SWREG270rs>;
///VENC segment 2: intra 4x4 mode 2-4 penalty register
pub mod swreg270;
/**SWREG271 (rw) register accessor: VENC segment 2: intra 4x4 mode 5-7 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg271::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg271::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG271)

For information about available fields see [`mod@swreg271`] module*/
pub type SWREG271 = crate::Reg<swreg271::SWREG271rs>;
///VENC segment 2: intra 4x4 mode 5-7 penalty register
pub mod swreg271;
/**SWREG272 (rw) register accessor: VENC segment 2: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 register

You can [`read`](crate::Reg::read) this register and get [`swreg272::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg272::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG272)

For information about available fields see [`mod@swreg272`] module*/
pub type SWREG272 = crate::Reg<swreg272::SWREG272rs>;
///VENC segment 2: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 register
pub mod swreg272;
/**SWREG273 (rw) register accessor: VENC segment 2: bit cost of inter type, intra 16x16 mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg273::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg273::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG273)

For information about available fields see [`mod@swreg273`] module*/
pub type SWREG273 = crate::Reg<swreg273::SWREG273rs>;
///VENC segment 2: bit cost of inter type, intra 16x16 mode favor register
pub mod swreg273;
/**SWREG274 (rw) register accessor: VENC segment 2: inter MB mode favor, skip mode penalty, penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg274::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg274::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG274)

For information about available fields see [`mod@swreg274`] module*/
pub type SWREG274 = crate::Reg<swreg274::SWREG274rs>;
///VENC segment 2: inter MB mode favor, skip mode penalty, penalty value register
pub mod swreg274;
/**SWREG275 (rw) register accessor: VENC segment 2: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg275::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg275::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG275)

For information about available fields see [`mod@swreg275`] module*/
pub type SWREG275 = crate::Reg<swreg275::SWREG275rs>;
///VENC segment 2: penalty value register
pub mod swreg275;
/**SWREG276 (rw) register accessor: VENC segment 2: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg276::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg276::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG276)

For information about available fields see [`mod@swreg276`] module*/
pub type SWREG276 = crate::Reg<swreg276::SWREG276rs>;
///VENC segment 2: penalty value register
pub mod swreg276;
/**SWREG277 (rw) register accessor: VENC segment 2: deadzone rate multiplier for plane 0-1 register

You can [`read`](crate::Reg::read) this register and get [`swreg277::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg277::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG277)

For information about available fields see [`mod@swreg277`] module*/
pub type SWREG277 = crate::Reg<swreg277::SWREG277rs>;
///VENC segment 2: deadzone rate multiplier for plane 0-1 register
pub mod swreg277;
/**SWREG278 (rw) register accessor: VENC segment 2: deadzone rate multiplier for plane 2-3 register

You can [`read`](crate::Reg::read) this register and get [`swreg278::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg278::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG278)

For information about available fields see [`mod@swreg278`] module*/
pub type SWREG278 = crate::Reg<swreg278::SWREG278rs>;
///VENC segment 2: deadzone rate multiplier for plane 2-3 register
pub mod swreg278;
/**SWREG279 (rw) register accessor: VENC segment 2: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register

You can [`read`](crate::Reg::read) this register and get [`swreg279::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg279::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG279)

For information about available fields see [`mod@swreg279`] module*/
pub type SWREG279 = crate::Reg<swreg279::SWREG279rs>;
///VENC segment 2: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register
pub mod swreg279;
/**SWREG280 (rw) register accessor: VENC segment 3: intra 16x16 mode 0-2 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg280::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg280::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG280)

For information about available fields see [`mod@swreg280`] module*/
pub type SWREG280 = crate::Reg<swreg280::SWREG280rs>;
///VENC segment 3: intra 16x16 mode 0-2 penalty register
pub mod swreg280;
/**SWREG281 (rw) register accessor: VENC segment 3: intra 16x16 mode 3 penalty, intra 4x4 mode 0-1 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg281::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg281::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG281)

For information about available fields see [`mod@swreg281`] module*/
pub type SWREG281 = crate::Reg<swreg281::SWREG281rs>;
///VENC segment 3: intra 16x16 mode 3 penalty, intra 4x4 mode 0-1 penalty register
pub mod swreg281;
/**SWREG282 (rw) register accessor: VENC segment 3: intra 4x4 mode 2-4 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg282::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg282::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG282)

For information about available fields see [`mod@swreg282`] module*/
pub type SWREG282 = crate::Reg<swreg282::SWREG282rs>;
///VENC segment 3: intra 4x4 mode 2-4 penalty register
pub mod swreg282;
/**SWREG283 (rw) register accessor: VENC segment 3: intra 4x4 mode 5-7 penalty register

You can [`read`](crate::Reg::read) this register and get [`swreg283::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg283::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG283)

For information about available fields see [`mod@swreg283`] module*/
pub type SWREG283 = crate::Reg<swreg283::SWREG283rs>;
///VENC segment 3: intra 4x4 mode 5-7 penalty register
pub mod swreg283;
/**SWREG284 (rw) register accessor: VENC segment 3: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 register

You can [`read`](crate::Reg::read) this register and get [`swreg284::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg284::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG284)

For information about available fields see [`mod@swreg284`] module*/
pub type SWREG284 = crate::Reg<swreg284::SWREG284rs>;
///VENC segment 3: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 register
pub mod swreg284;
/**SWREG285 (rw) register accessor: VENC segment 3: bit cost of inter type, intra 16x16 mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg285::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg285::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG285)

For information about available fields see [`mod@swreg285`] module*/
pub type SWREG285 = crate::Reg<swreg285::SWREG285rs>;
///VENC segment 3: bit cost of inter type, intra 16x16 mode favor register
pub mod swreg285;
/**SWREG286 (rw) register accessor: VENC segment 3: inter MB mode favor in intra/inter selection, inter MB mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg286::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg286::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG286)

For information about available fields see [`mod@swreg286`] module*/
pub type SWREG286 = crate::Reg<swreg286::SWREG286rs>;
///VENC segment 3: inter MB mode favor in intra/inter selection, inter MB mode favor, penalty value for second reference frame register
pub mod swreg286;
/**SWREG287 (rw) register accessor: VENC segment 3: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg287::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg287::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG287)

For information about available fields see [`mod@swreg287`] module*/
pub type SWREG287 = crate::Reg<swreg287::SWREG287rs>;
///VENC segment 3: penalty value register
pub mod swreg287;
/**SWREG288 (rw) register accessor: VENC segment 3: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg288::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg288::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG288)

For information about available fields see [`mod@swreg288`] module*/
pub type SWREG288 = crate::Reg<swreg288::SWREG288rs>;
///VENC segment 3: penalty value register
pub mod swreg288;
/**SWREG289 (rw) register accessor: VENC segment 3: deadzone rate multiplier for plane 0-1 register

You can [`read`](crate::Reg::read) this register and get [`swreg289::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg289::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG289)

For information about available fields see [`mod@swreg289`] module*/
pub type SWREG289 = crate::Reg<swreg289::SWREG289rs>;
///VENC segment 3: deadzone rate multiplier for plane 0-1 register
pub mod swreg289;
/**SWREG290 (rw) register accessor: VENC segment 3: deadzone rate multiplier for plane 2-3 register

You can [`read`](crate::Reg::read) this register and get [`swreg290::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg290::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG290)

For information about available fields see [`mod@swreg290`] module*/
pub type SWREG290 = crate::Reg<swreg290::SWREG290rs>;
///VENC segment 3: deadzone rate multiplier for plane 2-3 register
pub mod swreg290;
/**SWREG291 (rw) register accessor: VENC segment 3: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register

You can [`read`](crate::Reg::read) this register and get [`swreg291::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg291::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG291)

For information about available fields see [`mod@swreg291`] module*/
pub type SWREG291 = crate::Reg<swreg291::SWREG291rs>;
///VENC segment 3: deadzone rate for macroblock skip token 0-1, dmv penalty coefficient register
pub mod swreg291;
/**SWREG294 (rw) register accessor: VENC Mb boost register

You can [`read`](crate::Reg::read) this register and get [`swreg294::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg294::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG294)

For information about available fields see [`mod@swreg294`] module*/
pub type SWREG294 = crate::Reg<swreg294::SWREG294rs>;
///VENC Mb boost register
pub mod swreg294;
/**SWREG295 (rw) register accessor: VENC variance control, Pskop conding mode register

You can [`read`](crate::Reg::read) this register and get [`swreg295::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg295::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG295)

For information about available fields see [`mod@swreg295`] module*/
pub type SWREG295 = crate::Reg<swreg295::SWREG295rs>;
///VENC variance control, Pskop conding mode register
pub mod swreg295;
/**SWREG296 (r) register accessor: VENC synthesis configuration register encoder 1 read only register

You can [`read`](crate::Reg::read) this register and get [`swreg296::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG296)

For information about available fields see [`mod@swreg296`] module*/
pub type SWREG296 = crate::Reg<swreg296::SWREG296rs>;
///VENC synthesis configuration register encoder 1 read only register
pub mod swreg296;
/**SWREG297 (rw) register accessor: VENC MBRC control register

You can [`read`](crate::Reg::read) this register and get [`swreg297::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg297::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG297)

For information about available fields see [`mod@swreg297`] module*/
pub type SWREG297 = crate::Reg<swreg297::SWREG297rs>;
///VENC MBRC control register
pub mod swreg297;
/**SWREG298 (rw) register accessor: VENC segment 4: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg298::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg298::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG298)

For information about available fields see [`mod@swreg298`] module*/
pub type SWREG298 = crate::Reg<swreg298::SWREG298rs>;
///VENC segment 4: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg298;
/**SWREG299 (rw) register accessor: VENC segment 4: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg299::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg299::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG299)

For information about available fields see [`mod@swreg299`] module*/
pub type SWREG299 = crate::Reg<swreg299::SWREG299rs>;
///VENC segment 4: skip mode penalty, inter MB mode favor register
pub mod swreg299;
/**SWREG300 (rw) register accessor: VENC segment 4: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg300::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg300::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG300)

For information about available fields see [`mod@swreg300`] module*/
pub type SWREG300 = crate::Reg<swreg300::SWREG300rs>;
///VENC segment 4: penalty value register
pub mod swreg300;
/**SWREG301 (rw) register accessor: VENC segment 4: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg301::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg301::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG301)

For information about available fields see [`mod@swreg301`] module*/
pub type SWREG301 = crate::Reg<swreg301::SWREG301rs>;
///VENC segment 4: penalty value register
pub mod swreg301;
/**SWREG302 (rw) register accessor: VENC segment 5: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg302::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg302::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG302)

For information about available fields see [`mod@swreg302`] module*/
pub type SWREG302 = crate::Reg<swreg302::SWREG302rs>;
///VENC segment 5: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg302;
/**SWREG303 (rw) register accessor: VENC segment 5: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg303::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg303::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG303)

For information about available fields see [`mod@swreg303`] module*/
pub type SWREG303 = crate::Reg<swreg303::SWREG303rs>;
///VENC segment 5: skip mode penalty, inter MB mode favor register
pub mod swreg303;
/**SWREG304 (rw) register accessor: VENC segment 5: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg304::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg304::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG304)

For information about available fields see [`mod@swreg304`] module*/
pub type SWREG304 = crate::Reg<swreg304::SWREG304rs>;
///VENC segment 5: penalty value register
pub mod swreg304;
/**SWREG305 (rw) register accessor: VENC segment 5: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg305::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg305::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG305)

For information about available fields see [`mod@swreg305`] module*/
pub type SWREG305 = crate::Reg<swreg305::SWREG305rs>;
///VENC segment 5: penalty value register
pub mod swreg305;
/**SWREG306 (rw) register accessor: VENC segment 6: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg306::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg306::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG306)

For information about available fields see [`mod@swreg306`] module*/
pub type SWREG306 = crate::Reg<swreg306::SWREG306rs>;
///VENC segment 6: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg306;
/**SWREG307 (rw) register accessor: VENC segment 6: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg307::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg307::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG307)

For information about available fields see [`mod@swreg307`] module*/
pub type SWREG307 = crate::Reg<swreg307::SWREG307rs>;
///VENC segment 6: skip mode penalty, inter MB mode favor register
pub mod swreg307;
/**SWREG308 (rw) register accessor: VENC segment 6: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg308::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg308::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG308)

For information about available fields see [`mod@swreg308`] module*/
pub type SWREG308 = crate::Reg<swreg308::SWREG308rs>;
///VENC segment 6: penalty value register
pub mod swreg308;
/**SWREG309 (rw) register accessor: VENC segment 6: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg309::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg309::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG309)

For information about available fields see [`mod@swreg309`] module*/
pub type SWREG309 = crate::Reg<swreg309::SWREG309rs>;
///VENC segment 6: penalty value register
pub mod swreg309;
/**SWREG310 (rw) register accessor: VENC segment 7: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg310::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg310::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG310)

For information about available fields see [`mod@swreg310`] module*/
pub type SWREG310 = crate::Reg<swreg310::SWREG310rs>;
///VENC segment 7: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg310;
/**SWREG311 (rw) register accessor: VENC segment 7: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg311::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg311::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG311)

For information about available fields see [`mod@swreg311`] module*/
pub type SWREG311 = crate::Reg<swreg311::SWREG311rs>;
///VENC segment 7: skip mode penalty, inter MB mode favor register
pub mod swreg311;
/**SWREG312 (rw) register accessor: VENC segment 7: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg312::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg312::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG312)

For information about available fields see [`mod@swreg312`] module*/
pub type SWREG312 = crate::Reg<swreg312::SWREG312rs>;
///VENC segment 7: penalty value register
pub mod swreg312;
/**SWREG313 (rw) register accessor: VENC segment 7: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg313::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg313::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG313)

For information about available fields see [`mod@swreg313`] module*/
pub type SWREG313 = crate::Reg<swreg313::SWREG313rs>;
///VENC segment 7: penalty value register
pub mod swreg313;
/**SWREG314 (rw) register accessor: VENC segment 8: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg314::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg314::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG314)

For information about available fields see [`mod@swreg314`] module*/
pub type SWREG314 = crate::Reg<swreg314::SWREG314rs>;
///VENC segment 8: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg314;
/**SWREG315 (rw) register accessor: VENC segment 8: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg315::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg315::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG315)

For information about available fields see [`mod@swreg315`] module*/
pub type SWREG315 = crate::Reg<swreg315::SWREG315rs>;
///VENC segment 8: skip mode penalty, inter MB mode favor register
pub mod swreg315;
/**SWREG316 (rw) register accessor: VENC segment 8: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg316::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg316::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG316)

For information about available fields see [`mod@swreg316`] module*/
pub type SWREG316 = crate::Reg<swreg316::SWREG316rs>;
///VENC segment 8: penalty value register
pub mod swreg316;
/**SWREG317 (rw) register accessor: VENC segment 8: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg317::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg317::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG317)

For information about available fields see [`mod@swreg317`] module*/
pub type SWREG317 = crate::Reg<swreg317::SWREG317rs>;
///VENC segment 8: penalty value register
pub mod swreg317;
/**SWREG318 (rw) register accessor: VENC segment 9: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg318::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg318::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG318)

For information about available fields see [`mod@swreg318`] module*/
pub type SWREG318 = crate::Reg<swreg318::SWREG318rs>;
///VENC segment 9: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg318;
/**SWREG319 (rw) register accessor: VENC segment 9: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg319::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg319::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG319)

For information about available fields see [`mod@swreg319`] module*/
pub type SWREG319 = crate::Reg<swreg319::SWREG319rs>;
///VENC segment 9: skip mode penalty, inter MB mode favor register
pub mod swreg319;
/**SWREG320 (rw) register accessor: VENC segment 9: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg320::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg320::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG320)

For information about available fields see [`mod@swreg320`] module*/
pub type SWREG320 = crate::Reg<swreg320::SWREG320rs>;
///VENC segment 9: penalty value register
pub mod swreg320;
/**SWREG321 (rw) register accessor: VENC segment 9: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg321::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg321::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG321)

For information about available fields see [`mod@swreg321`] module*/
pub type SWREG321 = crate::Reg<swreg321::SWREG321rs>;
///VENC segment 9: penalty value register
pub mod swreg321;
/**SWREG322 (rw) register accessor: VENC segment 10: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg322::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg322::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG322)

For information about available fields see [`mod@swreg322`] module*/
pub type SWREG322 = crate::Reg<swreg322::SWREG322rs>;
///VENC segment 10: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg322;
/**SWREG323 (rw) register accessor: VENC segment 10: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg323::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg323::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG323)

For information about available fields see [`mod@swreg323`] module*/
pub type SWREG323 = crate::Reg<swreg323::SWREG323rs>;
///VENC segment 10: skip mode penalty, inter MB mode favor register
pub mod swreg323;
/**SWREG324 (rw) register accessor: VENC segment 10: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg324::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg324::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG324)

For information about available fields see [`mod@swreg324`] module*/
pub type SWREG324 = crate::Reg<swreg324::SWREG324rs>;
///VENC segment 10: penalty value register
pub mod swreg324;
/**SWREG325 (rw) register accessor: VENC segment 10: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg325::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg325::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG325)

For information about available fields see [`mod@swreg325`] module*/
pub type SWREG325 = crate::Reg<swreg325::SWREG325rs>;
///VENC segment 10: penalty value register
pub mod swreg325;
/**SWREG326 (rw) register accessor: VENC segment 11: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg326::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg326::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG326)

For information about available fields see [`mod@swreg326`] module*/
pub type SWREG326 = crate::Reg<swreg326::SWREG326rs>;
///VENC segment 11: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg326;
/**SWREG327 (rw) register accessor: VENC segment 11: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg327::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg327::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG327)

For information about available fields see [`mod@swreg327`] module*/
pub type SWREG327 = crate::Reg<swreg327::SWREG327rs>;
///VENC segment 11: skip mode penalty, inter MB mode favor register
pub mod swreg327;
/**SWREG328 (rw) register accessor: VENC segment 11: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg328::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg328::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG328)

For information about available fields see [`mod@swreg328`] module*/
pub type SWREG328 = crate::Reg<swreg328::SWREG328rs>;
///VENC segment 11: penalty value register
pub mod swreg328;
/**SWREG329 (rw) register accessor: VENC segment 11: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg329::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg329::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG329)

For information about available fields see [`mod@swreg329`] module*/
pub type SWREG329 = crate::Reg<swreg329::SWREG329rs>;
///VENC segment 11: penalty value register
pub mod swreg329;
/**SWREG330 (rw) register accessor: VENC segment 12: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg330::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg330::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG330)

For information about available fields see [`mod@swreg330`] module*/
pub type SWREG330 = crate::Reg<swreg330::SWREG330rs>;
///VENC segment 12: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg330;
/**SWREG331 (rw) register accessor: VENC segment 12: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg331::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg331::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG331)

For information about available fields see [`mod@swreg331`] module*/
pub type SWREG331 = crate::Reg<swreg331::SWREG331rs>;
///VENC segment 12: skip mode penalty, inter MB mode favor register
pub mod swreg331;
/**SWREG332 (rw) register accessor: VENC segment 12: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg332::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg332::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG332)

For information about available fields see [`mod@swreg332`] module*/
pub type SWREG332 = crate::Reg<swreg332::SWREG332rs>;
///VENC segment 12: penalty value register
pub mod swreg332;
/**SWREG333 (rw) register accessor: VENC segment 12: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg333::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg333::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG333)

For information about available fields see [`mod@swreg333`] module*/
pub type SWREG333 = crate::Reg<swreg333::SWREG333rs>;
///VENC segment 12: penalty value register
pub mod swreg333;
/**SWREG334 (rw) register accessor: VENC segment 13: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg334::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg334::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG334)

For information about available fields see [`mod@swreg334`] module*/
pub type SWREG334 = crate::Reg<swreg334::SWREG334rs>;
///VENC segment 13: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg334;
/**SWREG335 (rw) register accessor: VENC segment 13: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg335::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg335::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG335)

For information about available fields see [`mod@swreg335`] module*/
pub type SWREG335 = crate::Reg<swreg335::SWREG335rs>;
///VENC segment 13: skip mode penalty, inter MB mode favor register
pub mod swreg335;
/**SWREG336 (rw) register accessor: VENC segment 13: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg336::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg336::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG336)

For information about available fields see [`mod@swreg336`] module*/
pub type SWREG336 = crate::Reg<swreg336::SWREG336rs>;
///VENC segment 13: penalty value register
pub mod swreg336;
/**SWREG337 (rw) register accessor: VENC segment 13: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg337::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg337::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG337)

For information about available fields see [`mod@swreg337`] module*/
pub type SWREG337 = crate::Reg<swreg337::SWREG337rs>;
///VENC segment 13: penalty value register
pub mod swreg337;
/**SWREG338 (rw) register accessor: VENC segment 14: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg338::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg338::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG338)

For information about available fields see [`mod@swreg338`] module*/
pub type SWREG338 = crate::Reg<swreg338::SWREG338rs>;
///VENC segment 14: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg338;
/**SWREG339 (rw) register accessor: VENC segment 14: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg339::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg339::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG339)

For information about available fields see [`mod@swreg339`] module*/
pub type SWREG339 = crate::Reg<swreg339::SWREG339rs>;
///VENC segment 14: skip mode penalty, inter MB mode favor register
pub mod swreg339;
/**SWREG340 (rw) register accessor: VENC segment 14: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg340::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg340::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG340)

For information about available fields see [`mod@swreg340`] module*/
pub type SWREG340 = crate::Reg<swreg340::SWREG340rs>;
///VENC segment 14: penalty value register
pub mod swreg340;
/**SWREG341 (rw) register accessor: VENC segment 14: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg341::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg341::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG341)

For information about available fields see [`mod@swreg341`] module*/
pub type SWREG341 = crate::Reg<swreg341::SWREG341rs>;
///VENC segment 14: penalty value register
pub mod swreg341;
/**SWREG342 (rw) register accessor: VENC segment 15: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg342::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg342::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG342)

For information about available fields see [`mod@swreg342`] module*/
pub type SWREG342 = crate::Reg<swreg342::SWREG342rs>;
///VENC segment 15: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg342;
/**SWREG343 (rw) register accessor: VENC segment 15: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg343::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg343::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG343)

For information about available fields see [`mod@swreg343`] module*/
pub type SWREG343 = crate::Reg<swreg343::SWREG343rs>;
///VENC segment 15: skip mode penalty, inter MB mode favor register
pub mod swreg343;
/**SWREG344 (rw) register accessor: VENC segment 15: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg344::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg344::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG344)

For information about available fields see [`mod@swreg344`] module*/
pub type SWREG344 = crate::Reg<swreg344::SWREG344rs>;
///VENC segment 15: penalty value register
pub mod swreg344;
/**SWREG345 (rw) register accessor: VENC segment 15: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg345::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg345::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG345)

For information about available fields see [`mod@swreg345`] module*/
pub type SWREG345 = crate::Reg<swreg345::SWREG345rs>;
///VENC segment 15: penalty value register
pub mod swreg345;
/**SWREG346 (rw) register accessor: VENC segment 16: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg346::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg346::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG346)

For information about available fields see [`mod@swreg346`] module*/
pub type SWREG346 = crate::Reg<swreg346::SWREG346rs>;
///VENC segment 16: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg346;
/**SWREG347 (rw) register accessor: VENC segment 16: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg347::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg347::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG347)

For information about available fields see [`mod@swreg347`] module*/
pub type SWREG347 = crate::Reg<swreg347::SWREG347rs>;
///VENC segment 16: skip mode penalty, inter MB mode favor register
pub mod swreg347;
/**SWREG348 (rw) register accessor: VENC segment 16: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg348::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg348::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG348)

For information about available fields see [`mod@swreg348`] module*/
pub type SWREG348 = crate::Reg<swreg348::SWREG348rs>;
///VENC segment 16: penalty value register
pub mod swreg348;
/**SWREG349 (rw) register accessor: VENC segment 16: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg349::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg349::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG349)

For information about available fields see [`mod@swreg349`] module*/
pub type SWREG349 = crate::Reg<swreg349::SWREG349rs>;
///VENC segment 16: penalty value register
pub mod swreg349;
/**SWREG350 (rw) register accessor: VENC segment 17: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg350::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg350::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG350)

For information about available fields see [`mod@swreg350`] module*/
pub type SWREG350 = crate::Reg<swreg350::SWREG350rs>;
///VENC segment 17: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg350;
/**SWREG351 (rw) register accessor: VENC segment 17: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg351::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg351::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG351)

For information about available fields see [`mod@swreg351`] module*/
pub type SWREG351 = crate::Reg<swreg351::SWREG351rs>;
///VENC segment 17: skip mode penalty, inter MB mode favor register
pub mod swreg351;
/**SWREG352 (rw) register accessor: VENC segment 17: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg352::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg352::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG352)

For information about available fields see [`mod@swreg352`] module*/
pub type SWREG352 = crate::Reg<swreg352::SWREG352rs>;
///VENC segment 17: penalty value register
pub mod swreg352;
/**SWREG353 (rw) register accessor: VENC segment 17: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg353::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg353::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG353)

For information about available fields see [`mod@swreg353`] module*/
pub type SWREG353 = crate::Reg<swreg353::SWREG353rs>;
///VENC segment 17: penalty value register
pub mod swreg353;
/**SWREG354 (rw) register accessor: VENC segment 18: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg354::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg354::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG354)

For information about available fields see [`mod@swreg354`] module*/
pub type SWREG354 = crate::Reg<swreg354::SWREG354rs>;
///VENC segment 18: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg354;
/**SWREG355 (rw) register accessor: VENC segment 18: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg355::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg355::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG355)

For information about available fields see [`mod@swreg355`] module*/
pub type SWREG355 = crate::Reg<swreg355::SWREG355rs>;
///VENC segment 18: skip mode penalty, inter MB mode favor register
pub mod swreg355;
/**SWREG356 (rw) register accessor: VENC segment 18: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg356::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg356::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG356)

For information about available fields see [`mod@swreg356`] module*/
pub type SWREG356 = crate::Reg<swreg356::SWREG356rs>;
///VENC segment 18: penalty value register
pub mod swreg356;
/**SWREG357 (rw) register accessor: VENC segment 18: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg357::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg357::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG357)

For information about available fields see [`mod@swreg357`] module*/
pub type SWREG357 = crate::Reg<swreg357::SWREG357rs>;
///VENC segment 18: penalty value register
pub mod swreg357;
/**SWREG358 (rw) register accessor: VENC segment 19: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg358::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg358::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG358)

For information about available fields see [`mod@swreg358`] module*/
pub type SWREG358 = crate::Reg<swreg358::SWREG358rs>;
///VENC segment 19: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg358;
/**SWREG359 (rw) register accessor: VENC segment 19: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg359::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg359::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG359)

For information about available fields see [`mod@swreg359`] module*/
pub type SWREG359 = crate::Reg<swreg359::SWREG359rs>;
///VENC segment 19: skip mode penalty, inter MB mode favor register
pub mod swreg359;
/**SWREG360 (rw) register accessor: VENC segment 19: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg360::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg360::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG360)

For information about available fields see [`mod@swreg360`] module*/
pub type SWREG360 = crate::Reg<swreg360::SWREG360rs>;
///VENC segment 19: penalty value register
pub mod swreg360;
/**SWREG361 (rw) register accessor: VENC segment 19: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg361::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg361::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG361)

For information about available fields see [`mod@swreg361`] module*/
pub type SWREG361 = crate::Reg<swreg361::SWREG361rs>;
///VENC segment 19: penalty value register
pub mod swreg361;
/**SWREG362 (rw) register accessor: VENC segment 20: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg362::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg362::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG362)

For information about available fields see [`mod@swreg362`] module*/
pub type SWREG362 = crate::Reg<swreg362::SWREG362rs>;
///VENC segment 20: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg362;
/**SWREG363 (rw) register accessor: VENC segment 20: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg363::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg363::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG363)

For information about available fields see [`mod@swreg363`] module*/
pub type SWREG363 = crate::Reg<swreg363::SWREG363rs>;
///VENC segment 20: skip mode penalty, inter MB mode favor register
pub mod swreg363;
/**SWREG364 (rw) register accessor: VENC segment 20: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg364::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg364::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG364)

For information about available fields see [`mod@swreg364`] module*/
pub type SWREG364 = crate::Reg<swreg364::SWREG364rs>;
///VENC segment 20: penalty value register
pub mod swreg364;
/**SWREG365 (rw) register accessor: VENC segment 20: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg365::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg365::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG365)

For information about available fields see [`mod@swreg365`] module*/
pub type SWREG365 = crate::Reg<swreg365::SWREG365rs>;
///VENC segment 20: penalty value register
pub mod swreg365;
/**SWREG366 (rw) register accessor: VENC segment 21: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg366::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg366::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG366)

For information about available fields see [`mod@swreg366`] module*/
pub type SWREG366 = crate::Reg<swreg366::SWREG366rs>;
///VENC segment 21: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg366;
/**SWREG367 (rw) register accessor: VENC segment 21: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg367::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg367::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG367)

For information about available fields see [`mod@swreg367`] module*/
pub type SWREG367 = crate::Reg<swreg367::SWREG367rs>;
///VENC segment 21: skip mode penalty, inter MB mode favor register
pub mod swreg367;
/**SWREG368 (rw) register accessor: VENC segment 21: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg368::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg368::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG368)

For information about available fields see [`mod@swreg368`] module*/
pub type SWREG368 = crate::Reg<swreg368::SWREG368rs>;
///VENC segment 21: penalty value register
pub mod swreg368;
/**SWREG369 (rw) register accessor: VENC segment 21: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg369::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg369::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG369)

For information about available fields see [`mod@swreg369`] module*/
pub type SWREG369 = crate::Reg<swreg369::SWREG369rs>;
///VENC segment 21: penalty value register
pub mod swreg369;
/**SWREG370 (rw) register accessor: VENC segment 22: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg370::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg370::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG370)

For information about available fields see [`mod@swreg370`] module*/
pub type SWREG370 = crate::Reg<swreg370::SWREG370rs>;
///VENC segment 22: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg370;
/**SWREG371 (rw) register accessor: VENC segment 22: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg371::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg371::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG371)

For information about available fields see [`mod@swreg371`] module*/
pub type SWREG371 = crate::Reg<swreg371::SWREG371rs>;
///VENC segment 22: skip mode penalty, inter MB mode favor register
pub mod swreg371;
/**SWREG372 (rw) register accessor: VENC segment 22: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg372::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg372::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG372)

For information about available fields see [`mod@swreg372`] module*/
pub type SWREG372 = crate::Reg<swreg372::SWREG372rs>;
///VENC segment 22: penalty value register
pub mod swreg372;
/**SWREG373 (rw) register accessor: VENC segment 22: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg373::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg373::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG373)

For information about available fields see [`mod@swreg373`] module*/
pub type SWREG373 = crate::Reg<swreg373::SWREG373rs>;
///VENC segment 22: penalty value register
pub mod swreg373;
/**SWREG374 (rw) register accessor: VENC segment 23: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg374::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg374::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG374)

For information about available fields see [`mod@swreg374`] module*/
pub type SWREG374 = crate::Reg<swreg374::SWREG374rs>;
///VENC segment 23: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg374;
/**SWREG375 (rw) register accessor: VENC segment 23: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg375::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg375::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG375)

For information about available fields see [`mod@swreg375`] module*/
pub type SWREG375 = crate::Reg<swreg375::SWREG375rs>;
///VENC segment 23: skip mode penalty, inter MB mode favor register
pub mod swreg375;
/**SWREG376 (rw) register accessor: VENC segment 23: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg376::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg376::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG376)

For information about available fields see [`mod@swreg376`] module*/
pub type SWREG376 = crate::Reg<swreg376::SWREG376rs>;
///VENC segment 23: penalty value register
pub mod swreg376;
/**SWREG377 (rw) register accessor: VENC segment 23: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg377::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg377::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG377)

For information about available fields see [`mod@swreg377`] module*/
pub type SWREG377 = crate::Reg<swreg377::SWREG377rs>;
///VENC segment 23: penalty value register
pub mod swreg377;
/**SWREG378 (rw) register accessor: VENC segment 24: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg378::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg378::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG378)

For information about available fields see [`mod@swreg378`] module*/
pub type SWREG378 = crate::Reg<swreg378::SWREG378rs>;
///VENC segment 24: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg378;
/**SWREG379 (rw) register accessor: VENC segment 24: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg379::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg379::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG379)

For information about available fields see [`mod@swreg379`] module*/
pub type SWREG379 = crate::Reg<swreg379::SWREG379rs>;
///VENC segment 24: skip mode penalty, inter MB mode favor register
pub mod swreg379;
/**SWREG380 (rw) register accessor: VENC segment 24: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg380::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg380::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG380)

For information about available fields see [`mod@swreg380`] module*/
pub type SWREG380 = crate::Reg<swreg380::SWREG380rs>;
///VENC segment 24: penalty value register
pub mod swreg380;
/**SWREG381 (rw) register accessor: VENC segment 24: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg381::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg381::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG381)

For information about available fields see [`mod@swreg381`] module*/
pub type SWREG381 = crate::Reg<swreg381::SWREG381rs>;
///VENC segment 24: penalty value register
pub mod swreg381;
/**SWREG382 (rw) register accessor: VENC segment 25: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg382::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg382::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG382)

For information about available fields see [`mod@swreg382`] module*/
pub type SWREG382 = crate::Reg<swreg382::SWREG382rs>;
///VENC segment 25: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg382;
/**SWREG383 (rw) register accessor: VENC segment 25: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg383::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg383::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG383)

For information about available fields see [`mod@swreg383`] module*/
pub type SWREG383 = crate::Reg<swreg383::SWREG383rs>;
///VENC segment 25: skip mode penalty, inter MB mode favor register
pub mod swreg383;
/**SWREG384 (rw) register accessor: VENC segment 25: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg384::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg384::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG384)

For information about available fields see [`mod@swreg384`] module*/
pub type SWREG384 = crate::Reg<swreg384::SWREG384rs>;
///VENC segment 25: penalty value register
pub mod swreg384;
/**SWREG385 (rw) register accessor: VENC segment 25: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg385::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg385::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG385)

For information about available fields see [`mod@swreg385`] module*/
pub type SWREG385 = crate::Reg<swreg385::SWREG385rs>;
///VENC segment 25: penalty value register
pub mod swreg385;
/**SWREG386 (rw) register accessor: VENC segment 26: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg386::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg386::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG386)

For information about available fields see [`mod@swreg386`] module*/
pub type SWREG386 = crate::Reg<swreg386::SWREG386rs>;
///VENC segment 26: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg386;
/**SWREG387 (rw) register accessor: VENC segment 26: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg387::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg387::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG387)

For information about available fields see [`mod@swreg387`] module*/
pub type SWREG387 = crate::Reg<swreg387::SWREG387rs>;
///VENC segment 26: skip mode penalty, inter MB mode favor register
pub mod swreg387;
/**SWREG388 (rw) register accessor: VENC segment 26: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg388::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg388::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG388)

For information about available fields see [`mod@swreg388`] module*/
pub type SWREG388 = crate::Reg<swreg388::SWREG388rs>;
///VENC segment 26: penalty value register
pub mod swreg388;
/**SWREG389 (rw) register accessor: VENC segment 26: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg389::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg389::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG389)

For information about available fields see [`mod@swreg389`] module*/
pub type SWREG389 = crate::Reg<swreg389::SWREG389rs>;
///VENC segment 26: penalty value register
pub mod swreg389;
/**SWREG390 (rw) register accessor: VENC segment 27: intra 4x4 previous mode favor, intra 16x16mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg390::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg390::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG390)

For information about available fields see [`mod@swreg390`] module*/
pub type SWREG390 = crate::Reg<swreg390::SWREG390rs>;
///VENC segment 27: intra 4x4 previous mode favor, intra 16x16mode favor, penalty value for second reference frame register
pub mod swreg390;
/**SWREG391 (rw) register accessor: VENC segment 27: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg391::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg391::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG391)

For information about available fields see [`mod@swreg391`] module*/
pub type SWREG391 = crate::Reg<swreg391::SWREG391rs>;
///VENC segment 27: skip mode penalty, inter MB mode favor register
pub mod swreg391;
/**SWREG392 (rw) register accessor: VENC segment 27: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg392::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg392::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG392)

For information about available fields see [`mod@swreg392`] module*/
pub type SWREG392 = crate::Reg<swreg392::SWREG392rs>;
///VENC segment 27: penalty value register
pub mod swreg392;
/**SWREG393 (rw) register accessor: VENC segment 27: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg393::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg393::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG393)

For information about available fields see [`mod@swreg393`] module*/
pub type SWREG393 = crate::Reg<swreg393::SWREG393rs>;
///VENC segment 27: penalty value register
pub mod swreg393;
/**SWREG394 (rw) register accessor: VENC segment 28: intra 4x4 previous mode favor, intra 16x16mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg394::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg394::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG394)

For information about available fields see [`mod@swreg394`] module*/
pub type SWREG394 = crate::Reg<swreg394::SWREG394rs>;
///VENC segment 28: intra 4x4 previous mode favor, intra 16x16mode favor, penalty value for second reference frame register
pub mod swreg394;
/**SWREG395 (rw) register accessor: VENC segment 28: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg395::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg395::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG395)

For information about available fields see [`mod@swreg395`] module*/
pub type SWREG395 = crate::Reg<swreg395::SWREG395rs>;
///VENC segment 28: skip mode penalty, inter MB mode favor register
pub mod swreg395;
/**SWREG396 (rw) register accessor: VENC segment 28: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg396::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg396::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG396)

For information about available fields see [`mod@swreg396`] module*/
pub type SWREG396 = crate::Reg<swreg396::SWREG396rs>;
///VENC segment 28: penalty value register
pub mod swreg396;
/**SWREG397 (rw) register accessor: VENC segment 28: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg397::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg397::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG397)

For information about available fields see [`mod@swreg397`] module*/
pub type SWREG397 = crate::Reg<swreg397::SWREG397rs>;
///VENC segment 28: penalty value register
pub mod swreg397;
/**SWREG398 (rw) register accessor: VENC segment 29: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg398::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg398::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG398)

For information about available fields see [`mod@swreg398`] module*/
pub type SWREG398 = crate::Reg<swreg398::SWREG398rs>;
///VENC segment 29: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg398;
/**SWREG399 (rw) register accessor: VENC segment 29: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg399::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg399::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG399)

For information about available fields see [`mod@swreg399`] module*/
pub type SWREG399 = crate::Reg<swreg399::SWREG399rs>;
///VENC segment 29: skip mode penalty, inter MB mode favor register
pub mod swreg399;
/**SWREG400 (rw) register accessor: VENC segment 29: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg400::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg400::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG400)

For information about available fields see [`mod@swreg400`] module*/
pub type SWREG400 = crate::Reg<swreg400::SWREG400rs>;
///VENC segment 29: penalty value register
pub mod swreg400;
/**SWREG401 (rw) register accessor: VENC segment 29: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg401::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg401::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG401)

For information about available fields see [`mod@swreg401`] module*/
pub type SWREG401 = crate::Reg<swreg401::SWREG401rs>;
///VENC segment 29: penalty value register
pub mod swreg401;
/**SWREG402 (rw) register accessor: VENC segment 30: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg402::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg402::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG402)

For information about available fields see [`mod@swreg402`] module*/
pub type SWREG402 = crate::Reg<swreg402::SWREG402rs>;
///VENC segment 30: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg402;
/**SWREG403 (rw) register accessor: VENC segment 30: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg403::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg403::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG403)

For information about available fields see [`mod@swreg403`] module*/
pub type SWREG403 = crate::Reg<swreg403::SWREG403rs>;
///VENC segment 30: skip mode penalty, inter MB mode favor register
pub mod swreg403;
/**SWREG404 (rw) register accessor: VENC segment 30: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg404::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg404::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG404)

For information about available fields see [`mod@swreg404`] module*/
pub type SWREG404 = crate::Reg<swreg404::SWREG404rs>;
///VENC segment 30: penalty value register
pub mod swreg404;
/**SWREG405 (rw) register accessor: VENC segment 30: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg405::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg405::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG405)

For information about available fields see [`mod@swreg405`] module*/
pub type SWREG405 = crate::Reg<swreg405::SWREG405rs>;
///VENC segment 30: penalty value register
pub mod swreg405;
/**SWREG406 (rw) register accessor: VENC segment 31: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register

You can [`read`](crate::Reg::read) this register and get [`swreg406::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg406::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG406)

For information about available fields see [`mod@swreg406`] module*/
pub type SWREG406 = crate::Reg<swreg406::SWREG406rs>;
///VENC segment 31: intra 4x4 previous mode favor, intra 16x16 mode favor, penalty value for second reference frame register
pub mod swreg406;
/**SWREG407 (rw) register accessor: VENC segment 31: skip mode penalty, inter MB mode favor register

You can [`read`](crate::Reg::read) this register and get [`swreg407::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg407::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG407)

For information about available fields see [`mod@swreg407`] module*/
pub type SWREG407 = crate::Reg<swreg407::SWREG407rs>;
///VENC segment 31: skip mode penalty, inter MB mode favor register
pub mod swreg407;
/**SWREG408 (rw) register accessor: VENC segment 31: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg408::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg408::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG408)

For information about available fields see [`mod@swreg408`] module*/
pub type SWREG408 = crate::Reg<swreg408::SWREG408rs>;
///VENC segment 31: penalty value register
pub mod swreg408;
/**SWREG409 (rw) register accessor: VENC segment 31: penalty value register

You can [`read`](crate::Reg::read) this register and get [`swreg409::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg409::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG409)

For information about available fields see [`mod@swreg409`] module*/
pub type SWREG409 = crate::Reg<swreg409::SWREG409rs>;
///VENC segment 31: penalty value register
pub mod swreg409;
/**SWREG410 (rw) register accessor: VENC MBRC control, QP, offset, enable register

You can [`read`](crate::Reg::read) this register and get [`swreg410::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg410::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG410)

For information about available fields see [`mod@swreg410`] module*/
pub type SWREG410 = crate::Reg<swreg410::SWREG410rs>;
///VENC MBRC control, QP, offset, enable register
pub mod swreg410;
/**SWREG411 (rw) register accessor: VENC gain of MB QP delta. 8.8 format register

You can [`read`](crate::Reg::read) this register and get [`swreg411::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg411::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG411)

For information about available fields see [`mod@swreg411`] module*/
pub type SWREG411 = crate::Reg<swreg411::SWREG411rs>;
///VENC gain of MB QP delta. 8.8 format register
pub mod swreg411;
/**SWREG412 (rw) register accessor: VENC average of MB complexity register

You can [`read`](crate::Reg::read) this register and get [`swreg412::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg412::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG412)

For information about available fields see [`mod@swreg412`] module*/
pub type SWREG412 = crate::Reg<swreg412::SWREG412rs>;
///VENC average of MB complexity register
pub mod swreg412;
/**SWREG413 (rw) register accessor: VENC reference compression control register

You can [`read`](crate::Reg::read) this register and get [`swreg413::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg413::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG413)

For information about available fields see [`mod@swreg413`] module*/
pub type SWREG413 = crate::Reg<swreg413::SWREG413rs>;
///VENC reference compression control register
pub mod swreg413;
/**SWREG414 (rw) register accessor: VENC base address for reference luma register

You can [`read`](crate::Reg::read) this register and get [`swreg414::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg414::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG414)

For information about available fields see [`mod@swreg414`] module*/
pub type SWREG414 = crate::Reg<swreg414::SWREG414rs>;
///VENC base address for reference luma register
pub mod swreg414;
/**SWREG415 (rw) register accessor: VENC base address for reference chroma register

You can [`read`](crate::Reg::read) this register and get [`swreg415::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg415::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG415)

For information about available fields see [`mod@swreg415`] module*/
pub type SWREG415 = crate::Reg<swreg415::SWREG415rs>;
///VENC base address for reference chroma register
pub mod swreg415;
/**SWREG416 (rw) register accessor: VENC base address for reconstructed luma register

You can [`read`](crate::Reg::read) this register and get [`swreg416::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg416::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG416)

For information about available fields see [`mod@swreg416`] module*/
pub type SWREG416 = crate::Reg<swreg416::SWREG416rs>;
///VENC base address for reconstructed luma register
pub mod swreg416;
/**SWREG417 (rw) register accessor: VENC base address for reconstructed chroma register

You can [`read`](crate::Reg::read) this register and get [`swreg417::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg417::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG417)

For information about available fields see [`mod@swreg417`] module*/
pub type SWREG417 = crate::Reg<swreg417::SWREG417rs>;
///VENC base address for reconstructed chroma register
pub mod swreg417;
/**SWREG418 (rw) register accessor: VENC base address for second reference luma register

You can [`read`](crate::Reg::read) this register and get [`swreg418::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg418::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG418)

For information about available fields see [`mod@swreg418`] module*/
pub type SWREG418 = crate::Reg<swreg418::SWREG418rs>;
///VENC base address for second reference luma register
pub mod swreg418;
/**SWREG419 (rw) register accessor: VENC base address for second reference chroma register

You can [`read`](crate::Reg::read) this register and get [`swreg419::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg419::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG419)

For information about available fields see [`mod@swreg419`] module*/
pub type SWREG419 = crate::Reg<swreg419::SWREG419rs>;
///VENC base address for second reference chroma register
pub mod swreg419;
/**SWREG420 (rw) register accessor: VENC limit of chroma RFC buffer register

You can [`read`](crate::Reg::read) this register and get [`swreg420::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg420::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG420)

For information about available fields see [`mod@swreg420`] module*/
pub type SWREG420 = crate::Reg<swreg420::SWREG420rs>;
///VENC limit of chroma RFC buffer register
pub mod swreg420;
/**SWREG421 (rw) register accessor: VENC reorder control register

You can [`read`](crate::Reg::read) this register and get [`swreg421::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg421::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG421)

For information about available fields see [`mod@swreg421`] module*/
pub type SWREG421 = crate::Reg<swreg421::SWREG421rs>;
///VENC reorder control register
pub mod swreg421;
/**SWREG422 (rw) register accessor: VENC AXI read ID register

You can [`read`](crate::Reg::read) this register and get [`swreg422::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg422::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG422)

For information about available fields see [`mod@swreg422`] module*/
pub type SWREG422 = crate::Reg<swreg422::SWREG422rs>;
///VENC AXI read ID register
pub mod swreg422;
/**SWREG423 (rw) register accessor: VENC base address MSB for reference luma compression table register

You can [`read`](crate::Reg::read) this register and get [`swreg423::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg423::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG423)

For information about available fields see [`mod@swreg423`] module*/
pub type SWREG423 = crate::Reg<swreg423::SWREG423rs>;
///VENC base address MSB for reference luma compression table register
pub mod swreg423;
/**SWREG424 (rw) register accessor: VENC base address MSB for reference chroma compression table register

You can [`read`](crate::Reg::read) this register and get [`swreg424::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg424::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG424)

For information about available fields see [`mod@swreg424`] module*/
pub type SWREG424 = crate::Reg<swreg424::SWREG424rs>;
///VENC base address MSB for reference chroma compression table register
pub mod swreg424;
/**SWREG425 (rw) register accessor: VENC base address MSB for reconstructed luma compression table register

You can [`read`](crate::Reg::read) this register and get [`swreg425::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg425::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG425)

For information about available fields see [`mod@swreg425`] module*/
pub type SWREG425 = crate::Reg<swreg425::SWREG425rs>;
///VENC base address MSB for reconstructed luma compression table register
pub mod swreg425;
/**SWREG426 (rw) register accessor: VENC base address for reconstructed chroma compression table register

You can [`read`](crate::Reg::read) this register and get [`swreg426::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg426::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG426)

For information about available fields see [`mod@swreg426`] module*/
pub type SWREG426 = crate::Reg<swreg426::SWREG426rs>;
///VENC base address for reconstructed chroma compression table register
pub mod swreg426;
/**SWREG427 (rw) register accessor: VENC base address MSB for second reference luma compression table register

You can [`read`](crate::Reg::read) this register and get [`swreg427::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg427::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG427)

For information about available fields see [`mod@swreg427`] module*/
pub type SWREG427 = crate::Reg<swreg427::SWREG427rs>;
///VENC base address MSB for second reference luma compression table register
pub mod swreg427;
/**SWREG428 (rw) register accessor: VENC base address MSB for second reference chroma compression table register

You can [`read`](crate::Reg::read) this register and get [`swreg428::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg428::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG428)

For information about available fields see [`mod@swreg428`] module*/
pub type SWREG428 = crate::Reg<swreg428::SWREG428rs>;
///VENC base address MSB for second reference chroma compression table register
pub mod swreg428;
/**SWREG429 (rw) register accessor: VENC high 32 bits of base address for output stream data register

You can [`read`](crate::Reg::read) this register and get [`swreg429::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg429::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG429)

For information about available fields see [`mod@swreg429`] module*/
pub type SWREG429 = crate::Reg<swreg429::SWREG429rs>;
///VENC high 32 bits of base address for output stream data register
pub mod swreg429;
/**SWREG430 (rw) register accessor: VENC high 32 bits of base address for output control data register

You can [`read`](crate::Reg::read) this register and get [`swreg430::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg430::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG430)

For information about available fields see [`mod@swreg430`] module*/
pub type SWREG430 = crate::Reg<swreg430::SWREG430rs>;
///VENC high 32 bits of base address for output control data register
pub mod swreg430;
/**SWREG431 (rw) register accessor: VENC high 32 bits of base address for reference luma register

You can [`read`](crate::Reg::read) this register and get [`swreg431::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg431::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG431)

For information about available fields see [`mod@swreg431`] module*/
pub type SWREG431 = crate::Reg<swreg431::SWREG431rs>;
///VENC high 32 bits of base address for reference luma register
pub mod swreg431;
/**SWREG432 (rw) register accessor: VENC high 32 bits of base address for reference chroma register

You can [`read`](crate::Reg::read) this register and get [`swreg432::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg432::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG432)

For information about available fields see [`mod@swreg432`] module*/
pub type SWREG432 = crate::Reg<swreg432::SWREG432rs>;
///VENC high 32 bits of base address for reference chroma register
pub mod swreg432;
/**SWREG433 (rw) register accessor: VENC high 32 bits of base address for reconstructed luma register

You can [`read`](crate::Reg::read) this register and get [`swreg433::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg433::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG433)

For information about available fields see [`mod@swreg433`] module*/
pub type SWREG433 = crate::Reg<swreg433::SWREG433rs>;
///VENC high 32 bits of base address for reconstructed luma register
pub mod swreg433;
/**SWREG434 (rw) register accessor: VENC high 32 bits of base address for reconstructed chroma register

You can [`read`](crate::Reg::read) this register and get [`swreg434::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg434::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG434)

For information about available fields see [`mod@swreg434`] module*/
pub type SWREG434 = crate::Reg<swreg434::SWREG434rs>;
///VENC high 32 bits of base address for reconstructed chroma register
pub mod swreg434;
/**SWREG435 (rw) register accessor: VENC high 32 bits of base address for input picture luma register

You can [`read`](crate::Reg::read) this register and get [`swreg435::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg435::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG435)

For information about available fields see [`mod@swreg435`] module*/
pub type SWREG435 = crate::Reg<swreg435::SWREG435rs>;
///VENC high 32 bits of base address for input picture luma register
pub mod swreg435;
/**SWREG436 (rw) register accessor: VENC high 32 bits of base address for input picture cb register

You can [`read`](crate::Reg::read) this register and get [`swreg436::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg436::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG436)

For information about available fields see [`mod@swreg436`] module*/
pub type SWREG436 = crate::Reg<swreg436::SWREG436rs>;
///VENC high 32 bits of base address for input picture cb register
pub mod swreg436;
/**SWREG437 (rw) register accessor: VENC high 32 bits of base address for input picture cr register

You can [`read`](crate::Reg::read) this register and get [`swreg437::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg437::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG437)

For information about available fields see [`mod@swreg437`] module*/
pub type SWREG437 = crate::Reg<swreg437::SWREG437rs>;
///VENC high 32 bits of base address for input picture cr register
pub mod swreg437;
/**SWREG438 (rw) register accessor: VENC high 32 bits of base address for second reference luma register

You can [`read`](crate::Reg::read) this register and get [`swreg438::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg438::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG438)

For information about available fields see [`mod@swreg438`] module*/
pub type SWREG438 = crate::Reg<swreg438::SWREG438rs>;
///VENC high 32 bits of base address for second reference luma register
pub mod swreg438;
/**SWREG439 (rw) register accessor: VENC high 32 bits of base address for second reference chroma register

You can [`read`](crate::Reg::read) this register and get [`swreg439::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg439::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG439)

For information about available fields see [`mod@swreg439`] module*/
pub type SWREG439 = crate::Reg<swreg439::SWREG439rs>;
///VENC high 32 bits of base address for second reference chroma register
pub mod swreg439;
/**SWREG440 (rw) register accessor: VENC high 32 bits of H264 secondary ref pic base register

You can [`read`](crate::Reg::read) this register and get [`swreg440::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg440::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG440)

For information about available fields see [`mod@swreg440`] module*/
pub type SWREG440 = crate::Reg<swreg440::SWREG440rs>;
///VENC high 32 bits of H264 secondary ref pic base register
pub mod swreg440;
/**SWREG441 (rw) register accessor: VENC high 32 bits of H264 secondary ref pic base register

You can [`read`](crate::Reg::read) this register and get [`swreg441::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg441::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG441)

For information about available fields see [`mod@swreg441`] module*/
pub type SWREG441 = crate::Reg<swreg441::SWREG441rs>;
///VENC high 32 bits of H264 secondary ref pic base register
pub mod swreg441;
/**SWREG442 (rw) register accessor: VENC high 32 bits of base address for next pic luminance register

You can [`read`](crate::Reg::read) this register and get [`swreg442::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg442::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG442)

For information about available fields see [`mod@swreg442`] module*/
pub type SWREG442 = crate::Reg<swreg442::SWREG442rs>;
///VENC high 32 bits of base address for next pic luminance register
pub mod swreg442;
/**SWREG443 (rw) register accessor: VENC high 32 bits of base address for cabac context tables H264 register

You can [`read`](crate::Reg::read) this register and get [`swreg443::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg443::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG443)

For information about available fields see [`mod@swreg443`] module*/
pub type SWREG443 = crate::Reg<swreg443::SWREG443rs>;
///VENC high 32 bits of base address for cabac context tables H264 register
pub mod swreg443;
/**SWREG444 (rw) register accessor: VENC high 32 bits of base address for MV output writing register

You can [`read`](crate::Reg::read) this register and get [`swreg444::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg444::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG444)

For information about available fields see [`mod@swreg444`] module*/
pub type SWREG444 = crate::Reg<swreg444::SWREG444rs>;
///VENC high 32 bits of base address for MV output writing register
pub mod swreg444;
/**SWREG449 (rw) register accessor: VENC high 32 bits of base address for output of down-scaled encoder image in YUYV 4:2:2 format register

You can [`read`](crate::Reg::read) this register and get [`swreg449::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg449::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG449)

For information about available fields see [`mod@swreg449`] module*/
pub type SWREG449 = crate::Reg<swreg449::SWREG449rs>;
///VENC high 32 bits of base address for output of down-scaled encoder image in YUYV 4:2:2 format register
pub mod swreg449;
/**SWREG497 (rw) register accessor: VENC low-latency control register

You can [`read`](crate::Reg::read) this register and get [`swreg497::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg497::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG497)

For information about available fields see [`mod@swreg497`] module*/
pub type SWREG497 = crate::Reg<swreg497::SWREG497rs>;
///VENC low-latency control register
pub mod swreg497;
/**SWREG498 (rw) register accessor: VENC encoder line buffer offset register

You can [`read`](crate::Reg::read) this register and get [`swreg498::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg498::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#VENC:SWREG498)

For information about available fields see [`mod@swreg498`] module*/
pub type SWREG498 = crate::Reg<swreg498::SWREG498rs>;
///VENC encoder line buffer offset register
pub mod swreg498;
