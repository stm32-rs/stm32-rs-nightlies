#[doc = "Register `PLL1DIVR` reader"]
pub type R = crate::R<PLL1DIVRrs>;
#[doc = "Register `PLL1DIVR` writer"]
pub type W = crate::W<PLL1DIVRrs>;
#[doc = "Field `DIVN1` reader - Multiplication factor for PLL1 VCO"]
pub type DIVN1_R = crate::FieldReader<u16>;
#[doc = "Field `DIVN1` writer - Multiplication factor for PLL1 VCO"]
pub type DIVN1_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "PLL1 DIVP division factor\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVP1 {
    #[doc = "0: pll_p_ck = vco_ck"]
    Div1 = 0,
    #[doc = "1: pll_p_ck = vco_ck / 2"]
    Div2 = 1,
    #[doc = "3: pll_p_ck = vco_ck / 4"]
    Div4 = 3,
    #[doc = "5: pll_p_ck = vco_ck / 6"]
    Div6 = 5,
    #[doc = "7: pll_p_ck = vco_ck / 8"]
    Div8 = 7,
    #[doc = "9: pll_p_ck = vco_ck / 10"]
    Div10 = 9,
    #[doc = "11: pll_p_ck = vco_ck / 12"]
    Div12 = 11,
    #[doc = "13: pll_p_ck = vco_ck / 14"]
    Div14 = 13,
    #[doc = "15: pll_p_ck = vco_ck / 16"]
    Div16 = 15,
    #[doc = "17: pll_p_ck = vco_ck / 18"]
    Div18 = 17,
    #[doc = "19: pll_p_ck = vco_ck / 20"]
    Div20 = 19,
    #[doc = "21: pll_p_ck = vco_ck / 22"]
    Div22 = 21,
    #[doc = "23: pll_p_ck = vco_ck / 24"]
    Div24 = 23,
    #[doc = "25: pll_p_ck = vco_ck / 26"]
    Div26 = 25,
    #[doc = "27: pll_p_ck = vco_ck / 28"]
    Div28 = 27,
    #[doc = "29: pll_p_ck = vco_ck / 30"]
    Div30 = 29,
    #[doc = "31: pll_p_ck = vco_ck / 32"]
    Div32 = 31,
    #[doc = "33: pll_p_ck = vco_ck / 34"]
    Div34 = 33,
    #[doc = "35: pll_p_ck = vco_ck / 36"]
    Div36 = 35,
    #[doc = "37: pll_p_ck = vco_ck / 38"]
    Div38 = 37,
    #[doc = "39: pll_p_ck = vco_ck / 40"]
    Div40 = 39,
    #[doc = "41: pll_p_ck = vco_ck / 42"]
    Div42 = 41,
    #[doc = "43: pll_p_ck = vco_ck / 44"]
    Div44 = 43,
    #[doc = "45: pll_p_ck = vco_ck / 46"]
    Div46 = 45,
    #[doc = "47: pll_p_ck = vco_ck / 48"]
    Div48 = 47,
    #[doc = "49: pll_p_ck = vco_ck / 50"]
    Div50 = 49,
    #[doc = "51: pll_p_ck = vco_ck / 52"]
    Div52 = 51,
    #[doc = "53: pll_p_ck = vco_ck / 54"]
    Div54 = 53,
    #[doc = "55: pll_p_ck = vco_ck / 56"]
    Div56 = 55,
    #[doc = "57: pll_p_ck = vco_ck / 58"]
    Div58 = 57,
    #[doc = "59: pll_p_ck = vco_ck / 60"]
    Div60 = 59,
    #[doc = "61: pll_p_ck = vco_ck / 62"]
    Div62 = 61,
    #[doc = "63: pll_p_ck = vco_ck / 64"]
    Div64 = 63,
    #[doc = "65: pll_p_ck = vco_ck / 66"]
    Div66 = 65,
    #[doc = "67: pll_p_ck = vco_ck / 68"]
    Div68 = 67,
    #[doc = "69: pll_p_ck = vco_ck / 70"]
    Div70 = 69,
    #[doc = "71: pll_p_ck = vco_ck / 72"]
    Div72 = 71,
    #[doc = "73: pll_p_ck = vco_ck / 74"]
    Div74 = 73,
    #[doc = "75: pll_p_ck = vco_ck / 76"]
    Div76 = 75,
    #[doc = "77: pll_p_ck = vco_ck / 78"]
    Div78 = 77,
    #[doc = "79: pll_p_ck = vco_ck / 80"]
    Div80 = 79,
    #[doc = "81: pll_p_ck = vco_ck / 82"]
    Div82 = 81,
    #[doc = "83: pll_p_ck = vco_ck / 84"]
    Div84 = 83,
    #[doc = "85: pll_p_ck = vco_ck / 86"]
    Div86 = 85,
    #[doc = "87: pll_p_ck = vco_ck / 88"]
    Div88 = 87,
    #[doc = "89: pll_p_ck = vco_ck / 90"]
    Div90 = 89,
    #[doc = "91: pll_p_ck = vco_ck / 92"]
    Div92 = 91,
    #[doc = "93: pll_p_ck = vco_ck / 94"]
    Div94 = 93,
    #[doc = "95: pll_p_ck = vco_ck / 96"]
    Div96 = 95,
    #[doc = "97: pll_p_ck = vco_ck / 98"]
    Div98 = 97,
    #[doc = "99: pll_p_ck = vco_ck / 100"]
    Div100 = 99,
    #[doc = "101: pll_p_ck = vco_ck / 102"]
    Div102 = 101,
    #[doc = "103: pll_p_ck = vco_ck / 104"]
    Div104 = 103,
    #[doc = "105: pll_p_ck = vco_ck / 106"]
    Div106 = 105,
    #[doc = "107: pll_p_ck = vco_ck / 108"]
    Div108 = 107,
    #[doc = "109: pll_p_ck = vco_ck / 110"]
    Div110 = 109,
    #[doc = "111: pll_p_ck = vco_ck / 112"]
    Div112 = 111,
    #[doc = "113: pll_p_ck = vco_ck / 114"]
    Div114 = 113,
    #[doc = "115: pll_p_ck = vco_ck / 116"]
    Div116 = 115,
    #[doc = "117: pll_p_ck = vco_ck / 118"]
    Div118 = 117,
    #[doc = "119: pll_p_ck = vco_ck / 120"]
    Div120 = 119,
    #[doc = "121: pll_p_ck = vco_ck / 122"]
    Div122 = 121,
    #[doc = "123: pll_p_ck = vco_ck / 124"]
    Div124 = 123,
    #[doc = "125: pll_p_ck = vco_ck / 126"]
    Div126 = 125,
    #[doc = "127: pll_p_ck = vco_ck / 128"]
    Div128 = 127,
}
impl From<DIVP1> for u8 {
    #[inline(always)]
    fn from(variant: DIVP1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIVP1 {
    type Ux = u8;
}
#[doc = "Field `DIVP1` reader - PLL1 DIVP division factor"]
pub type DIVP1_R = crate::FieldReader<DIVP1>;
impl DIVP1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIVP1> {
        match self.bits {
            0 => Some(DIVP1::Div1),
            1 => Some(DIVP1::Div2),
            3 => Some(DIVP1::Div4),
            5 => Some(DIVP1::Div6),
            7 => Some(DIVP1::Div8),
            9 => Some(DIVP1::Div10),
            11 => Some(DIVP1::Div12),
            13 => Some(DIVP1::Div14),
            15 => Some(DIVP1::Div16),
            17 => Some(DIVP1::Div18),
            19 => Some(DIVP1::Div20),
            21 => Some(DIVP1::Div22),
            23 => Some(DIVP1::Div24),
            25 => Some(DIVP1::Div26),
            27 => Some(DIVP1::Div28),
            29 => Some(DIVP1::Div30),
            31 => Some(DIVP1::Div32),
            33 => Some(DIVP1::Div34),
            35 => Some(DIVP1::Div36),
            37 => Some(DIVP1::Div38),
            39 => Some(DIVP1::Div40),
            41 => Some(DIVP1::Div42),
            43 => Some(DIVP1::Div44),
            45 => Some(DIVP1::Div46),
            47 => Some(DIVP1::Div48),
            49 => Some(DIVP1::Div50),
            51 => Some(DIVP1::Div52),
            53 => Some(DIVP1::Div54),
            55 => Some(DIVP1::Div56),
            57 => Some(DIVP1::Div58),
            59 => Some(DIVP1::Div60),
            61 => Some(DIVP1::Div62),
            63 => Some(DIVP1::Div64),
            65 => Some(DIVP1::Div66),
            67 => Some(DIVP1::Div68),
            69 => Some(DIVP1::Div70),
            71 => Some(DIVP1::Div72),
            73 => Some(DIVP1::Div74),
            75 => Some(DIVP1::Div76),
            77 => Some(DIVP1::Div78),
            79 => Some(DIVP1::Div80),
            81 => Some(DIVP1::Div82),
            83 => Some(DIVP1::Div84),
            85 => Some(DIVP1::Div86),
            87 => Some(DIVP1::Div88),
            89 => Some(DIVP1::Div90),
            91 => Some(DIVP1::Div92),
            93 => Some(DIVP1::Div94),
            95 => Some(DIVP1::Div96),
            97 => Some(DIVP1::Div98),
            99 => Some(DIVP1::Div100),
            101 => Some(DIVP1::Div102),
            103 => Some(DIVP1::Div104),
            105 => Some(DIVP1::Div106),
            107 => Some(DIVP1::Div108),
            109 => Some(DIVP1::Div110),
            111 => Some(DIVP1::Div112),
            113 => Some(DIVP1::Div114),
            115 => Some(DIVP1::Div116),
            117 => Some(DIVP1::Div118),
            119 => Some(DIVP1::Div120),
            121 => Some(DIVP1::Div122),
            123 => Some(DIVP1::Div124),
            125 => Some(DIVP1::Div126),
            127 => Some(DIVP1::Div128),
            _ => None,
        }
    }
    #[doc = "pll_p_ck = vco_ck"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == DIVP1::Div1
    }
    #[doc = "pll_p_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == DIVP1::Div2
    }
    #[doc = "pll_p_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == DIVP1::Div4
    }
    #[doc = "pll_p_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == DIVP1::Div6
    }
    #[doc = "pll_p_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == DIVP1::Div8
    }
    #[doc = "pll_p_ck = vco_ck / 10"]
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == DIVP1::Div10
    }
    #[doc = "pll_p_ck = vco_ck / 12"]
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == DIVP1::Div12
    }
    #[doc = "pll_p_ck = vco_ck / 14"]
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == DIVP1::Div14
    }
    #[doc = "pll_p_ck = vco_ck / 16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == DIVP1::Div16
    }
    #[doc = "pll_p_ck = vco_ck / 18"]
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == DIVP1::Div18
    }
    #[doc = "pll_p_ck = vco_ck / 20"]
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == DIVP1::Div20
    }
    #[doc = "pll_p_ck = vco_ck / 22"]
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == DIVP1::Div22
    }
    #[doc = "pll_p_ck = vco_ck / 24"]
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == DIVP1::Div24
    }
    #[doc = "pll_p_ck = vco_ck / 26"]
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == DIVP1::Div26
    }
    #[doc = "pll_p_ck = vco_ck / 28"]
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == DIVP1::Div28
    }
    #[doc = "pll_p_ck = vco_ck / 30"]
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == DIVP1::Div30
    }
    #[doc = "pll_p_ck = vco_ck / 32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == DIVP1::Div32
    }
    #[doc = "pll_p_ck = vco_ck / 34"]
    #[inline(always)]
    pub fn is_div34(&self) -> bool {
        *self == DIVP1::Div34
    }
    #[doc = "pll_p_ck = vco_ck / 36"]
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        *self == DIVP1::Div36
    }
    #[doc = "pll_p_ck = vco_ck / 38"]
    #[inline(always)]
    pub fn is_div38(&self) -> bool {
        *self == DIVP1::Div38
    }
    #[doc = "pll_p_ck = vco_ck / 40"]
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == DIVP1::Div40
    }
    #[doc = "pll_p_ck = vco_ck / 42"]
    #[inline(always)]
    pub fn is_div42(&self) -> bool {
        *self == DIVP1::Div42
    }
    #[doc = "pll_p_ck = vco_ck / 44"]
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        *self == DIVP1::Div44
    }
    #[doc = "pll_p_ck = vco_ck / 46"]
    #[inline(always)]
    pub fn is_div46(&self) -> bool {
        *self == DIVP1::Div46
    }
    #[doc = "pll_p_ck = vco_ck / 48"]
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == DIVP1::Div48
    }
    #[doc = "pll_p_ck = vco_ck / 50"]
    #[inline(always)]
    pub fn is_div50(&self) -> bool {
        *self == DIVP1::Div50
    }
    #[doc = "pll_p_ck = vco_ck / 52"]
    #[inline(always)]
    pub fn is_div52(&self) -> bool {
        *self == DIVP1::Div52
    }
    #[doc = "pll_p_ck = vco_ck / 54"]
    #[inline(always)]
    pub fn is_div54(&self) -> bool {
        *self == DIVP1::Div54
    }
    #[doc = "pll_p_ck = vco_ck / 56"]
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        *self == DIVP1::Div56
    }
    #[doc = "pll_p_ck = vco_ck / 58"]
    #[inline(always)]
    pub fn is_div58(&self) -> bool {
        *self == DIVP1::Div58
    }
    #[doc = "pll_p_ck = vco_ck / 60"]
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        *self == DIVP1::Div60
    }
    #[doc = "pll_p_ck = vco_ck / 62"]
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        *self == DIVP1::Div62
    }
    #[doc = "pll_p_ck = vco_ck / 64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == DIVP1::Div64
    }
    #[doc = "pll_p_ck = vco_ck / 66"]
    #[inline(always)]
    pub fn is_div66(&self) -> bool {
        *self == DIVP1::Div66
    }
    #[doc = "pll_p_ck = vco_ck / 68"]
    #[inline(always)]
    pub fn is_div68(&self) -> bool {
        *self == DIVP1::Div68
    }
    #[doc = "pll_p_ck = vco_ck / 70"]
    #[inline(always)]
    pub fn is_div70(&self) -> bool {
        *self == DIVP1::Div70
    }
    #[doc = "pll_p_ck = vco_ck / 72"]
    #[inline(always)]
    pub fn is_div72(&self) -> bool {
        *self == DIVP1::Div72
    }
    #[doc = "pll_p_ck = vco_ck / 74"]
    #[inline(always)]
    pub fn is_div74(&self) -> bool {
        *self == DIVP1::Div74
    }
    #[doc = "pll_p_ck = vco_ck / 76"]
    #[inline(always)]
    pub fn is_div76(&self) -> bool {
        *self == DIVP1::Div76
    }
    #[doc = "pll_p_ck = vco_ck / 78"]
    #[inline(always)]
    pub fn is_div78(&self) -> bool {
        *self == DIVP1::Div78
    }
    #[doc = "pll_p_ck = vco_ck / 80"]
    #[inline(always)]
    pub fn is_div80(&self) -> bool {
        *self == DIVP1::Div80
    }
    #[doc = "pll_p_ck = vco_ck / 82"]
    #[inline(always)]
    pub fn is_div82(&self) -> bool {
        *self == DIVP1::Div82
    }
    #[doc = "pll_p_ck = vco_ck / 84"]
    #[inline(always)]
    pub fn is_div84(&self) -> bool {
        *self == DIVP1::Div84
    }
    #[doc = "pll_p_ck = vco_ck / 86"]
    #[inline(always)]
    pub fn is_div86(&self) -> bool {
        *self == DIVP1::Div86
    }
    #[doc = "pll_p_ck = vco_ck / 88"]
    #[inline(always)]
    pub fn is_div88(&self) -> bool {
        *self == DIVP1::Div88
    }
    #[doc = "pll_p_ck = vco_ck / 90"]
    #[inline(always)]
    pub fn is_div90(&self) -> bool {
        *self == DIVP1::Div90
    }
    #[doc = "pll_p_ck = vco_ck / 92"]
    #[inline(always)]
    pub fn is_div92(&self) -> bool {
        *self == DIVP1::Div92
    }
    #[doc = "pll_p_ck = vco_ck / 94"]
    #[inline(always)]
    pub fn is_div94(&self) -> bool {
        *self == DIVP1::Div94
    }
    #[doc = "pll_p_ck = vco_ck / 96"]
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == DIVP1::Div96
    }
    #[doc = "pll_p_ck = vco_ck / 98"]
    #[inline(always)]
    pub fn is_div98(&self) -> bool {
        *self == DIVP1::Div98
    }
    #[doc = "pll_p_ck = vco_ck / 100"]
    #[inline(always)]
    pub fn is_div100(&self) -> bool {
        *self == DIVP1::Div100
    }
    #[doc = "pll_p_ck = vco_ck / 102"]
    #[inline(always)]
    pub fn is_div102(&self) -> bool {
        *self == DIVP1::Div102
    }
    #[doc = "pll_p_ck = vco_ck / 104"]
    #[inline(always)]
    pub fn is_div104(&self) -> bool {
        *self == DIVP1::Div104
    }
    #[doc = "pll_p_ck = vco_ck / 106"]
    #[inline(always)]
    pub fn is_div106(&self) -> bool {
        *self == DIVP1::Div106
    }
    #[doc = "pll_p_ck = vco_ck / 108"]
    #[inline(always)]
    pub fn is_div108(&self) -> bool {
        *self == DIVP1::Div108
    }
    #[doc = "pll_p_ck = vco_ck / 110"]
    #[inline(always)]
    pub fn is_div110(&self) -> bool {
        *self == DIVP1::Div110
    }
    #[doc = "pll_p_ck = vco_ck / 112"]
    #[inline(always)]
    pub fn is_div112(&self) -> bool {
        *self == DIVP1::Div112
    }
    #[doc = "pll_p_ck = vco_ck / 114"]
    #[inline(always)]
    pub fn is_div114(&self) -> bool {
        *self == DIVP1::Div114
    }
    #[doc = "pll_p_ck = vco_ck / 116"]
    #[inline(always)]
    pub fn is_div116(&self) -> bool {
        *self == DIVP1::Div116
    }
    #[doc = "pll_p_ck = vco_ck / 118"]
    #[inline(always)]
    pub fn is_div118(&self) -> bool {
        *self == DIVP1::Div118
    }
    #[doc = "pll_p_ck = vco_ck / 120"]
    #[inline(always)]
    pub fn is_div120(&self) -> bool {
        *self == DIVP1::Div120
    }
    #[doc = "pll_p_ck = vco_ck / 122"]
    #[inline(always)]
    pub fn is_div122(&self) -> bool {
        *self == DIVP1::Div122
    }
    #[doc = "pll_p_ck = vco_ck / 124"]
    #[inline(always)]
    pub fn is_div124(&self) -> bool {
        *self == DIVP1::Div124
    }
    #[doc = "pll_p_ck = vco_ck / 126"]
    #[inline(always)]
    pub fn is_div126(&self) -> bool {
        *self == DIVP1::Div126
    }
    #[doc = "pll_p_ck = vco_ck / 128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == DIVP1::Div128
    }
}
#[doc = "Field `DIVP1` writer - PLL1 DIVP division factor"]
pub type DIVP1_W<'a, REG> = crate::FieldWriter<'a, REG, 7, DIVP1>;
impl<'a, REG> DIVP1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pll_p_ck = vco_ck"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div1)
    }
    #[doc = "pll_p_ck = vco_ck / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div2)
    }
    #[doc = "pll_p_ck = vco_ck / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div4)
    }
    #[doc = "pll_p_ck = vco_ck / 6"]
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div6)
    }
    #[doc = "pll_p_ck = vco_ck / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div8)
    }
    #[doc = "pll_p_ck = vco_ck / 10"]
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div10)
    }
    #[doc = "pll_p_ck = vco_ck / 12"]
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div12)
    }
    #[doc = "pll_p_ck = vco_ck / 14"]
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div14)
    }
    #[doc = "pll_p_ck = vco_ck / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div16)
    }
    #[doc = "pll_p_ck = vco_ck / 18"]
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div18)
    }
    #[doc = "pll_p_ck = vco_ck / 20"]
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div20)
    }
    #[doc = "pll_p_ck = vco_ck / 22"]
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div22)
    }
    #[doc = "pll_p_ck = vco_ck / 24"]
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div24)
    }
    #[doc = "pll_p_ck = vco_ck / 26"]
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div26)
    }
    #[doc = "pll_p_ck = vco_ck / 28"]
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div28)
    }
    #[doc = "pll_p_ck = vco_ck / 30"]
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div30)
    }
    #[doc = "pll_p_ck = vco_ck / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div32)
    }
    #[doc = "pll_p_ck = vco_ck / 34"]
    #[inline(always)]
    pub fn div34(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div34)
    }
    #[doc = "pll_p_ck = vco_ck / 36"]
    #[inline(always)]
    pub fn div36(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div36)
    }
    #[doc = "pll_p_ck = vco_ck / 38"]
    #[inline(always)]
    pub fn div38(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div38)
    }
    #[doc = "pll_p_ck = vco_ck / 40"]
    #[inline(always)]
    pub fn div40(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div40)
    }
    #[doc = "pll_p_ck = vco_ck / 42"]
    #[inline(always)]
    pub fn div42(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div42)
    }
    #[doc = "pll_p_ck = vco_ck / 44"]
    #[inline(always)]
    pub fn div44(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div44)
    }
    #[doc = "pll_p_ck = vco_ck / 46"]
    #[inline(always)]
    pub fn div46(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div46)
    }
    #[doc = "pll_p_ck = vco_ck / 48"]
    #[inline(always)]
    pub fn div48(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div48)
    }
    #[doc = "pll_p_ck = vco_ck / 50"]
    #[inline(always)]
    pub fn div50(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div50)
    }
    #[doc = "pll_p_ck = vco_ck / 52"]
    #[inline(always)]
    pub fn div52(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div52)
    }
    #[doc = "pll_p_ck = vco_ck / 54"]
    #[inline(always)]
    pub fn div54(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div54)
    }
    #[doc = "pll_p_ck = vco_ck / 56"]
    #[inline(always)]
    pub fn div56(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div56)
    }
    #[doc = "pll_p_ck = vco_ck / 58"]
    #[inline(always)]
    pub fn div58(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div58)
    }
    #[doc = "pll_p_ck = vco_ck / 60"]
    #[inline(always)]
    pub fn div60(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div60)
    }
    #[doc = "pll_p_ck = vco_ck / 62"]
    #[inline(always)]
    pub fn div62(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div62)
    }
    #[doc = "pll_p_ck = vco_ck / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div64)
    }
    #[doc = "pll_p_ck = vco_ck / 66"]
    #[inline(always)]
    pub fn div66(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div66)
    }
    #[doc = "pll_p_ck = vco_ck / 68"]
    #[inline(always)]
    pub fn div68(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div68)
    }
    #[doc = "pll_p_ck = vco_ck / 70"]
    #[inline(always)]
    pub fn div70(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div70)
    }
    #[doc = "pll_p_ck = vco_ck / 72"]
    #[inline(always)]
    pub fn div72(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div72)
    }
    #[doc = "pll_p_ck = vco_ck / 74"]
    #[inline(always)]
    pub fn div74(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div74)
    }
    #[doc = "pll_p_ck = vco_ck / 76"]
    #[inline(always)]
    pub fn div76(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div76)
    }
    #[doc = "pll_p_ck = vco_ck / 78"]
    #[inline(always)]
    pub fn div78(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div78)
    }
    #[doc = "pll_p_ck = vco_ck / 80"]
    #[inline(always)]
    pub fn div80(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div80)
    }
    #[doc = "pll_p_ck = vco_ck / 82"]
    #[inline(always)]
    pub fn div82(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div82)
    }
    #[doc = "pll_p_ck = vco_ck / 84"]
    #[inline(always)]
    pub fn div84(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div84)
    }
    #[doc = "pll_p_ck = vco_ck / 86"]
    #[inline(always)]
    pub fn div86(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div86)
    }
    #[doc = "pll_p_ck = vco_ck / 88"]
    #[inline(always)]
    pub fn div88(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div88)
    }
    #[doc = "pll_p_ck = vco_ck / 90"]
    #[inline(always)]
    pub fn div90(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div90)
    }
    #[doc = "pll_p_ck = vco_ck / 92"]
    #[inline(always)]
    pub fn div92(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div92)
    }
    #[doc = "pll_p_ck = vco_ck / 94"]
    #[inline(always)]
    pub fn div94(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div94)
    }
    #[doc = "pll_p_ck = vco_ck / 96"]
    #[inline(always)]
    pub fn div96(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div96)
    }
    #[doc = "pll_p_ck = vco_ck / 98"]
    #[inline(always)]
    pub fn div98(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div98)
    }
    #[doc = "pll_p_ck = vco_ck / 100"]
    #[inline(always)]
    pub fn div100(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div100)
    }
    #[doc = "pll_p_ck = vco_ck / 102"]
    #[inline(always)]
    pub fn div102(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div102)
    }
    #[doc = "pll_p_ck = vco_ck / 104"]
    #[inline(always)]
    pub fn div104(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div104)
    }
    #[doc = "pll_p_ck = vco_ck / 106"]
    #[inline(always)]
    pub fn div106(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div106)
    }
    #[doc = "pll_p_ck = vco_ck / 108"]
    #[inline(always)]
    pub fn div108(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div108)
    }
    #[doc = "pll_p_ck = vco_ck / 110"]
    #[inline(always)]
    pub fn div110(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div110)
    }
    #[doc = "pll_p_ck = vco_ck / 112"]
    #[inline(always)]
    pub fn div112(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div112)
    }
    #[doc = "pll_p_ck = vco_ck / 114"]
    #[inline(always)]
    pub fn div114(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div114)
    }
    #[doc = "pll_p_ck = vco_ck / 116"]
    #[inline(always)]
    pub fn div116(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div116)
    }
    #[doc = "pll_p_ck = vco_ck / 118"]
    #[inline(always)]
    pub fn div118(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div118)
    }
    #[doc = "pll_p_ck = vco_ck / 120"]
    #[inline(always)]
    pub fn div120(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div120)
    }
    #[doc = "pll_p_ck = vco_ck / 122"]
    #[inline(always)]
    pub fn div122(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div122)
    }
    #[doc = "pll_p_ck = vco_ck / 124"]
    #[inline(always)]
    pub fn div124(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div124)
    }
    #[doc = "pll_p_ck = vco_ck / 126"]
    #[inline(always)]
    pub fn div126(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div126)
    }
    #[doc = "pll_p_ck = vco_ck / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(DIVP1::Div128)
    }
}
#[doc = "Field `DIVQ1` reader - PLL1 DIVQ division factor"]
pub type DIVQ1_R = crate::FieldReader;
#[doc = "Field `DIVQ1` writer - PLL1 DIVQ division factor"]
pub type DIVQ1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
#[doc = "Field `DIVR1` reader - PLL1 DIVR division factor"]
pub type DIVR1_R = crate::FieldReader;
#[doc = "Field `DIVR1` writer - PLL1 DIVR division factor"]
pub type DIVR1_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 7>;
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
    #[must_use]
    pub fn divn1(&mut self) -> DIVN1_W<PLL1DIVRrs> {
        DIVN1_W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divp1(&mut self) -> DIVP1_W<PLL1DIVRrs> {
        DIVP1_W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divq1(&mut self) -> DIVQ1_W<PLL1DIVRrs> {
        DIVQ1_W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    #[must_use]
    pub fn divr1(&mut self) -> DIVR1_W<PLL1DIVRrs> {
        DIVR1_W::new(self, 24)
    }
}
#[doc = "RCC PLL1 Dividers Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pll1divr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pll1divr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PLL1DIVRrs;
impl crate::RegisterSpec for PLL1DIVRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1divr::R`](R) reader structure"]
impl crate::Readable for PLL1DIVRrs {}
#[doc = "`write(|w| ..)` method takes [`pll1divr::W`](W) writer structure"]
impl crate::Writable for PLL1DIVRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PLL1DIVR to value 0x0101_0280"]
impl crate::Resettable for PLL1DIVRrs {
    const RESET_VALUE: u32 = 0x0101_0280;
}
