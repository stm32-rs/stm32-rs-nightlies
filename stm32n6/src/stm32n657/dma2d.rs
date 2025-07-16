#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cr: CR,
    isr: ISR,
    ifcr: IFCR,
    fgmar: FGMAR,
    fgor: FGOR,
    bgmar: BGMAR,
    bgor: BGOR,
    fgpfccr: FGPFCCR,
    fgcolr: FGCOLR,
    bgpfccr: BGPFCCR,
    bgcolr: BGCOLR,
    fgcmar: FGCMAR,
    bgcmar: BGCMAR,
    opfccr: OPFCCR,
    _reserved_14_ocolr: [u8; 0x04],
    omar: OMAR,
    oor: OOR,
    nlr: NLR,
    lwr: LWR,
    amtcr: AMTCR,
    _reserved20: [u8; 0x03b0],
    fgclut0: FGCLUT0,
    fgclut1: FGCLUT1,
    fgclut2: FGCLUT2,
    fgclut3: FGCLUT3,
    fgclut4: FGCLUT4,
    fgclut5: FGCLUT5,
    fgclut6: FGCLUT6,
    fgclut7: FGCLUT7,
    fgclut8: FGCLUT8,
    fgclut9: FGCLUT9,
    fgclut10: FGCLUT10,
    fgclut11: FGCLUT11,
    fgclut12: FGCLUT12,
    fgclut13: FGCLUT13,
    fgclut14: FGCLUT14,
    fgclut15: FGCLUT15,
    fgclut16: FGCLUT16,
    fgclut17: FGCLUT17,
    fgclut18: FGCLUT18,
    fgclut19: FGCLUT19,
    fgclut20: FGCLUT20,
    fgclut21: FGCLUT21,
    fgclut22: FGCLUT22,
    fgclut23: FGCLUT23,
    fgclut24: FGCLUT24,
    fgclut25: FGCLUT25,
    fgclut26: FGCLUT26,
    fgclut27: FGCLUT27,
    fgclut28: FGCLUT28,
    fgclut29: FGCLUT29,
    fgclut30: FGCLUT30,
    fgclut31: FGCLUT31,
    fgclut32: FGCLUT32,
    fgclut33: FGCLUT33,
    fgclut34: FGCLUT34,
    fgclut35: FGCLUT35,
    fgclut36: FGCLUT36,
    fgclut37: FGCLUT37,
    fgclut38: FGCLUT38,
    fgclut39: FGCLUT39,
    fgclut40: FGCLUT40,
    fgclut41: FGCLUT41,
    fgclut42: FGCLUT42,
    fgclut43: FGCLUT43,
    fgclut44: FGCLUT44,
    fgclut45: FGCLUT45,
    fgclut46: FGCLUT46,
    fgclut47: FGCLUT47,
    fgclut48: FGCLUT48,
    fgclut49: FGCLUT49,
    fgclut50: FGCLUT50,
    fgclut51: FGCLUT51,
    fgclut52: FGCLUT52,
    fgclut53: FGCLUT53,
    fgclut54: FGCLUT54,
    fgclut55: FGCLUT55,
    fgclut56: FGCLUT56,
    fgclut57: FGCLUT57,
    fgclut58: FGCLUT58,
    fgclut59: FGCLUT59,
    fgclut60: FGCLUT60,
    fgclut61: FGCLUT61,
    fgclut62: FGCLUT62,
    fgclut63: FGCLUT63,
    fgclut64: FGCLUT64,
    fgclut65: FGCLUT65,
    fgclut66: FGCLUT66,
    fgclut67: FGCLUT67,
    fgclut68: FGCLUT68,
    fgclut69: FGCLUT69,
    fgclut70: FGCLUT70,
    fgclut71: FGCLUT71,
    fgclut72: FGCLUT72,
    fgclut73: FGCLUT73,
    fgclut74: FGCLUT74,
    fgclut75: FGCLUT75,
    fgclut76: FGCLUT76,
    fgclut77: FGCLUT77,
    fgclut78: FGCLUT78,
    fgclut79: FGCLUT79,
    fgclut80: FGCLUT80,
    fgclut81: FGCLUT81,
    fgclut82: FGCLUT82,
    fgclut83: FGCLUT83,
    fgclut84: FGCLUT84,
    fgclut85: FGCLUT85,
    fgclut86: FGCLUT86,
    fgclut87: FGCLUT87,
    fgclut88: FGCLUT88,
    fgclut89: FGCLUT89,
    fgclut90: FGCLUT90,
    fgclut91: FGCLUT91,
    fgclut92: FGCLUT92,
    fgclut93: FGCLUT93,
    fgclut94: FGCLUT94,
    fgclut95: FGCLUT95,
    fgclut96: FGCLUT96,
    fgclut97: FGCLUT97,
    fgclut98: FGCLUT98,
    fgclut99: FGCLUT99,
    fgclut100: FGCLUT100,
    fgclut101: FGCLUT101,
    fgclut102: FGCLUT102,
    fgclut103: FGCLUT103,
    fgclut104: FGCLUT104,
    fgclut105: FGCLUT105,
    fgclut106: FGCLUT106,
    fgclut107: FGCLUT107,
    fgclut108: FGCLUT108,
    fgclut109: FGCLUT109,
    fgclut110: FGCLUT110,
    fgclut111: FGCLUT111,
    fgclut112: FGCLUT112,
    fgclut113: FGCLUT113,
    fgclut114: FGCLUT114,
    fgclut115: FGCLUT115,
    fgclut116: FGCLUT116,
    fgclut117: FGCLUT117,
    fgclut118: FGCLUT118,
    fgclut119: FGCLUT119,
    fgclut120: FGCLUT120,
    fgclut121: FGCLUT121,
    fgclut122: FGCLUT122,
    fgclut123: FGCLUT123,
    fgclut124: FGCLUT124,
    fgclut125: FGCLUT125,
    fgclut126: FGCLUT126,
    fgclut127: FGCLUT127,
    fgclut128: FGCLUT128,
    fgclut129: FGCLUT129,
    fgclut130: FGCLUT130,
    fgclut131: FGCLUT131,
    fgclut132: FGCLUT132,
    fgclut133: FGCLUT133,
    fgclut134: FGCLUT134,
    fgclut135: FGCLUT135,
    fgclut136: FGCLUT136,
    fgclut137: FGCLUT137,
    fgclut138: FGCLUT138,
    fgclut139: FGCLUT139,
    fgclut140: FGCLUT140,
    fgclut141: FGCLUT141,
    fgclut142: FGCLUT142,
    fgclut143: FGCLUT143,
    fgclut144: FGCLUT144,
    fgclut145: FGCLUT145,
    fgclut146: FGCLUT146,
    fgclut147: FGCLUT147,
    fgclut148: FGCLUT148,
    fgclut149: FGCLUT149,
    fgclut150: FGCLUT150,
    fgclut151: FGCLUT151,
    fgclut152: FGCLUT152,
    fgclut153: FGCLUT153,
    fgclut154: FGCLUT154,
    fgclut155: FGCLUT155,
    fgclut156: FGCLUT156,
    fgclut157: FGCLUT157,
    fgclut158: FGCLUT158,
    fgclut159: FGCLUT159,
    fgclut160: FGCLUT160,
    fgclut161: FGCLUT161,
    fgclut162: FGCLUT162,
    fgclut163: FGCLUT163,
    fgclut164: FGCLUT164,
    fgclut165: FGCLUT165,
    fgclut166: FGCLUT166,
    fgclut167: FGCLUT167,
    fgclut168: FGCLUT168,
    fgclut169: FGCLUT169,
    fgclut170: FGCLUT170,
    fgclut171: FGCLUT171,
    fgclut172: FGCLUT172,
    fgclut173: FGCLUT173,
    fgclut174: FGCLUT174,
    fgclut175: FGCLUT175,
    fgclut176: FGCLUT176,
    fgclut177: FGCLUT177,
    fgclut178: FGCLUT178,
    fgclut179: FGCLUT179,
    fgclut180: FGCLUT180,
    fgclut181: FGCLUT181,
    fgclut182: FGCLUT182,
    fgclut183: FGCLUT183,
    fgclut184: FGCLUT184,
    fgclut185: FGCLUT185,
    fgclut186: FGCLUT186,
    fgclut187: FGCLUT187,
    fgclut188: FGCLUT188,
    fgclut189: FGCLUT189,
    fgclut190: FGCLUT190,
    fgclut191: FGCLUT191,
    fgclut192: FGCLUT192,
    fgclut193: FGCLUT193,
    fgclut194: FGCLUT194,
    fgclut195: FGCLUT195,
    fgclut196: FGCLUT196,
    fgclut197: FGCLUT197,
    fgclut198: FGCLUT198,
    fgclut199: FGCLUT199,
    fgclut200: FGCLUT200,
    fgclut201: FGCLUT201,
    fgclut202: FGCLUT202,
    fgclut203: FGCLUT203,
    fgclut204: FGCLUT204,
    fgclut205: FGCLUT205,
    fgclut206: FGCLUT206,
    fgclut207: FGCLUT207,
    fgclut208: FGCLUT208,
    fgclut209: FGCLUT209,
    fgclut210: FGCLUT210,
    fgclut211: FGCLUT211,
    fgclut212: FGCLUT212,
    fgclut213: FGCLUT213,
    fgclut214: FGCLUT214,
    fgclut215: FGCLUT215,
    fgclut216: FGCLUT216,
    fgclut217: FGCLUT217,
    fgclut218: FGCLUT218,
    fgclut219: FGCLUT219,
    fgclut220: FGCLUT220,
    fgclut221: FGCLUT221,
    fgclut222: FGCLUT222,
    fgclut223: FGCLUT223,
    fgclut224: FGCLUT224,
    fgclut225: FGCLUT225,
    fgclut226: FGCLUT226,
    fgclut227: FGCLUT227,
    fgclut228: FGCLUT228,
    fgclut229: FGCLUT229,
    fgclut230: FGCLUT230,
    fgclut231: FGCLUT231,
    fgclut232: FGCLUT232,
    fgclut233: FGCLUT233,
    fgclut234: FGCLUT234,
    fgclut235: FGCLUT235,
    fgclut236: FGCLUT236,
    fgclut237: FGCLUT237,
    fgclut238: FGCLUT238,
    fgclut239: FGCLUT239,
    fgclut240: FGCLUT240,
    fgclut241: FGCLUT241,
    fgclut242: FGCLUT242,
    fgclut243: FGCLUT243,
    fgclut244: FGCLUT244,
    fgclut245: FGCLUT245,
    fgclut246: FGCLUT246,
    fgclut247: FGCLUT247,
    fgclut248: FGCLUT248,
    fgclut249: FGCLUT249,
    fgclut250: FGCLUT250,
    fgclut251: FGCLUT251,
    fgclut252: FGCLUT252,
    fgclut253: FGCLUT253,
    fgclut254: FGCLUT254,
    fgclut255: FGCLUT255,
    bgclut0: BGCLUT0,
    bgclut1: BGCLUT1,
    bgclut2: BGCLUT2,
    bgclut3: BGCLUT3,
    bgclut4: BGCLUT4,
    bgclut5: BGCLUT5,
    bgclut6: BGCLUT6,
    bgclut7: BGCLUT7,
    bgclut8: BGCLUT8,
    bgclut9: BGCLUT9,
    bgclut10: BGCLUT10,
    bgclut11: BGCLUT11,
    bgclut12: BGCLUT12,
    bgclut13: BGCLUT13,
    bgclut14: BGCLUT14,
    bgclut15: BGCLUT15,
    bgclut16: BGCLUT16,
    bgclut17: BGCLUT17,
    bgclut18: BGCLUT18,
    bgclut19: BGCLUT19,
    bgclut20: BGCLUT20,
    bgclut21: BGCLUT21,
    bgclut22: BGCLUT22,
    bgclut23: BGCLUT23,
    bgclut24: BGCLUT24,
    bgclut25: BGCLUT25,
    bgclut26: BGCLUT26,
    bgclut27: BGCLUT27,
    bgclut28: BGCLUT28,
    bgclut29: BGCLUT29,
    bgclut30: BGCLUT30,
    bgclut31: BGCLUT31,
    bgclut32: BGCLUT32,
    bgclut33: BGCLUT33,
    bgclut34: BGCLUT34,
    bgclut35: BGCLUT35,
    bgclut36: BGCLUT36,
    bgclut37: BGCLUT37,
    bgclut38: BGCLUT38,
    bgclut39: BGCLUT39,
    bgclut40: BGCLUT40,
    bgclut41: BGCLUT41,
    bgclut42: BGCLUT42,
    bgclut43: BGCLUT43,
    bgclut44: BGCLUT44,
    bgclut45: BGCLUT45,
    bgclut46: BGCLUT46,
    bgclut47: BGCLUT47,
    bgclut48: BGCLUT48,
    bgclut49: BGCLUT49,
    bgclut50: BGCLUT50,
    bgclut51: BGCLUT51,
    bgclut52: BGCLUT52,
    bgclut53: BGCLUT53,
    bgclut54: BGCLUT54,
    bgclut55: BGCLUT55,
    bgclut56: BGCLUT56,
    bgclut57: BGCLUT57,
    bgclut58: BGCLUT58,
    bgclut59: BGCLUT59,
    bgclut60: BGCLUT60,
    bgclut61: BGCLUT61,
    bgclut62: BGCLUT62,
    bgclut63: BGCLUT63,
    bgclut64: BGCLUT64,
    bgclut65: BGCLUT65,
    bgclut66: BGCLUT66,
    bgclut67: BGCLUT67,
    bgclut68: BGCLUT68,
    bgclut69: BGCLUT69,
    bgclut70: BGCLUT70,
    bgclut71: BGCLUT71,
    bgclut72: BGCLUT72,
    bgclut73: BGCLUT73,
    bgclut74: BGCLUT74,
    bgclut75: BGCLUT75,
    bgclut76: BGCLUT76,
    bgclut77: BGCLUT77,
    bgclut78: BGCLUT78,
    bgclut79: BGCLUT79,
    bgclut80: BGCLUT80,
    bgclut81: BGCLUT81,
    bgclut82: BGCLUT82,
    bgclut83: BGCLUT83,
    bgclut84: BGCLUT84,
    bgclut85: BGCLUT85,
    bgclut86: BGCLUT86,
    bgclut87: BGCLUT87,
    bgclut88: BGCLUT88,
    bgclut89: BGCLUT89,
    bgclut90: BGCLUT90,
    bgclut91: BGCLUT91,
    bgclut92: BGCLUT92,
    bgclut93: BGCLUT93,
    bgclut94: BGCLUT94,
    bgclut95: BGCLUT95,
    bgclut96: BGCLUT96,
    bgclut97: BGCLUT97,
    bgclut98: BGCLUT98,
    bgclut99: BGCLUT99,
    bgclut100: BGCLUT100,
    bgclut101: BGCLUT101,
    bgclut102: BGCLUT102,
    bgclut103: BGCLUT103,
    bgclut104: BGCLUT104,
    bgclut105: BGCLUT105,
    bgclut106: BGCLUT106,
    bgclut107: BGCLUT107,
    bgclut108: BGCLUT108,
    bgclut109: BGCLUT109,
    bgclut110: BGCLUT110,
    bgclut111: BGCLUT111,
    bgclut112: BGCLUT112,
    bgclut113: BGCLUT113,
    bgclut114: BGCLUT114,
    bgclut115: BGCLUT115,
    bgclut116: BGCLUT116,
    bgclut117: BGCLUT117,
    bgclut118: BGCLUT118,
    bgclut119: BGCLUT119,
    bgclut120: BGCLUT120,
    bgclut121: BGCLUT121,
    bgclut122: BGCLUT122,
    bgclut123: BGCLUT123,
    bgclut124: BGCLUT124,
    bgclut125: BGCLUT125,
    bgclut126: BGCLUT126,
    bgclut127: BGCLUT127,
    bgclut128: BGCLUT128,
    bgclut129: BGCLUT129,
    bgclut130: BGCLUT130,
    bgclut131: BGCLUT131,
    bgclut132: BGCLUT132,
    bgclut133: BGCLUT133,
    bgclut134: BGCLUT134,
    bgclut135: BGCLUT135,
    bgclut136: BGCLUT136,
    bgclut137: BGCLUT137,
    bgclut138: BGCLUT138,
    bgclut139: BGCLUT139,
    bgclut140: BGCLUT140,
    bgclut141: BGCLUT141,
    bgclut142: BGCLUT142,
    bgclut143: BGCLUT143,
    bgclut144: BGCLUT144,
    bgclut145: BGCLUT145,
    bgclut146: BGCLUT146,
    bgclut147: BGCLUT147,
    bgclut148: BGCLUT148,
    bgclut149: BGCLUT149,
    bgclut150: BGCLUT150,
    bgclut151: BGCLUT151,
    bgclut152: BGCLUT152,
    bgclut153: BGCLUT153,
    bgclut154: BGCLUT154,
    bgclut155: BGCLUT155,
    bgclut156: BGCLUT156,
    bgclut157: BGCLUT157,
    bgclut158: BGCLUT158,
    bgclut159: BGCLUT159,
    bgclut160: BGCLUT160,
    bgclut161: BGCLUT161,
    bgclut162: BGCLUT162,
    bgclut163: BGCLUT163,
    bgclut164: BGCLUT164,
    bgclut165: BGCLUT165,
    bgclut166: BGCLUT166,
    bgclut167: BGCLUT167,
    bgclut168: BGCLUT168,
    bgclut169: BGCLUT169,
    bgclut170: BGCLUT170,
    bgclut171: BGCLUT171,
    bgclut172: BGCLUT172,
    bgclut173: BGCLUT173,
    bgclut174: BGCLUT174,
    bgclut175: BGCLUT175,
    bgclut176: BGCLUT176,
    bgclut177: BGCLUT177,
    bgclut178: BGCLUT178,
    bgclut179: BGCLUT179,
    bgclut180: BGCLUT180,
    bgclut181: BGCLUT181,
    bgclut182: BGCLUT182,
    bgclut183: BGCLUT183,
    bgclut184: BGCLUT184,
    bgclut185: BGCLUT185,
    bgclut186: BGCLUT186,
    bgclut187: BGCLUT187,
    bgclut188: BGCLUT188,
    bgclut189: BGCLUT189,
    bgclut190: BGCLUT190,
    bgclut191: BGCLUT191,
    bgclut192: BGCLUT192,
    bgclut193: BGCLUT193,
    bgclut194: BGCLUT194,
    bgclut195: BGCLUT195,
    bgclut196: BGCLUT196,
    bgclut197: BGCLUT197,
    bgclut198: BGCLUT198,
    bgclut199: BGCLUT199,
    bgclut200: BGCLUT200,
    bgclut201: BGCLUT201,
    bgclut202: BGCLUT202,
    bgclut203: BGCLUT203,
    bgclut204: BGCLUT204,
    bgclut205: BGCLUT205,
    bgclut206: BGCLUT206,
    bgclut207: BGCLUT207,
    bgclut208: BGCLUT208,
    bgclut209: BGCLUT209,
    bgclut210: BGCLUT210,
    bgclut211: BGCLUT211,
    bgclut212: BGCLUT212,
    bgclut213: BGCLUT213,
    bgclut214: BGCLUT214,
    bgclut215: BGCLUT215,
    bgclut216: BGCLUT216,
    bgclut217: BGCLUT217,
    bgclut218: BGCLUT218,
    bgclut219: BGCLUT219,
    bgclut220: BGCLUT220,
    bgclut221: BGCLUT221,
    bgclut222: BGCLUT222,
    bgclut223: BGCLUT223,
    bgclut224: BGCLUT224,
    bgclut225: BGCLUT225,
    bgclut226: BGCLUT226,
    bgclut227: BGCLUT227,
    bgclut228: BGCLUT228,
    bgclut229: BGCLUT229,
    bgclut230: BGCLUT230,
    bgclut231: BGCLUT231,
    bgclut232: BGCLUT232,
    bgclut233: BGCLUT233,
    bgclut234: BGCLUT234,
    bgclut235: BGCLUT235,
    bgclut236: BGCLUT236,
    bgclut237: BGCLUT237,
    bgclut238: BGCLUT238,
    bgclut239: BGCLUT239,
    bgclut240: BGCLUT240,
    bgclut241: BGCLUT241,
    bgclut242: BGCLUT242,
    bgclut243: BGCLUT243,
    bgclut244: BGCLUT244,
    bgclut245: BGCLUT245,
    bgclut246: BGCLUT246,
    bgclut247: BGCLUT247,
    bgclut248: BGCLUT248,
    bgclut249: BGCLUT249,
    bgclut250: BGCLUT250,
    bgclut251: BGCLUT251,
    bgclut252: BGCLUT252,
    bgclut253: BGCLUT253,
    bgclut254: BGCLUT254,
    bgclut255: BGCLUT255,
}
impl RegisterBlock {
    ///0x00 - DMA2D control register
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    ///0x04 - DMA2D interrupt status register
    #[inline(always)]
    pub const fn isr(&self) -> &ISR {
        &self.isr
    }
    ///0x08 - DMA2D interrupt flag clear register
    #[inline(always)]
    pub const fn ifcr(&self) -> &IFCR {
        &self.ifcr
    }
    ///0x0c - DMA2D foreground memory address register
    #[inline(always)]
    pub const fn fgmar(&self) -> &FGMAR {
        &self.fgmar
    }
    ///0x10 - DMA2D foreground offset register
    #[inline(always)]
    pub const fn fgor(&self) -> &FGOR {
        &self.fgor
    }
    ///0x14 - DMA2D background memory address register
    #[inline(always)]
    pub const fn bgmar(&self) -> &BGMAR {
        &self.bgmar
    }
    ///0x18 - DMA2D background offset register
    #[inline(always)]
    pub const fn bgor(&self) -> &BGOR {
        &self.bgor
    }
    ///0x1c - DMA2D foreground PFC control register
    #[inline(always)]
    pub const fn fgpfccr(&self) -> &FGPFCCR {
        &self.fgpfccr
    }
    ///0x20 - DMA2D foreground color register
    #[inline(always)]
    pub const fn fgcolr(&self) -> &FGCOLR {
        &self.fgcolr
    }
    ///0x24 - DMA2D background PFC control register
    #[inline(always)]
    pub const fn bgpfccr(&self) -> &BGPFCCR {
        &self.bgpfccr
    }
    ///0x28 - DMA2D background color register
    #[inline(always)]
    pub const fn bgcolr(&self) -> &BGCOLR {
        &self.bgcolr
    }
    ///0x2c - DMA2D foreground CLUT memory address register
    #[inline(always)]
    pub const fn fgcmar(&self) -> &FGCMAR {
        &self.fgcmar
    }
    ///0x30 - DMA2D background CLUT memory address register
    #[inline(always)]
    pub const fn bgcmar(&self) -> &BGCMAR {
        &self.bgcmar
    }
    ///0x34 - DMA2D output PFC control register
    #[inline(always)]
    pub const fn opfccr(&self) -> &OPFCCR {
        &self.opfccr
    }
    ///0x38 - DMA2D output color register
    #[inline(always)]
    pub const fn ocolr_argb4444(&self) -> &OCOLR_ARGB4444 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x38 - DMA2D output color register
    #[inline(always)]
    pub const fn ocolr_argb1555(&self) -> &OCOLR_ARGB1555 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x38 - DMA2D output color register
    #[inline(always)]
    pub const fn ocolr_rgb565(&self) -> &OCOLR_RGB565 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x38 - DMA2D output color register
    #[inline(always)]
    pub const fn ocolr_rgb888(&self) -> &OCOLR_RGB888 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(56).cast() }
    }
    ///0x3c - DMA2D output memory address register
    #[inline(always)]
    pub const fn omar(&self) -> &OMAR {
        &self.omar
    }
    ///0x40 - DMA2D output offset register
    #[inline(always)]
    pub const fn oor(&self) -> &OOR {
        &self.oor
    }
    ///0x44 - DMA2D number of line register
    #[inline(always)]
    pub const fn nlr(&self) -> &NLR {
        &self.nlr
    }
    ///0x48 - DMA2D line watermark register
    #[inline(always)]
    pub const fn lwr(&self) -> &LWR {
        &self.lwr
    }
    ///0x4c - DMA2D AXI master timer configuration register
    #[inline(always)]
    pub const fn amtcr(&self) -> &AMTCR {
        &self.amtcr
    }
    ///0x400 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut0(&self) -> &FGCLUT0 {
        &self.fgclut0
    }
    ///0x404 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut1(&self) -> &FGCLUT1 {
        &self.fgclut1
    }
    ///0x408 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut2(&self) -> &FGCLUT2 {
        &self.fgclut2
    }
    ///0x40c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut3(&self) -> &FGCLUT3 {
        &self.fgclut3
    }
    ///0x410 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut4(&self) -> &FGCLUT4 {
        &self.fgclut4
    }
    ///0x414 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut5(&self) -> &FGCLUT5 {
        &self.fgclut5
    }
    ///0x418 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut6(&self) -> &FGCLUT6 {
        &self.fgclut6
    }
    ///0x41c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut7(&self) -> &FGCLUT7 {
        &self.fgclut7
    }
    ///0x420 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut8(&self) -> &FGCLUT8 {
        &self.fgclut8
    }
    ///0x424 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut9(&self) -> &FGCLUT9 {
        &self.fgclut9
    }
    ///0x428 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut10(&self) -> &FGCLUT10 {
        &self.fgclut10
    }
    ///0x42c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut11(&self) -> &FGCLUT11 {
        &self.fgclut11
    }
    ///0x430 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut12(&self) -> &FGCLUT12 {
        &self.fgclut12
    }
    ///0x434 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut13(&self) -> &FGCLUT13 {
        &self.fgclut13
    }
    ///0x438 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut14(&self) -> &FGCLUT14 {
        &self.fgclut14
    }
    ///0x43c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut15(&self) -> &FGCLUT15 {
        &self.fgclut15
    }
    ///0x440 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut16(&self) -> &FGCLUT16 {
        &self.fgclut16
    }
    ///0x444 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut17(&self) -> &FGCLUT17 {
        &self.fgclut17
    }
    ///0x448 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut18(&self) -> &FGCLUT18 {
        &self.fgclut18
    }
    ///0x44c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut19(&self) -> &FGCLUT19 {
        &self.fgclut19
    }
    ///0x450 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut20(&self) -> &FGCLUT20 {
        &self.fgclut20
    }
    ///0x454 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut21(&self) -> &FGCLUT21 {
        &self.fgclut21
    }
    ///0x458 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut22(&self) -> &FGCLUT22 {
        &self.fgclut22
    }
    ///0x45c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut23(&self) -> &FGCLUT23 {
        &self.fgclut23
    }
    ///0x460 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut24(&self) -> &FGCLUT24 {
        &self.fgclut24
    }
    ///0x464 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut25(&self) -> &FGCLUT25 {
        &self.fgclut25
    }
    ///0x468 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut26(&self) -> &FGCLUT26 {
        &self.fgclut26
    }
    ///0x46c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut27(&self) -> &FGCLUT27 {
        &self.fgclut27
    }
    ///0x470 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut28(&self) -> &FGCLUT28 {
        &self.fgclut28
    }
    ///0x474 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut29(&self) -> &FGCLUT29 {
        &self.fgclut29
    }
    ///0x478 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut30(&self) -> &FGCLUT30 {
        &self.fgclut30
    }
    ///0x47c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut31(&self) -> &FGCLUT31 {
        &self.fgclut31
    }
    ///0x480 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut32(&self) -> &FGCLUT32 {
        &self.fgclut32
    }
    ///0x484 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut33(&self) -> &FGCLUT33 {
        &self.fgclut33
    }
    ///0x488 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut34(&self) -> &FGCLUT34 {
        &self.fgclut34
    }
    ///0x48c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut35(&self) -> &FGCLUT35 {
        &self.fgclut35
    }
    ///0x490 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut36(&self) -> &FGCLUT36 {
        &self.fgclut36
    }
    ///0x494 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut37(&self) -> &FGCLUT37 {
        &self.fgclut37
    }
    ///0x498 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut38(&self) -> &FGCLUT38 {
        &self.fgclut38
    }
    ///0x49c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut39(&self) -> &FGCLUT39 {
        &self.fgclut39
    }
    ///0x4a0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut40(&self) -> &FGCLUT40 {
        &self.fgclut40
    }
    ///0x4a4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut41(&self) -> &FGCLUT41 {
        &self.fgclut41
    }
    ///0x4a8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut42(&self) -> &FGCLUT42 {
        &self.fgclut42
    }
    ///0x4ac - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut43(&self) -> &FGCLUT43 {
        &self.fgclut43
    }
    ///0x4b0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut44(&self) -> &FGCLUT44 {
        &self.fgclut44
    }
    ///0x4b4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut45(&self) -> &FGCLUT45 {
        &self.fgclut45
    }
    ///0x4b8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut46(&self) -> &FGCLUT46 {
        &self.fgclut46
    }
    ///0x4bc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut47(&self) -> &FGCLUT47 {
        &self.fgclut47
    }
    ///0x4c0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut48(&self) -> &FGCLUT48 {
        &self.fgclut48
    }
    ///0x4c4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut49(&self) -> &FGCLUT49 {
        &self.fgclut49
    }
    ///0x4c8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut50(&self) -> &FGCLUT50 {
        &self.fgclut50
    }
    ///0x4cc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut51(&self) -> &FGCLUT51 {
        &self.fgclut51
    }
    ///0x4d0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut52(&self) -> &FGCLUT52 {
        &self.fgclut52
    }
    ///0x4d4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut53(&self) -> &FGCLUT53 {
        &self.fgclut53
    }
    ///0x4d8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut54(&self) -> &FGCLUT54 {
        &self.fgclut54
    }
    ///0x4dc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut55(&self) -> &FGCLUT55 {
        &self.fgclut55
    }
    ///0x4e0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut56(&self) -> &FGCLUT56 {
        &self.fgclut56
    }
    ///0x4e4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut57(&self) -> &FGCLUT57 {
        &self.fgclut57
    }
    ///0x4e8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut58(&self) -> &FGCLUT58 {
        &self.fgclut58
    }
    ///0x4ec - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut59(&self) -> &FGCLUT59 {
        &self.fgclut59
    }
    ///0x4f0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut60(&self) -> &FGCLUT60 {
        &self.fgclut60
    }
    ///0x4f4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut61(&self) -> &FGCLUT61 {
        &self.fgclut61
    }
    ///0x4f8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut62(&self) -> &FGCLUT62 {
        &self.fgclut62
    }
    ///0x4fc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut63(&self) -> &FGCLUT63 {
        &self.fgclut63
    }
    ///0x500 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut64(&self) -> &FGCLUT64 {
        &self.fgclut64
    }
    ///0x504 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut65(&self) -> &FGCLUT65 {
        &self.fgclut65
    }
    ///0x508 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut66(&self) -> &FGCLUT66 {
        &self.fgclut66
    }
    ///0x50c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut67(&self) -> &FGCLUT67 {
        &self.fgclut67
    }
    ///0x510 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut68(&self) -> &FGCLUT68 {
        &self.fgclut68
    }
    ///0x514 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut69(&self) -> &FGCLUT69 {
        &self.fgclut69
    }
    ///0x518 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut70(&self) -> &FGCLUT70 {
        &self.fgclut70
    }
    ///0x51c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut71(&self) -> &FGCLUT71 {
        &self.fgclut71
    }
    ///0x520 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut72(&self) -> &FGCLUT72 {
        &self.fgclut72
    }
    ///0x524 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut73(&self) -> &FGCLUT73 {
        &self.fgclut73
    }
    ///0x528 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut74(&self) -> &FGCLUT74 {
        &self.fgclut74
    }
    ///0x52c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut75(&self) -> &FGCLUT75 {
        &self.fgclut75
    }
    ///0x530 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut76(&self) -> &FGCLUT76 {
        &self.fgclut76
    }
    ///0x534 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut77(&self) -> &FGCLUT77 {
        &self.fgclut77
    }
    ///0x538 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut78(&self) -> &FGCLUT78 {
        &self.fgclut78
    }
    ///0x53c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut79(&self) -> &FGCLUT79 {
        &self.fgclut79
    }
    ///0x540 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut80(&self) -> &FGCLUT80 {
        &self.fgclut80
    }
    ///0x544 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut81(&self) -> &FGCLUT81 {
        &self.fgclut81
    }
    ///0x548 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut82(&self) -> &FGCLUT82 {
        &self.fgclut82
    }
    ///0x54c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut83(&self) -> &FGCLUT83 {
        &self.fgclut83
    }
    ///0x550 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut84(&self) -> &FGCLUT84 {
        &self.fgclut84
    }
    ///0x554 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut85(&self) -> &FGCLUT85 {
        &self.fgclut85
    }
    ///0x558 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut86(&self) -> &FGCLUT86 {
        &self.fgclut86
    }
    ///0x55c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut87(&self) -> &FGCLUT87 {
        &self.fgclut87
    }
    ///0x560 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut88(&self) -> &FGCLUT88 {
        &self.fgclut88
    }
    ///0x564 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut89(&self) -> &FGCLUT89 {
        &self.fgclut89
    }
    ///0x568 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut90(&self) -> &FGCLUT90 {
        &self.fgclut90
    }
    ///0x56c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut91(&self) -> &FGCLUT91 {
        &self.fgclut91
    }
    ///0x570 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut92(&self) -> &FGCLUT92 {
        &self.fgclut92
    }
    ///0x574 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut93(&self) -> &FGCLUT93 {
        &self.fgclut93
    }
    ///0x578 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut94(&self) -> &FGCLUT94 {
        &self.fgclut94
    }
    ///0x57c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut95(&self) -> &FGCLUT95 {
        &self.fgclut95
    }
    ///0x580 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut96(&self) -> &FGCLUT96 {
        &self.fgclut96
    }
    ///0x584 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut97(&self) -> &FGCLUT97 {
        &self.fgclut97
    }
    ///0x588 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut98(&self) -> &FGCLUT98 {
        &self.fgclut98
    }
    ///0x58c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut99(&self) -> &FGCLUT99 {
        &self.fgclut99
    }
    ///0x590 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut100(&self) -> &FGCLUT100 {
        &self.fgclut100
    }
    ///0x594 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut101(&self) -> &FGCLUT101 {
        &self.fgclut101
    }
    ///0x598 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut102(&self) -> &FGCLUT102 {
        &self.fgclut102
    }
    ///0x59c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut103(&self) -> &FGCLUT103 {
        &self.fgclut103
    }
    ///0x5a0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut104(&self) -> &FGCLUT104 {
        &self.fgclut104
    }
    ///0x5a4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut105(&self) -> &FGCLUT105 {
        &self.fgclut105
    }
    ///0x5a8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut106(&self) -> &FGCLUT106 {
        &self.fgclut106
    }
    ///0x5ac - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut107(&self) -> &FGCLUT107 {
        &self.fgclut107
    }
    ///0x5b0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut108(&self) -> &FGCLUT108 {
        &self.fgclut108
    }
    ///0x5b4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut109(&self) -> &FGCLUT109 {
        &self.fgclut109
    }
    ///0x5b8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut110(&self) -> &FGCLUT110 {
        &self.fgclut110
    }
    ///0x5bc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut111(&self) -> &FGCLUT111 {
        &self.fgclut111
    }
    ///0x5c0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut112(&self) -> &FGCLUT112 {
        &self.fgclut112
    }
    ///0x5c4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut113(&self) -> &FGCLUT113 {
        &self.fgclut113
    }
    ///0x5c8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut114(&self) -> &FGCLUT114 {
        &self.fgclut114
    }
    ///0x5cc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut115(&self) -> &FGCLUT115 {
        &self.fgclut115
    }
    ///0x5d0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut116(&self) -> &FGCLUT116 {
        &self.fgclut116
    }
    ///0x5d4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut117(&self) -> &FGCLUT117 {
        &self.fgclut117
    }
    ///0x5d8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut118(&self) -> &FGCLUT118 {
        &self.fgclut118
    }
    ///0x5dc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut119(&self) -> &FGCLUT119 {
        &self.fgclut119
    }
    ///0x5e0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut120(&self) -> &FGCLUT120 {
        &self.fgclut120
    }
    ///0x5e4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut121(&self) -> &FGCLUT121 {
        &self.fgclut121
    }
    ///0x5e8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut122(&self) -> &FGCLUT122 {
        &self.fgclut122
    }
    ///0x5ec - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut123(&self) -> &FGCLUT123 {
        &self.fgclut123
    }
    ///0x5f0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut124(&self) -> &FGCLUT124 {
        &self.fgclut124
    }
    ///0x5f4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut125(&self) -> &FGCLUT125 {
        &self.fgclut125
    }
    ///0x5f8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut126(&self) -> &FGCLUT126 {
        &self.fgclut126
    }
    ///0x5fc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut127(&self) -> &FGCLUT127 {
        &self.fgclut127
    }
    ///0x600 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut128(&self) -> &FGCLUT128 {
        &self.fgclut128
    }
    ///0x604 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut129(&self) -> &FGCLUT129 {
        &self.fgclut129
    }
    ///0x608 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut130(&self) -> &FGCLUT130 {
        &self.fgclut130
    }
    ///0x60c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut131(&self) -> &FGCLUT131 {
        &self.fgclut131
    }
    ///0x610 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut132(&self) -> &FGCLUT132 {
        &self.fgclut132
    }
    ///0x614 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut133(&self) -> &FGCLUT133 {
        &self.fgclut133
    }
    ///0x618 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut134(&self) -> &FGCLUT134 {
        &self.fgclut134
    }
    ///0x61c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut135(&self) -> &FGCLUT135 {
        &self.fgclut135
    }
    ///0x620 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut136(&self) -> &FGCLUT136 {
        &self.fgclut136
    }
    ///0x624 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut137(&self) -> &FGCLUT137 {
        &self.fgclut137
    }
    ///0x628 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut138(&self) -> &FGCLUT138 {
        &self.fgclut138
    }
    ///0x62c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut139(&self) -> &FGCLUT139 {
        &self.fgclut139
    }
    ///0x630 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut140(&self) -> &FGCLUT140 {
        &self.fgclut140
    }
    ///0x634 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut141(&self) -> &FGCLUT141 {
        &self.fgclut141
    }
    ///0x638 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut142(&self) -> &FGCLUT142 {
        &self.fgclut142
    }
    ///0x63c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut143(&self) -> &FGCLUT143 {
        &self.fgclut143
    }
    ///0x640 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut144(&self) -> &FGCLUT144 {
        &self.fgclut144
    }
    ///0x644 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut145(&self) -> &FGCLUT145 {
        &self.fgclut145
    }
    ///0x648 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut146(&self) -> &FGCLUT146 {
        &self.fgclut146
    }
    ///0x64c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut147(&self) -> &FGCLUT147 {
        &self.fgclut147
    }
    ///0x650 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut148(&self) -> &FGCLUT148 {
        &self.fgclut148
    }
    ///0x654 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut149(&self) -> &FGCLUT149 {
        &self.fgclut149
    }
    ///0x658 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut150(&self) -> &FGCLUT150 {
        &self.fgclut150
    }
    ///0x65c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut151(&self) -> &FGCLUT151 {
        &self.fgclut151
    }
    ///0x660 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut152(&self) -> &FGCLUT152 {
        &self.fgclut152
    }
    ///0x664 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut153(&self) -> &FGCLUT153 {
        &self.fgclut153
    }
    ///0x668 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut154(&self) -> &FGCLUT154 {
        &self.fgclut154
    }
    ///0x66c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut155(&self) -> &FGCLUT155 {
        &self.fgclut155
    }
    ///0x670 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut156(&self) -> &FGCLUT156 {
        &self.fgclut156
    }
    ///0x674 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut157(&self) -> &FGCLUT157 {
        &self.fgclut157
    }
    ///0x678 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut158(&self) -> &FGCLUT158 {
        &self.fgclut158
    }
    ///0x67c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut159(&self) -> &FGCLUT159 {
        &self.fgclut159
    }
    ///0x680 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut160(&self) -> &FGCLUT160 {
        &self.fgclut160
    }
    ///0x684 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut161(&self) -> &FGCLUT161 {
        &self.fgclut161
    }
    ///0x688 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut162(&self) -> &FGCLUT162 {
        &self.fgclut162
    }
    ///0x68c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut163(&self) -> &FGCLUT163 {
        &self.fgclut163
    }
    ///0x690 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut164(&self) -> &FGCLUT164 {
        &self.fgclut164
    }
    ///0x694 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut165(&self) -> &FGCLUT165 {
        &self.fgclut165
    }
    ///0x698 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut166(&self) -> &FGCLUT166 {
        &self.fgclut166
    }
    ///0x69c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut167(&self) -> &FGCLUT167 {
        &self.fgclut167
    }
    ///0x6a0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut168(&self) -> &FGCLUT168 {
        &self.fgclut168
    }
    ///0x6a4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut169(&self) -> &FGCLUT169 {
        &self.fgclut169
    }
    ///0x6a8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut170(&self) -> &FGCLUT170 {
        &self.fgclut170
    }
    ///0x6ac - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut171(&self) -> &FGCLUT171 {
        &self.fgclut171
    }
    ///0x6b0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut172(&self) -> &FGCLUT172 {
        &self.fgclut172
    }
    ///0x6b4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut173(&self) -> &FGCLUT173 {
        &self.fgclut173
    }
    ///0x6b8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut174(&self) -> &FGCLUT174 {
        &self.fgclut174
    }
    ///0x6bc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut175(&self) -> &FGCLUT175 {
        &self.fgclut175
    }
    ///0x6c0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut176(&self) -> &FGCLUT176 {
        &self.fgclut176
    }
    ///0x6c4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut177(&self) -> &FGCLUT177 {
        &self.fgclut177
    }
    ///0x6c8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut178(&self) -> &FGCLUT178 {
        &self.fgclut178
    }
    ///0x6cc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut179(&self) -> &FGCLUT179 {
        &self.fgclut179
    }
    ///0x6d0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut180(&self) -> &FGCLUT180 {
        &self.fgclut180
    }
    ///0x6d4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut181(&self) -> &FGCLUT181 {
        &self.fgclut181
    }
    ///0x6d8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut182(&self) -> &FGCLUT182 {
        &self.fgclut182
    }
    ///0x6dc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut183(&self) -> &FGCLUT183 {
        &self.fgclut183
    }
    ///0x6e0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut184(&self) -> &FGCLUT184 {
        &self.fgclut184
    }
    ///0x6e4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut185(&self) -> &FGCLUT185 {
        &self.fgclut185
    }
    ///0x6e8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut186(&self) -> &FGCLUT186 {
        &self.fgclut186
    }
    ///0x6ec - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut187(&self) -> &FGCLUT187 {
        &self.fgclut187
    }
    ///0x6f0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut188(&self) -> &FGCLUT188 {
        &self.fgclut188
    }
    ///0x6f4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut189(&self) -> &FGCLUT189 {
        &self.fgclut189
    }
    ///0x6f8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut190(&self) -> &FGCLUT190 {
        &self.fgclut190
    }
    ///0x6fc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut191(&self) -> &FGCLUT191 {
        &self.fgclut191
    }
    ///0x700 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut192(&self) -> &FGCLUT192 {
        &self.fgclut192
    }
    ///0x704 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut193(&self) -> &FGCLUT193 {
        &self.fgclut193
    }
    ///0x708 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut194(&self) -> &FGCLUT194 {
        &self.fgclut194
    }
    ///0x70c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut195(&self) -> &FGCLUT195 {
        &self.fgclut195
    }
    ///0x710 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut196(&self) -> &FGCLUT196 {
        &self.fgclut196
    }
    ///0x714 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut197(&self) -> &FGCLUT197 {
        &self.fgclut197
    }
    ///0x718 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut198(&self) -> &FGCLUT198 {
        &self.fgclut198
    }
    ///0x71c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut199(&self) -> &FGCLUT199 {
        &self.fgclut199
    }
    ///0x720 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut200(&self) -> &FGCLUT200 {
        &self.fgclut200
    }
    ///0x724 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut201(&self) -> &FGCLUT201 {
        &self.fgclut201
    }
    ///0x728 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut202(&self) -> &FGCLUT202 {
        &self.fgclut202
    }
    ///0x72c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut203(&self) -> &FGCLUT203 {
        &self.fgclut203
    }
    ///0x730 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut204(&self) -> &FGCLUT204 {
        &self.fgclut204
    }
    ///0x734 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut205(&self) -> &FGCLUT205 {
        &self.fgclut205
    }
    ///0x738 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut206(&self) -> &FGCLUT206 {
        &self.fgclut206
    }
    ///0x73c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut207(&self) -> &FGCLUT207 {
        &self.fgclut207
    }
    ///0x740 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut208(&self) -> &FGCLUT208 {
        &self.fgclut208
    }
    ///0x744 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut209(&self) -> &FGCLUT209 {
        &self.fgclut209
    }
    ///0x748 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut210(&self) -> &FGCLUT210 {
        &self.fgclut210
    }
    ///0x74c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut211(&self) -> &FGCLUT211 {
        &self.fgclut211
    }
    ///0x750 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut212(&self) -> &FGCLUT212 {
        &self.fgclut212
    }
    ///0x754 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut213(&self) -> &FGCLUT213 {
        &self.fgclut213
    }
    ///0x758 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut214(&self) -> &FGCLUT214 {
        &self.fgclut214
    }
    ///0x75c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut215(&self) -> &FGCLUT215 {
        &self.fgclut215
    }
    ///0x760 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut216(&self) -> &FGCLUT216 {
        &self.fgclut216
    }
    ///0x764 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut217(&self) -> &FGCLUT217 {
        &self.fgclut217
    }
    ///0x768 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut218(&self) -> &FGCLUT218 {
        &self.fgclut218
    }
    ///0x76c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut219(&self) -> &FGCLUT219 {
        &self.fgclut219
    }
    ///0x770 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut220(&self) -> &FGCLUT220 {
        &self.fgclut220
    }
    ///0x774 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut221(&self) -> &FGCLUT221 {
        &self.fgclut221
    }
    ///0x778 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut222(&self) -> &FGCLUT222 {
        &self.fgclut222
    }
    ///0x77c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut223(&self) -> &FGCLUT223 {
        &self.fgclut223
    }
    ///0x780 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut224(&self) -> &FGCLUT224 {
        &self.fgclut224
    }
    ///0x784 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut225(&self) -> &FGCLUT225 {
        &self.fgclut225
    }
    ///0x788 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut226(&self) -> &FGCLUT226 {
        &self.fgclut226
    }
    ///0x78c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut227(&self) -> &FGCLUT227 {
        &self.fgclut227
    }
    ///0x790 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut228(&self) -> &FGCLUT228 {
        &self.fgclut228
    }
    ///0x794 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut229(&self) -> &FGCLUT229 {
        &self.fgclut229
    }
    ///0x798 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut230(&self) -> &FGCLUT230 {
        &self.fgclut230
    }
    ///0x79c - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut231(&self) -> &FGCLUT231 {
        &self.fgclut231
    }
    ///0x7a0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut232(&self) -> &FGCLUT232 {
        &self.fgclut232
    }
    ///0x7a4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut233(&self) -> &FGCLUT233 {
        &self.fgclut233
    }
    ///0x7a8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut234(&self) -> &FGCLUT234 {
        &self.fgclut234
    }
    ///0x7ac - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut235(&self) -> &FGCLUT235 {
        &self.fgclut235
    }
    ///0x7b0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut236(&self) -> &FGCLUT236 {
        &self.fgclut236
    }
    ///0x7b4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut237(&self) -> &FGCLUT237 {
        &self.fgclut237
    }
    ///0x7b8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut238(&self) -> &FGCLUT238 {
        &self.fgclut238
    }
    ///0x7bc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut239(&self) -> &FGCLUT239 {
        &self.fgclut239
    }
    ///0x7c0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut240(&self) -> &FGCLUT240 {
        &self.fgclut240
    }
    ///0x7c4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut241(&self) -> &FGCLUT241 {
        &self.fgclut241
    }
    ///0x7c8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut242(&self) -> &FGCLUT242 {
        &self.fgclut242
    }
    ///0x7cc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut243(&self) -> &FGCLUT243 {
        &self.fgclut243
    }
    ///0x7d0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut244(&self) -> &FGCLUT244 {
        &self.fgclut244
    }
    ///0x7d4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut245(&self) -> &FGCLUT245 {
        &self.fgclut245
    }
    ///0x7d8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut246(&self) -> &FGCLUT246 {
        &self.fgclut246
    }
    ///0x7dc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut247(&self) -> &FGCLUT247 {
        &self.fgclut247
    }
    ///0x7e0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut248(&self) -> &FGCLUT248 {
        &self.fgclut248
    }
    ///0x7e4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut249(&self) -> &FGCLUT249 {
        &self.fgclut249
    }
    ///0x7e8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut250(&self) -> &FGCLUT250 {
        &self.fgclut250
    }
    ///0x7ec - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut251(&self) -> &FGCLUT251 {
        &self.fgclut251
    }
    ///0x7f0 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut252(&self) -> &FGCLUT252 {
        &self.fgclut252
    }
    ///0x7f4 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut253(&self) -> &FGCLUT253 {
        &self.fgclut253
    }
    ///0x7f8 - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut254(&self) -> &FGCLUT254 {
        &self.fgclut254
    }
    ///0x7fc - DMA2D foreground CLUT
    #[inline(always)]
    pub const fn fgclut255(&self) -> &FGCLUT255 {
        &self.fgclut255
    }
    ///0x800 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut0(&self) -> &BGCLUT0 {
        &self.bgclut0
    }
    ///0x804 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut1(&self) -> &BGCLUT1 {
        &self.bgclut1
    }
    ///0x808 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut2(&self) -> &BGCLUT2 {
        &self.bgclut2
    }
    ///0x80c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut3(&self) -> &BGCLUT3 {
        &self.bgclut3
    }
    ///0x810 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut4(&self) -> &BGCLUT4 {
        &self.bgclut4
    }
    ///0x814 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut5(&self) -> &BGCLUT5 {
        &self.bgclut5
    }
    ///0x818 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut6(&self) -> &BGCLUT6 {
        &self.bgclut6
    }
    ///0x81c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut7(&self) -> &BGCLUT7 {
        &self.bgclut7
    }
    ///0x820 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut8(&self) -> &BGCLUT8 {
        &self.bgclut8
    }
    ///0x824 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut9(&self) -> &BGCLUT9 {
        &self.bgclut9
    }
    ///0x828 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut10(&self) -> &BGCLUT10 {
        &self.bgclut10
    }
    ///0x82c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut11(&self) -> &BGCLUT11 {
        &self.bgclut11
    }
    ///0x830 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut12(&self) -> &BGCLUT12 {
        &self.bgclut12
    }
    ///0x834 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut13(&self) -> &BGCLUT13 {
        &self.bgclut13
    }
    ///0x838 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut14(&self) -> &BGCLUT14 {
        &self.bgclut14
    }
    ///0x83c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut15(&self) -> &BGCLUT15 {
        &self.bgclut15
    }
    ///0x840 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut16(&self) -> &BGCLUT16 {
        &self.bgclut16
    }
    ///0x844 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut17(&self) -> &BGCLUT17 {
        &self.bgclut17
    }
    ///0x848 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut18(&self) -> &BGCLUT18 {
        &self.bgclut18
    }
    ///0x84c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut19(&self) -> &BGCLUT19 {
        &self.bgclut19
    }
    ///0x850 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut20(&self) -> &BGCLUT20 {
        &self.bgclut20
    }
    ///0x854 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut21(&self) -> &BGCLUT21 {
        &self.bgclut21
    }
    ///0x858 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut22(&self) -> &BGCLUT22 {
        &self.bgclut22
    }
    ///0x85c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut23(&self) -> &BGCLUT23 {
        &self.bgclut23
    }
    ///0x860 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut24(&self) -> &BGCLUT24 {
        &self.bgclut24
    }
    ///0x864 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut25(&self) -> &BGCLUT25 {
        &self.bgclut25
    }
    ///0x868 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut26(&self) -> &BGCLUT26 {
        &self.bgclut26
    }
    ///0x86c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut27(&self) -> &BGCLUT27 {
        &self.bgclut27
    }
    ///0x870 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut28(&self) -> &BGCLUT28 {
        &self.bgclut28
    }
    ///0x874 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut29(&self) -> &BGCLUT29 {
        &self.bgclut29
    }
    ///0x878 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut30(&self) -> &BGCLUT30 {
        &self.bgclut30
    }
    ///0x87c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut31(&self) -> &BGCLUT31 {
        &self.bgclut31
    }
    ///0x880 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut32(&self) -> &BGCLUT32 {
        &self.bgclut32
    }
    ///0x884 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut33(&self) -> &BGCLUT33 {
        &self.bgclut33
    }
    ///0x888 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut34(&self) -> &BGCLUT34 {
        &self.bgclut34
    }
    ///0x88c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut35(&self) -> &BGCLUT35 {
        &self.bgclut35
    }
    ///0x890 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut36(&self) -> &BGCLUT36 {
        &self.bgclut36
    }
    ///0x894 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut37(&self) -> &BGCLUT37 {
        &self.bgclut37
    }
    ///0x898 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut38(&self) -> &BGCLUT38 {
        &self.bgclut38
    }
    ///0x89c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut39(&self) -> &BGCLUT39 {
        &self.bgclut39
    }
    ///0x8a0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut40(&self) -> &BGCLUT40 {
        &self.bgclut40
    }
    ///0x8a4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut41(&self) -> &BGCLUT41 {
        &self.bgclut41
    }
    ///0x8a8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut42(&self) -> &BGCLUT42 {
        &self.bgclut42
    }
    ///0x8ac - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut43(&self) -> &BGCLUT43 {
        &self.bgclut43
    }
    ///0x8b0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut44(&self) -> &BGCLUT44 {
        &self.bgclut44
    }
    ///0x8b4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut45(&self) -> &BGCLUT45 {
        &self.bgclut45
    }
    ///0x8b8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut46(&self) -> &BGCLUT46 {
        &self.bgclut46
    }
    ///0x8bc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut47(&self) -> &BGCLUT47 {
        &self.bgclut47
    }
    ///0x8c0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut48(&self) -> &BGCLUT48 {
        &self.bgclut48
    }
    ///0x8c4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut49(&self) -> &BGCLUT49 {
        &self.bgclut49
    }
    ///0x8c8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut50(&self) -> &BGCLUT50 {
        &self.bgclut50
    }
    ///0x8cc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut51(&self) -> &BGCLUT51 {
        &self.bgclut51
    }
    ///0x8d0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut52(&self) -> &BGCLUT52 {
        &self.bgclut52
    }
    ///0x8d4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut53(&self) -> &BGCLUT53 {
        &self.bgclut53
    }
    ///0x8d8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut54(&self) -> &BGCLUT54 {
        &self.bgclut54
    }
    ///0x8dc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut55(&self) -> &BGCLUT55 {
        &self.bgclut55
    }
    ///0x8e0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut56(&self) -> &BGCLUT56 {
        &self.bgclut56
    }
    ///0x8e4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut57(&self) -> &BGCLUT57 {
        &self.bgclut57
    }
    ///0x8e8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut58(&self) -> &BGCLUT58 {
        &self.bgclut58
    }
    ///0x8ec - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut59(&self) -> &BGCLUT59 {
        &self.bgclut59
    }
    ///0x8f0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut60(&self) -> &BGCLUT60 {
        &self.bgclut60
    }
    ///0x8f4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut61(&self) -> &BGCLUT61 {
        &self.bgclut61
    }
    ///0x8f8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut62(&self) -> &BGCLUT62 {
        &self.bgclut62
    }
    ///0x8fc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut63(&self) -> &BGCLUT63 {
        &self.bgclut63
    }
    ///0x900 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut64(&self) -> &BGCLUT64 {
        &self.bgclut64
    }
    ///0x904 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut65(&self) -> &BGCLUT65 {
        &self.bgclut65
    }
    ///0x908 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut66(&self) -> &BGCLUT66 {
        &self.bgclut66
    }
    ///0x90c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut67(&self) -> &BGCLUT67 {
        &self.bgclut67
    }
    ///0x910 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut68(&self) -> &BGCLUT68 {
        &self.bgclut68
    }
    ///0x914 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut69(&self) -> &BGCLUT69 {
        &self.bgclut69
    }
    ///0x918 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut70(&self) -> &BGCLUT70 {
        &self.bgclut70
    }
    ///0x91c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut71(&self) -> &BGCLUT71 {
        &self.bgclut71
    }
    ///0x920 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut72(&self) -> &BGCLUT72 {
        &self.bgclut72
    }
    ///0x924 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut73(&self) -> &BGCLUT73 {
        &self.bgclut73
    }
    ///0x928 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut74(&self) -> &BGCLUT74 {
        &self.bgclut74
    }
    ///0x92c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut75(&self) -> &BGCLUT75 {
        &self.bgclut75
    }
    ///0x930 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut76(&self) -> &BGCLUT76 {
        &self.bgclut76
    }
    ///0x934 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut77(&self) -> &BGCLUT77 {
        &self.bgclut77
    }
    ///0x938 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut78(&self) -> &BGCLUT78 {
        &self.bgclut78
    }
    ///0x93c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut79(&self) -> &BGCLUT79 {
        &self.bgclut79
    }
    ///0x940 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut80(&self) -> &BGCLUT80 {
        &self.bgclut80
    }
    ///0x944 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut81(&self) -> &BGCLUT81 {
        &self.bgclut81
    }
    ///0x948 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut82(&self) -> &BGCLUT82 {
        &self.bgclut82
    }
    ///0x94c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut83(&self) -> &BGCLUT83 {
        &self.bgclut83
    }
    ///0x950 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut84(&self) -> &BGCLUT84 {
        &self.bgclut84
    }
    ///0x954 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut85(&self) -> &BGCLUT85 {
        &self.bgclut85
    }
    ///0x958 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut86(&self) -> &BGCLUT86 {
        &self.bgclut86
    }
    ///0x95c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut87(&self) -> &BGCLUT87 {
        &self.bgclut87
    }
    ///0x960 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut88(&self) -> &BGCLUT88 {
        &self.bgclut88
    }
    ///0x964 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut89(&self) -> &BGCLUT89 {
        &self.bgclut89
    }
    ///0x968 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut90(&self) -> &BGCLUT90 {
        &self.bgclut90
    }
    ///0x96c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut91(&self) -> &BGCLUT91 {
        &self.bgclut91
    }
    ///0x970 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut92(&self) -> &BGCLUT92 {
        &self.bgclut92
    }
    ///0x974 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut93(&self) -> &BGCLUT93 {
        &self.bgclut93
    }
    ///0x978 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut94(&self) -> &BGCLUT94 {
        &self.bgclut94
    }
    ///0x97c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut95(&self) -> &BGCLUT95 {
        &self.bgclut95
    }
    ///0x980 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut96(&self) -> &BGCLUT96 {
        &self.bgclut96
    }
    ///0x984 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut97(&self) -> &BGCLUT97 {
        &self.bgclut97
    }
    ///0x988 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut98(&self) -> &BGCLUT98 {
        &self.bgclut98
    }
    ///0x98c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut99(&self) -> &BGCLUT99 {
        &self.bgclut99
    }
    ///0x990 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut100(&self) -> &BGCLUT100 {
        &self.bgclut100
    }
    ///0x994 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut101(&self) -> &BGCLUT101 {
        &self.bgclut101
    }
    ///0x998 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut102(&self) -> &BGCLUT102 {
        &self.bgclut102
    }
    ///0x99c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut103(&self) -> &BGCLUT103 {
        &self.bgclut103
    }
    ///0x9a0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut104(&self) -> &BGCLUT104 {
        &self.bgclut104
    }
    ///0x9a4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut105(&self) -> &BGCLUT105 {
        &self.bgclut105
    }
    ///0x9a8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut106(&self) -> &BGCLUT106 {
        &self.bgclut106
    }
    ///0x9ac - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut107(&self) -> &BGCLUT107 {
        &self.bgclut107
    }
    ///0x9b0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut108(&self) -> &BGCLUT108 {
        &self.bgclut108
    }
    ///0x9b4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut109(&self) -> &BGCLUT109 {
        &self.bgclut109
    }
    ///0x9b8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut110(&self) -> &BGCLUT110 {
        &self.bgclut110
    }
    ///0x9bc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut111(&self) -> &BGCLUT111 {
        &self.bgclut111
    }
    ///0x9c0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut112(&self) -> &BGCLUT112 {
        &self.bgclut112
    }
    ///0x9c4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut113(&self) -> &BGCLUT113 {
        &self.bgclut113
    }
    ///0x9c8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut114(&self) -> &BGCLUT114 {
        &self.bgclut114
    }
    ///0x9cc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut115(&self) -> &BGCLUT115 {
        &self.bgclut115
    }
    ///0x9d0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut116(&self) -> &BGCLUT116 {
        &self.bgclut116
    }
    ///0x9d4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut117(&self) -> &BGCLUT117 {
        &self.bgclut117
    }
    ///0x9d8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut118(&self) -> &BGCLUT118 {
        &self.bgclut118
    }
    ///0x9dc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut119(&self) -> &BGCLUT119 {
        &self.bgclut119
    }
    ///0x9e0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut120(&self) -> &BGCLUT120 {
        &self.bgclut120
    }
    ///0x9e4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut121(&self) -> &BGCLUT121 {
        &self.bgclut121
    }
    ///0x9e8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut122(&self) -> &BGCLUT122 {
        &self.bgclut122
    }
    ///0x9ec - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut123(&self) -> &BGCLUT123 {
        &self.bgclut123
    }
    ///0x9f0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut124(&self) -> &BGCLUT124 {
        &self.bgclut124
    }
    ///0x9f4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut125(&self) -> &BGCLUT125 {
        &self.bgclut125
    }
    ///0x9f8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut126(&self) -> &BGCLUT126 {
        &self.bgclut126
    }
    ///0x9fc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut127(&self) -> &BGCLUT127 {
        &self.bgclut127
    }
    ///0xa00 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut128(&self) -> &BGCLUT128 {
        &self.bgclut128
    }
    ///0xa04 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut129(&self) -> &BGCLUT129 {
        &self.bgclut129
    }
    ///0xa08 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut130(&self) -> &BGCLUT130 {
        &self.bgclut130
    }
    ///0xa0c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut131(&self) -> &BGCLUT131 {
        &self.bgclut131
    }
    ///0xa10 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut132(&self) -> &BGCLUT132 {
        &self.bgclut132
    }
    ///0xa14 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut133(&self) -> &BGCLUT133 {
        &self.bgclut133
    }
    ///0xa18 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut134(&self) -> &BGCLUT134 {
        &self.bgclut134
    }
    ///0xa1c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut135(&self) -> &BGCLUT135 {
        &self.bgclut135
    }
    ///0xa20 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut136(&self) -> &BGCLUT136 {
        &self.bgclut136
    }
    ///0xa24 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut137(&self) -> &BGCLUT137 {
        &self.bgclut137
    }
    ///0xa28 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut138(&self) -> &BGCLUT138 {
        &self.bgclut138
    }
    ///0xa2c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut139(&self) -> &BGCLUT139 {
        &self.bgclut139
    }
    ///0xa30 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut140(&self) -> &BGCLUT140 {
        &self.bgclut140
    }
    ///0xa34 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut141(&self) -> &BGCLUT141 {
        &self.bgclut141
    }
    ///0xa38 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut142(&self) -> &BGCLUT142 {
        &self.bgclut142
    }
    ///0xa3c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut143(&self) -> &BGCLUT143 {
        &self.bgclut143
    }
    ///0xa40 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut144(&self) -> &BGCLUT144 {
        &self.bgclut144
    }
    ///0xa44 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut145(&self) -> &BGCLUT145 {
        &self.bgclut145
    }
    ///0xa48 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut146(&self) -> &BGCLUT146 {
        &self.bgclut146
    }
    ///0xa4c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut147(&self) -> &BGCLUT147 {
        &self.bgclut147
    }
    ///0xa50 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut148(&self) -> &BGCLUT148 {
        &self.bgclut148
    }
    ///0xa54 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut149(&self) -> &BGCLUT149 {
        &self.bgclut149
    }
    ///0xa58 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut150(&self) -> &BGCLUT150 {
        &self.bgclut150
    }
    ///0xa5c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut151(&self) -> &BGCLUT151 {
        &self.bgclut151
    }
    ///0xa60 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut152(&self) -> &BGCLUT152 {
        &self.bgclut152
    }
    ///0xa64 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut153(&self) -> &BGCLUT153 {
        &self.bgclut153
    }
    ///0xa68 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut154(&self) -> &BGCLUT154 {
        &self.bgclut154
    }
    ///0xa6c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut155(&self) -> &BGCLUT155 {
        &self.bgclut155
    }
    ///0xa70 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut156(&self) -> &BGCLUT156 {
        &self.bgclut156
    }
    ///0xa74 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut157(&self) -> &BGCLUT157 {
        &self.bgclut157
    }
    ///0xa78 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut158(&self) -> &BGCLUT158 {
        &self.bgclut158
    }
    ///0xa7c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut159(&self) -> &BGCLUT159 {
        &self.bgclut159
    }
    ///0xa80 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut160(&self) -> &BGCLUT160 {
        &self.bgclut160
    }
    ///0xa84 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut161(&self) -> &BGCLUT161 {
        &self.bgclut161
    }
    ///0xa88 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut162(&self) -> &BGCLUT162 {
        &self.bgclut162
    }
    ///0xa8c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut163(&self) -> &BGCLUT163 {
        &self.bgclut163
    }
    ///0xa90 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut164(&self) -> &BGCLUT164 {
        &self.bgclut164
    }
    ///0xa94 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut165(&self) -> &BGCLUT165 {
        &self.bgclut165
    }
    ///0xa98 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut166(&self) -> &BGCLUT166 {
        &self.bgclut166
    }
    ///0xa9c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut167(&self) -> &BGCLUT167 {
        &self.bgclut167
    }
    ///0xaa0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut168(&self) -> &BGCLUT168 {
        &self.bgclut168
    }
    ///0xaa4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut169(&self) -> &BGCLUT169 {
        &self.bgclut169
    }
    ///0xaa8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut170(&self) -> &BGCLUT170 {
        &self.bgclut170
    }
    ///0xaac - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut171(&self) -> &BGCLUT171 {
        &self.bgclut171
    }
    ///0xab0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut172(&self) -> &BGCLUT172 {
        &self.bgclut172
    }
    ///0xab4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut173(&self) -> &BGCLUT173 {
        &self.bgclut173
    }
    ///0xab8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut174(&self) -> &BGCLUT174 {
        &self.bgclut174
    }
    ///0xabc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut175(&self) -> &BGCLUT175 {
        &self.bgclut175
    }
    ///0xac0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut176(&self) -> &BGCLUT176 {
        &self.bgclut176
    }
    ///0xac4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut177(&self) -> &BGCLUT177 {
        &self.bgclut177
    }
    ///0xac8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut178(&self) -> &BGCLUT178 {
        &self.bgclut178
    }
    ///0xacc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut179(&self) -> &BGCLUT179 {
        &self.bgclut179
    }
    ///0xad0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut180(&self) -> &BGCLUT180 {
        &self.bgclut180
    }
    ///0xad4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut181(&self) -> &BGCLUT181 {
        &self.bgclut181
    }
    ///0xad8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut182(&self) -> &BGCLUT182 {
        &self.bgclut182
    }
    ///0xadc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut183(&self) -> &BGCLUT183 {
        &self.bgclut183
    }
    ///0xae0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut184(&self) -> &BGCLUT184 {
        &self.bgclut184
    }
    ///0xae4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut185(&self) -> &BGCLUT185 {
        &self.bgclut185
    }
    ///0xae8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut186(&self) -> &BGCLUT186 {
        &self.bgclut186
    }
    ///0xaec - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut187(&self) -> &BGCLUT187 {
        &self.bgclut187
    }
    ///0xaf0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut188(&self) -> &BGCLUT188 {
        &self.bgclut188
    }
    ///0xaf4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut189(&self) -> &BGCLUT189 {
        &self.bgclut189
    }
    ///0xaf8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut190(&self) -> &BGCLUT190 {
        &self.bgclut190
    }
    ///0xafc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut191(&self) -> &BGCLUT191 {
        &self.bgclut191
    }
    ///0xb00 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut192(&self) -> &BGCLUT192 {
        &self.bgclut192
    }
    ///0xb04 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut193(&self) -> &BGCLUT193 {
        &self.bgclut193
    }
    ///0xb08 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut194(&self) -> &BGCLUT194 {
        &self.bgclut194
    }
    ///0xb0c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut195(&self) -> &BGCLUT195 {
        &self.bgclut195
    }
    ///0xb10 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut196(&self) -> &BGCLUT196 {
        &self.bgclut196
    }
    ///0xb14 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut197(&self) -> &BGCLUT197 {
        &self.bgclut197
    }
    ///0xb18 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut198(&self) -> &BGCLUT198 {
        &self.bgclut198
    }
    ///0xb1c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut199(&self) -> &BGCLUT199 {
        &self.bgclut199
    }
    ///0xb20 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut200(&self) -> &BGCLUT200 {
        &self.bgclut200
    }
    ///0xb24 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut201(&self) -> &BGCLUT201 {
        &self.bgclut201
    }
    ///0xb28 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut202(&self) -> &BGCLUT202 {
        &self.bgclut202
    }
    ///0xb2c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut203(&self) -> &BGCLUT203 {
        &self.bgclut203
    }
    ///0xb30 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut204(&self) -> &BGCLUT204 {
        &self.bgclut204
    }
    ///0xb34 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut205(&self) -> &BGCLUT205 {
        &self.bgclut205
    }
    ///0xb38 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut206(&self) -> &BGCLUT206 {
        &self.bgclut206
    }
    ///0xb3c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut207(&self) -> &BGCLUT207 {
        &self.bgclut207
    }
    ///0xb40 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut208(&self) -> &BGCLUT208 {
        &self.bgclut208
    }
    ///0xb44 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut209(&self) -> &BGCLUT209 {
        &self.bgclut209
    }
    ///0xb48 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut210(&self) -> &BGCLUT210 {
        &self.bgclut210
    }
    ///0xb4c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut211(&self) -> &BGCLUT211 {
        &self.bgclut211
    }
    ///0xb50 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut212(&self) -> &BGCLUT212 {
        &self.bgclut212
    }
    ///0xb54 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut213(&self) -> &BGCLUT213 {
        &self.bgclut213
    }
    ///0xb58 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut214(&self) -> &BGCLUT214 {
        &self.bgclut214
    }
    ///0xb5c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut215(&self) -> &BGCLUT215 {
        &self.bgclut215
    }
    ///0xb60 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut216(&self) -> &BGCLUT216 {
        &self.bgclut216
    }
    ///0xb64 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut217(&self) -> &BGCLUT217 {
        &self.bgclut217
    }
    ///0xb68 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut218(&self) -> &BGCLUT218 {
        &self.bgclut218
    }
    ///0xb6c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut219(&self) -> &BGCLUT219 {
        &self.bgclut219
    }
    ///0xb70 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut220(&self) -> &BGCLUT220 {
        &self.bgclut220
    }
    ///0xb74 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut221(&self) -> &BGCLUT221 {
        &self.bgclut221
    }
    ///0xb78 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut222(&self) -> &BGCLUT222 {
        &self.bgclut222
    }
    ///0xb7c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut223(&self) -> &BGCLUT223 {
        &self.bgclut223
    }
    ///0xb80 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut224(&self) -> &BGCLUT224 {
        &self.bgclut224
    }
    ///0xb84 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut225(&self) -> &BGCLUT225 {
        &self.bgclut225
    }
    ///0xb88 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut226(&self) -> &BGCLUT226 {
        &self.bgclut226
    }
    ///0xb8c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut227(&self) -> &BGCLUT227 {
        &self.bgclut227
    }
    ///0xb90 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut228(&self) -> &BGCLUT228 {
        &self.bgclut228
    }
    ///0xb94 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut229(&self) -> &BGCLUT229 {
        &self.bgclut229
    }
    ///0xb98 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut230(&self) -> &BGCLUT230 {
        &self.bgclut230
    }
    ///0xb9c - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut231(&self) -> &BGCLUT231 {
        &self.bgclut231
    }
    ///0xba0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut232(&self) -> &BGCLUT232 {
        &self.bgclut232
    }
    ///0xba4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut233(&self) -> &BGCLUT233 {
        &self.bgclut233
    }
    ///0xba8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut234(&self) -> &BGCLUT234 {
        &self.bgclut234
    }
    ///0xbac - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut235(&self) -> &BGCLUT235 {
        &self.bgclut235
    }
    ///0xbb0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut236(&self) -> &BGCLUT236 {
        &self.bgclut236
    }
    ///0xbb4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut237(&self) -> &BGCLUT237 {
        &self.bgclut237
    }
    ///0xbb8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut238(&self) -> &BGCLUT238 {
        &self.bgclut238
    }
    ///0xbbc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut239(&self) -> &BGCLUT239 {
        &self.bgclut239
    }
    ///0xbc0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut240(&self) -> &BGCLUT240 {
        &self.bgclut240
    }
    ///0xbc4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut241(&self) -> &BGCLUT241 {
        &self.bgclut241
    }
    ///0xbc8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut242(&self) -> &BGCLUT242 {
        &self.bgclut242
    }
    ///0xbcc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut243(&self) -> &BGCLUT243 {
        &self.bgclut243
    }
    ///0xbd0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut244(&self) -> &BGCLUT244 {
        &self.bgclut244
    }
    ///0xbd4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut245(&self) -> &BGCLUT245 {
        &self.bgclut245
    }
    ///0xbd8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut246(&self) -> &BGCLUT246 {
        &self.bgclut246
    }
    ///0xbdc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut247(&self) -> &BGCLUT247 {
        &self.bgclut247
    }
    ///0xbe0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut248(&self) -> &BGCLUT248 {
        &self.bgclut248
    }
    ///0xbe4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut249(&self) -> &BGCLUT249 {
        &self.bgclut249
    }
    ///0xbe8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut250(&self) -> &BGCLUT250 {
        &self.bgclut250
    }
    ///0xbec - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut251(&self) -> &BGCLUT251 {
        &self.bgclut251
    }
    ///0xbf0 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut252(&self) -> &BGCLUT252 {
        &self.bgclut252
    }
    ///0xbf4 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut253(&self) -> &BGCLUT253 {
        &self.bgclut253
    }
    ///0xbf8 - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut254(&self) -> &BGCLUT254 {
        &self.bgclut254
    }
    ///0xbfc - DMA2D background CLUT
    #[inline(always)]
    pub const fn bgclut255(&self) -> &BGCLUT255 {
        &self.bgclut255
    }
}
/**CR (rw) register accessor: DMA2D control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:CR)

For information about available fields see [`mod@cr`] module*/
pub type CR = crate::Reg<cr::CRrs>;
///DMA2D control register
pub mod cr;
/**ISR (r) register accessor: DMA2D interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:ISR)

For information about available fields see [`mod@isr`] module*/
pub type ISR = crate::Reg<isr::ISRrs>;
///DMA2D interrupt status register
pub mod isr;
/**IFCR (rw) register accessor: DMA2D interrupt flag clear register

You can [`read`](crate::Reg::read) this register and get [`ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:IFCR)

For information about available fields see [`mod@ifcr`] module*/
pub type IFCR = crate::Reg<ifcr::IFCRrs>;
///DMA2D interrupt flag clear register
pub mod ifcr;
/**FGMAR (rw) register accessor: DMA2D foreground memory address register

You can [`read`](crate::Reg::read) this register and get [`fgmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGMAR)

For information about available fields see [`mod@fgmar`] module*/
pub type FGMAR = crate::Reg<fgmar::FGMARrs>;
///DMA2D foreground memory address register
pub mod fgmar;
/**FGOR (rw) register accessor: DMA2D foreground offset register

You can [`read`](crate::Reg::read) this register and get [`fgor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGOR)

For information about available fields see [`mod@fgor`] module*/
pub type FGOR = crate::Reg<fgor::FGORrs>;
///DMA2D foreground offset register
pub mod fgor;
/**BGMAR (rw) register accessor: DMA2D background memory address register

You can [`read`](crate::Reg::read) this register and get [`bgmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGMAR)

For information about available fields see [`mod@bgmar`] module*/
pub type BGMAR = crate::Reg<bgmar::BGMARrs>;
///DMA2D background memory address register
pub mod bgmar;
/**BGOR (rw) register accessor: DMA2D background offset register

You can [`read`](crate::Reg::read) this register and get [`bgor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGOR)

For information about available fields see [`mod@bgor`] module*/
pub type BGOR = crate::Reg<bgor::BGORrs>;
///DMA2D background offset register
pub mod bgor;
/**FGPFCCR (rw) register accessor: DMA2D foreground PFC control register

You can [`read`](crate::Reg::read) this register and get [`fgpfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgpfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGPFCCR)

For information about available fields see [`mod@fgpfccr`] module*/
pub type FGPFCCR = crate::Reg<fgpfccr::FGPFCCRrs>;
///DMA2D foreground PFC control register
pub mod fgpfccr;
/**FGCOLR (rw) register accessor: DMA2D foreground color register

You can [`read`](crate::Reg::read) this register and get [`fgcolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCOLR)

For information about available fields see [`mod@fgcolr`] module*/
pub type FGCOLR = crate::Reg<fgcolr::FGCOLRrs>;
///DMA2D foreground color register
pub mod fgcolr;
/**BGPFCCR (rw) register accessor: DMA2D background PFC control register

You can [`read`](crate::Reg::read) this register and get [`bgpfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgpfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGPFCCR)

For information about available fields see [`mod@bgpfccr`] module*/
pub type BGPFCCR = crate::Reg<bgpfccr::BGPFCCRrs>;
///DMA2D background PFC control register
pub mod bgpfccr;
/**BGCOLR (rw) register accessor: DMA2D background color register

You can [`read`](crate::Reg::read) this register and get [`bgcolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCOLR)

For information about available fields see [`mod@bgcolr`] module*/
pub type BGCOLR = crate::Reg<bgcolr::BGCOLRrs>;
///DMA2D background color register
pub mod bgcolr;
/**FGCMAR (rw) register accessor: DMA2D foreground CLUT memory address register

You can [`read`](crate::Reg::read) this register and get [`fgcmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCMAR)

For information about available fields see [`mod@fgcmar`] module*/
pub type FGCMAR = crate::Reg<fgcmar::FGCMARrs>;
///DMA2D foreground CLUT memory address register
pub mod fgcmar;
/**BGCMAR (rw) register accessor: DMA2D background CLUT memory address register

You can [`read`](crate::Reg::read) this register and get [`bgcmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCMAR)

For information about available fields see [`mod@bgcmar`] module*/
pub type BGCMAR = crate::Reg<bgcmar::BGCMARrs>;
///DMA2D background CLUT memory address register
pub mod bgcmar;
/**OPFCCR (rw) register accessor: DMA2D output PFC control register

You can [`read`](crate::Reg::read) this register and get [`opfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:OPFCCR)

For information about available fields see [`mod@opfccr`] module*/
pub type OPFCCR = crate::Reg<opfccr::OPFCCRrs>;
///DMA2D output PFC control register
pub mod opfccr;
/**OCOLR_RGB888 (rw) register accessor: DMA2D output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_rgb888::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_rgb888::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:OCOLR_RGB888)

For information about available fields see [`mod@ocolr_rgb888`] module*/
pub type OCOLR_RGB888 = crate::Reg<ocolr_rgb888::OCOLR_RGB888rs>;
///DMA2D output color register
pub mod ocolr_rgb888;
/**OCOLR_RGB565 (rw) register accessor: DMA2D output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_rgb565::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_rgb565::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:OCOLR_RGB565)

For information about available fields see [`mod@ocolr_rgb565`] module*/
pub type OCOLR_RGB565 = crate::Reg<ocolr_rgb565::OCOLR_RGB565rs>;
///DMA2D output color register
pub mod ocolr_rgb565;
/**OCOLR_ARGB1555 (rw) register accessor: DMA2D output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_argb1555::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_argb1555::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:OCOLR_ARGB1555)

For information about available fields see [`mod@ocolr_argb1555`] module*/
pub type OCOLR_ARGB1555 = crate::Reg<ocolr_argb1555::OCOLR_ARGB1555rs>;
///DMA2D output color register
pub mod ocolr_argb1555;
/**OCOLR_ARGB4444 (rw) register accessor: DMA2D output color register

You can [`read`](crate::Reg::read) this register and get [`ocolr_argb4444::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr_argb4444::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:OCOLR_ARGB4444)

For information about available fields see [`mod@ocolr_argb4444`] module*/
pub type OCOLR_ARGB4444 = crate::Reg<ocolr_argb4444::OCOLR_ARGB4444rs>;
///DMA2D output color register
pub mod ocolr_argb4444;
/**OMAR (rw) register accessor: DMA2D output memory address register

You can [`read`](crate::Reg::read) this register and get [`omar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:OMAR)

For information about available fields see [`mod@omar`] module*/
pub type OMAR = crate::Reg<omar::OMARrs>;
///DMA2D output memory address register
pub mod omar;
/**OOR (rw) register accessor: DMA2D output offset register

You can [`read`](crate::Reg::read) this register and get [`oor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:OOR)

For information about available fields see [`mod@oor`] module*/
pub type OOR = crate::Reg<oor::OORrs>;
///DMA2D output offset register
pub mod oor;
/**NLR (rw) register accessor: DMA2D number of line register

You can [`read`](crate::Reg::read) this register and get [`nlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:NLR)

For information about available fields see [`mod@nlr`] module*/
pub type NLR = crate::Reg<nlr::NLRrs>;
///DMA2D number of line register
pub mod nlr;
/**LWR (rw) register accessor: DMA2D line watermark register

You can [`read`](crate::Reg::read) this register and get [`lwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:LWR)

For information about available fields see [`mod@lwr`] module*/
pub type LWR = crate::Reg<lwr::LWRrs>;
///DMA2D line watermark register
pub mod lwr;
/**AMTCR (rw) register accessor: DMA2D AXI master timer configuration register

You can [`read`](crate::Reg::read) this register and get [`amtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:AMTCR)

For information about available fields see [`mod@amtcr`] module*/
pub type AMTCR = crate::Reg<amtcr::AMTCRrs>;
///DMA2D AXI master timer configuration register
pub mod amtcr;
/**FGCLUT0 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT0)

For information about available fields see [`mod@fgclut0`] module*/
pub type FGCLUT0 = crate::Reg<fgclut0::FGCLUT0rs>;
///DMA2D foreground CLUT
pub mod fgclut0;
/**FGCLUT1 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT1)

For information about available fields see [`mod@fgclut1`] module*/
pub type FGCLUT1 = crate::Reg<fgclut1::FGCLUT1rs>;
///DMA2D foreground CLUT
pub mod fgclut1;
/**FGCLUT2 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT2)

For information about available fields see [`mod@fgclut2`] module*/
pub type FGCLUT2 = crate::Reg<fgclut2::FGCLUT2rs>;
///DMA2D foreground CLUT
pub mod fgclut2;
/**FGCLUT3 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT3)

For information about available fields see [`mod@fgclut3`] module*/
pub type FGCLUT3 = crate::Reg<fgclut3::FGCLUT3rs>;
///DMA2D foreground CLUT
pub mod fgclut3;
/**FGCLUT4 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT4)

For information about available fields see [`mod@fgclut4`] module*/
pub type FGCLUT4 = crate::Reg<fgclut4::FGCLUT4rs>;
///DMA2D foreground CLUT
pub mod fgclut4;
/**FGCLUT5 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT5)

For information about available fields see [`mod@fgclut5`] module*/
pub type FGCLUT5 = crate::Reg<fgclut5::FGCLUT5rs>;
///DMA2D foreground CLUT
pub mod fgclut5;
/**FGCLUT6 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT6)

For information about available fields see [`mod@fgclut6`] module*/
pub type FGCLUT6 = crate::Reg<fgclut6::FGCLUT6rs>;
///DMA2D foreground CLUT
pub mod fgclut6;
/**FGCLUT7 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT7)

For information about available fields see [`mod@fgclut7`] module*/
pub type FGCLUT7 = crate::Reg<fgclut7::FGCLUT7rs>;
///DMA2D foreground CLUT
pub mod fgclut7;
/**FGCLUT8 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT8)

For information about available fields see [`mod@fgclut8`] module*/
pub type FGCLUT8 = crate::Reg<fgclut8::FGCLUT8rs>;
///DMA2D foreground CLUT
pub mod fgclut8;
/**FGCLUT9 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT9)

For information about available fields see [`mod@fgclut9`] module*/
pub type FGCLUT9 = crate::Reg<fgclut9::FGCLUT9rs>;
///DMA2D foreground CLUT
pub mod fgclut9;
/**FGCLUT10 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT10)

For information about available fields see [`mod@fgclut10`] module*/
pub type FGCLUT10 = crate::Reg<fgclut10::FGCLUT10rs>;
///DMA2D foreground CLUT
pub mod fgclut10;
/**FGCLUT11 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT11)

For information about available fields see [`mod@fgclut11`] module*/
pub type FGCLUT11 = crate::Reg<fgclut11::FGCLUT11rs>;
///DMA2D foreground CLUT
pub mod fgclut11;
/**FGCLUT12 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT12)

For information about available fields see [`mod@fgclut12`] module*/
pub type FGCLUT12 = crate::Reg<fgclut12::FGCLUT12rs>;
///DMA2D foreground CLUT
pub mod fgclut12;
/**FGCLUT13 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT13)

For information about available fields see [`mod@fgclut13`] module*/
pub type FGCLUT13 = crate::Reg<fgclut13::FGCLUT13rs>;
///DMA2D foreground CLUT
pub mod fgclut13;
/**FGCLUT14 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT14)

For information about available fields see [`mod@fgclut14`] module*/
pub type FGCLUT14 = crate::Reg<fgclut14::FGCLUT14rs>;
///DMA2D foreground CLUT
pub mod fgclut14;
/**FGCLUT15 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT15)

For information about available fields see [`mod@fgclut15`] module*/
pub type FGCLUT15 = crate::Reg<fgclut15::FGCLUT15rs>;
///DMA2D foreground CLUT
pub mod fgclut15;
/**FGCLUT16 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT16)

For information about available fields see [`mod@fgclut16`] module*/
pub type FGCLUT16 = crate::Reg<fgclut16::FGCLUT16rs>;
///DMA2D foreground CLUT
pub mod fgclut16;
/**FGCLUT17 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT17)

For information about available fields see [`mod@fgclut17`] module*/
pub type FGCLUT17 = crate::Reg<fgclut17::FGCLUT17rs>;
///DMA2D foreground CLUT
pub mod fgclut17;
/**FGCLUT18 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT18)

For information about available fields see [`mod@fgclut18`] module*/
pub type FGCLUT18 = crate::Reg<fgclut18::FGCLUT18rs>;
///DMA2D foreground CLUT
pub mod fgclut18;
/**FGCLUT19 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT19)

For information about available fields see [`mod@fgclut19`] module*/
pub type FGCLUT19 = crate::Reg<fgclut19::FGCLUT19rs>;
///DMA2D foreground CLUT
pub mod fgclut19;
/**FGCLUT20 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT20)

For information about available fields see [`mod@fgclut20`] module*/
pub type FGCLUT20 = crate::Reg<fgclut20::FGCLUT20rs>;
///DMA2D foreground CLUT
pub mod fgclut20;
/**FGCLUT21 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT21)

For information about available fields see [`mod@fgclut21`] module*/
pub type FGCLUT21 = crate::Reg<fgclut21::FGCLUT21rs>;
///DMA2D foreground CLUT
pub mod fgclut21;
/**FGCLUT22 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT22)

For information about available fields see [`mod@fgclut22`] module*/
pub type FGCLUT22 = crate::Reg<fgclut22::FGCLUT22rs>;
///DMA2D foreground CLUT
pub mod fgclut22;
/**FGCLUT23 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT23)

For information about available fields see [`mod@fgclut23`] module*/
pub type FGCLUT23 = crate::Reg<fgclut23::FGCLUT23rs>;
///DMA2D foreground CLUT
pub mod fgclut23;
/**FGCLUT24 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT24)

For information about available fields see [`mod@fgclut24`] module*/
pub type FGCLUT24 = crate::Reg<fgclut24::FGCLUT24rs>;
///DMA2D foreground CLUT
pub mod fgclut24;
/**FGCLUT25 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT25)

For information about available fields see [`mod@fgclut25`] module*/
pub type FGCLUT25 = crate::Reg<fgclut25::FGCLUT25rs>;
///DMA2D foreground CLUT
pub mod fgclut25;
/**FGCLUT26 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT26)

For information about available fields see [`mod@fgclut26`] module*/
pub type FGCLUT26 = crate::Reg<fgclut26::FGCLUT26rs>;
///DMA2D foreground CLUT
pub mod fgclut26;
/**FGCLUT27 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT27)

For information about available fields see [`mod@fgclut27`] module*/
pub type FGCLUT27 = crate::Reg<fgclut27::FGCLUT27rs>;
///DMA2D foreground CLUT
pub mod fgclut27;
/**FGCLUT28 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT28)

For information about available fields see [`mod@fgclut28`] module*/
pub type FGCLUT28 = crate::Reg<fgclut28::FGCLUT28rs>;
///DMA2D foreground CLUT
pub mod fgclut28;
/**FGCLUT29 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT29)

For information about available fields see [`mod@fgclut29`] module*/
pub type FGCLUT29 = crate::Reg<fgclut29::FGCLUT29rs>;
///DMA2D foreground CLUT
pub mod fgclut29;
/**FGCLUT30 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT30)

For information about available fields see [`mod@fgclut30`] module*/
pub type FGCLUT30 = crate::Reg<fgclut30::FGCLUT30rs>;
///DMA2D foreground CLUT
pub mod fgclut30;
/**FGCLUT31 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT31)

For information about available fields see [`mod@fgclut31`] module*/
pub type FGCLUT31 = crate::Reg<fgclut31::FGCLUT31rs>;
///DMA2D foreground CLUT
pub mod fgclut31;
/**FGCLUT32 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT32)

For information about available fields see [`mod@fgclut32`] module*/
pub type FGCLUT32 = crate::Reg<fgclut32::FGCLUT32rs>;
///DMA2D foreground CLUT
pub mod fgclut32;
/**FGCLUT33 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT33)

For information about available fields see [`mod@fgclut33`] module*/
pub type FGCLUT33 = crate::Reg<fgclut33::FGCLUT33rs>;
///DMA2D foreground CLUT
pub mod fgclut33;
/**FGCLUT34 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT34)

For information about available fields see [`mod@fgclut34`] module*/
pub type FGCLUT34 = crate::Reg<fgclut34::FGCLUT34rs>;
///DMA2D foreground CLUT
pub mod fgclut34;
/**FGCLUT35 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT35)

For information about available fields see [`mod@fgclut35`] module*/
pub type FGCLUT35 = crate::Reg<fgclut35::FGCLUT35rs>;
///DMA2D foreground CLUT
pub mod fgclut35;
/**FGCLUT36 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT36)

For information about available fields see [`mod@fgclut36`] module*/
pub type FGCLUT36 = crate::Reg<fgclut36::FGCLUT36rs>;
///DMA2D foreground CLUT
pub mod fgclut36;
/**FGCLUT37 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT37)

For information about available fields see [`mod@fgclut37`] module*/
pub type FGCLUT37 = crate::Reg<fgclut37::FGCLUT37rs>;
///DMA2D foreground CLUT
pub mod fgclut37;
/**FGCLUT38 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT38)

For information about available fields see [`mod@fgclut38`] module*/
pub type FGCLUT38 = crate::Reg<fgclut38::FGCLUT38rs>;
///DMA2D foreground CLUT
pub mod fgclut38;
/**FGCLUT39 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT39)

For information about available fields see [`mod@fgclut39`] module*/
pub type FGCLUT39 = crate::Reg<fgclut39::FGCLUT39rs>;
///DMA2D foreground CLUT
pub mod fgclut39;
/**FGCLUT40 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT40)

For information about available fields see [`mod@fgclut40`] module*/
pub type FGCLUT40 = crate::Reg<fgclut40::FGCLUT40rs>;
///DMA2D foreground CLUT
pub mod fgclut40;
/**FGCLUT41 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT41)

For information about available fields see [`mod@fgclut41`] module*/
pub type FGCLUT41 = crate::Reg<fgclut41::FGCLUT41rs>;
///DMA2D foreground CLUT
pub mod fgclut41;
/**FGCLUT42 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT42)

For information about available fields see [`mod@fgclut42`] module*/
pub type FGCLUT42 = crate::Reg<fgclut42::FGCLUT42rs>;
///DMA2D foreground CLUT
pub mod fgclut42;
/**FGCLUT43 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT43)

For information about available fields see [`mod@fgclut43`] module*/
pub type FGCLUT43 = crate::Reg<fgclut43::FGCLUT43rs>;
///DMA2D foreground CLUT
pub mod fgclut43;
/**FGCLUT44 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT44)

For information about available fields see [`mod@fgclut44`] module*/
pub type FGCLUT44 = crate::Reg<fgclut44::FGCLUT44rs>;
///DMA2D foreground CLUT
pub mod fgclut44;
/**FGCLUT45 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT45)

For information about available fields see [`mod@fgclut45`] module*/
pub type FGCLUT45 = crate::Reg<fgclut45::FGCLUT45rs>;
///DMA2D foreground CLUT
pub mod fgclut45;
/**FGCLUT46 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT46)

For information about available fields see [`mod@fgclut46`] module*/
pub type FGCLUT46 = crate::Reg<fgclut46::FGCLUT46rs>;
///DMA2D foreground CLUT
pub mod fgclut46;
/**FGCLUT47 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT47)

For information about available fields see [`mod@fgclut47`] module*/
pub type FGCLUT47 = crate::Reg<fgclut47::FGCLUT47rs>;
///DMA2D foreground CLUT
pub mod fgclut47;
/**FGCLUT48 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT48)

For information about available fields see [`mod@fgclut48`] module*/
pub type FGCLUT48 = crate::Reg<fgclut48::FGCLUT48rs>;
///DMA2D foreground CLUT
pub mod fgclut48;
/**FGCLUT49 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT49)

For information about available fields see [`mod@fgclut49`] module*/
pub type FGCLUT49 = crate::Reg<fgclut49::FGCLUT49rs>;
///DMA2D foreground CLUT
pub mod fgclut49;
/**FGCLUT50 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT50)

For information about available fields see [`mod@fgclut50`] module*/
pub type FGCLUT50 = crate::Reg<fgclut50::FGCLUT50rs>;
///DMA2D foreground CLUT
pub mod fgclut50;
/**FGCLUT51 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT51)

For information about available fields see [`mod@fgclut51`] module*/
pub type FGCLUT51 = crate::Reg<fgclut51::FGCLUT51rs>;
///DMA2D foreground CLUT
pub mod fgclut51;
/**FGCLUT52 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT52)

For information about available fields see [`mod@fgclut52`] module*/
pub type FGCLUT52 = crate::Reg<fgclut52::FGCLUT52rs>;
///DMA2D foreground CLUT
pub mod fgclut52;
/**FGCLUT53 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT53)

For information about available fields see [`mod@fgclut53`] module*/
pub type FGCLUT53 = crate::Reg<fgclut53::FGCLUT53rs>;
///DMA2D foreground CLUT
pub mod fgclut53;
/**FGCLUT54 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT54)

For information about available fields see [`mod@fgclut54`] module*/
pub type FGCLUT54 = crate::Reg<fgclut54::FGCLUT54rs>;
///DMA2D foreground CLUT
pub mod fgclut54;
/**FGCLUT55 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT55)

For information about available fields see [`mod@fgclut55`] module*/
pub type FGCLUT55 = crate::Reg<fgclut55::FGCLUT55rs>;
///DMA2D foreground CLUT
pub mod fgclut55;
/**FGCLUT56 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT56)

For information about available fields see [`mod@fgclut56`] module*/
pub type FGCLUT56 = crate::Reg<fgclut56::FGCLUT56rs>;
///DMA2D foreground CLUT
pub mod fgclut56;
/**FGCLUT57 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT57)

For information about available fields see [`mod@fgclut57`] module*/
pub type FGCLUT57 = crate::Reg<fgclut57::FGCLUT57rs>;
///DMA2D foreground CLUT
pub mod fgclut57;
/**FGCLUT58 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT58)

For information about available fields see [`mod@fgclut58`] module*/
pub type FGCLUT58 = crate::Reg<fgclut58::FGCLUT58rs>;
///DMA2D foreground CLUT
pub mod fgclut58;
/**FGCLUT59 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT59)

For information about available fields see [`mod@fgclut59`] module*/
pub type FGCLUT59 = crate::Reg<fgclut59::FGCLUT59rs>;
///DMA2D foreground CLUT
pub mod fgclut59;
/**FGCLUT60 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT60)

For information about available fields see [`mod@fgclut60`] module*/
pub type FGCLUT60 = crate::Reg<fgclut60::FGCLUT60rs>;
///DMA2D foreground CLUT
pub mod fgclut60;
/**FGCLUT61 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT61)

For information about available fields see [`mod@fgclut61`] module*/
pub type FGCLUT61 = crate::Reg<fgclut61::FGCLUT61rs>;
///DMA2D foreground CLUT
pub mod fgclut61;
/**FGCLUT62 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT62)

For information about available fields see [`mod@fgclut62`] module*/
pub type FGCLUT62 = crate::Reg<fgclut62::FGCLUT62rs>;
///DMA2D foreground CLUT
pub mod fgclut62;
/**FGCLUT63 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT63)

For information about available fields see [`mod@fgclut63`] module*/
pub type FGCLUT63 = crate::Reg<fgclut63::FGCLUT63rs>;
///DMA2D foreground CLUT
pub mod fgclut63;
/**FGCLUT64 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT64)

For information about available fields see [`mod@fgclut64`] module*/
pub type FGCLUT64 = crate::Reg<fgclut64::FGCLUT64rs>;
///DMA2D foreground CLUT
pub mod fgclut64;
/**FGCLUT65 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT65)

For information about available fields see [`mod@fgclut65`] module*/
pub type FGCLUT65 = crate::Reg<fgclut65::FGCLUT65rs>;
///DMA2D foreground CLUT
pub mod fgclut65;
/**FGCLUT66 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT66)

For information about available fields see [`mod@fgclut66`] module*/
pub type FGCLUT66 = crate::Reg<fgclut66::FGCLUT66rs>;
///DMA2D foreground CLUT
pub mod fgclut66;
/**FGCLUT67 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT67)

For information about available fields see [`mod@fgclut67`] module*/
pub type FGCLUT67 = crate::Reg<fgclut67::FGCLUT67rs>;
///DMA2D foreground CLUT
pub mod fgclut67;
/**FGCLUT68 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT68)

For information about available fields see [`mod@fgclut68`] module*/
pub type FGCLUT68 = crate::Reg<fgclut68::FGCLUT68rs>;
///DMA2D foreground CLUT
pub mod fgclut68;
/**FGCLUT69 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT69)

For information about available fields see [`mod@fgclut69`] module*/
pub type FGCLUT69 = crate::Reg<fgclut69::FGCLUT69rs>;
///DMA2D foreground CLUT
pub mod fgclut69;
/**FGCLUT70 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT70)

For information about available fields see [`mod@fgclut70`] module*/
pub type FGCLUT70 = crate::Reg<fgclut70::FGCLUT70rs>;
///DMA2D foreground CLUT
pub mod fgclut70;
/**FGCLUT71 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT71)

For information about available fields see [`mod@fgclut71`] module*/
pub type FGCLUT71 = crate::Reg<fgclut71::FGCLUT71rs>;
///DMA2D foreground CLUT
pub mod fgclut71;
/**FGCLUT72 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT72)

For information about available fields see [`mod@fgclut72`] module*/
pub type FGCLUT72 = crate::Reg<fgclut72::FGCLUT72rs>;
///DMA2D foreground CLUT
pub mod fgclut72;
/**FGCLUT73 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT73)

For information about available fields see [`mod@fgclut73`] module*/
pub type FGCLUT73 = crate::Reg<fgclut73::FGCLUT73rs>;
///DMA2D foreground CLUT
pub mod fgclut73;
/**FGCLUT74 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT74)

For information about available fields see [`mod@fgclut74`] module*/
pub type FGCLUT74 = crate::Reg<fgclut74::FGCLUT74rs>;
///DMA2D foreground CLUT
pub mod fgclut74;
/**FGCLUT75 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT75)

For information about available fields see [`mod@fgclut75`] module*/
pub type FGCLUT75 = crate::Reg<fgclut75::FGCLUT75rs>;
///DMA2D foreground CLUT
pub mod fgclut75;
/**FGCLUT76 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT76)

For information about available fields see [`mod@fgclut76`] module*/
pub type FGCLUT76 = crate::Reg<fgclut76::FGCLUT76rs>;
///DMA2D foreground CLUT
pub mod fgclut76;
/**FGCLUT77 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT77)

For information about available fields see [`mod@fgclut77`] module*/
pub type FGCLUT77 = crate::Reg<fgclut77::FGCLUT77rs>;
///DMA2D foreground CLUT
pub mod fgclut77;
/**FGCLUT78 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT78)

For information about available fields see [`mod@fgclut78`] module*/
pub type FGCLUT78 = crate::Reg<fgclut78::FGCLUT78rs>;
///DMA2D foreground CLUT
pub mod fgclut78;
/**FGCLUT79 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT79)

For information about available fields see [`mod@fgclut79`] module*/
pub type FGCLUT79 = crate::Reg<fgclut79::FGCLUT79rs>;
///DMA2D foreground CLUT
pub mod fgclut79;
/**FGCLUT80 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT80)

For information about available fields see [`mod@fgclut80`] module*/
pub type FGCLUT80 = crate::Reg<fgclut80::FGCLUT80rs>;
///DMA2D foreground CLUT
pub mod fgclut80;
/**FGCLUT81 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT81)

For information about available fields see [`mod@fgclut81`] module*/
pub type FGCLUT81 = crate::Reg<fgclut81::FGCLUT81rs>;
///DMA2D foreground CLUT
pub mod fgclut81;
/**FGCLUT82 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT82)

For information about available fields see [`mod@fgclut82`] module*/
pub type FGCLUT82 = crate::Reg<fgclut82::FGCLUT82rs>;
///DMA2D foreground CLUT
pub mod fgclut82;
/**FGCLUT83 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT83)

For information about available fields see [`mod@fgclut83`] module*/
pub type FGCLUT83 = crate::Reg<fgclut83::FGCLUT83rs>;
///DMA2D foreground CLUT
pub mod fgclut83;
/**FGCLUT84 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT84)

For information about available fields see [`mod@fgclut84`] module*/
pub type FGCLUT84 = crate::Reg<fgclut84::FGCLUT84rs>;
///DMA2D foreground CLUT
pub mod fgclut84;
/**FGCLUT85 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT85)

For information about available fields see [`mod@fgclut85`] module*/
pub type FGCLUT85 = crate::Reg<fgclut85::FGCLUT85rs>;
///DMA2D foreground CLUT
pub mod fgclut85;
/**FGCLUT86 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT86)

For information about available fields see [`mod@fgclut86`] module*/
pub type FGCLUT86 = crate::Reg<fgclut86::FGCLUT86rs>;
///DMA2D foreground CLUT
pub mod fgclut86;
/**FGCLUT87 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT87)

For information about available fields see [`mod@fgclut87`] module*/
pub type FGCLUT87 = crate::Reg<fgclut87::FGCLUT87rs>;
///DMA2D foreground CLUT
pub mod fgclut87;
/**FGCLUT88 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT88)

For information about available fields see [`mod@fgclut88`] module*/
pub type FGCLUT88 = crate::Reg<fgclut88::FGCLUT88rs>;
///DMA2D foreground CLUT
pub mod fgclut88;
/**FGCLUT89 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT89)

For information about available fields see [`mod@fgclut89`] module*/
pub type FGCLUT89 = crate::Reg<fgclut89::FGCLUT89rs>;
///DMA2D foreground CLUT
pub mod fgclut89;
/**FGCLUT90 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT90)

For information about available fields see [`mod@fgclut90`] module*/
pub type FGCLUT90 = crate::Reg<fgclut90::FGCLUT90rs>;
///DMA2D foreground CLUT
pub mod fgclut90;
/**FGCLUT91 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT91)

For information about available fields see [`mod@fgclut91`] module*/
pub type FGCLUT91 = crate::Reg<fgclut91::FGCLUT91rs>;
///DMA2D foreground CLUT
pub mod fgclut91;
/**FGCLUT92 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT92)

For information about available fields see [`mod@fgclut92`] module*/
pub type FGCLUT92 = crate::Reg<fgclut92::FGCLUT92rs>;
///DMA2D foreground CLUT
pub mod fgclut92;
/**FGCLUT93 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT93)

For information about available fields see [`mod@fgclut93`] module*/
pub type FGCLUT93 = crate::Reg<fgclut93::FGCLUT93rs>;
///DMA2D foreground CLUT
pub mod fgclut93;
/**FGCLUT94 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT94)

For information about available fields see [`mod@fgclut94`] module*/
pub type FGCLUT94 = crate::Reg<fgclut94::FGCLUT94rs>;
///DMA2D foreground CLUT
pub mod fgclut94;
/**FGCLUT95 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT95)

For information about available fields see [`mod@fgclut95`] module*/
pub type FGCLUT95 = crate::Reg<fgclut95::FGCLUT95rs>;
///DMA2D foreground CLUT
pub mod fgclut95;
/**FGCLUT96 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT96)

For information about available fields see [`mod@fgclut96`] module*/
pub type FGCLUT96 = crate::Reg<fgclut96::FGCLUT96rs>;
///DMA2D foreground CLUT
pub mod fgclut96;
/**FGCLUT97 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT97)

For information about available fields see [`mod@fgclut97`] module*/
pub type FGCLUT97 = crate::Reg<fgclut97::FGCLUT97rs>;
///DMA2D foreground CLUT
pub mod fgclut97;
/**FGCLUT98 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT98)

For information about available fields see [`mod@fgclut98`] module*/
pub type FGCLUT98 = crate::Reg<fgclut98::FGCLUT98rs>;
///DMA2D foreground CLUT
pub mod fgclut98;
/**FGCLUT99 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT99)

For information about available fields see [`mod@fgclut99`] module*/
pub type FGCLUT99 = crate::Reg<fgclut99::FGCLUT99rs>;
///DMA2D foreground CLUT
pub mod fgclut99;
/**FGCLUT100 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT100)

For information about available fields see [`mod@fgclut100`] module*/
pub type FGCLUT100 = crate::Reg<fgclut100::FGCLUT100rs>;
///DMA2D foreground CLUT
pub mod fgclut100;
/**FGCLUT101 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT101)

For information about available fields see [`mod@fgclut101`] module*/
pub type FGCLUT101 = crate::Reg<fgclut101::FGCLUT101rs>;
///DMA2D foreground CLUT
pub mod fgclut101;
/**FGCLUT102 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT102)

For information about available fields see [`mod@fgclut102`] module*/
pub type FGCLUT102 = crate::Reg<fgclut102::FGCLUT102rs>;
///DMA2D foreground CLUT
pub mod fgclut102;
/**FGCLUT103 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut103::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT103)

For information about available fields see [`mod@fgclut103`] module*/
pub type FGCLUT103 = crate::Reg<fgclut103::FGCLUT103rs>;
///DMA2D foreground CLUT
pub mod fgclut103;
/**FGCLUT104 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT104)

For information about available fields see [`mod@fgclut104`] module*/
pub type FGCLUT104 = crate::Reg<fgclut104::FGCLUT104rs>;
///DMA2D foreground CLUT
pub mod fgclut104;
/**FGCLUT105 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut105::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT105)

For information about available fields see [`mod@fgclut105`] module*/
pub type FGCLUT105 = crate::Reg<fgclut105::FGCLUT105rs>;
///DMA2D foreground CLUT
pub mod fgclut105;
/**FGCLUT106 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut106::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT106)

For information about available fields see [`mod@fgclut106`] module*/
pub type FGCLUT106 = crate::Reg<fgclut106::FGCLUT106rs>;
///DMA2D foreground CLUT
pub mod fgclut106;
/**FGCLUT107 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut107::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT107)

For information about available fields see [`mod@fgclut107`] module*/
pub type FGCLUT107 = crate::Reg<fgclut107::FGCLUT107rs>;
///DMA2D foreground CLUT
pub mod fgclut107;
/**FGCLUT108 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT108)

For information about available fields see [`mod@fgclut108`] module*/
pub type FGCLUT108 = crate::Reg<fgclut108::FGCLUT108rs>;
///DMA2D foreground CLUT
pub mod fgclut108;
/**FGCLUT109 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut109::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT109)

For information about available fields see [`mod@fgclut109`] module*/
pub type FGCLUT109 = crate::Reg<fgclut109::FGCLUT109rs>;
///DMA2D foreground CLUT
pub mod fgclut109;
/**FGCLUT110 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT110)

For information about available fields see [`mod@fgclut110`] module*/
pub type FGCLUT110 = crate::Reg<fgclut110::FGCLUT110rs>;
///DMA2D foreground CLUT
pub mod fgclut110;
/**FGCLUT111 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT111)

For information about available fields see [`mod@fgclut111`] module*/
pub type FGCLUT111 = crate::Reg<fgclut111::FGCLUT111rs>;
///DMA2D foreground CLUT
pub mod fgclut111;
/**FGCLUT112 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT112)

For information about available fields see [`mod@fgclut112`] module*/
pub type FGCLUT112 = crate::Reg<fgclut112::FGCLUT112rs>;
///DMA2D foreground CLUT
pub mod fgclut112;
/**FGCLUT113 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT113)

For information about available fields see [`mod@fgclut113`] module*/
pub type FGCLUT113 = crate::Reg<fgclut113::FGCLUT113rs>;
///DMA2D foreground CLUT
pub mod fgclut113;
/**FGCLUT114 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT114)

For information about available fields see [`mod@fgclut114`] module*/
pub type FGCLUT114 = crate::Reg<fgclut114::FGCLUT114rs>;
///DMA2D foreground CLUT
pub mod fgclut114;
/**FGCLUT115 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT115)

For information about available fields see [`mod@fgclut115`] module*/
pub type FGCLUT115 = crate::Reg<fgclut115::FGCLUT115rs>;
///DMA2D foreground CLUT
pub mod fgclut115;
/**FGCLUT116 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT116)

For information about available fields see [`mod@fgclut116`] module*/
pub type FGCLUT116 = crate::Reg<fgclut116::FGCLUT116rs>;
///DMA2D foreground CLUT
pub mod fgclut116;
/**FGCLUT117 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT117)

For information about available fields see [`mod@fgclut117`] module*/
pub type FGCLUT117 = crate::Reg<fgclut117::FGCLUT117rs>;
///DMA2D foreground CLUT
pub mod fgclut117;
/**FGCLUT118 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT118)

For information about available fields see [`mod@fgclut118`] module*/
pub type FGCLUT118 = crate::Reg<fgclut118::FGCLUT118rs>;
///DMA2D foreground CLUT
pub mod fgclut118;
/**FGCLUT119 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT119)

For information about available fields see [`mod@fgclut119`] module*/
pub type FGCLUT119 = crate::Reg<fgclut119::FGCLUT119rs>;
///DMA2D foreground CLUT
pub mod fgclut119;
/**FGCLUT120 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT120)

For information about available fields see [`mod@fgclut120`] module*/
pub type FGCLUT120 = crate::Reg<fgclut120::FGCLUT120rs>;
///DMA2D foreground CLUT
pub mod fgclut120;
/**FGCLUT121 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT121)

For information about available fields see [`mod@fgclut121`] module*/
pub type FGCLUT121 = crate::Reg<fgclut121::FGCLUT121rs>;
///DMA2D foreground CLUT
pub mod fgclut121;
/**FGCLUT122 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT122)

For information about available fields see [`mod@fgclut122`] module*/
pub type FGCLUT122 = crate::Reg<fgclut122::FGCLUT122rs>;
///DMA2D foreground CLUT
pub mod fgclut122;
/**FGCLUT123 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT123)

For information about available fields see [`mod@fgclut123`] module*/
pub type FGCLUT123 = crate::Reg<fgclut123::FGCLUT123rs>;
///DMA2D foreground CLUT
pub mod fgclut123;
/**FGCLUT124 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT124)

For information about available fields see [`mod@fgclut124`] module*/
pub type FGCLUT124 = crate::Reg<fgclut124::FGCLUT124rs>;
///DMA2D foreground CLUT
pub mod fgclut124;
/**FGCLUT125 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT125)

For information about available fields see [`mod@fgclut125`] module*/
pub type FGCLUT125 = crate::Reg<fgclut125::FGCLUT125rs>;
///DMA2D foreground CLUT
pub mod fgclut125;
/**FGCLUT126 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT126)

For information about available fields see [`mod@fgclut126`] module*/
pub type FGCLUT126 = crate::Reg<fgclut126::FGCLUT126rs>;
///DMA2D foreground CLUT
pub mod fgclut126;
/**FGCLUT127 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT127)

For information about available fields see [`mod@fgclut127`] module*/
pub type FGCLUT127 = crate::Reg<fgclut127::FGCLUT127rs>;
///DMA2D foreground CLUT
pub mod fgclut127;
/**FGCLUT128 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT128)

For information about available fields see [`mod@fgclut128`] module*/
pub type FGCLUT128 = crate::Reg<fgclut128::FGCLUT128rs>;
///DMA2D foreground CLUT
pub mod fgclut128;
/**FGCLUT129 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut129::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut129::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT129)

For information about available fields see [`mod@fgclut129`] module*/
pub type FGCLUT129 = crate::Reg<fgclut129::FGCLUT129rs>;
///DMA2D foreground CLUT
pub mod fgclut129;
/**FGCLUT130 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT130)

For information about available fields see [`mod@fgclut130`] module*/
pub type FGCLUT130 = crate::Reg<fgclut130::FGCLUT130rs>;
///DMA2D foreground CLUT
pub mod fgclut130;
/**FGCLUT131 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut131::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut131::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT131)

For information about available fields see [`mod@fgclut131`] module*/
pub type FGCLUT131 = crate::Reg<fgclut131::FGCLUT131rs>;
///DMA2D foreground CLUT
pub mod fgclut131;
/**FGCLUT132 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut132::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut132::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT132)

For information about available fields see [`mod@fgclut132`] module*/
pub type FGCLUT132 = crate::Reg<fgclut132::FGCLUT132rs>;
///DMA2D foreground CLUT
pub mod fgclut132;
/**FGCLUT133 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut133::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut133::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT133)

For information about available fields see [`mod@fgclut133`] module*/
pub type FGCLUT133 = crate::Reg<fgclut133::FGCLUT133rs>;
///DMA2D foreground CLUT
pub mod fgclut133;
/**FGCLUT134 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT134)

For information about available fields see [`mod@fgclut134`] module*/
pub type FGCLUT134 = crate::Reg<fgclut134::FGCLUT134rs>;
///DMA2D foreground CLUT
pub mod fgclut134;
/**FGCLUT135 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut135::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut135::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT135)

For information about available fields see [`mod@fgclut135`] module*/
pub type FGCLUT135 = crate::Reg<fgclut135::FGCLUT135rs>;
///DMA2D foreground CLUT
pub mod fgclut135;
/**FGCLUT136 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut136::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut136::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT136)

For information about available fields see [`mod@fgclut136`] module*/
pub type FGCLUT136 = crate::Reg<fgclut136::FGCLUT136rs>;
///DMA2D foreground CLUT
pub mod fgclut136;
/**FGCLUT137 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut137::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut137::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT137)

For information about available fields see [`mod@fgclut137`] module*/
pub type FGCLUT137 = crate::Reg<fgclut137::FGCLUT137rs>;
///DMA2D foreground CLUT
pub mod fgclut137;
/**FGCLUT138 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT138)

For information about available fields see [`mod@fgclut138`] module*/
pub type FGCLUT138 = crate::Reg<fgclut138::FGCLUT138rs>;
///DMA2D foreground CLUT
pub mod fgclut138;
/**FGCLUT139 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut139::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut139::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT139)

For information about available fields see [`mod@fgclut139`] module*/
pub type FGCLUT139 = crate::Reg<fgclut139::FGCLUT139rs>;
///DMA2D foreground CLUT
pub mod fgclut139;
/**FGCLUT140 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT140)

For information about available fields see [`mod@fgclut140`] module*/
pub type FGCLUT140 = crate::Reg<fgclut140::FGCLUT140rs>;
///DMA2D foreground CLUT
pub mod fgclut140;
/**FGCLUT141 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut141::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut141::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT141)

For information about available fields see [`mod@fgclut141`] module*/
pub type FGCLUT141 = crate::Reg<fgclut141::FGCLUT141rs>;
///DMA2D foreground CLUT
pub mod fgclut141;
/**FGCLUT142 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut142::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut142::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT142)

For information about available fields see [`mod@fgclut142`] module*/
pub type FGCLUT142 = crate::Reg<fgclut142::FGCLUT142rs>;
///DMA2D foreground CLUT
pub mod fgclut142;
/**FGCLUT143 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut143::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut143::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT143)

For information about available fields see [`mod@fgclut143`] module*/
pub type FGCLUT143 = crate::Reg<fgclut143::FGCLUT143rs>;
///DMA2D foreground CLUT
pub mod fgclut143;
/**FGCLUT144 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT144)

For information about available fields see [`mod@fgclut144`] module*/
pub type FGCLUT144 = crate::Reg<fgclut144::FGCLUT144rs>;
///DMA2D foreground CLUT
pub mod fgclut144;
/**FGCLUT145 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut145::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut145::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT145)

For information about available fields see [`mod@fgclut145`] module*/
pub type FGCLUT145 = crate::Reg<fgclut145::FGCLUT145rs>;
///DMA2D foreground CLUT
pub mod fgclut145;
/**FGCLUT146 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut146::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut146::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT146)

For information about available fields see [`mod@fgclut146`] module*/
pub type FGCLUT146 = crate::Reg<fgclut146::FGCLUT146rs>;
///DMA2D foreground CLUT
pub mod fgclut146;
/**FGCLUT147 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut147::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut147::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT147)

For information about available fields see [`mod@fgclut147`] module*/
pub type FGCLUT147 = crate::Reg<fgclut147::FGCLUT147rs>;
///DMA2D foreground CLUT
pub mod fgclut147;
/**FGCLUT148 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT148)

For information about available fields see [`mod@fgclut148`] module*/
pub type FGCLUT148 = crate::Reg<fgclut148::FGCLUT148rs>;
///DMA2D foreground CLUT
pub mod fgclut148;
/**FGCLUT149 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut149::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut149::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT149)

For information about available fields see [`mod@fgclut149`] module*/
pub type FGCLUT149 = crate::Reg<fgclut149::FGCLUT149rs>;
///DMA2D foreground CLUT
pub mod fgclut149;
/**FGCLUT150 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT150)

For information about available fields see [`mod@fgclut150`] module*/
pub type FGCLUT150 = crate::Reg<fgclut150::FGCLUT150rs>;
///DMA2D foreground CLUT
pub mod fgclut150;
/**FGCLUT151 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut151::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT151)

For information about available fields see [`mod@fgclut151`] module*/
pub type FGCLUT151 = crate::Reg<fgclut151::FGCLUT151rs>;
///DMA2D foreground CLUT
pub mod fgclut151;
/**FGCLUT152 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut152::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT152)

For information about available fields see [`mod@fgclut152`] module*/
pub type FGCLUT152 = crate::Reg<fgclut152::FGCLUT152rs>;
///DMA2D foreground CLUT
pub mod fgclut152;
/**FGCLUT153 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut153::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT153)

For information about available fields see [`mod@fgclut153`] module*/
pub type FGCLUT153 = crate::Reg<fgclut153::FGCLUT153rs>;
///DMA2D foreground CLUT
pub mod fgclut153;
/**FGCLUT154 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT154)

For information about available fields see [`mod@fgclut154`] module*/
pub type FGCLUT154 = crate::Reg<fgclut154::FGCLUT154rs>;
///DMA2D foreground CLUT
pub mod fgclut154;
/**FGCLUT155 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut155::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT155)

For information about available fields see [`mod@fgclut155`] module*/
pub type FGCLUT155 = crate::Reg<fgclut155::FGCLUT155rs>;
///DMA2D foreground CLUT
pub mod fgclut155;
/**FGCLUT156 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut156::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT156)

For information about available fields see [`mod@fgclut156`] module*/
pub type FGCLUT156 = crate::Reg<fgclut156::FGCLUT156rs>;
///DMA2D foreground CLUT
pub mod fgclut156;
/**FGCLUT157 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut157::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT157)

For information about available fields see [`mod@fgclut157`] module*/
pub type FGCLUT157 = crate::Reg<fgclut157::FGCLUT157rs>;
///DMA2D foreground CLUT
pub mod fgclut157;
/**FGCLUT158 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT158)

For information about available fields see [`mod@fgclut158`] module*/
pub type FGCLUT158 = crate::Reg<fgclut158::FGCLUT158rs>;
///DMA2D foreground CLUT
pub mod fgclut158;
/**FGCLUT159 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut159::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT159)

For information about available fields see [`mod@fgclut159`] module*/
pub type FGCLUT159 = crate::Reg<fgclut159::FGCLUT159rs>;
///DMA2D foreground CLUT
pub mod fgclut159;
/**FGCLUT160 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT160)

For information about available fields see [`mod@fgclut160`] module*/
pub type FGCLUT160 = crate::Reg<fgclut160::FGCLUT160rs>;
///DMA2D foreground CLUT
pub mod fgclut160;
/**FGCLUT161 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut161::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut161::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT161)

For information about available fields see [`mod@fgclut161`] module*/
pub type FGCLUT161 = crate::Reg<fgclut161::FGCLUT161rs>;
///DMA2D foreground CLUT
pub mod fgclut161;
/**FGCLUT162 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut162::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut162::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT162)

For information about available fields see [`mod@fgclut162`] module*/
pub type FGCLUT162 = crate::Reg<fgclut162::FGCLUT162rs>;
///DMA2D foreground CLUT
pub mod fgclut162;
/**FGCLUT163 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut163::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut163::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT163)

For information about available fields see [`mod@fgclut163`] module*/
pub type FGCLUT163 = crate::Reg<fgclut163::FGCLUT163rs>;
///DMA2D foreground CLUT
pub mod fgclut163;
/**FGCLUT164 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT164)

For information about available fields see [`mod@fgclut164`] module*/
pub type FGCLUT164 = crate::Reg<fgclut164::FGCLUT164rs>;
///DMA2D foreground CLUT
pub mod fgclut164;
/**FGCLUT165 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut165::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut165::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT165)

For information about available fields see [`mod@fgclut165`] module*/
pub type FGCLUT165 = crate::Reg<fgclut165::FGCLUT165rs>;
///DMA2D foreground CLUT
pub mod fgclut165;
/**FGCLUT166 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut166::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut166::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT166)

For information about available fields see [`mod@fgclut166`] module*/
pub type FGCLUT166 = crate::Reg<fgclut166::FGCLUT166rs>;
///DMA2D foreground CLUT
pub mod fgclut166;
/**FGCLUT167 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut167::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut167::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT167)

For information about available fields see [`mod@fgclut167`] module*/
pub type FGCLUT167 = crate::Reg<fgclut167::FGCLUT167rs>;
///DMA2D foreground CLUT
pub mod fgclut167;
/**FGCLUT168 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT168)

For information about available fields see [`mod@fgclut168`] module*/
pub type FGCLUT168 = crate::Reg<fgclut168::FGCLUT168rs>;
///DMA2D foreground CLUT
pub mod fgclut168;
/**FGCLUT169 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut169::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut169::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT169)

For information about available fields see [`mod@fgclut169`] module*/
pub type FGCLUT169 = crate::Reg<fgclut169::FGCLUT169rs>;
///DMA2D foreground CLUT
pub mod fgclut169;
/**FGCLUT170 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT170)

For information about available fields see [`mod@fgclut170`] module*/
pub type FGCLUT170 = crate::Reg<fgclut170::FGCLUT170rs>;
///DMA2D foreground CLUT
pub mod fgclut170;
/**FGCLUT171 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut171::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut171::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT171)

For information about available fields see [`mod@fgclut171`] module*/
pub type FGCLUT171 = crate::Reg<fgclut171::FGCLUT171rs>;
///DMA2D foreground CLUT
pub mod fgclut171;
/**FGCLUT172 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut172::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut172::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT172)

For information about available fields see [`mod@fgclut172`] module*/
pub type FGCLUT172 = crate::Reg<fgclut172::FGCLUT172rs>;
///DMA2D foreground CLUT
pub mod fgclut172;
/**FGCLUT173 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut173::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut173::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT173)

For information about available fields see [`mod@fgclut173`] module*/
pub type FGCLUT173 = crate::Reg<fgclut173::FGCLUT173rs>;
///DMA2D foreground CLUT
pub mod fgclut173;
/**FGCLUT174 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT174)

For information about available fields see [`mod@fgclut174`] module*/
pub type FGCLUT174 = crate::Reg<fgclut174::FGCLUT174rs>;
///DMA2D foreground CLUT
pub mod fgclut174;
/**FGCLUT175 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut175::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut175::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT175)

For information about available fields see [`mod@fgclut175`] module*/
pub type FGCLUT175 = crate::Reg<fgclut175::FGCLUT175rs>;
///DMA2D foreground CLUT
pub mod fgclut175;
/**FGCLUT176 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut176::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut176::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT176)

For information about available fields see [`mod@fgclut176`] module*/
pub type FGCLUT176 = crate::Reg<fgclut176::FGCLUT176rs>;
///DMA2D foreground CLUT
pub mod fgclut176;
/**FGCLUT177 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut177::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut177::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT177)

For information about available fields see [`mod@fgclut177`] module*/
pub type FGCLUT177 = crate::Reg<fgclut177::FGCLUT177rs>;
///DMA2D foreground CLUT
pub mod fgclut177;
/**FGCLUT178 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT178)

For information about available fields see [`mod@fgclut178`] module*/
pub type FGCLUT178 = crate::Reg<fgclut178::FGCLUT178rs>;
///DMA2D foreground CLUT
pub mod fgclut178;
/**FGCLUT179 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut179::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut179::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT179)

For information about available fields see [`mod@fgclut179`] module*/
pub type FGCLUT179 = crate::Reg<fgclut179::FGCLUT179rs>;
///DMA2D foreground CLUT
pub mod fgclut179;
/**FGCLUT180 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT180)

For information about available fields see [`mod@fgclut180`] module*/
pub type FGCLUT180 = crate::Reg<fgclut180::FGCLUT180rs>;
///DMA2D foreground CLUT
pub mod fgclut180;
/**FGCLUT181 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut181::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut181::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT181)

For information about available fields see [`mod@fgclut181`] module*/
pub type FGCLUT181 = crate::Reg<fgclut181::FGCLUT181rs>;
///DMA2D foreground CLUT
pub mod fgclut181;
/**FGCLUT182 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut182::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut182::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT182)

For information about available fields see [`mod@fgclut182`] module*/
pub type FGCLUT182 = crate::Reg<fgclut182::FGCLUT182rs>;
///DMA2D foreground CLUT
pub mod fgclut182;
/**FGCLUT183 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut183::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut183::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT183)

For information about available fields see [`mod@fgclut183`] module*/
pub type FGCLUT183 = crate::Reg<fgclut183::FGCLUT183rs>;
///DMA2D foreground CLUT
pub mod fgclut183;
/**FGCLUT184 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT184)

For information about available fields see [`mod@fgclut184`] module*/
pub type FGCLUT184 = crate::Reg<fgclut184::FGCLUT184rs>;
///DMA2D foreground CLUT
pub mod fgclut184;
/**FGCLUT185 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut185::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut185::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT185)

For information about available fields see [`mod@fgclut185`] module*/
pub type FGCLUT185 = crate::Reg<fgclut185::FGCLUT185rs>;
///DMA2D foreground CLUT
pub mod fgclut185;
/**FGCLUT186 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut186::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut186::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT186)

For information about available fields see [`mod@fgclut186`] module*/
pub type FGCLUT186 = crate::Reg<fgclut186::FGCLUT186rs>;
///DMA2D foreground CLUT
pub mod fgclut186;
/**FGCLUT187 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut187::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut187::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT187)

For information about available fields see [`mod@fgclut187`] module*/
pub type FGCLUT187 = crate::Reg<fgclut187::FGCLUT187rs>;
///DMA2D foreground CLUT
pub mod fgclut187;
/**FGCLUT188 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT188)

For information about available fields see [`mod@fgclut188`] module*/
pub type FGCLUT188 = crate::Reg<fgclut188::FGCLUT188rs>;
///DMA2D foreground CLUT
pub mod fgclut188;
/**FGCLUT189 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut189::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut189::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT189)

For information about available fields see [`mod@fgclut189`] module*/
pub type FGCLUT189 = crate::Reg<fgclut189::FGCLUT189rs>;
///DMA2D foreground CLUT
pub mod fgclut189;
/**FGCLUT190 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT190)

For information about available fields see [`mod@fgclut190`] module*/
pub type FGCLUT190 = crate::Reg<fgclut190::FGCLUT190rs>;
///DMA2D foreground CLUT
pub mod fgclut190;
/**FGCLUT191 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut191::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut191::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT191)

For information about available fields see [`mod@fgclut191`] module*/
pub type FGCLUT191 = crate::Reg<fgclut191::FGCLUT191rs>;
///DMA2D foreground CLUT
pub mod fgclut191;
/**FGCLUT192 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut192::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut192::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT192)

For information about available fields see [`mod@fgclut192`] module*/
pub type FGCLUT192 = crate::Reg<fgclut192::FGCLUT192rs>;
///DMA2D foreground CLUT
pub mod fgclut192;
/**FGCLUT193 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut193::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut193::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT193)

For information about available fields see [`mod@fgclut193`] module*/
pub type FGCLUT193 = crate::Reg<fgclut193::FGCLUT193rs>;
///DMA2D foreground CLUT
pub mod fgclut193;
/**FGCLUT194 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT194)

For information about available fields see [`mod@fgclut194`] module*/
pub type FGCLUT194 = crate::Reg<fgclut194::FGCLUT194rs>;
///DMA2D foreground CLUT
pub mod fgclut194;
/**FGCLUT195 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut195::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut195::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT195)

For information about available fields see [`mod@fgclut195`] module*/
pub type FGCLUT195 = crate::Reg<fgclut195::FGCLUT195rs>;
///DMA2D foreground CLUT
pub mod fgclut195;
/**FGCLUT196 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut196::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut196::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT196)

For information about available fields see [`mod@fgclut196`] module*/
pub type FGCLUT196 = crate::Reg<fgclut196::FGCLUT196rs>;
///DMA2D foreground CLUT
pub mod fgclut196;
/**FGCLUT197 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut197::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut197::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT197)

For information about available fields see [`mod@fgclut197`] module*/
pub type FGCLUT197 = crate::Reg<fgclut197::FGCLUT197rs>;
///DMA2D foreground CLUT
pub mod fgclut197;
/**FGCLUT198 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT198)

For information about available fields see [`mod@fgclut198`] module*/
pub type FGCLUT198 = crate::Reg<fgclut198::FGCLUT198rs>;
///DMA2D foreground CLUT
pub mod fgclut198;
/**FGCLUT199 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut199::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut199::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT199)

For information about available fields see [`mod@fgclut199`] module*/
pub type FGCLUT199 = crate::Reg<fgclut199::FGCLUT199rs>;
///DMA2D foreground CLUT
pub mod fgclut199;
/**FGCLUT200 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT200)

For information about available fields see [`mod@fgclut200`] module*/
pub type FGCLUT200 = crate::Reg<fgclut200::FGCLUT200rs>;
///DMA2D foreground CLUT
pub mod fgclut200;
/**FGCLUT201 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut201::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut201::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT201)

For information about available fields see [`mod@fgclut201`] module*/
pub type FGCLUT201 = crate::Reg<fgclut201::FGCLUT201rs>;
///DMA2D foreground CLUT
pub mod fgclut201;
/**FGCLUT202 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut202::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut202::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT202)

For information about available fields see [`mod@fgclut202`] module*/
pub type FGCLUT202 = crate::Reg<fgclut202::FGCLUT202rs>;
///DMA2D foreground CLUT
pub mod fgclut202;
/**FGCLUT203 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut203::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut203::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT203)

For information about available fields see [`mod@fgclut203`] module*/
pub type FGCLUT203 = crate::Reg<fgclut203::FGCLUT203rs>;
///DMA2D foreground CLUT
pub mod fgclut203;
/**FGCLUT204 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT204)

For information about available fields see [`mod@fgclut204`] module*/
pub type FGCLUT204 = crate::Reg<fgclut204::FGCLUT204rs>;
///DMA2D foreground CLUT
pub mod fgclut204;
/**FGCLUT205 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut205::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut205::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT205)

For information about available fields see [`mod@fgclut205`] module*/
pub type FGCLUT205 = crate::Reg<fgclut205::FGCLUT205rs>;
///DMA2D foreground CLUT
pub mod fgclut205;
/**FGCLUT206 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut206::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut206::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT206)

For information about available fields see [`mod@fgclut206`] module*/
pub type FGCLUT206 = crate::Reg<fgclut206::FGCLUT206rs>;
///DMA2D foreground CLUT
pub mod fgclut206;
/**FGCLUT207 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut207::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut207::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT207)

For information about available fields see [`mod@fgclut207`] module*/
pub type FGCLUT207 = crate::Reg<fgclut207::FGCLUT207rs>;
///DMA2D foreground CLUT
pub mod fgclut207;
/**FGCLUT208 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT208)

For information about available fields see [`mod@fgclut208`] module*/
pub type FGCLUT208 = crate::Reg<fgclut208::FGCLUT208rs>;
///DMA2D foreground CLUT
pub mod fgclut208;
/**FGCLUT209 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut209::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut209::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT209)

For information about available fields see [`mod@fgclut209`] module*/
pub type FGCLUT209 = crate::Reg<fgclut209::FGCLUT209rs>;
///DMA2D foreground CLUT
pub mod fgclut209;
/**FGCLUT210 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT210)

For information about available fields see [`mod@fgclut210`] module*/
pub type FGCLUT210 = crate::Reg<fgclut210::FGCLUT210rs>;
///DMA2D foreground CLUT
pub mod fgclut210;
/**FGCLUT211 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut211::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut211::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT211)

For information about available fields see [`mod@fgclut211`] module*/
pub type FGCLUT211 = crate::Reg<fgclut211::FGCLUT211rs>;
///DMA2D foreground CLUT
pub mod fgclut211;
/**FGCLUT212 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut212::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut212::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT212)

For information about available fields see [`mod@fgclut212`] module*/
pub type FGCLUT212 = crate::Reg<fgclut212::FGCLUT212rs>;
///DMA2D foreground CLUT
pub mod fgclut212;
/**FGCLUT213 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut213::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut213::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT213)

For information about available fields see [`mod@fgclut213`] module*/
pub type FGCLUT213 = crate::Reg<fgclut213::FGCLUT213rs>;
///DMA2D foreground CLUT
pub mod fgclut213;
/**FGCLUT214 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT214)

For information about available fields see [`mod@fgclut214`] module*/
pub type FGCLUT214 = crate::Reg<fgclut214::FGCLUT214rs>;
///DMA2D foreground CLUT
pub mod fgclut214;
/**FGCLUT215 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut215::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut215::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT215)

For information about available fields see [`mod@fgclut215`] module*/
pub type FGCLUT215 = crate::Reg<fgclut215::FGCLUT215rs>;
///DMA2D foreground CLUT
pub mod fgclut215;
/**FGCLUT216 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut216::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut216::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT216)

For information about available fields see [`mod@fgclut216`] module*/
pub type FGCLUT216 = crate::Reg<fgclut216::FGCLUT216rs>;
///DMA2D foreground CLUT
pub mod fgclut216;
/**FGCLUT217 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut217::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut217::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT217)

For information about available fields see [`mod@fgclut217`] module*/
pub type FGCLUT217 = crate::Reg<fgclut217::FGCLUT217rs>;
///DMA2D foreground CLUT
pub mod fgclut217;
/**FGCLUT218 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT218)

For information about available fields see [`mod@fgclut218`] module*/
pub type FGCLUT218 = crate::Reg<fgclut218::FGCLUT218rs>;
///DMA2D foreground CLUT
pub mod fgclut218;
/**FGCLUT219 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut219::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut219::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT219)

For information about available fields see [`mod@fgclut219`] module*/
pub type FGCLUT219 = crate::Reg<fgclut219::FGCLUT219rs>;
///DMA2D foreground CLUT
pub mod fgclut219;
/**FGCLUT220 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT220)

For information about available fields see [`mod@fgclut220`] module*/
pub type FGCLUT220 = crate::Reg<fgclut220::FGCLUT220rs>;
///DMA2D foreground CLUT
pub mod fgclut220;
/**FGCLUT221 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut221::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut221::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT221)

For information about available fields see [`mod@fgclut221`] module*/
pub type FGCLUT221 = crate::Reg<fgclut221::FGCLUT221rs>;
///DMA2D foreground CLUT
pub mod fgclut221;
/**FGCLUT222 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut222::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut222::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT222)

For information about available fields see [`mod@fgclut222`] module*/
pub type FGCLUT222 = crate::Reg<fgclut222::FGCLUT222rs>;
///DMA2D foreground CLUT
pub mod fgclut222;
/**FGCLUT223 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut223::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut223::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT223)

For information about available fields see [`mod@fgclut223`] module*/
pub type FGCLUT223 = crate::Reg<fgclut223::FGCLUT223rs>;
///DMA2D foreground CLUT
pub mod fgclut223;
/**FGCLUT224 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT224)

For information about available fields see [`mod@fgclut224`] module*/
pub type FGCLUT224 = crate::Reg<fgclut224::FGCLUT224rs>;
///DMA2D foreground CLUT
pub mod fgclut224;
/**FGCLUT225 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut225::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut225::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT225)

For information about available fields see [`mod@fgclut225`] module*/
pub type FGCLUT225 = crate::Reg<fgclut225::FGCLUT225rs>;
///DMA2D foreground CLUT
pub mod fgclut225;
/**FGCLUT226 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut226::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut226::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT226)

For information about available fields see [`mod@fgclut226`] module*/
pub type FGCLUT226 = crate::Reg<fgclut226::FGCLUT226rs>;
///DMA2D foreground CLUT
pub mod fgclut226;
/**FGCLUT227 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut227::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut227::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT227)

For information about available fields see [`mod@fgclut227`] module*/
pub type FGCLUT227 = crate::Reg<fgclut227::FGCLUT227rs>;
///DMA2D foreground CLUT
pub mod fgclut227;
/**FGCLUT228 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT228)

For information about available fields see [`mod@fgclut228`] module*/
pub type FGCLUT228 = crate::Reg<fgclut228::FGCLUT228rs>;
///DMA2D foreground CLUT
pub mod fgclut228;
/**FGCLUT229 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut229::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut229::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT229)

For information about available fields see [`mod@fgclut229`] module*/
pub type FGCLUT229 = crate::Reg<fgclut229::FGCLUT229rs>;
///DMA2D foreground CLUT
pub mod fgclut229;
/**FGCLUT230 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT230)

For information about available fields see [`mod@fgclut230`] module*/
pub type FGCLUT230 = crate::Reg<fgclut230::FGCLUT230rs>;
///DMA2D foreground CLUT
pub mod fgclut230;
/**FGCLUT231 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut231::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut231::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT231)

For information about available fields see [`mod@fgclut231`] module*/
pub type FGCLUT231 = crate::Reg<fgclut231::FGCLUT231rs>;
///DMA2D foreground CLUT
pub mod fgclut231;
/**FGCLUT232 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut232::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut232::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT232)

For information about available fields see [`mod@fgclut232`] module*/
pub type FGCLUT232 = crate::Reg<fgclut232::FGCLUT232rs>;
///DMA2D foreground CLUT
pub mod fgclut232;
/**FGCLUT233 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut233::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut233::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT233)

For information about available fields see [`mod@fgclut233`] module*/
pub type FGCLUT233 = crate::Reg<fgclut233::FGCLUT233rs>;
///DMA2D foreground CLUT
pub mod fgclut233;
/**FGCLUT234 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT234)

For information about available fields see [`mod@fgclut234`] module*/
pub type FGCLUT234 = crate::Reg<fgclut234::FGCLUT234rs>;
///DMA2D foreground CLUT
pub mod fgclut234;
/**FGCLUT235 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut235::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut235::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT235)

For information about available fields see [`mod@fgclut235`] module*/
pub type FGCLUT235 = crate::Reg<fgclut235::FGCLUT235rs>;
///DMA2D foreground CLUT
pub mod fgclut235;
/**FGCLUT236 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut236::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut236::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT236)

For information about available fields see [`mod@fgclut236`] module*/
pub type FGCLUT236 = crate::Reg<fgclut236::FGCLUT236rs>;
///DMA2D foreground CLUT
pub mod fgclut236;
/**FGCLUT237 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut237::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut237::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT237)

For information about available fields see [`mod@fgclut237`] module*/
pub type FGCLUT237 = crate::Reg<fgclut237::FGCLUT237rs>;
///DMA2D foreground CLUT
pub mod fgclut237;
/**FGCLUT238 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT238)

For information about available fields see [`mod@fgclut238`] module*/
pub type FGCLUT238 = crate::Reg<fgclut238::FGCLUT238rs>;
///DMA2D foreground CLUT
pub mod fgclut238;
/**FGCLUT239 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut239::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut239::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT239)

For information about available fields see [`mod@fgclut239`] module*/
pub type FGCLUT239 = crate::Reg<fgclut239::FGCLUT239rs>;
///DMA2D foreground CLUT
pub mod fgclut239;
/**FGCLUT240 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT240)

For information about available fields see [`mod@fgclut240`] module*/
pub type FGCLUT240 = crate::Reg<fgclut240::FGCLUT240rs>;
///DMA2D foreground CLUT
pub mod fgclut240;
/**FGCLUT241 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut241::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut241::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT241)

For information about available fields see [`mod@fgclut241`] module*/
pub type FGCLUT241 = crate::Reg<fgclut241::FGCLUT241rs>;
///DMA2D foreground CLUT
pub mod fgclut241;
/**FGCLUT242 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut242::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut242::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT242)

For information about available fields see [`mod@fgclut242`] module*/
pub type FGCLUT242 = crate::Reg<fgclut242::FGCLUT242rs>;
///DMA2D foreground CLUT
pub mod fgclut242;
/**FGCLUT243 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut243::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut243::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT243)

For information about available fields see [`mod@fgclut243`] module*/
pub type FGCLUT243 = crate::Reg<fgclut243::FGCLUT243rs>;
///DMA2D foreground CLUT
pub mod fgclut243;
/**FGCLUT244 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT244)

For information about available fields see [`mod@fgclut244`] module*/
pub type FGCLUT244 = crate::Reg<fgclut244::FGCLUT244rs>;
///DMA2D foreground CLUT
pub mod fgclut244;
/**FGCLUT245 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut245::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut245::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT245)

For information about available fields see [`mod@fgclut245`] module*/
pub type FGCLUT245 = crate::Reg<fgclut245::FGCLUT245rs>;
///DMA2D foreground CLUT
pub mod fgclut245;
/**FGCLUT246 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut246::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut246::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT246)

For information about available fields see [`mod@fgclut246`] module*/
pub type FGCLUT246 = crate::Reg<fgclut246::FGCLUT246rs>;
///DMA2D foreground CLUT
pub mod fgclut246;
/**FGCLUT247 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut247::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut247::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT247)

For information about available fields see [`mod@fgclut247`] module*/
pub type FGCLUT247 = crate::Reg<fgclut247::FGCLUT247rs>;
///DMA2D foreground CLUT
pub mod fgclut247;
/**FGCLUT248 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT248)

For information about available fields see [`mod@fgclut248`] module*/
pub type FGCLUT248 = crate::Reg<fgclut248::FGCLUT248rs>;
///DMA2D foreground CLUT
pub mod fgclut248;
/**FGCLUT249 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut249::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut249::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT249)

For information about available fields see [`mod@fgclut249`] module*/
pub type FGCLUT249 = crate::Reg<fgclut249::FGCLUT249rs>;
///DMA2D foreground CLUT
pub mod fgclut249;
/**FGCLUT250 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT250)

For information about available fields see [`mod@fgclut250`] module*/
pub type FGCLUT250 = crate::Reg<fgclut250::FGCLUT250rs>;
///DMA2D foreground CLUT
pub mod fgclut250;
/**FGCLUT251 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut251::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut251::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT251)

For information about available fields see [`mod@fgclut251`] module*/
pub type FGCLUT251 = crate::Reg<fgclut251::FGCLUT251rs>;
///DMA2D foreground CLUT
pub mod fgclut251;
/**FGCLUT252 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut252::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut252::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT252)

For information about available fields see [`mod@fgclut252`] module*/
pub type FGCLUT252 = crate::Reg<fgclut252::FGCLUT252rs>;
///DMA2D foreground CLUT
pub mod fgclut252;
/**FGCLUT253 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut253::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut253::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT253)

For information about available fields see [`mod@fgclut253`] module*/
pub type FGCLUT253 = crate::Reg<fgclut253::FGCLUT253rs>;
///DMA2D foreground CLUT
pub mod fgclut253;
/**FGCLUT254 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT254)

For information about available fields see [`mod@fgclut254`] module*/
pub type FGCLUT254 = crate::Reg<fgclut254::FGCLUT254rs>;
///DMA2D foreground CLUT
pub mod fgclut254;
/**FGCLUT255 (rw) register accessor: DMA2D foreground CLUT

You can [`read`](crate::Reg::read) this register and get [`fgclut255::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgclut255::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:FGCLUT255)

For information about available fields see [`mod@fgclut255`] module*/
pub type FGCLUT255 = crate::Reg<fgclut255::FGCLUT255rs>;
///DMA2D foreground CLUT
pub mod fgclut255;
/**BGCLUT0 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT0)

For information about available fields see [`mod@bgclut0`] module*/
pub type BGCLUT0 = crate::Reg<bgclut0::BGCLUT0rs>;
///DMA2D background CLUT
pub mod bgclut0;
/**BGCLUT1 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT1)

For information about available fields see [`mod@bgclut1`] module*/
pub type BGCLUT1 = crate::Reg<bgclut1::BGCLUT1rs>;
///DMA2D background CLUT
pub mod bgclut1;
/**BGCLUT2 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT2)

For information about available fields see [`mod@bgclut2`] module*/
pub type BGCLUT2 = crate::Reg<bgclut2::BGCLUT2rs>;
///DMA2D background CLUT
pub mod bgclut2;
/**BGCLUT3 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT3)

For information about available fields see [`mod@bgclut3`] module*/
pub type BGCLUT3 = crate::Reg<bgclut3::BGCLUT3rs>;
///DMA2D background CLUT
pub mod bgclut3;
/**BGCLUT4 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT4)

For information about available fields see [`mod@bgclut4`] module*/
pub type BGCLUT4 = crate::Reg<bgclut4::BGCLUT4rs>;
///DMA2D background CLUT
pub mod bgclut4;
/**BGCLUT5 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT5)

For information about available fields see [`mod@bgclut5`] module*/
pub type BGCLUT5 = crate::Reg<bgclut5::BGCLUT5rs>;
///DMA2D background CLUT
pub mod bgclut5;
/**BGCLUT6 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT6)

For information about available fields see [`mod@bgclut6`] module*/
pub type BGCLUT6 = crate::Reg<bgclut6::BGCLUT6rs>;
///DMA2D background CLUT
pub mod bgclut6;
/**BGCLUT7 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT7)

For information about available fields see [`mod@bgclut7`] module*/
pub type BGCLUT7 = crate::Reg<bgclut7::BGCLUT7rs>;
///DMA2D background CLUT
pub mod bgclut7;
/**BGCLUT8 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT8)

For information about available fields see [`mod@bgclut8`] module*/
pub type BGCLUT8 = crate::Reg<bgclut8::BGCLUT8rs>;
///DMA2D background CLUT
pub mod bgclut8;
/**BGCLUT9 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT9)

For information about available fields see [`mod@bgclut9`] module*/
pub type BGCLUT9 = crate::Reg<bgclut9::BGCLUT9rs>;
///DMA2D background CLUT
pub mod bgclut9;
/**BGCLUT10 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT10)

For information about available fields see [`mod@bgclut10`] module*/
pub type BGCLUT10 = crate::Reg<bgclut10::BGCLUT10rs>;
///DMA2D background CLUT
pub mod bgclut10;
/**BGCLUT11 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT11)

For information about available fields see [`mod@bgclut11`] module*/
pub type BGCLUT11 = crate::Reg<bgclut11::BGCLUT11rs>;
///DMA2D background CLUT
pub mod bgclut11;
/**BGCLUT12 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT12)

For information about available fields see [`mod@bgclut12`] module*/
pub type BGCLUT12 = crate::Reg<bgclut12::BGCLUT12rs>;
///DMA2D background CLUT
pub mod bgclut12;
/**BGCLUT13 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT13)

For information about available fields see [`mod@bgclut13`] module*/
pub type BGCLUT13 = crate::Reg<bgclut13::BGCLUT13rs>;
///DMA2D background CLUT
pub mod bgclut13;
/**BGCLUT14 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT14)

For information about available fields see [`mod@bgclut14`] module*/
pub type BGCLUT14 = crate::Reg<bgclut14::BGCLUT14rs>;
///DMA2D background CLUT
pub mod bgclut14;
/**BGCLUT15 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT15)

For information about available fields see [`mod@bgclut15`] module*/
pub type BGCLUT15 = crate::Reg<bgclut15::BGCLUT15rs>;
///DMA2D background CLUT
pub mod bgclut15;
/**BGCLUT16 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT16)

For information about available fields see [`mod@bgclut16`] module*/
pub type BGCLUT16 = crate::Reg<bgclut16::BGCLUT16rs>;
///DMA2D background CLUT
pub mod bgclut16;
/**BGCLUT17 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT17)

For information about available fields see [`mod@bgclut17`] module*/
pub type BGCLUT17 = crate::Reg<bgclut17::BGCLUT17rs>;
///DMA2D background CLUT
pub mod bgclut17;
/**BGCLUT18 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT18)

For information about available fields see [`mod@bgclut18`] module*/
pub type BGCLUT18 = crate::Reg<bgclut18::BGCLUT18rs>;
///DMA2D background CLUT
pub mod bgclut18;
/**BGCLUT19 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT19)

For information about available fields see [`mod@bgclut19`] module*/
pub type BGCLUT19 = crate::Reg<bgclut19::BGCLUT19rs>;
///DMA2D background CLUT
pub mod bgclut19;
/**BGCLUT20 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT20)

For information about available fields see [`mod@bgclut20`] module*/
pub type BGCLUT20 = crate::Reg<bgclut20::BGCLUT20rs>;
///DMA2D background CLUT
pub mod bgclut20;
/**BGCLUT21 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT21)

For information about available fields see [`mod@bgclut21`] module*/
pub type BGCLUT21 = crate::Reg<bgclut21::BGCLUT21rs>;
///DMA2D background CLUT
pub mod bgclut21;
/**BGCLUT22 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT22)

For information about available fields see [`mod@bgclut22`] module*/
pub type BGCLUT22 = crate::Reg<bgclut22::BGCLUT22rs>;
///DMA2D background CLUT
pub mod bgclut22;
/**BGCLUT23 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT23)

For information about available fields see [`mod@bgclut23`] module*/
pub type BGCLUT23 = crate::Reg<bgclut23::BGCLUT23rs>;
///DMA2D background CLUT
pub mod bgclut23;
/**BGCLUT24 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT24)

For information about available fields see [`mod@bgclut24`] module*/
pub type BGCLUT24 = crate::Reg<bgclut24::BGCLUT24rs>;
///DMA2D background CLUT
pub mod bgclut24;
/**BGCLUT25 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT25)

For information about available fields see [`mod@bgclut25`] module*/
pub type BGCLUT25 = crate::Reg<bgclut25::BGCLUT25rs>;
///DMA2D background CLUT
pub mod bgclut25;
/**BGCLUT26 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT26)

For information about available fields see [`mod@bgclut26`] module*/
pub type BGCLUT26 = crate::Reg<bgclut26::BGCLUT26rs>;
///DMA2D background CLUT
pub mod bgclut26;
/**BGCLUT27 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT27)

For information about available fields see [`mod@bgclut27`] module*/
pub type BGCLUT27 = crate::Reg<bgclut27::BGCLUT27rs>;
///DMA2D background CLUT
pub mod bgclut27;
/**BGCLUT28 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT28)

For information about available fields see [`mod@bgclut28`] module*/
pub type BGCLUT28 = crate::Reg<bgclut28::BGCLUT28rs>;
///DMA2D background CLUT
pub mod bgclut28;
/**BGCLUT29 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT29)

For information about available fields see [`mod@bgclut29`] module*/
pub type BGCLUT29 = crate::Reg<bgclut29::BGCLUT29rs>;
///DMA2D background CLUT
pub mod bgclut29;
/**BGCLUT30 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT30)

For information about available fields see [`mod@bgclut30`] module*/
pub type BGCLUT30 = crate::Reg<bgclut30::BGCLUT30rs>;
///DMA2D background CLUT
pub mod bgclut30;
/**BGCLUT31 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT31)

For information about available fields see [`mod@bgclut31`] module*/
pub type BGCLUT31 = crate::Reg<bgclut31::BGCLUT31rs>;
///DMA2D background CLUT
pub mod bgclut31;
/**BGCLUT32 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut32::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut32::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT32)

For information about available fields see [`mod@bgclut32`] module*/
pub type BGCLUT32 = crate::Reg<bgclut32::BGCLUT32rs>;
///DMA2D background CLUT
pub mod bgclut32;
/**BGCLUT33 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut33::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut33::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT33)

For information about available fields see [`mod@bgclut33`] module*/
pub type BGCLUT33 = crate::Reg<bgclut33::BGCLUT33rs>;
///DMA2D background CLUT
pub mod bgclut33;
/**BGCLUT34 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut34::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut34::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT34)

For information about available fields see [`mod@bgclut34`] module*/
pub type BGCLUT34 = crate::Reg<bgclut34::BGCLUT34rs>;
///DMA2D background CLUT
pub mod bgclut34;
/**BGCLUT35 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut35::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut35::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT35)

For information about available fields see [`mod@bgclut35`] module*/
pub type BGCLUT35 = crate::Reg<bgclut35::BGCLUT35rs>;
///DMA2D background CLUT
pub mod bgclut35;
/**BGCLUT36 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut36::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut36::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT36)

For information about available fields see [`mod@bgclut36`] module*/
pub type BGCLUT36 = crate::Reg<bgclut36::BGCLUT36rs>;
///DMA2D background CLUT
pub mod bgclut36;
/**BGCLUT37 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut37::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut37::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT37)

For information about available fields see [`mod@bgclut37`] module*/
pub type BGCLUT37 = crate::Reg<bgclut37::BGCLUT37rs>;
///DMA2D background CLUT
pub mod bgclut37;
/**BGCLUT38 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut38::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut38::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT38)

For information about available fields see [`mod@bgclut38`] module*/
pub type BGCLUT38 = crate::Reg<bgclut38::BGCLUT38rs>;
///DMA2D background CLUT
pub mod bgclut38;
/**BGCLUT39 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut39::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut39::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT39)

For information about available fields see [`mod@bgclut39`] module*/
pub type BGCLUT39 = crate::Reg<bgclut39::BGCLUT39rs>;
///DMA2D background CLUT
pub mod bgclut39;
/**BGCLUT40 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut40::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut40::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT40)

For information about available fields see [`mod@bgclut40`] module*/
pub type BGCLUT40 = crate::Reg<bgclut40::BGCLUT40rs>;
///DMA2D background CLUT
pub mod bgclut40;
/**BGCLUT41 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut41::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut41::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT41)

For information about available fields see [`mod@bgclut41`] module*/
pub type BGCLUT41 = crate::Reg<bgclut41::BGCLUT41rs>;
///DMA2D background CLUT
pub mod bgclut41;
/**BGCLUT42 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut42::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut42::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT42)

For information about available fields see [`mod@bgclut42`] module*/
pub type BGCLUT42 = crate::Reg<bgclut42::BGCLUT42rs>;
///DMA2D background CLUT
pub mod bgclut42;
/**BGCLUT43 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut43::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut43::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT43)

For information about available fields see [`mod@bgclut43`] module*/
pub type BGCLUT43 = crate::Reg<bgclut43::BGCLUT43rs>;
///DMA2D background CLUT
pub mod bgclut43;
/**BGCLUT44 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut44::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut44::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT44)

For information about available fields see [`mod@bgclut44`] module*/
pub type BGCLUT44 = crate::Reg<bgclut44::BGCLUT44rs>;
///DMA2D background CLUT
pub mod bgclut44;
/**BGCLUT45 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut45::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut45::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT45)

For information about available fields see [`mod@bgclut45`] module*/
pub type BGCLUT45 = crate::Reg<bgclut45::BGCLUT45rs>;
///DMA2D background CLUT
pub mod bgclut45;
/**BGCLUT46 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut46::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut46::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT46)

For information about available fields see [`mod@bgclut46`] module*/
pub type BGCLUT46 = crate::Reg<bgclut46::BGCLUT46rs>;
///DMA2D background CLUT
pub mod bgclut46;
/**BGCLUT47 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut47::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut47::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT47)

For information about available fields see [`mod@bgclut47`] module*/
pub type BGCLUT47 = crate::Reg<bgclut47::BGCLUT47rs>;
///DMA2D background CLUT
pub mod bgclut47;
/**BGCLUT48 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut48::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut48::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT48)

For information about available fields see [`mod@bgclut48`] module*/
pub type BGCLUT48 = crate::Reg<bgclut48::BGCLUT48rs>;
///DMA2D background CLUT
pub mod bgclut48;
/**BGCLUT49 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut49::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut49::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT49)

For information about available fields see [`mod@bgclut49`] module*/
pub type BGCLUT49 = crate::Reg<bgclut49::BGCLUT49rs>;
///DMA2D background CLUT
pub mod bgclut49;
/**BGCLUT50 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut50::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut50::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT50)

For information about available fields see [`mod@bgclut50`] module*/
pub type BGCLUT50 = crate::Reg<bgclut50::BGCLUT50rs>;
///DMA2D background CLUT
pub mod bgclut50;
/**BGCLUT51 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut51::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut51::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT51)

For information about available fields see [`mod@bgclut51`] module*/
pub type BGCLUT51 = crate::Reg<bgclut51::BGCLUT51rs>;
///DMA2D background CLUT
pub mod bgclut51;
/**BGCLUT52 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut52::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut52::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT52)

For information about available fields see [`mod@bgclut52`] module*/
pub type BGCLUT52 = crate::Reg<bgclut52::BGCLUT52rs>;
///DMA2D background CLUT
pub mod bgclut52;
/**BGCLUT53 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut53::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut53::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT53)

For information about available fields see [`mod@bgclut53`] module*/
pub type BGCLUT53 = crate::Reg<bgclut53::BGCLUT53rs>;
///DMA2D background CLUT
pub mod bgclut53;
/**BGCLUT54 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut54::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut54::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT54)

For information about available fields see [`mod@bgclut54`] module*/
pub type BGCLUT54 = crate::Reg<bgclut54::BGCLUT54rs>;
///DMA2D background CLUT
pub mod bgclut54;
/**BGCLUT55 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut55::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut55::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT55)

For information about available fields see [`mod@bgclut55`] module*/
pub type BGCLUT55 = crate::Reg<bgclut55::BGCLUT55rs>;
///DMA2D background CLUT
pub mod bgclut55;
/**BGCLUT56 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut56::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut56::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT56)

For information about available fields see [`mod@bgclut56`] module*/
pub type BGCLUT56 = crate::Reg<bgclut56::BGCLUT56rs>;
///DMA2D background CLUT
pub mod bgclut56;
/**BGCLUT57 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut57::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut57::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT57)

For information about available fields see [`mod@bgclut57`] module*/
pub type BGCLUT57 = crate::Reg<bgclut57::BGCLUT57rs>;
///DMA2D background CLUT
pub mod bgclut57;
/**BGCLUT58 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut58::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut58::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT58)

For information about available fields see [`mod@bgclut58`] module*/
pub type BGCLUT58 = crate::Reg<bgclut58::BGCLUT58rs>;
///DMA2D background CLUT
pub mod bgclut58;
/**BGCLUT59 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut59::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut59::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT59)

For information about available fields see [`mod@bgclut59`] module*/
pub type BGCLUT59 = crate::Reg<bgclut59::BGCLUT59rs>;
///DMA2D background CLUT
pub mod bgclut59;
/**BGCLUT60 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut60::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut60::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT60)

For information about available fields see [`mod@bgclut60`] module*/
pub type BGCLUT60 = crate::Reg<bgclut60::BGCLUT60rs>;
///DMA2D background CLUT
pub mod bgclut60;
/**BGCLUT61 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut61::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut61::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT61)

For information about available fields see [`mod@bgclut61`] module*/
pub type BGCLUT61 = crate::Reg<bgclut61::BGCLUT61rs>;
///DMA2D background CLUT
pub mod bgclut61;
/**BGCLUT62 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut62::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut62::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT62)

For information about available fields see [`mod@bgclut62`] module*/
pub type BGCLUT62 = crate::Reg<bgclut62::BGCLUT62rs>;
///DMA2D background CLUT
pub mod bgclut62;
/**BGCLUT63 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut63::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut63::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT63)

For information about available fields see [`mod@bgclut63`] module*/
pub type BGCLUT63 = crate::Reg<bgclut63::BGCLUT63rs>;
///DMA2D background CLUT
pub mod bgclut63;
/**BGCLUT64 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut64::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut64::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT64)

For information about available fields see [`mod@bgclut64`] module*/
pub type BGCLUT64 = crate::Reg<bgclut64::BGCLUT64rs>;
///DMA2D background CLUT
pub mod bgclut64;
/**BGCLUT65 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut65::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut65::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT65)

For information about available fields see [`mod@bgclut65`] module*/
pub type BGCLUT65 = crate::Reg<bgclut65::BGCLUT65rs>;
///DMA2D background CLUT
pub mod bgclut65;
/**BGCLUT66 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut66::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut66::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT66)

For information about available fields see [`mod@bgclut66`] module*/
pub type BGCLUT66 = crate::Reg<bgclut66::BGCLUT66rs>;
///DMA2D background CLUT
pub mod bgclut66;
/**BGCLUT67 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut67::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut67::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT67)

For information about available fields see [`mod@bgclut67`] module*/
pub type BGCLUT67 = crate::Reg<bgclut67::BGCLUT67rs>;
///DMA2D background CLUT
pub mod bgclut67;
/**BGCLUT68 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut68::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut68::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT68)

For information about available fields see [`mod@bgclut68`] module*/
pub type BGCLUT68 = crate::Reg<bgclut68::BGCLUT68rs>;
///DMA2D background CLUT
pub mod bgclut68;
/**BGCLUT69 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut69::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut69::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT69)

For information about available fields see [`mod@bgclut69`] module*/
pub type BGCLUT69 = crate::Reg<bgclut69::BGCLUT69rs>;
///DMA2D background CLUT
pub mod bgclut69;
/**BGCLUT70 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut70::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut70::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT70)

For information about available fields see [`mod@bgclut70`] module*/
pub type BGCLUT70 = crate::Reg<bgclut70::BGCLUT70rs>;
///DMA2D background CLUT
pub mod bgclut70;
/**BGCLUT71 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut71::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut71::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT71)

For information about available fields see [`mod@bgclut71`] module*/
pub type BGCLUT71 = crate::Reg<bgclut71::BGCLUT71rs>;
///DMA2D background CLUT
pub mod bgclut71;
/**BGCLUT72 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut72::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut72::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT72)

For information about available fields see [`mod@bgclut72`] module*/
pub type BGCLUT72 = crate::Reg<bgclut72::BGCLUT72rs>;
///DMA2D background CLUT
pub mod bgclut72;
/**BGCLUT73 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut73::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut73::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT73)

For information about available fields see [`mod@bgclut73`] module*/
pub type BGCLUT73 = crate::Reg<bgclut73::BGCLUT73rs>;
///DMA2D background CLUT
pub mod bgclut73;
/**BGCLUT74 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut74::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut74::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT74)

For information about available fields see [`mod@bgclut74`] module*/
pub type BGCLUT74 = crate::Reg<bgclut74::BGCLUT74rs>;
///DMA2D background CLUT
pub mod bgclut74;
/**BGCLUT75 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut75::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut75::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT75)

For information about available fields see [`mod@bgclut75`] module*/
pub type BGCLUT75 = crate::Reg<bgclut75::BGCLUT75rs>;
///DMA2D background CLUT
pub mod bgclut75;
/**BGCLUT76 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut76::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut76::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT76)

For information about available fields see [`mod@bgclut76`] module*/
pub type BGCLUT76 = crate::Reg<bgclut76::BGCLUT76rs>;
///DMA2D background CLUT
pub mod bgclut76;
/**BGCLUT77 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut77::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut77::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT77)

For information about available fields see [`mod@bgclut77`] module*/
pub type BGCLUT77 = crate::Reg<bgclut77::BGCLUT77rs>;
///DMA2D background CLUT
pub mod bgclut77;
/**BGCLUT78 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut78::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut78::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT78)

For information about available fields see [`mod@bgclut78`] module*/
pub type BGCLUT78 = crate::Reg<bgclut78::BGCLUT78rs>;
///DMA2D background CLUT
pub mod bgclut78;
/**BGCLUT79 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut79::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut79::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT79)

For information about available fields see [`mod@bgclut79`] module*/
pub type BGCLUT79 = crate::Reg<bgclut79::BGCLUT79rs>;
///DMA2D background CLUT
pub mod bgclut79;
/**BGCLUT80 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut80::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut80::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT80)

For information about available fields see [`mod@bgclut80`] module*/
pub type BGCLUT80 = crate::Reg<bgclut80::BGCLUT80rs>;
///DMA2D background CLUT
pub mod bgclut80;
/**BGCLUT81 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut81::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut81::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT81)

For information about available fields see [`mod@bgclut81`] module*/
pub type BGCLUT81 = crate::Reg<bgclut81::BGCLUT81rs>;
///DMA2D background CLUT
pub mod bgclut81;
/**BGCLUT82 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut82::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut82::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT82)

For information about available fields see [`mod@bgclut82`] module*/
pub type BGCLUT82 = crate::Reg<bgclut82::BGCLUT82rs>;
///DMA2D background CLUT
pub mod bgclut82;
/**BGCLUT83 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut83::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut83::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT83)

For information about available fields see [`mod@bgclut83`] module*/
pub type BGCLUT83 = crate::Reg<bgclut83::BGCLUT83rs>;
///DMA2D background CLUT
pub mod bgclut83;
/**BGCLUT84 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut84::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut84::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT84)

For information about available fields see [`mod@bgclut84`] module*/
pub type BGCLUT84 = crate::Reg<bgclut84::BGCLUT84rs>;
///DMA2D background CLUT
pub mod bgclut84;
/**BGCLUT85 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut85::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut85::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT85)

For information about available fields see [`mod@bgclut85`] module*/
pub type BGCLUT85 = crate::Reg<bgclut85::BGCLUT85rs>;
///DMA2D background CLUT
pub mod bgclut85;
/**BGCLUT86 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut86::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut86::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT86)

For information about available fields see [`mod@bgclut86`] module*/
pub type BGCLUT86 = crate::Reg<bgclut86::BGCLUT86rs>;
///DMA2D background CLUT
pub mod bgclut86;
/**BGCLUT87 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut87::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut87::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT87)

For information about available fields see [`mod@bgclut87`] module*/
pub type BGCLUT87 = crate::Reg<bgclut87::BGCLUT87rs>;
///DMA2D background CLUT
pub mod bgclut87;
/**BGCLUT88 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut88::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut88::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT88)

For information about available fields see [`mod@bgclut88`] module*/
pub type BGCLUT88 = crate::Reg<bgclut88::BGCLUT88rs>;
///DMA2D background CLUT
pub mod bgclut88;
/**BGCLUT89 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut89::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut89::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT89)

For information about available fields see [`mod@bgclut89`] module*/
pub type BGCLUT89 = crate::Reg<bgclut89::BGCLUT89rs>;
///DMA2D background CLUT
pub mod bgclut89;
/**BGCLUT90 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut90::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut90::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT90)

For information about available fields see [`mod@bgclut90`] module*/
pub type BGCLUT90 = crate::Reg<bgclut90::BGCLUT90rs>;
///DMA2D background CLUT
pub mod bgclut90;
/**BGCLUT91 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut91::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut91::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT91)

For information about available fields see [`mod@bgclut91`] module*/
pub type BGCLUT91 = crate::Reg<bgclut91::BGCLUT91rs>;
///DMA2D background CLUT
pub mod bgclut91;
/**BGCLUT92 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut92::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut92::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT92)

For information about available fields see [`mod@bgclut92`] module*/
pub type BGCLUT92 = crate::Reg<bgclut92::BGCLUT92rs>;
///DMA2D background CLUT
pub mod bgclut92;
/**BGCLUT93 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut93::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut93::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT93)

For information about available fields see [`mod@bgclut93`] module*/
pub type BGCLUT93 = crate::Reg<bgclut93::BGCLUT93rs>;
///DMA2D background CLUT
pub mod bgclut93;
/**BGCLUT94 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut94::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut94::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT94)

For information about available fields see [`mod@bgclut94`] module*/
pub type BGCLUT94 = crate::Reg<bgclut94::BGCLUT94rs>;
///DMA2D background CLUT
pub mod bgclut94;
/**BGCLUT95 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut95::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut95::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT95)

For information about available fields see [`mod@bgclut95`] module*/
pub type BGCLUT95 = crate::Reg<bgclut95::BGCLUT95rs>;
///DMA2D background CLUT
pub mod bgclut95;
/**BGCLUT96 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut96::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut96::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT96)

For information about available fields see [`mod@bgclut96`] module*/
pub type BGCLUT96 = crate::Reg<bgclut96::BGCLUT96rs>;
///DMA2D background CLUT
pub mod bgclut96;
/**BGCLUT97 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut97::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut97::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT97)

For information about available fields see [`mod@bgclut97`] module*/
pub type BGCLUT97 = crate::Reg<bgclut97::BGCLUT97rs>;
///DMA2D background CLUT
pub mod bgclut97;
/**BGCLUT98 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut98::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut98::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT98)

For information about available fields see [`mod@bgclut98`] module*/
pub type BGCLUT98 = crate::Reg<bgclut98::BGCLUT98rs>;
///DMA2D background CLUT
pub mod bgclut98;
/**BGCLUT99 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut99::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut99::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT99)

For information about available fields see [`mod@bgclut99`] module*/
pub type BGCLUT99 = crate::Reg<bgclut99::BGCLUT99rs>;
///DMA2D background CLUT
pub mod bgclut99;
/**BGCLUT100 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut100::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut100::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT100)

For information about available fields see [`mod@bgclut100`] module*/
pub type BGCLUT100 = crate::Reg<bgclut100::BGCLUT100rs>;
///DMA2D background CLUT
pub mod bgclut100;
/**BGCLUT101 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut101::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut101::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT101)

For information about available fields see [`mod@bgclut101`] module*/
pub type BGCLUT101 = crate::Reg<bgclut101::BGCLUT101rs>;
///DMA2D background CLUT
pub mod bgclut101;
/**BGCLUT102 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut102::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut102::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT102)

For information about available fields see [`mod@bgclut102`] module*/
pub type BGCLUT102 = crate::Reg<bgclut102::BGCLUT102rs>;
///DMA2D background CLUT
pub mod bgclut102;
/**BGCLUT103 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut103::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut103::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT103)

For information about available fields see [`mod@bgclut103`] module*/
pub type BGCLUT103 = crate::Reg<bgclut103::BGCLUT103rs>;
///DMA2D background CLUT
pub mod bgclut103;
/**BGCLUT104 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut104::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut104::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT104)

For information about available fields see [`mod@bgclut104`] module*/
pub type BGCLUT104 = crate::Reg<bgclut104::BGCLUT104rs>;
///DMA2D background CLUT
pub mod bgclut104;
/**BGCLUT105 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut105::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut105::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT105)

For information about available fields see [`mod@bgclut105`] module*/
pub type BGCLUT105 = crate::Reg<bgclut105::BGCLUT105rs>;
///DMA2D background CLUT
pub mod bgclut105;
/**BGCLUT106 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut106::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut106::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT106)

For information about available fields see [`mod@bgclut106`] module*/
pub type BGCLUT106 = crate::Reg<bgclut106::BGCLUT106rs>;
///DMA2D background CLUT
pub mod bgclut106;
/**BGCLUT107 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut107::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut107::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT107)

For information about available fields see [`mod@bgclut107`] module*/
pub type BGCLUT107 = crate::Reg<bgclut107::BGCLUT107rs>;
///DMA2D background CLUT
pub mod bgclut107;
/**BGCLUT108 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut108::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut108::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT108)

For information about available fields see [`mod@bgclut108`] module*/
pub type BGCLUT108 = crate::Reg<bgclut108::BGCLUT108rs>;
///DMA2D background CLUT
pub mod bgclut108;
/**BGCLUT109 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut109::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut109::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT109)

For information about available fields see [`mod@bgclut109`] module*/
pub type BGCLUT109 = crate::Reg<bgclut109::BGCLUT109rs>;
///DMA2D background CLUT
pub mod bgclut109;
/**BGCLUT110 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut110::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut110::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT110)

For information about available fields see [`mod@bgclut110`] module*/
pub type BGCLUT110 = crate::Reg<bgclut110::BGCLUT110rs>;
///DMA2D background CLUT
pub mod bgclut110;
/**BGCLUT111 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut111::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut111::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT111)

For information about available fields see [`mod@bgclut111`] module*/
pub type BGCLUT111 = crate::Reg<bgclut111::BGCLUT111rs>;
///DMA2D background CLUT
pub mod bgclut111;
/**BGCLUT112 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut112::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut112::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT112)

For information about available fields see [`mod@bgclut112`] module*/
pub type BGCLUT112 = crate::Reg<bgclut112::BGCLUT112rs>;
///DMA2D background CLUT
pub mod bgclut112;
/**BGCLUT113 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut113::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut113::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT113)

For information about available fields see [`mod@bgclut113`] module*/
pub type BGCLUT113 = crate::Reg<bgclut113::BGCLUT113rs>;
///DMA2D background CLUT
pub mod bgclut113;
/**BGCLUT114 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut114::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut114::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT114)

For information about available fields see [`mod@bgclut114`] module*/
pub type BGCLUT114 = crate::Reg<bgclut114::BGCLUT114rs>;
///DMA2D background CLUT
pub mod bgclut114;
/**BGCLUT115 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut115::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut115::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT115)

For information about available fields see [`mod@bgclut115`] module*/
pub type BGCLUT115 = crate::Reg<bgclut115::BGCLUT115rs>;
///DMA2D background CLUT
pub mod bgclut115;
/**BGCLUT116 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut116::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut116::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT116)

For information about available fields see [`mod@bgclut116`] module*/
pub type BGCLUT116 = crate::Reg<bgclut116::BGCLUT116rs>;
///DMA2D background CLUT
pub mod bgclut116;
/**BGCLUT117 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut117::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut117::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT117)

For information about available fields see [`mod@bgclut117`] module*/
pub type BGCLUT117 = crate::Reg<bgclut117::BGCLUT117rs>;
///DMA2D background CLUT
pub mod bgclut117;
/**BGCLUT118 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut118::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut118::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT118)

For information about available fields see [`mod@bgclut118`] module*/
pub type BGCLUT118 = crate::Reg<bgclut118::BGCLUT118rs>;
///DMA2D background CLUT
pub mod bgclut118;
/**BGCLUT119 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut119::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut119::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT119)

For information about available fields see [`mod@bgclut119`] module*/
pub type BGCLUT119 = crate::Reg<bgclut119::BGCLUT119rs>;
///DMA2D background CLUT
pub mod bgclut119;
/**BGCLUT120 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut120::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut120::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT120)

For information about available fields see [`mod@bgclut120`] module*/
pub type BGCLUT120 = crate::Reg<bgclut120::BGCLUT120rs>;
///DMA2D background CLUT
pub mod bgclut120;
/**BGCLUT121 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut121::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut121::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT121)

For information about available fields see [`mod@bgclut121`] module*/
pub type BGCLUT121 = crate::Reg<bgclut121::BGCLUT121rs>;
///DMA2D background CLUT
pub mod bgclut121;
/**BGCLUT122 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut122::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut122::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT122)

For information about available fields see [`mod@bgclut122`] module*/
pub type BGCLUT122 = crate::Reg<bgclut122::BGCLUT122rs>;
///DMA2D background CLUT
pub mod bgclut122;
/**BGCLUT123 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut123::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut123::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT123)

For information about available fields see [`mod@bgclut123`] module*/
pub type BGCLUT123 = crate::Reg<bgclut123::BGCLUT123rs>;
///DMA2D background CLUT
pub mod bgclut123;
/**BGCLUT124 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut124::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut124::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT124)

For information about available fields see [`mod@bgclut124`] module*/
pub type BGCLUT124 = crate::Reg<bgclut124::BGCLUT124rs>;
///DMA2D background CLUT
pub mod bgclut124;
/**BGCLUT125 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut125::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut125::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT125)

For information about available fields see [`mod@bgclut125`] module*/
pub type BGCLUT125 = crate::Reg<bgclut125::BGCLUT125rs>;
///DMA2D background CLUT
pub mod bgclut125;
/**BGCLUT126 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut126::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut126::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT126)

For information about available fields see [`mod@bgclut126`] module*/
pub type BGCLUT126 = crate::Reg<bgclut126::BGCLUT126rs>;
///DMA2D background CLUT
pub mod bgclut126;
/**BGCLUT127 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut127::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut127::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT127)

For information about available fields see [`mod@bgclut127`] module*/
pub type BGCLUT127 = crate::Reg<bgclut127::BGCLUT127rs>;
///DMA2D background CLUT
pub mod bgclut127;
/**BGCLUT128 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut128::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut128::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT128)

For information about available fields see [`mod@bgclut128`] module*/
pub type BGCLUT128 = crate::Reg<bgclut128::BGCLUT128rs>;
///DMA2D background CLUT
pub mod bgclut128;
/**BGCLUT129 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut129::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut129::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT129)

For information about available fields see [`mod@bgclut129`] module*/
pub type BGCLUT129 = crate::Reg<bgclut129::BGCLUT129rs>;
///DMA2D background CLUT
pub mod bgclut129;
/**BGCLUT130 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut130::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut130::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT130)

For information about available fields see [`mod@bgclut130`] module*/
pub type BGCLUT130 = crate::Reg<bgclut130::BGCLUT130rs>;
///DMA2D background CLUT
pub mod bgclut130;
/**BGCLUT131 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut131::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut131::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT131)

For information about available fields see [`mod@bgclut131`] module*/
pub type BGCLUT131 = crate::Reg<bgclut131::BGCLUT131rs>;
///DMA2D background CLUT
pub mod bgclut131;
/**BGCLUT132 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut132::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut132::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT132)

For information about available fields see [`mod@bgclut132`] module*/
pub type BGCLUT132 = crate::Reg<bgclut132::BGCLUT132rs>;
///DMA2D background CLUT
pub mod bgclut132;
/**BGCLUT133 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut133::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut133::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT133)

For information about available fields see [`mod@bgclut133`] module*/
pub type BGCLUT133 = crate::Reg<bgclut133::BGCLUT133rs>;
///DMA2D background CLUT
pub mod bgclut133;
/**BGCLUT134 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut134::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut134::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT134)

For information about available fields see [`mod@bgclut134`] module*/
pub type BGCLUT134 = crate::Reg<bgclut134::BGCLUT134rs>;
///DMA2D background CLUT
pub mod bgclut134;
/**BGCLUT135 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut135::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut135::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT135)

For information about available fields see [`mod@bgclut135`] module*/
pub type BGCLUT135 = crate::Reg<bgclut135::BGCLUT135rs>;
///DMA2D background CLUT
pub mod bgclut135;
/**BGCLUT136 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut136::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut136::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT136)

For information about available fields see [`mod@bgclut136`] module*/
pub type BGCLUT136 = crate::Reg<bgclut136::BGCLUT136rs>;
///DMA2D background CLUT
pub mod bgclut136;
/**BGCLUT137 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut137::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut137::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT137)

For information about available fields see [`mod@bgclut137`] module*/
pub type BGCLUT137 = crate::Reg<bgclut137::BGCLUT137rs>;
///DMA2D background CLUT
pub mod bgclut137;
/**BGCLUT138 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut138::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut138::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT138)

For information about available fields see [`mod@bgclut138`] module*/
pub type BGCLUT138 = crate::Reg<bgclut138::BGCLUT138rs>;
///DMA2D background CLUT
pub mod bgclut138;
/**BGCLUT139 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut139::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut139::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT139)

For information about available fields see [`mod@bgclut139`] module*/
pub type BGCLUT139 = crate::Reg<bgclut139::BGCLUT139rs>;
///DMA2D background CLUT
pub mod bgclut139;
/**BGCLUT140 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut140::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut140::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT140)

For information about available fields see [`mod@bgclut140`] module*/
pub type BGCLUT140 = crate::Reg<bgclut140::BGCLUT140rs>;
///DMA2D background CLUT
pub mod bgclut140;
/**BGCLUT141 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut141::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut141::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT141)

For information about available fields see [`mod@bgclut141`] module*/
pub type BGCLUT141 = crate::Reg<bgclut141::BGCLUT141rs>;
///DMA2D background CLUT
pub mod bgclut141;
/**BGCLUT142 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut142::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut142::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT142)

For information about available fields see [`mod@bgclut142`] module*/
pub type BGCLUT142 = crate::Reg<bgclut142::BGCLUT142rs>;
///DMA2D background CLUT
pub mod bgclut142;
/**BGCLUT143 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut143::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut143::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT143)

For information about available fields see [`mod@bgclut143`] module*/
pub type BGCLUT143 = crate::Reg<bgclut143::BGCLUT143rs>;
///DMA2D background CLUT
pub mod bgclut143;
/**BGCLUT144 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut144::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut144::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT144)

For information about available fields see [`mod@bgclut144`] module*/
pub type BGCLUT144 = crate::Reg<bgclut144::BGCLUT144rs>;
///DMA2D background CLUT
pub mod bgclut144;
/**BGCLUT145 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut145::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut145::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT145)

For information about available fields see [`mod@bgclut145`] module*/
pub type BGCLUT145 = crate::Reg<bgclut145::BGCLUT145rs>;
///DMA2D background CLUT
pub mod bgclut145;
/**BGCLUT146 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut146::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut146::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT146)

For information about available fields see [`mod@bgclut146`] module*/
pub type BGCLUT146 = crate::Reg<bgclut146::BGCLUT146rs>;
///DMA2D background CLUT
pub mod bgclut146;
/**BGCLUT147 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut147::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut147::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT147)

For information about available fields see [`mod@bgclut147`] module*/
pub type BGCLUT147 = crate::Reg<bgclut147::BGCLUT147rs>;
///DMA2D background CLUT
pub mod bgclut147;
/**BGCLUT148 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut148::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut148::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT148)

For information about available fields see [`mod@bgclut148`] module*/
pub type BGCLUT148 = crate::Reg<bgclut148::BGCLUT148rs>;
///DMA2D background CLUT
pub mod bgclut148;
/**BGCLUT149 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut149::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut149::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT149)

For information about available fields see [`mod@bgclut149`] module*/
pub type BGCLUT149 = crate::Reg<bgclut149::BGCLUT149rs>;
///DMA2D background CLUT
pub mod bgclut149;
/**BGCLUT150 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut150::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut150::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT150)

For information about available fields see [`mod@bgclut150`] module*/
pub type BGCLUT150 = crate::Reg<bgclut150::BGCLUT150rs>;
///DMA2D background CLUT
pub mod bgclut150;
/**BGCLUT151 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut151::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut151::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT151)

For information about available fields see [`mod@bgclut151`] module*/
pub type BGCLUT151 = crate::Reg<bgclut151::BGCLUT151rs>;
///DMA2D background CLUT
pub mod bgclut151;
/**BGCLUT152 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut152::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut152::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT152)

For information about available fields see [`mod@bgclut152`] module*/
pub type BGCLUT152 = crate::Reg<bgclut152::BGCLUT152rs>;
///DMA2D background CLUT
pub mod bgclut152;
/**BGCLUT153 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut153::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut153::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT153)

For information about available fields see [`mod@bgclut153`] module*/
pub type BGCLUT153 = crate::Reg<bgclut153::BGCLUT153rs>;
///DMA2D background CLUT
pub mod bgclut153;
/**BGCLUT154 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut154::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut154::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT154)

For information about available fields see [`mod@bgclut154`] module*/
pub type BGCLUT154 = crate::Reg<bgclut154::BGCLUT154rs>;
///DMA2D background CLUT
pub mod bgclut154;
/**BGCLUT155 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut155::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut155::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT155)

For information about available fields see [`mod@bgclut155`] module*/
pub type BGCLUT155 = crate::Reg<bgclut155::BGCLUT155rs>;
///DMA2D background CLUT
pub mod bgclut155;
/**BGCLUT156 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut156::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut156::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT156)

For information about available fields see [`mod@bgclut156`] module*/
pub type BGCLUT156 = crate::Reg<bgclut156::BGCLUT156rs>;
///DMA2D background CLUT
pub mod bgclut156;
/**BGCLUT157 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut157::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut157::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT157)

For information about available fields see [`mod@bgclut157`] module*/
pub type BGCLUT157 = crate::Reg<bgclut157::BGCLUT157rs>;
///DMA2D background CLUT
pub mod bgclut157;
/**BGCLUT158 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut158::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut158::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT158)

For information about available fields see [`mod@bgclut158`] module*/
pub type BGCLUT158 = crate::Reg<bgclut158::BGCLUT158rs>;
///DMA2D background CLUT
pub mod bgclut158;
/**BGCLUT159 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut159::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut159::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT159)

For information about available fields see [`mod@bgclut159`] module*/
pub type BGCLUT159 = crate::Reg<bgclut159::BGCLUT159rs>;
///DMA2D background CLUT
pub mod bgclut159;
/**BGCLUT160 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut160::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut160::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT160)

For information about available fields see [`mod@bgclut160`] module*/
pub type BGCLUT160 = crate::Reg<bgclut160::BGCLUT160rs>;
///DMA2D background CLUT
pub mod bgclut160;
/**BGCLUT161 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut161::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut161::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT161)

For information about available fields see [`mod@bgclut161`] module*/
pub type BGCLUT161 = crate::Reg<bgclut161::BGCLUT161rs>;
///DMA2D background CLUT
pub mod bgclut161;
/**BGCLUT162 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut162::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut162::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT162)

For information about available fields see [`mod@bgclut162`] module*/
pub type BGCLUT162 = crate::Reg<bgclut162::BGCLUT162rs>;
///DMA2D background CLUT
pub mod bgclut162;
/**BGCLUT163 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut163::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut163::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT163)

For information about available fields see [`mod@bgclut163`] module*/
pub type BGCLUT163 = crate::Reg<bgclut163::BGCLUT163rs>;
///DMA2D background CLUT
pub mod bgclut163;
/**BGCLUT164 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut164::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut164::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT164)

For information about available fields see [`mod@bgclut164`] module*/
pub type BGCLUT164 = crate::Reg<bgclut164::BGCLUT164rs>;
///DMA2D background CLUT
pub mod bgclut164;
/**BGCLUT165 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut165::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut165::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT165)

For information about available fields see [`mod@bgclut165`] module*/
pub type BGCLUT165 = crate::Reg<bgclut165::BGCLUT165rs>;
///DMA2D background CLUT
pub mod bgclut165;
/**BGCLUT166 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut166::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut166::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT166)

For information about available fields see [`mod@bgclut166`] module*/
pub type BGCLUT166 = crate::Reg<bgclut166::BGCLUT166rs>;
///DMA2D background CLUT
pub mod bgclut166;
/**BGCLUT167 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut167::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut167::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT167)

For information about available fields see [`mod@bgclut167`] module*/
pub type BGCLUT167 = crate::Reg<bgclut167::BGCLUT167rs>;
///DMA2D background CLUT
pub mod bgclut167;
/**BGCLUT168 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut168::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut168::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT168)

For information about available fields see [`mod@bgclut168`] module*/
pub type BGCLUT168 = crate::Reg<bgclut168::BGCLUT168rs>;
///DMA2D background CLUT
pub mod bgclut168;
/**BGCLUT169 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut169::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut169::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT169)

For information about available fields see [`mod@bgclut169`] module*/
pub type BGCLUT169 = crate::Reg<bgclut169::BGCLUT169rs>;
///DMA2D background CLUT
pub mod bgclut169;
/**BGCLUT170 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut170::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut170::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT170)

For information about available fields see [`mod@bgclut170`] module*/
pub type BGCLUT170 = crate::Reg<bgclut170::BGCLUT170rs>;
///DMA2D background CLUT
pub mod bgclut170;
/**BGCLUT171 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut171::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut171::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT171)

For information about available fields see [`mod@bgclut171`] module*/
pub type BGCLUT171 = crate::Reg<bgclut171::BGCLUT171rs>;
///DMA2D background CLUT
pub mod bgclut171;
/**BGCLUT172 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut172::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut172::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT172)

For information about available fields see [`mod@bgclut172`] module*/
pub type BGCLUT172 = crate::Reg<bgclut172::BGCLUT172rs>;
///DMA2D background CLUT
pub mod bgclut172;
/**BGCLUT173 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut173::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut173::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT173)

For information about available fields see [`mod@bgclut173`] module*/
pub type BGCLUT173 = crate::Reg<bgclut173::BGCLUT173rs>;
///DMA2D background CLUT
pub mod bgclut173;
/**BGCLUT174 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut174::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut174::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT174)

For information about available fields see [`mod@bgclut174`] module*/
pub type BGCLUT174 = crate::Reg<bgclut174::BGCLUT174rs>;
///DMA2D background CLUT
pub mod bgclut174;
/**BGCLUT175 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut175::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut175::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT175)

For information about available fields see [`mod@bgclut175`] module*/
pub type BGCLUT175 = crate::Reg<bgclut175::BGCLUT175rs>;
///DMA2D background CLUT
pub mod bgclut175;
/**BGCLUT176 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut176::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut176::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT176)

For information about available fields see [`mod@bgclut176`] module*/
pub type BGCLUT176 = crate::Reg<bgclut176::BGCLUT176rs>;
///DMA2D background CLUT
pub mod bgclut176;
/**BGCLUT177 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut177::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut177::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT177)

For information about available fields see [`mod@bgclut177`] module*/
pub type BGCLUT177 = crate::Reg<bgclut177::BGCLUT177rs>;
///DMA2D background CLUT
pub mod bgclut177;
/**BGCLUT178 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut178::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut178::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT178)

For information about available fields see [`mod@bgclut178`] module*/
pub type BGCLUT178 = crate::Reg<bgclut178::BGCLUT178rs>;
///DMA2D background CLUT
pub mod bgclut178;
/**BGCLUT179 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut179::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut179::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT179)

For information about available fields see [`mod@bgclut179`] module*/
pub type BGCLUT179 = crate::Reg<bgclut179::BGCLUT179rs>;
///DMA2D background CLUT
pub mod bgclut179;
/**BGCLUT180 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut180::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut180::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT180)

For information about available fields see [`mod@bgclut180`] module*/
pub type BGCLUT180 = crate::Reg<bgclut180::BGCLUT180rs>;
///DMA2D background CLUT
pub mod bgclut180;
/**BGCLUT181 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut181::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut181::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT181)

For information about available fields see [`mod@bgclut181`] module*/
pub type BGCLUT181 = crate::Reg<bgclut181::BGCLUT181rs>;
///DMA2D background CLUT
pub mod bgclut181;
/**BGCLUT182 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut182::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut182::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT182)

For information about available fields see [`mod@bgclut182`] module*/
pub type BGCLUT182 = crate::Reg<bgclut182::BGCLUT182rs>;
///DMA2D background CLUT
pub mod bgclut182;
/**BGCLUT183 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut183::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut183::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT183)

For information about available fields see [`mod@bgclut183`] module*/
pub type BGCLUT183 = crate::Reg<bgclut183::BGCLUT183rs>;
///DMA2D background CLUT
pub mod bgclut183;
/**BGCLUT184 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut184::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut184::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT184)

For information about available fields see [`mod@bgclut184`] module*/
pub type BGCLUT184 = crate::Reg<bgclut184::BGCLUT184rs>;
///DMA2D background CLUT
pub mod bgclut184;
/**BGCLUT185 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut185::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut185::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT185)

For information about available fields see [`mod@bgclut185`] module*/
pub type BGCLUT185 = crate::Reg<bgclut185::BGCLUT185rs>;
///DMA2D background CLUT
pub mod bgclut185;
/**BGCLUT186 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut186::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut186::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT186)

For information about available fields see [`mod@bgclut186`] module*/
pub type BGCLUT186 = crate::Reg<bgclut186::BGCLUT186rs>;
///DMA2D background CLUT
pub mod bgclut186;
/**BGCLUT187 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut187::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut187::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT187)

For information about available fields see [`mod@bgclut187`] module*/
pub type BGCLUT187 = crate::Reg<bgclut187::BGCLUT187rs>;
///DMA2D background CLUT
pub mod bgclut187;
/**BGCLUT188 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut188::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut188::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT188)

For information about available fields see [`mod@bgclut188`] module*/
pub type BGCLUT188 = crate::Reg<bgclut188::BGCLUT188rs>;
///DMA2D background CLUT
pub mod bgclut188;
/**BGCLUT189 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut189::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut189::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT189)

For information about available fields see [`mod@bgclut189`] module*/
pub type BGCLUT189 = crate::Reg<bgclut189::BGCLUT189rs>;
///DMA2D background CLUT
pub mod bgclut189;
/**BGCLUT190 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut190::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut190::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT190)

For information about available fields see [`mod@bgclut190`] module*/
pub type BGCLUT190 = crate::Reg<bgclut190::BGCLUT190rs>;
///DMA2D background CLUT
pub mod bgclut190;
/**BGCLUT191 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut191::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut191::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT191)

For information about available fields see [`mod@bgclut191`] module*/
pub type BGCLUT191 = crate::Reg<bgclut191::BGCLUT191rs>;
///DMA2D background CLUT
pub mod bgclut191;
/**BGCLUT192 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut192::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut192::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT192)

For information about available fields see [`mod@bgclut192`] module*/
pub type BGCLUT192 = crate::Reg<bgclut192::BGCLUT192rs>;
///DMA2D background CLUT
pub mod bgclut192;
/**BGCLUT193 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut193::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut193::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT193)

For information about available fields see [`mod@bgclut193`] module*/
pub type BGCLUT193 = crate::Reg<bgclut193::BGCLUT193rs>;
///DMA2D background CLUT
pub mod bgclut193;
/**BGCLUT194 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut194::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut194::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT194)

For information about available fields see [`mod@bgclut194`] module*/
pub type BGCLUT194 = crate::Reg<bgclut194::BGCLUT194rs>;
///DMA2D background CLUT
pub mod bgclut194;
/**BGCLUT195 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut195::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut195::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT195)

For information about available fields see [`mod@bgclut195`] module*/
pub type BGCLUT195 = crate::Reg<bgclut195::BGCLUT195rs>;
///DMA2D background CLUT
pub mod bgclut195;
/**BGCLUT196 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut196::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut196::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT196)

For information about available fields see [`mod@bgclut196`] module*/
pub type BGCLUT196 = crate::Reg<bgclut196::BGCLUT196rs>;
///DMA2D background CLUT
pub mod bgclut196;
/**BGCLUT197 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut197::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut197::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT197)

For information about available fields see [`mod@bgclut197`] module*/
pub type BGCLUT197 = crate::Reg<bgclut197::BGCLUT197rs>;
///DMA2D background CLUT
pub mod bgclut197;
/**BGCLUT198 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut198::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut198::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT198)

For information about available fields see [`mod@bgclut198`] module*/
pub type BGCLUT198 = crate::Reg<bgclut198::BGCLUT198rs>;
///DMA2D background CLUT
pub mod bgclut198;
/**BGCLUT199 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut199::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut199::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT199)

For information about available fields see [`mod@bgclut199`] module*/
pub type BGCLUT199 = crate::Reg<bgclut199::BGCLUT199rs>;
///DMA2D background CLUT
pub mod bgclut199;
/**BGCLUT200 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut200::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut200::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT200)

For information about available fields see [`mod@bgclut200`] module*/
pub type BGCLUT200 = crate::Reg<bgclut200::BGCLUT200rs>;
///DMA2D background CLUT
pub mod bgclut200;
/**BGCLUT201 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut201::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut201::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT201)

For information about available fields see [`mod@bgclut201`] module*/
pub type BGCLUT201 = crate::Reg<bgclut201::BGCLUT201rs>;
///DMA2D background CLUT
pub mod bgclut201;
/**BGCLUT202 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut202::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut202::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT202)

For information about available fields see [`mod@bgclut202`] module*/
pub type BGCLUT202 = crate::Reg<bgclut202::BGCLUT202rs>;
///DMA2D background CLUT
pub mod bgclut202;
/**BGCLUT203 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut203::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut203::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT203)

For information about available fields see [`mod@bgclut203`] module*/
pub type BGCLUT203 = crate::Reg<bgclut203::BGCLUT203rs>;
///DMA2D background CLUT
pub mod bgclut203;
/**BGCLUT204 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut204::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut204::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT204)

For information about available fields see [`mod@bgclut204`] module*/
pub type BGCLUT204 = crate::Reg<bgclut204::BGCLUT204rs>;
///DMA2D background CLUT
pub mod bgclut204;
/**BGCLUT205 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut205::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut205::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT205)

For information about available fields see [`mod@bgclut205`] module*/
pub type BGCLUT205 = crate::Reg<bgclut205::BGCLUT205rs>;
///DMA2D background CLUT
pub mod bgclut205;
/**BGCLUT206 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut206::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut206::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT206)

For information about available fields see [`mod@bgclut206`] module*/
pub type BGCLUT206 = crate::Reg<bgclut206::BGCLUT206rs>;
///DMA2D background CLUT
pub mod bgclut206;
/**BGCLUT207 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut207::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut207::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT207)

For information about available fields see [`mod@bgclut207`] module*/
pub type BGCLUT207 = crate::Reg<bgclut207::BGCLUT207rs>;
///DMA2D background CLUT
pub mod bgclut207;
/**BGCLUT208 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut208::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut208::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT208)

For information about available fields see [`mod@bgclut208`] module*/
pub type BGCLUT208 = crate::Reg<bgclut208::BGCLUT208rs>;
///DMA2D background CLUT
pub mod bgclut208;
/**BGCLUT209 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut209::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut209::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT209)

For information about available fields see [`mod@bgclut209`] module*/
pub type BGCLUT209 = crate::Reg<bgclut209::BGCLUT209rs>;
///DMA2D background CLUT
pub mod bgclut209;
/**BGCLUT210 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut210::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut210::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT210)

For information about available fields see [`mod@bgclut210`] module*/
pub type BGCLUT210 = crate::Reg<bgclut210::BGCLUT210rs>;
///DMA2D background CLUT
pub mod bgclut210;
/**BGCLUT211 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut211::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut211::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT211)

For information about available fields see [`mod@bgclut211`] module*/
pub type BGCLUT211 = crate::Reg<bgclut211::BGCLUT211rs>;
///DMA2D background CLUT
pub mod bgclut211;
/**BGCLUT212 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut212::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut212::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT212)

For information about available fields see [`mod@bgclut212`] module*/
pub type BGCLUT212 = crate::Reg<bgclut212::BGCLUT212rs>;
///DMA2D background CLUT
pub mod bgclut212;
/**BGCLUT213 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut213::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut213::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT213)

For information about available fields see [`mod@bgclut213`] module*/
pub type BGCLUT213 = crate::Reg<bgclut213::BGCLUT213rs>;
///DMA2D background CLUT
pub mod bgclut213;
/**BGCLUT214 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut214::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut214::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT214)

For information about available fields see [`mod@bgclut214`] module*/
pub type BGCLUT214 = crate::Reg<bgclut214::BGCLUT214rs>;
///DMA2D background CLUT
pub mod bgclut214;
/**BGCLUT215 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut215::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut215::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT215)

For information about available fields see [`mod@bgclut215`] module*/
pub type BGCLUT215 = crate::Reg<bgclut215::BGCLUT215rs>;
///DMA2D background CLUT
pub mod bgclut215;
/**BGCLUT216 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut216::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut216::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT216)

For information about available fields see [`mod@bgclut216`] module*/
pub type BGCLUT216 = crate::Reg<bgclut216::BGCLUT216rs>;
///DMA2D background CLUT
pub mod bgclut216;
/**BGCLUT217 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut217::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut217::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT217)

For information about available fields see [`mod@bgclut217`] module*/
pub type BGCLUT217 = crate::Reg<bgclut217::BGCLUT217rs>;
///DMA2D background CLUT
pub mod bgclut217;
/**BGCLUT218 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut218::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut218::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT218)

For information about available fields see [`mod@bgclut218`] module*/
pub type BGCLUT218 = crate::Reg<bgclut218::BGCLUT218rs>;
///DMA2D background CLUT
pub mod bgclut218;
/**BGCLUT219 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut219::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut219::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT219)

For information about available fields see [`mod@bgclut219`] module*/
pub type BGCLUT219 = crate::Reg<bgclut219::BGCLUT219rs>;
///DMA2D background CLUT
pub mod bgclut219;
/**BGCLUT220 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut220::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut220::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT220)

For information about available fields see [`mod@bgclut220`] module*/
pub type BGCLUT220 = crate::Reg<bgclut220::BGCLUT220rs>;
///DMA2D background CLUT
pub mod bgclut220;
/**BGCLUT221 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut221::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut221::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT221)

For information about available fields see [`mod@bgclut221`] module*/
pub type BGCLUT221 = crate::Reg<bgclut221::BGCLUT221rs>;
///DMA2D background CLUT
pub mod bgclut221;
/**BGCLUT222 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut222::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut222::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT222)

For information about available fields see [`mod@bgclut222`] module*/
pub type BGCLUT222 = crate::Reg<bgclut222::BGCLUT222rs>;
///DMA2D background CLUT
pub mod bgclut222;
/**BGCLUT223 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut223::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut223::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT223)

For information about available fields see [`mod@bgclut223`] module*/
pub type BGCLUT223 = crate::Reg<bgclut223::BGCLUT223rs>;
///DMA2D background CLUT
pub mod bgclut223;
/**BGCLUT224 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut224::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut224::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT224)

For information about available fields see [`mod@bgclut224`] module*/
pub type BGCLUT224 = crate::Reg<bgclut224::BGCLUT224rs>;
///DMA2D background CLUT
pub mod bgclut224;
/**BGCLUT225 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut225::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut225::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT225)

For information about available fields see [`mod@bgclut225`] module*/
pub type BGCLUT225 = crate::Reg<bgclut225::BGCLUT225rs>;
///DMA2D background CLUT
pub mod bgclut225;
/**BGCLUT226 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut226::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut226::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT226)

For information about available fields see [`mod@bgclut226`] module*/
pub type BGCLUT226 = crate::Reg<bgclut226::BGCLUT226rs>;
///DMA2D background CLUT
pub mod bgclut226;
/**BGCLUT227 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut227::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut227::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT227)

For information about available fields see [`mod@bgclut227`] module*/
pub type BGCLUT227 = crate::Reg<bgclut227::BGCLUT227rs>;
///DMA2D background CLUT
pub mod bgclut227;
/**BGCLUT228 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut228::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut228::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT228)

For information about available fields see [`mod@bgclut228`] module*/
pub type BGCLUT228 = crate::Reg<bgclut228::BGCLUT228rs>;
///DMA2D background CLUT
pub mod bgclut228;
/**BGCLUT229 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut229::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut229::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT229)

For information about available fields see [`mod@bgclut229`] module*/
pub type BGCLUT229 = crate::Reg<bgclut229::BGCLUT229rs>;
///DMA2D background CLUT
pub mod bgclut229;
/**BGCLUT230 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut230::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut230::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT230)

For information about available fields see [`mod@bgclut230`] module*/
pub type BGCLUT230 = crate::Reg<bgclut230::BGCLUT230rs>;
///DMA2D background CLUT
pub mod bgclut230;
/**BGCLUT231 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut231::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut231::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT231)

For information about available fields see [`mod@bgclut231`] module*/
pub type BGCLUT231 = crate::Reg<bgclut231::BGCLUT231rs>;
///DMA2D background CLUT
pub mod bgclut231;
/**BGCLUT232 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut232::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut232::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT232)

For information about available fields see [`mod@bgclut232`] module*/
pub type BGCLUT232 = crate::Reg<bgclut232::BGCLUT232rs>;
///DMA2D background CLUT
pub mod bgclut232;
/**BGCLUT233 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut233::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut233::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT233)

For information about available fields see [`mod@bgclut233`] module*/
pub type BGCLUT233 = crate::Reg<bgclut233::BGCLUT233rs>;
///DMA2D background CLUT
pub mod bgclut233;
/**BGCLUT234 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut234::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut234::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT234)

For information about available fields see [`mod@bgclut234`] module*/
pub type BGCLUT234 = crate::Reg<bgclut234::BGCLUT234rs>;
///DMA2D background CLUT
pub mod bgclut234;
/**BGCLUT235 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut235::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut235::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT235)

For information about available fields see [`mod@bgclut235`] module*/
pub type BGCLUT235 = crate::Reg<bgclut235::BGCLUT235rs>;
///DMA2D background CLUT
pub mod bgclut235;
/**BGCLUT236 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut236::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut236::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT236)

For information about available fields see [`mod@bgclut236`] module*/
pub type BGCLUT236 = crate::Reg<bgclut236::BGCLUT236rs>;
///DMA2D background CLUT
pub mod bgclut236;
/**BGCLUT237 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut237::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut237::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT237)

For information about available fields see [`mod@bgclut237`] module*/
pub type BGCLUT237 = crate::Reg<bgclut237::BGCLUT237rs>;
///DMA2D background CLUT
pub mod bgclut237;
/**BGCLUT238 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut238::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut238::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT238)

For information about available fields see [`mod@bgclut238`] module*/
pub type BGCLUT238 = crate::Reg<bgclut238::BGCLUT238rs>;
///DMA2D background CLUT
pub mod bgclut238;
/**BGCLUT239 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut239::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut239::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT239)

For information about available fields see [`mod@bgclut239`] module*/
pub type BGCLUT239 = crate::Reg<bgclut239::BGCLUT239rs>;
///DMA2D background CLUT
pub mod bgclut239;
/**BGCLUT240 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut240::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut240::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT240)

For information about available fields see [`mod@bgclut240`] module*/
pub type BGCLUT240 = crate::Reg<bgclut240::BGCLUT240rs>;
///DMA2D background CLUT
pub mod bgclut240;
/**BGCLUT241 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut241::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut241::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT241)

For information about available fields see [`mod@bgclut241`] module*/
pub type BGCLUT241 = crate::Reg<bgclut241::BGCLUT241rs>;
///DMA2D background CLUT
pub mod bgclut241;
/**BGCLUT242 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut242::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut242::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT242)

For information about available fields see [`mod@bgclut242`] module*/
pub type BGCLUT242 = crate::Reg<bgclut242::BGCLUT242rs>;
///DMA2D background CLUT
pub mod bgclut242;
/**BGCLUT243 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut243::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut243::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT243)

For information about available fields see [`mod@bgclut243`] module*/
pub type BGCLUT243 = crate::Reg<bgclut243::BGCLUT243rs>;
///DMA2D background CLUT
pub mod bgclut243;
/**BGCLUT244 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut244::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut244::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT244)

For information about available fields see [`mod@bgclut244`] module*/
pub type BGCLUT244 = crate::Reg<bgclut244::BGCLUT244rs>;
///DMA2D background CLUT
pub mod bgclut244;
/**BGCLUT245 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut245::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut245::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT245)

For information about available fields see [`mod@bgclut245`] module*/
pub type BGCLUT245 = crate::Reg<bgclut245::BGCLUT245rs>;
///DMA2D background CLUT
pub mod bgclut245;
/**BGCLUT246 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut246::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut246::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT246)

For information about available fields see [`mod@bgclut246`] module*/
pub type BGCLUT246 = crate::Reg<bgclut246::BGCLUT246rs>;
///DMA2D background CLUT
pub mod bgclut246;
/**BGCLUT247 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut247::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut247::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT247)

For information about available fields see [`mod@bgclut247`] module*/
pub type BGCLUT247 = crate::Reg<bgclut247::BGCLUT247rs>;
///DMA2D background CLUT
pub mod bgclut247;
/**BGCLUT248 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut248::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut248::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT248)

For information about available fields see [`mod@bgclut248`] module*/
pub type BGCLUT248 = crate::Reg<bgclut248::BGCLUT248rs>;
///DMA2D background CLUT
pub mod bgclut248;
/**BGCLUT249 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut249::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut249::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT249)

For information about available fields see [`mod@bgclut249`] module*/
pub type BGCLUT249 = crate::Reg<bgclut249::BGCLUT249rs>;
///DMA2D background CLUT
pub mod bgclut249;
/**BGCLUT250 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut250::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut250::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT250)

For information about available fields see [`mod@bgclut250`] module*/
pub type BGCLUT250 = crate::Reg<bgclut250::BGCLUT250rs>;
///DMA2D background CLUT
pub mod bgclut250;
/**BGCLUT251 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut251::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut251::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT251)

For information about available fields see [`mod@bgclut251`] module*/
pub type BGCLUT251 = crate::Reg<bgclut251::BGCLUT251rs>;
///DMA2D background CLUT
pub mod bgclut251;
/**BGCLUT252 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut252::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut252::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT252)

For information about available fields see [`mod@bgclut252`] module*/
pub type BGCLUT252 = crate::Reg<bgclut252::BGCLUT252rs>;
///DMA2D background CLUT
pub mod bgclut252;
/**BGCLUT253 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut253::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut253::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT253)

For information about available fields see [`mod@bgclut253`] module*/
pub type BGCLUT253 = crate::Reg<bgclut253::BGCLUT253rs>;
///DMA2D background CLUT
pub mod bgclut253;
/**BGCLUT254 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut254::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut254::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT254)

For information about available fields see [`mod@bgclut254`] module*/
pub type BGCLUT254 = crate::Reg<bgclut254::BGCLUT254rs>;
///DMA2D background CLUT
pub mod bgclut254;
/**BGCLUT255 (rw) register accessor: DMA2D background CLUT

You can [`read`](crate::Reg::read) this register and get [`bgclut255::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgclut255::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DMA2D:BGCLUT255)

For information about available fields see [`mod@bgclut255`] module*/
pub type BGCLUT255 = crate::Reg<bgclut255::BGCLUT255rs>;
///DMA2D background CLUT
pub mod bgclut255;
