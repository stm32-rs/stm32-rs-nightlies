#[doc = "Reader of register PLL1DIVR"]
pub type R = crate::R<u32, super::PLL1DIVR>;
#[doc = "Writer for register PLL1DIVR"]
pub type W = crate::W<u32, super::PLL1DIVR>;
#[doc = "Register PLL1DIVR `reset()`'s with value 0x0101_0280"]
impl crate::ResetValue for super::PLL1DIVR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0x0101_0280
    }
}
#[doc = "Reader of field `DIVN1`"]
pub type DIVN1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `DIVN1`"]
pub struct DIVN1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVN1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
#[doc = "PLL1 DIVP division factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DIVP1_A {
    #[doc = "0: pll_p_ck = vco_ck"]
    DIV1 = 0,
    #[doc = "1: pll_p_ck = vco_ck / 2"]
    DIV2 = 1,
    #[doc = "3: pll_p_ck = vco_ck / 4"]
    DIV4 = 3,
    #[doc = "5: pll_p_ck = vco_ck / 6"]
    DIV6 = 5,
    #[doc = "7: pll_p_ck = vco_ck / 8"]
    DIV8 = 7,
    #[doc = "9: pll_p_ck = vco_ck / 10"]
    DIV10 = 9,
    #[doc = "11: pll_p_ck = vco_ck / 12"]
    DIV12 = 11,
    #[doc = "13: pll_p_ck = vco_ck / 14"]
    DIV14 = 13,
    #[doc = "15: pll_p_ck = vco_ck / 16"]
    DIV16 = 15,
    #[doc = "17: pll_p_ck = vco_ck / 18"]
    DIV18 = 17,
    #[doc = "19: pll_p_ck = vco_ck / 20"]
    DIV20 = 19,
    #[doc = "21: pll_p_ck = vco_ck / 22"]
    DIV22 = 21,
    #[doc = "23: pll_p_ck = vco_ck / 24"]
    DIV24 = 23,
    #[doc = "25: pll_p_ck = vco_ck / 26"]
    DIV26 = 25,
    #[doc = "27: pll_p_ck = vco_ck / 28"]
    DIV28 = 27,
    #[doc = "29: pll_p_ck = vco_ck / 30"]
    DIV30 = 29,
    #[doc = "31: pll_p_ck = vco_ck / 32"]
    DIV32 = 31,
    #[doc = "33: pll_p_ck = vco_ck / 34"]
    DIV34 = 33,
    #[doc = "35: pll_p_ck = vco_ck / 36"]
    DIV36 = 35,
    #[doc = "37: pll_p_ck = vco_ck / 38"]
    DIV38 = 37,
    #[doc = "39: pll_p_ck = vco_ck / 40"]
    DIV40 = 39,
    #[doc = "41: pll_p_ck = vco_ck / 42"]
    DIV42 = 41,
    #[doc = "43: pll_p_ck = vco_ck / 44"]
    DIV44 = 43,
    #[doc = "45: pll_p_ck = vco_ck / 46"]
    DIV46 = 45,
    #[doc = "47: pll_p_ck = vco_ck / 48"]
    DIV48 = 47,
    #[doc = "49: pll_p_ck = vco_ck / 50"]
    DIV50 = 49,
    #[doc = "51: pll_p_ck = vco_ck / 52"]
    DIV52 = 51,
    #[doc = "53: pll_p_ck = vco_ck / 54"]
    DIV54 = 53,
    #[doc = "55: pll_p_ck = vco_ck / 56"]
    DIV56 = 55,
    #[doc = "57: pll_p_ck = vco_ck / 58"]
    DIV58 = 57,
    #[doc = "59: pll_p_ck = vco_ck / 60"]
    DIV60 = 59,
    #[doc = "61: pll_p_ck = vco_ck / 62"]
    DIV62 = 61,
    #[doc = "63: pll_p_ck = vco_ck / 64"]
    DIV64 = 63,
    #[doc = "65: pll_p_ck = vco_ck / 66"]
    DIV66 = 65,
    #[doc = "67: pll_p_ck = vco_ck / 68"]
    DIV68 = 67,
    #[doc = "69: pll_p_ck = vco_ck / 70"]
    DIV70 = 69,
    #[doc = "71: pll_p_ck = vco_ck / 72"]
    DIV72 = 71,
    #[doc = "73: pll_p_ck = vco_ck / 74"]
    DIV74 = 73,
    #[doc = "75: pll_p_ck = vco_ck / 76"]
    DIV76 = 75,
    #[doc = "77: pll_p_ck = vco_ck / 78"]
    DIV78 = 77,
    #[doc = "79: pll_p_ck = vco_ck / 80"]
    DIV80 = 79,
    #[doc = "81: pll_p_ck = vco_ck / 82"]
    DIV82 = 81,
    #[doc = "83: pll_p_ck = vco_ck / 84"]
    DIV84 = 83,
    #[doc = "85: pll_p_ck = vco_ck / 86"]
    DIV86 = 85,
    #[doc = "87: pll_p_ck = vco_ck / 88"]
    DIV88 = 87,
    #[doc = "89: pll_p_ck = vco_ck / 90"]
    DIV90 = 89,
    #[doc = "91: pll_p_ck = vco_ck / 92"]
    DIV92 = 91,
    #[doc = "93: pll_p_ck = vco_ck / 94"]
    DIV94 = 93,
    #[doc = "95: pll_p_ck = vco_ck / 96"]
    DIV96 = 95,
    #[doc = "97: pll_p_ck = vco_ck / 98"]
    DIV98 = 97,
    #[doc = "99: pll_p_ck = vco_ck / 100"]
    DIV100 = 99,
    #[doc = "101: pll_p_ck = vco_ck / 102"]
    DIV102 = 101,
    #[doc = "103: pll_p_ck = vco_ck / 104"]
    DIV104 = 103,
    #[doc = "105: pll_p_ck = vco_ck / 106"]
    DIV106 = 105,
    #[doc = "107: pll_p_ck = vco_ck / 108"]
    DIV108 = 107,
    #[doc = "109: pll_p_ck = vco_ck / 110"]
    DIV110 = 109,
    #[doc = "111: pll_p_ck = vco_ck / 112"]
    DIV112 = 111,
    #[doc = "113: pll_p_ck = vco_ck / 114"]
    DIV114 = 113,
    #[doc = "115: pll_p_ck = vco_ck / 116"]
    DIV116 = 115,
    #[doc = "117: pll_p_ck = vco_ck / 118"]
    DIV118 = 117,
    #[doc = "119: pll_p_ck = vco_ck / 120"]
    DIV120 = 119,
    #[doc = "121: pll_p_ck = vco_ck / 122"]
    DIV122 = 121,
    #[doc = "123: pll_p_ck = vco_ck / 124"]
    DIV124 = 123,
    #[doc = "125: pll_p_ck = vco_ck / 126"]
    DIV126 = 125,
    #[doc = "127: pll_p_ck = vco_ck / 128"]
    DIV128 = 127,
}
impl From<DIVP1_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVP1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `DIVP1`"]
pub type DIVP1_R = crate::R<u8, DIVP1_A>;
impl DIVP1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, DIVP1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(DIVP1_A::DIV1),
            1 => Val(DIVP1_A::DIV2),
            3 => Val(DIVP1_A::DIV4),
            5 => Val(DIVP1_A::DIV6),
            7 => Val(DIVP1_A::DIV8),
            9 => Val(DIVP1_A::DIV10),
            11 => Val(DIVP1_A::DIV12),
            13 => Val(DIVP1_A::DIV14),
            15 => Val(DIVP1_A::DIV16),
            17 => Val(DIVP1_A::DIV18),
            19 => Val(DIVP1_A::DIV20),
            21 => Val(DIVP1_A::DIV22),
            23 => Val(DIVP1_A::DIV24),
            25 => Val(DIVP1_A::DIV26),
            27 => Val(DIVP1_A::DIV28),
            29 => Val(DIVP1_A::DIV30),
            31 => Val(DIVP1_A::DIV32),
            33 => Val(DIVP1_A::DIV34),
            35 => Val(DIVP1_A::DIV36),
            37 => Val(DIVP1_A::DIV38),
            39 => Val(DIVP1_A::DIV40),
            41 => Val(DIVP1_A::DIV42),
            43 => Val(DIVP1_A::DIV44),
            45 => Val(DIVP1_A::DIV46),
            47 => Val(DIVP1_A::DIV48),
            49 => Val(DIVP1_A::DIV50),
            51 => Val(DIVP1_A::DIV52),
            53 => Val(DIVP1_A::DIV54),
            55 => Val(DIVP1_A::DIV56),
            57 => Val(DIVP1_A::DIV58),
            59 => Val(DIVP1_A::DIV60),
            61 => Val(DIVP1_A::DIV62),
            63 => Val(DIVP1_A::DIV64),
            65 => Val(DIVP1_A::DIV66),
            67 => Val(DIVP1_A::DIV68),
            69 => Val(DIVP1_A::DIV70),
            71 => Val(DIVP1_A::DIV72),
            73 => Val(DIVP1_A::DIV74),
            75 => Val(DIVP1_A::DIV76),
            77 => Val(DIVP1_A::DIV78),
            79 => Val(DIVP1_A::DIV80),
            81 => Val(DIVP1_A::DIV82),
            83 => Val(DIVP1_A::DIV84),
            85 => Val(DIVP1_A::DIV86),
            87 => Val(DIVP1_A::DIV88),
            89 => Val(DIVP1_A::DIV90),
            91 => Val(DIVP1_A::DIV92),
            93 => Val(DIVP1_A::DIV94),
            95 => Val(DIVP1_A::DIV96),
            97 => Val(DIVP1_A::DIV98),
            99 => Val(DIVP1_A::DIV100),
            101 => Val(DIVP1_A::DIV102),
            103 => Val(DIVP1_A::DIV104),
            105 => Val(DIVP1_A::DIV106),
            107 => Val(DIVP1_A::DIV108),
            109 => Val(DIVP1_A::DIV110),
            111 => Val(DIVP1_A::DIV112),
            113 => Val(DIVP1_A::DIV114),
            115 => Val(DIVP1_A::DIV116),
            117 => Val(DIVP1_A::DIV118),
            119 => Val(DIVP1_A::DIV120),
            121 => Val(DIVP1_A::DIV122),
            123 => Val(DIVP1_A::DIV124),
            125 => Val(DIVP1_A::DIV126),
            127 => Val(DIVP1_A::DIV128),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVP1_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIVP1_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIVP1_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV6`"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == DIVP1_A::DIV6
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIVP1_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV10`"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == DIVP1_A::DIV10
    }
    #[doc = "Checks if the value of the field is `DIV12`"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == DIVP1_A::DIV12
    }
    #[doc = "Checks if the value of the field is `DIV14`"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == DIVP1_A::DIV14
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DIVP1_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV18`"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == DIVP1_A::DIV18
    }
    #[doc = "Checks if the value of the field is `DIV20`"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == DIVP1_A::DIV20
    }
    #[doc = "Checks if the value of the field is `DIV22`"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == DIVP1_A::DIV22
    }
    #[doc = "Checks if the value of the field is `DIV24`"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == DIVP1_A::DIV24
    }
    #[doc = "Checks if the value of the field is `DIV26`"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == DIVP1_A::DIV26
    }
    #[doc = "Checks if the value of the field is `DIV28`"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == DIVP1_A::DIV28
    }
    #[doc = "Checks if the value of the field is `DIV30`"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == DIVP1_A::DIV30
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DIVP1_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV34`"]
    #[inline(always)]
    pub fn is_div34(&self) -> bool {
        *self == DIVP1_A::DIV34
    }
    #[doc = "Checks if the value of the field is `DIV36`"]
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        *self == DIVP1_A::DIV36
    }
    #[doc = "Checks if the value of the field is `DIV38`"]
    #[inline(always)]
    pub fn is_div38(&self) -> bool {
        *self == DIVP1_A::DIV38
    }
    #[doc = "Checks if the value of the field is `DIV40`"]
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == DIVP1_A::DIV40
    }
    #[doc = "Checks if the value of the field is `DIV42`"]
    #[inline(always)]
    pub fn is_div42(&self) -> bool {
        *self == DIVP1_A::DIV42
    }
    #[doc = "Checks if the value of the field is `DIV44`"]
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        *self == DIVP1_A::DIV44
    }
    #[doc = "Checks if the value of the field is `DIV46`"]
    #[inline(always)]
    pub fn is_div46(&self) -> bool {
        *self == DIVP1_A::DIV46
    }
    #[doc = "Checks if the value of the field is `DIV48`"]
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == DIVP1_A::DIV48
    }
    #[doc = "Checks if the value of the field is `DIV50`"]
    #[inline(always)]
    pub fn is_div50(&self) -> bool {
        *self == DIVP1_A::DIV50
    }
    #[doc = "Checks if the value of the field is `DIV52`"]
    #[inline(always)]
    pub fn is_div52(&self) -> bool {
        *self == DIVP1_A::DIV52
    }
    #[doc = "Checks if the value of the field is `DIV54`"]
    #[inline(always)]
    pub fn is_div54(&self) -> bool {
        *self == DIVP1_A::DIV54
    }
    #[doc = "Checks if the value of the field is `DIV56`"]
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        *self == DIVP1_A::DIV56
    }
    #[doc = "Checks if the value of the field is `DIV58`"]
    #[inline(always)]
    pub fn is_div58(&self) -> bool {
        *self == DIVP1_A::DIV58
    }
    #[doc = "Checks if the value of the field is `DIV60`"]
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        *self == DIVP1_A::DIV60
    }
    #[doc = "Checks if the value of the field is `DIV62`"]
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        *self == DIVP1_A::DIV62
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DIVP1_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV66`"]
    #[inline(always)]
    pub fn is_div66(&self) -> bool {
        *self == DIVP1_A::DIV66
    }
    #[doc = "Checks if the value of the field is `DIV68`"]
    #[inline(always)]
    pub fn is_div68(&self) -> bool {
        *self == DIVP1_A::DIV68
    }
    #[doc = "Checks if the value of the field is `DIV70`"]
    #[inline(always)]
    pub fn is_div70(&self) -> bool {
        *self == DIVP1_A::DIV70
    }
    #[doc = "Checks if the value of the field is `DIV72`"]
    #[inline(always)]
    pub fn is_div72(&self) -> bool {
        *self == DIVP1_A::DIV72
    }
    #[doc = "Checks if the value of the field is `DIV74`"]
    #[inline(always)]
    pub fn is_div74(&self) -> bool {
        *self == DIVP1_A::DIV74
    }
    #[doc = "Checks if the value of the field is `DIV76`"]
    #[inline(always)]
    pub fn is_div76(&self) -> bool {
        *self == DIVP1_A::DIV76
    }
    #[doc = "Checks if the value of the field is `DIV78`"]
    #[inline(always)]
    pub fn is_div78(&self) -> bool {
        *self == DIVP1_A::DIV78
    }
    #[doc = "Checks if the value of the field is `DIV80`"]
    #[inline(always)]
    pub fn is_div80(&self) -> bool {
        *self == DIVP1_A::DIV80
    }
    #[doc = "Checks if the value of the field is `DIV82`"]
    #[inline(always)]
    pub fn is_div82(&self) -> bool {
        *self == DIVP1_A::DIV82
    }
    #[doc = "Checks if the value of the field is `DIV84`"]
    #[inline(always)]
    pub fn is_div84(&self) -> bool {
        *self == DIVP1_A::DIV84
    }
    #[doc = "Checks if the value of the field is `DIV86`"]
    #[inline(always)]
    pub fn is_div86(&self) -> bool {
        *self == DIVP1_A::DIV86
    }
    #[doc = "Checks if the value of the field is `DIV88`"]
    #[inline(always)]
    pub fn is_div88(&self) -> bool {
        *self == DIVP1_A::DIV88
    }
    #[doc = "Checks if the value of the field is `DIV90`"]
    #[inline(always)]
    pub fn is_div90(&self) -> bool {
        *self == DIVP1_A::DIV90
    }
    #[doc = "Checks if the value of the field is `DIV92`"]
    #[inline(always)]
    pub fn is_div92(&self) -> bool {
        *self == DIVP1_A::DIV92
    }
    #[doc = "Checks if the value of the field is `DIV94`"]
    #[inline(always)]
    pub fn is_div94(&self) -> bool {
        *self == DIVP1_A::DIV94
    }
    #[doc = "Checks if the value of the field is `DIV96`"]
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == DIVP1_A::DIV96
    }
    #[doc = "Checks if the value of the field is `DIV98`"]
    #[inline(always)]
    pub fn is_div98(&self) -> bool {
        *self == DIVP1_A::DIV98
    }
    #[doc = "Checks if the value of the field is `DIV100`"]
    #[inline(always)]
    pub fn is_div100(&self) -> bool {
        *self == DIVP1_A::DIV100
    }
    #[doc = "Checks if the value of the field is `DIV102`"]
    #[inline(always)]
    pub fn is_div102(&self) -> bool {
        *self == DIVP1_A::DIV102
    }
    #[doc = "Checks if the value of the field is `DIV104`"]
    #[inline(always)]
    pub fn is_div104(&self) -> bool {
        *self == DIVP1_A::DIV104
    }
    #[doc = "Checks if the value of the field is `DIV106`"]
    #[inline(always)]
    pub fn is_div106(&self) -> bool {
        *self == DIVP1_A::DIV106
    }
    #[doc = "Checks if the value of the field is `DIV108`"]
    #[inline(always)]
    pub fn is_div108(&self) -> bool {
        *self == DIVP1_A::DIV108
    }
    #[doc = "Checks if the value of the field is `DIV110`"]
    #[inline(always)]
    pub fn is_div110(&self) -> bool {
        *self == DIVP1_A::DIV110
    }
    #[doc = "Checks if the value of the field is `DIV112`"]
    #[inline(always)]
    pub fn is_div112(&self) -> bool {
        *self == DIVP1_A::DIV112
    }
    #[doc = "Checks if the value of the field is `DIV114`"]
    #[inline(always)]
    pub fn is_div114(&self) -> bool {
        *self == DIVP1_A::DIV114
    }
    #[doc = "Checks if the value of the field is `DIV116`"]
    #[inline(always)]
    pub fn is_div116(&self) -> bool {
        *self == DIVP1_A::DIV116
    }
    #[doc = "Checks if the value of the field is `DIV118`"]
    #[inline(always)]
    pub fn is_div118(&self) -> bool {
        *self == DIVP1_A::DIV118
    }
    #[doc = "Checks if the value of the field is `DIV120`"]
    #[inline(always)]
    pub fn is_div120(&self) -> bool {
        *self == DIVP1_A::DIV120
    }
    #[doc = "Checks if the value of the field is `DIV122`"]
    #[inline(always)]
    pub fn is_div122(&self) -> bool {
        *self == DIVP1_A::DIV122
    }
    #[doc = "Checks if the value of the field is `DIV124`"]
    #[inline(always)]
    pub fn is_div124(&self) -> bool {
        *self == DIVP1_A::DIV124
    }
    #[doc = "Checks if the value of the field is `DIV126`"]
    #[inline(always)]
    pub fn is_div126(&self) -> bool {
        *self == DIVP1_A::DIV126
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DIVP1_A::DIV128
    }
}
#[doc = "Write proxy for field `DIVP1`"]
pub struct DIVP1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVP1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DIVP1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll_p_ck = vco_ck"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV1)
    }
    #[doc = "pll_p_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV2)
    }
    #[doc = "pll_p_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV4)
    }
    #[doc = "pll_p_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV6)
    }
    #[doc = "pll_p_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV8)
    }
    #[doc = "pll_p_ck = vco_ck / 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV10)
    }
    #[doc = "pll_p_ck = vco_ck / 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV12)
    }
    #[doc = "pll_p_ck = vco_ck / 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV14)
    }
    #[doc = "pll_p_ck = vco_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV16)
    }
    #[doc = "pll_p_ck = vco_ck / 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV18)
    }
    #[doc = "pll_p_ck = vco_ck / 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV20)
    }
    #[doc = "pll_p_ck = vco_ck / 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV22)
    }
    #[doc = "pll_p_ck = vco_ck / 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV24)
    }
    #[doc = "pll_p_ck = vco_ck / 26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV26)
    }
    #[doc = "pll_p_ck = vco_ck / 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV28)
    }
    #[doc = "pll_p_ck = vco_ck / 30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV30)
    }
    #[doc = "pll_p_ck = vco_ck / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV32)
    }
    #[doc = "pll_p_ck = vco_ck / 34"]
    #[inline(always)]
    pub fn div34(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV34)
    }
    #[doc = "pll_p_ck = vco_ck / 36"]
    #[inline(always)]
    pub fn div36(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV36)
    }
    #[doc = "pll_p_ck = vco_ck / 38"]
    #[inline(always)]
    pub fn div38(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV38)
    }
    #[doc = "pll_p_ck = vco_ck / 40"]
    #[inline(always)]
    pub fn div40(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV40)
    }
    #[doc = "pll_p_ck = vco_ck / 42"]
    #[inline(always)]
    pub fn div42(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV42)
    }
    #[doc = "pll_p_ck = vco_ck / 44"]
    #[inline(always)]
    pub fn div44(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV44)
    }
    #[doc = "pll_p_ck = vco_ck / 46"]
    #[inline(always)]
    pub fn div46(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV46)
    }
    #[doc = "pll_p_ck = vco_ck / 48"]
    #[inline(always)]
    pub fn div48(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV48)
    }
    #[doc = "pll_p_ck = vco_ck / 50"]
    #[inline(always)]
    pub fn div50(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV50)
    }
    #[doc = "pll_p_ck = vco_ck / 52"]
    #[inline(always)]
    pub fn div52(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV52)
    }
    #[doc = "pll_p_ck = vco_ck / 54"]
    #[inline(always)]
    pub fn div54(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV54)
    }
    #[doc = "pll_p_ck = vco_ck / 56"]
    #[inline(always)]
    pub fn div56(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV56)
    }
    #[doc = "pll_p_ck = vco_ck / 58"]
    #[inline(always)]
    pub fn div58(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV58)
    }
    #[doc = "pll_p_ck = vco_ck / 60"]
    #[inline(always)]
    pub fn div60(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV60)
    }
    #[doc = "pll_p_ck = vco_ck / 62"]
    #[inline(always)]
    pub fn div62(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV62)
    }
    #[doc = "pll_p_ck = vco_ck / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV64)
    }
    #[doc = "pll_p_ck = vco_ck / 66"]
    #[inline(always)]
    pub fn div66(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV66)
    }
    #[doc = "pll_p_ck = vco_ck / 68"]
    #[inline(always)]
    pub fn div68(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV68)
    }
    #[doc = "pll_p_ck = vco_ck / 70"]
    #[inline(always)]
    pub fn div70(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV70)
    }
    #[doc = "pll_p_ck = vco_ck / 72"]
    #[inline(always)]
    pub fn div72(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV72)
    }
    #[doc = "pll_p_ck = vco_ck / 74"]
    #[inline(always)]
    pub fn div74(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV74)
    }
    #[doc = "pll_p_ck = vco_ck / 76"]
    #[inline(always)]
    pub fn div76(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV76)
    }
    #[doc = "pll_p_ck = vco_ck / 78"]
    #[inline(always)]
    pub fn div78(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV78)
    }
    #[doc = "pll_p_ck = vco_ck / 80"]
    #[inline(always)]
    pub fn div80(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV80)
    }
    #[doc = "pll_p_ck = vco_ck / 82"]
    #[inline(always)]
    pub fn div82(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV82)
    }
    #[doc = "pll_p_ck = vco_ck / 84"]
    #[inline(always)]
    pub fn div84(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV84)
    }
    #[doc = "pll_p_ck = vco_ck / 86"]
    #[inline(always)]
    pub fn div86(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV86)
    }
    #[doc = "pll_p_ck = vco_ck / 88"]
    #[inline(always)]
    pub fn div88(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV88)
    }
    #[doc = "pll_p_ck = vco_ck / 90"]
    #[inline(always)]
    pub fn div90(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV90)
    }
    #[doc = "pll_p_ck = vco_ck / 92"]
    #[inline(always)]
    pub fn div92(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV92)
    }
    #[doc = "pll_p_ck = vco_ck / 94"]
    #[inline(always)]
    pub fn div94(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV94)
    }
    #[doc = "pll_p_ck = vco_ck / 96"]
    #[inline(always)]
    pub fn div96(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV96)
    }
    #[doc = "pll_p_ck = vco_ck / 98"]
    #[inline(always)]
    pub fn div98(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV98)
    }
    #[doc = "pll_p_ck = vco_ck / 100"]
    #[inline(always)]
    pub fn div100(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV100)
    }
    #[doc = "pll_p_ck = vco_ck / 102"]
    #[inline(always)]
    pub fn div102(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV102)
    }
    #[doc = "pll_p_ck = vco_ck / 104"]
    #[inline(always)]
    pub fn div104(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV104)
    }
    #[doc = "pll_p_ck = vco_ck / 106"]
    #[inline(always)]
    pub fn div106(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV106)
    }
    #[doc = "pll_p_ck = vco_ck / 108"]
    #[inline(always)]
    pub fn div108(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV108)
    }
    #[doc = "pll_p_ck = vco_ck / 110"]
    #[inline(always)]
    pub fn div110(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV110)
    }
    #[doc = "pll_p_ck = vco_ck / 112"]
    #[inline(always)]
    pub fn div112(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV112)
    }
    #[doc = "pll_p_ck = vco_ck / 114"]
    #[inline(always)]
    pub fn div114(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV114)
    }
    #[doc = "pll_p_ck = vco_ck / 116"]
    #[inline(always)]
    pub fn div116(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV116)
    }
    #[doc = "pll_p_ck = vco_ck / 118"]
    #[inline(always)]
    pub fn div118(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV118)
    }
    #[doc = "pll_p_ck = vco_ck / 120"]
    #[inline(always)]
    pub fn div120(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV120)
    }
    #[doc = "pll_p_ck = vco_ck / 122"]
    #[inline(always)]
    pub fn div122(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV122)
    }
    #[doc = "pll_p_ck = vco_ck / 124"]
    #[inline(always)]
    pub fn div124(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV124)
    }
    #[doc = "pll_p_ck = vco_ck / 126"]
    #[inline(always)]
    pub fn div126(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV126)
    }
    #[doc = "pll_p_ck = vco_ck / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(DIVP1_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 9)) | (((value as u32) & 0x7f) << 9);
        self.w
    }
}
#[doc = "Reader of field `DIVQ1`"]
pub type DIVQ1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVQ1`"]
pub struct DIVQ1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVQ1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 16)) | (((value as u32) & 0x7f) << 16);
        self.w
    }
}
#[doc = "Reader of field `DIVR1`"]
pub type DIVR1_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DIVR1`"]
pub struct DIVR1_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVR1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 24)) | (((value as u32) & 0x7f) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn1(&self) -> DIVN1_R {
        DIVN1_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp1(&self) -> DIVP1_R {
        DIVP1_R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq1(&self) -> DIVQ1_R {
        DIVQ1_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr1(&self) -> DIVR1_R {
        DIVR1_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn1(&mut self) -> DIVN1_W {
        DIVN1_W { w: self }
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp1(&mut self) -> DIVP1_W {
        DIVP1_W { w: self }
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq1(&mut self) -> DIVQ1_W {
        DIVQ1_W { w: self }
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr1(&mut self) -> DIVR1_W {
        DIVR1_W { w: self }
    }
}
