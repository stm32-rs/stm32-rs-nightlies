///Register `PLLCFGR` reader
pub type R = crate::R<PLLCFGRrs>;
///Register `PLLCFGR` writer
pub type W = crate::W<PLLCFGRrs>;
/**Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC {
    ///0: No clock sent to PLL
    None = 0,
    ///2: HSI16 sent to PLL input
    Hsi16 = 2,
    ///3: HSE sent to PLL input
    Hse = 3,
}
impl From<PLLSRC> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSRC {
    type Ux = u8;
}
impl crate::IsEnum for PLLSRC {}
///Field `PLLSRC` reader - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00.
pub type PLLSRC_R = crate::FieldReader<PLLSRC>;
impl PLLSRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLSRC> {
        match self.bits {
            0 => Some(PLLSRC::None),
            2 => Some(PLLSRC::Hsi16),
            3 => Some(PLLSRC::Hse),
            _ => None,
        }
    }
    ///No clock sent to PLL
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PLLSRC::None
    }
    ///HSI16 sent to PLL input
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        *self == PLLSRC::Hsi16
    }
    ///HSE sent to PLL input
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == PLLSRC::Hse
    }
}
///Field `PLLSRC` writer - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00.
pub type PLLSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLSRC>;
impl<'a, REG> PLLSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No clock sent to PLL
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::None)
    }
    ///HSI16 sent to PLL input
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hsi16)
    }
    ///HSE sent to PLL input
    #[inline(always)]
    pub fn hse(self) -> &'a mut crate::W<REG> {
        self.variant(PLLSRC::Hse)
    }
}
/**Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLM {
    ///0: pll_p_ck = vco_ck / 1
    Div1 = 0,
    ///1: pll_p_ck = vco_ck / 2
    Div2 = 1,
    ///2: pll_p_ck = vco_ck / 3
    Div3 = 2,
    ///3: pll_p_ck = vco_ck / 4
    Div4 = 3,
    ///4: pll_p_ck = vco_ck / 5
    Div5 = 4,
    ///5: pll_p_ck = vco_ck / 6
    Div6 = 5,
    ///6: pll_p_ck = vco_ck / 7
    Div7 = 6,
    ///7: pll_p_ck = vco_ck / 8
    Div8 = 7,
    ///8: pll_p_ck = vco_ck / 9
    Div9 = 8,
    ///9: pll_p_ck = vco_ck / 10
    Div10 = 9,
    ///10: pll_p_ck = vco_ck / 11
    Div11 = 10,
    ///11: pll_p_ck = vco_ck / 12
    Div12 = 11,
    ///12: pll_p_ck = vco_ck / 13
    Div13 = 12,
    ///13: pll_p_ck = vco_ck / 14
    Div14 = 13,
    ///14: pll_p_ck = vco_ck / 15
    Div15 = 14,
    ///15: pll_p_ck = vco_ck / 16
    Div16 = 15,
}
impl From<PLLM> for u8 {
    #[inline(always)]
    fn from(variant: PLLM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLM {
    type Ux = u8;
}
impl crate::IsEnum for PLLM {}
///Field `PLLM` reader - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet.
pub type PLLM_R = crate::FieldReader<PLLM>;
impl PLLM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLM {
        match self.bits {
            0 => PLLM::Div1,
            1 => PLLM::Div2,
            2 => PLLM::Div3,
            3 => PLLM::Div4,
            4 => PLLM::Div5,
            5 => PLLM::Div6,
            6 => PLLM::Div7,
            7 => PLLM::Div8,
            8 => PLLM::Div9,
            9 => PLLM::Div10,
            10 => PLLM::Div11,
            11 => PLLM::Div12,
            12 => PLLM::Div13,
            13 => PLLM::Div14,
            14 => PLLM::Div15,
            15 => PLLM::Div16,
            _ => unreachable!(),
        }
    }
    ///pll_p_ck = vco_ck / 1
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLM::Div1
    }
    ///pll_p_ck = vco_ck / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLM::Div2
    }
    ///pll_p_ck = vco_ck / 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLM::Div3
    }
    ///pll_p_ck = vco_ck / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLM::Div4
    }
    ///pll_p_ck = vco_ck / 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLM::Div5
    }
    ///pll_p_ck = vco_ck / 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLM::Div6
    }
    ///pll_p_ck = vco_ck / 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLM::Div7
    }
    ///pll_p_ck = vco_ck / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLM::Div8
    }
    ///pll_p_ck = vco_ck / 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLM::Div9
    }
    ///pll_p_ck = vco_ck / 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLM::Div10
    }
    ///pll_p_ck = vco_ck / 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLM::Div11
    }
    ///pll_p_ck = vco_ck / 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLM::Div12
    }
    ///pll_p_ck = vco_ck / 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLM::Div13
    }
    ///pll_p_ck = vco_ck / 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLM::Div14
    }
    ///pll_p_ck = vco_ck / 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLM::Div15
    }
    ///pll_p_ck = vco_ck / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLM::Div16
    }
}
///Field `PLLM` writer - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet.
pub type PLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 4, PLLM, crate::Safe>;
impl<'a, REG> PLLM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll_p_ck = vco_ck / 1
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div1)
    }
    ///pll_p_ck = vco_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div2)
    }
    ///pll_p_ck = vco_ck / 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div3)
    }
    ///pll_p_ck = vco_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div4)
    }
    ///pll_p_ck = vco_ck / 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div5)
    }
    ///pll_p_ck = vco_ck / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div6)
    }
    ///pll_p_ck = vco_ck / 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div7)
    }
    ///pll_p_ck = vco_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div8)
    }
    ///pll_p_ck = vco_ck / 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div9)
    }
    ///pll_p_ck = vco_ck / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div10)
    }
    ///pll_p_ck = vco_ck / 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div11)
    }
    ///pll_p_ck = vco_ck / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div12)
    }
    ///pll_p_ck = vco_ck / 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div13)
    }
    ///pll_p_ck = vco_ck / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div14)
    }
    ///pll_p_ck = vco_ck / 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div15)
    }
    ///pll_p_ck = vco_ck / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLM::Div16)
    }
}
/**Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet.

Value on reset: 16*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLN {
    ///8: pll_n_ck = vco_ck / 8
    Div8 = 8,
    ///9: pll_n_ck = vco_ck / 9
    Div9 = 9,
    ///10: pll_n_ck = vco_ck / 10
    Div10 = 10,
    ///11: pll_n_ck = vco_ck / 11
    Div11 = 11,
    ///12: pll_n_ck = vco_ck / 12
    Div12 = 12,
    ///13: pll_n_ck = vco_ck / 13
    Div13 = 13,
    ///14: pll_n_ck = vco_ck / 14
    Div14 = 14,
    ///15: pll_n_ck = vco_ck / 15
    Div15 = 15,
    ///16: pll_n_ck = vco_ck / 16
    Div16 = 16,
    ///17: pll_n_ck = vco_ck / 17
    Div17 = 17,
    ///18: pll_n_ck = vco_ck / 18
    Div18 = 18,
    ///19: pll_n_ck = vco_ck / 19
    Div19 = 19,
    ///20: pll_n_ck = vco_ck / 20
    Div20 = 20,
    ///21: pll_n_ck = vco_ck / 21
    Div21 = 21,
    ///22: pll_n_ck = vco_ck / 22
    Div22 = 22,
    ///23: pll_n_ck = vco_ck / 23
    Div23 = 23,
    ///24: pll_n_ck = vco_ck / 24
    Div24 = 24,
    ///25: pll_n_ck = vco_ck / 25
    Div25 = 25,
    ///26: pll_n_ck = vco_ck / 26
    Div26 = 26,
    ///27: pll_n_ck = vco_ck / 27
    Div27 = 27,
    ///28: pll_n_ck = vco_ck / 28
    Div28 = 28,
    ///29: pll_n_ck = vco_ck / 29
    Div29 = 29,
    ///30: pll_n_ck = vco_ck / 30
    Div30 = 30,
    ///31: pll_n_ck = vco_ck / 31
    Div31 = 31,
    ///32: pll_n_ck = vco_ck / 32
    Div32 = 32,
    ///33: pll_n_ck = vco_ck / 33
    Div33 = 33,
    ///34: pll_n_ck = vco_ck / 34
    Div34 = 34,
    ///35: pll_n_ck = vco_ck / 35
    Div35 = 35,
    ///36: pll_n_ck = vco_ck / 36
    Div36 = 36,
    ///37: pll_n_ck = vco_ck / 37
    Div37 = 37,
    ///38: pll_n_ck = vco_ck / 38
    Div38 = 38,
    ///39: pll_n_ck = vco_ck / 39
    Div39 = 39,
    ///40: pll_n_ck = vco_ck / 40
    Div40 = 40,
    ///41: pll_n_ck = vco_ck / 41
    Div41 = 41,
    ///42: pll_n_ck = vco_ck / 42
    Div42 = 42,
    ///43: pll_n_ck = vco_ck / 43
    Div43 = 43,
    ///44: pll_n_ck = vco_ck / 44
    Div44 = 44,
    ///45: pll_n_ck = vco_ck / 45
    Div45 = 45,
    ///46: pll_n_ck = vco_ck / 46
    Div46 = 46,
    ///47: pll_n_ck = vco_ck / 47
    Div47 = 47,
    ///48: pll_n_ck = vco_ck / 48
    Div48 = 48,
    ///49: pll_n_ck = vco_ck / 49
    Div49 = 49,
    ///50: pll_n_ck = vco_ck / 50
    Div50 = 50,
    ///51: pll_n_ck = vco_ck / 51
    Div51 = 51,
    ///52: pll_n_ck = vco_ck / 52
    Div52 = 52,
    ///53: pll_n_ck = vco_ck / 53
    Div53 = 53,
    ///54: pll_n_ck = vco_ck / 54
    Div54 = 54,
    ///55: pll_n_ck = vco_ck / 55
    Div55 = 55,
    ///56: pll_n_ck = vco_ck / 56
    Div56 = 56,
    ///57: pll_n_ck = vco_ck / 57
    Div57 = 57,
    ///58: pll_n_ck = vco_ck / 58
    Div58 = 58,
    ///59: pll_n_ck = vco_ck / 59
    Div59 = 59,
    ///60: pll_n_ck = vco_ck / 60
    Div60 = 60,
    ///61: pll_n_ck = vco_ck / 61
    Div61 = 61,
    ///62: pll_n_ck = vco_ck / 62
    Div62 = 62,
    ///63: pll_n_ck = vco_ck / 63
    Div63 = 63,
    ///64: pll_n_ck = vco_ck / 64
    Div64 = 64,
    ///65: pll_n_ck = vco_ck / 65
    Div65 = 65,
    ///66: pll_n_ck = vco_ck / 66
    Div66 = 66,
    ///67: pll_n_ck = vco_ck / 67
    Div67 = 67,
    ///68: pll_n_ck = vco_ck / 68
    Div68 = 68,
    ///69: pll_n_ck = vco_ck / 69
    Div69 = 69,
    ///70: pll_n_ck = vco_ck / 70
    Div70 = 70,
    ///71: pll_n_ck = vco_ck / 71
    Div71 = 71,
    ///72: pll_n_ck = vco_ck / 72
    Div72 = 72,
    ///73: pll_n_ck = vco_ck / 73
    Div73 = 73,
    ///74: pll_n_ck = vco_ck / 74
    Div74 = 74,
    ///75: pll_n_ck = vco_ck / 75
    Div75 = 75,
    ///76: pll_n_ck = vco_ck / 76
    Div76 = 76,
    ///77: pll_n_ck = vco_ck / 77
    Div77 = 77,
    ///78: pll_n_ck = vco_ck / 78
    Div78 = 78,
    ///79: pll_n_ck = vco_ck / 79
    Div79 = 79,
    ///80: pll_n_ck = vco_ck / 80
    Div80 = 80,
    ///81: pll_n_ck = vco_ck / 81
    Div81 = 81,
    ///82: pll_n_ck = vco_ck / 82
    Div82 = 82,
    ///83: pll_n_ck = vco_ck / 83
    Div83 = 83,
    ///84: pll_n_ck = vco_ck / 84
    Div84 = 84,
    ///85: pll_n_ck = vco_ck / 85
    Div85 = 85,
    ///86: pll_n_ck = vco_ck / 86
    Div86 = 86,
    ///87: pll_n_ck = vco_ck / 87
    Div87 = 87,
    ///88: pll_n_ck = vco_ck / 88
    Div88 = 88,
    ///89: pll_n_ck = vco_ck / 89
    Div89 = 89,
    ///90: pll_n_ck = vco_ck / 90
    Div90 = 90,
    ///91: pll_n_ck = vco_ck / 91
    Div91 = 91,
    ///92: pll_n_ck = vco_ck / 92
    Div92 = 92,
    ///93: pll_n_ck = vco_ck / 93
    Div93 = 93,
    ///94: pll_n_ck = vco_ck / 94
    Div94 = 94,
    ///95: pll_n_ck = vco_ck / 95
    Div95 = 95,
    ///96: pll_n_ck = vco_ck / 96
    Div96 = 96,
    ///97: pll_n_ck = vco_ck / 97
    Div97 = 97,
    ///98: pll_n_ck = vco_ck / 98
    Div98 = 98,
    ///99: pll_n_ck = vco_ck / 99
    Div99 = 99,
    ///100: pll_n_ck = vco_ck / 100
    Div100 = 100,
    ///101: pll_n_ck = vco_ck / 101
    Div101 = 101,
    ///102: pll_n_ck = vco_ck / 102
    Div102 = 102,
    ///103: pll_n_ck = vco_ck / 103
    Div103 = 103,
    ///104: pll_n_ck = vco_ck / 104
    Div104 = 104,
    ///105: pll_n_ck = vco_ck / 105
    Div105 = 105,
    ///106: pll_n_ck = vco_ck / 106
    Div106 = 106,
    ///107: pll_n_ck = vco_ck / 107
    Div107 = 107,
    ///108: pll_n_ck = vco_ck / 108
    Div108 = 108,
    ///109: pll_n_ck = vco_ck / 109
    Div109 = 109,
    ///110: pll_n_ck = vco_ck / 110
    Div110 = 110,
    ///111: pll_n_ck = vco_ck / 111
    Div111 = 111,
    ///112: pll_n_ck = vco_ck / 112
    Div112 = 112,
    ///113: pll_n_ck = vco_ck / 113
    Div113 = 113,
    ///114: pll_n_ck = vco_ck / 114
    Div114 = 114,
    ///115: pll_n_ck = vco_ck / 115
    Div115 = 115,
    ///116: pll_n_ck = vco_ck / 116
    Div116 = 116,
    ///117: pll_n_ck = vco_ck / 117
    Div117 = 117,
    ///118: pll_n_ck = vco_ck / 118
    Div118 = 118,
    ///119: pll_n_ck = vco_ck / 119
    Div119 = 119,
    ///120: pll_n_ck = vco_ck / 120
    Div120 = 120,
    ///121: pll_n_ck = vco_ck / 121
    Div121 = 121,
    ///122: pll_n_ck = vco_ck / 122
    Div122 = 122,
    ///123: pll_n_ck = vco_ck / 123
    Div123 = 123,
    ///124: pll_n_ck = vco_ck / 124
    Div124 = 124,
    ///125: pll_n_ck = vco_ck / 125
    Div125 = 125,
    ///126: pll_n_ck = vco_ck / 126
    Div126 = 126,
    ///127: pll_n_ck = vco_ck / 127
    Div127 = 127,
}
impl From<PLLN> for u8 {
    #[inline(always)]
    fn from(variant: PLLN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLN {
    type Ux = u8;
}
impl crate::IsEnum for PLLN {}
///Field `PLLN` reader - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet.
pub type PLLN_R = crate::FieldReader<PLLN>;
impl PLLN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLN> {
        match self.bits {
            8 => Some(PLLN::Div8),
            9 => Some(PLLN::Div9),
            10 => Some(PLLN::Div10),
            11 => Some(PLLN::Div11),
            12 => Some(PLLN::Div12),
            13 => Some(PLLN::Div13),
            14 => Some(PLLN::Div14),
            15 => Some(PLLN::Div15),
            16 => Some(PLLN::Div16),
            17 => Some(PLLN::Div17),
            18 => Some(PLLN::Div18),
            19 => Some(PLLN::Div19),
            20 => Some(PLLN::Div20),
            21 => Some(PLLN::Div21),
            22 => Some(PLLN::Div22),
            23 => Some(PLLN::Div23),
            24 => Some(PLLN::Div24),
            25 => Some(PLLN::Div25),
            26 => Some(PLLN::Div26),
            27 => Some(PLLN::Div27),
            28 => Some(PLLN::Div28),
            29 => Some(PLLN::Div29),
            30 => Some(PLLN::Div30),
            31 => Some(PLLN::Div31),
            32 => Some(PLLN::Div32),
            33 => Some(PLLN::Div33),
            34 => Some(PLLN::Div34),
            35 => Some(PLLN::Div35),
            36 => Some(PLLN::Div36),
            37 => Some(PLLN::Div37),
            38 => Some(PLLN::Div38),
            39 => Some(PLLN::Div39),
            40 => Some(PLLN::Div40),
            41 => Some(PLLN::Div41),
            42 => Some(PLLN::Div42),
            43 => Some(PLLN::Div43),
            44 => Some(PLLN::Div44),
            45 => Some(PLLN::Div45),
            46 => Some(PLLN::Div46),
            47 => Some(PLLN::Div47),
            48 => Some(PLLN::Div48),
            49 => Some(PLLN::Div49),
            50 => Some(PLLN::Div50),
            51 => Some(PLLN::Div51),
            52 => Some(PLLN::Div52),
            53 => Some(PLLN::Div53),
            54 => Some(PLLN::Div54),
            55 => Some(PLLN::Div55),
            56 => Some(PLLN::Div56),
            57 => Some(PLLN::Div57),
            58 => Some(PLLN::Div58),
            59 => Some(PLLN::Div59),
            60 => Some(PLLN::Div60),
            61 => Some(PLLN::Div61),
            62 => Some(PLLN::Div62),
            63 => Some(PLLN::Div63),
            64 => Some(PLLN::Div64),
            65 => Some(PLLN::Div65),
            66 => Some(PLLN::Div66),
            67 => Some(PLLN::Div67),
            68 => Some(PLLN::Div68),
            69 => Some(PLLN::Div69),
            70 => Some(PLLN::Div70),
            71 => Some(PLLN::Div71),
            72 => Some(PLLN::Div72),
            73 => Some(PLLN::Div73),
            74 => Some(PLLN::Div74),
            75 => Some(PLLN::Div75),
            76 => Some(PLLN::Div76),
            77 => Some(PLLN::Div77),
            78 => Some(PLLN::Div78),
            79 => Some(PLLN::Div79),
            80 => Some(PLLN::Div80),
            81 => Some(PLLN::Div81),
            82 => Some(PLLN::Div82),
            83 => Some(PLLN::Div83),
            84 => Some(PLLN::Div84),
            85 => Some(PLLN::Div85),
            86 => Some(PLLN::Div86),
            87 => Some(PLLN::Div87),
            88 => Some(PLLN::Div88),
            89 => Some(PLLN::Div89),
            90 => Some(PLLN::Div90),
            91 => Some(PLLN::Div91),
            92 => Some(PLLN::Div92),
            93 => Some(PLLN::Div93),
            94 => Some(PLLN::Div94),
            95 => Some(PLLN::Div95),
            96 => Some(PLLN::Div96),
            97 => Some(PLLN::Div97),
            98 => Some(PLLN::Div98),
            99 => Some(PLLN::Div99),
            100 => Some(PLLN::Div100),
            101 => Some(PLLN::Div101),
            102 => Some(PLLN::Div102),
            103 => Some(PLLN::Div103),
            104 => Some(PLLN::Div104),
            105 => Some(PLLN::Div105),
            106 => Some(PLLN::Div106),
            107 => Some(PLLN::Div107),
            108 => Some(PLLN::Div108),
            109 => Some(PLLN::Div109),
            110 => Some(PLLN::Div110),
            111 => Some(PLLN::Div111),
            112 => Some(PLLN::Div112),
            113 => Some(PLLN::Div113),
            114 => Some(PLLN::Div114),
            115 => Some(PLLN::Div115),
            116 => Some(PLLN::Div116),
            117 => Some(PLLN::Div117),
            118 => Some(PLLN::Div118),
            119 => Some(PLLN::Div119),
            120 => Some(PLLN::Div120),
            121 => Some(PLLN::Div121),
            122 => Some(PLLN::Div122),
            123 => Some(PLLN::Div123),
            124 => Some(PLLN::Div124),
            125 => Some(PLLN::Div125),
            126 => Some(PLLN::Div126),
            127 => Some(PLLN::Div127),
            _ => None,
        }
    }
    ///pll_n_ck = vco_ck / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLN::Div8
    }
    ///pll_n_ck = vco_ck / 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLN::Div9
    }
    ///pll_n_ck = vco_ck / 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLN::Div10
    }
    ///pll_n_ck = vco_ck / 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLN::Div11
    }
    ///pll_n_ck = vco_ck / 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLN::Div12
    }
    ///pll_n_ck = vco_ck / 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLN::Div13
    }
    ///pll_n_ck = vco_ck / 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLN::Div14
    }
    ///pll_n_ck = vco_ck / 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLN::Div15
    }
    ///pll_n_ck = vco_ck / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLN::Div16
    }
    ///pll_n_ck = vco_ck / 17
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLN::Div17
    }
    ///pll_n_ck = vco_ck / 18
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLN::Div18
    }
    ///pll_n_ck = vco_ck / 19
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLN::Div19
    }
    ///pll_n_ck = vco_ck / 20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLN::Div20
    }
    ///pll_n_ck = vco_ck / 21
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLN::Div21
    }
    ///pll_n_ck = vco_ck / 22
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLN::Div22
    }
    ///pll_n_ck = vco_ck / 23
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLN::Div23
    }
    ///pll_n_ck = vco_ck / 24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLN::Div24
    }
    ///pll_n_ck = vco_ck / 25
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLN::Div25
    }
    ///pll_n_ck = vco_ck / 26
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLN::Div26
    }
    ///pll_n_ck = vco_ck / 27
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLN::Div27
    }
    ///pll_n_ck = vco_ck / 28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLN::Div28
    }
    ///pll_n_ck = vco_ck / 29
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLN::Div29
    }
    ///pll_n_ck = vco_ck / 30
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLN::Div30
    }
    ///pll_n_ck = vco_ck / 31
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLN::Div31
    }
    ///pll_n_ck = vco_ck / 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PLLN::Div32
    }
    ///pll_n_ck = vco_ck / 33
    #[inline(always)]
    pub fn is_div33(&self) -> bool {
        *self == PLLN::Div33
    }
    ///pll_n_ck = vco_ck / 34
    #[inline(always)]
    pub fn is_div34(&self) -> bool {
        *self == PLLN::Div34
    }
    ///pll_n_ck = vco_ck / 35
    #[inline(always)]
    pub fn is_div35(&self) -> bool {
        *self == PLLN::Div35
    }
    ///pll_n_ck = vco_ck / 36
    #[inline(always)]
    pub fn is_div36(&self) -> bool {
        *self == PLLN::Div36
    }
    ///pll_n_ck = vco_ck / 37
    #[inline(always)]
    pub fn is_div37(&self) -> bool {
        *self == PLLN::Div37
    }
    ///pll_n_ck = vco_ck / 38
    #[inline(always)]
    pub fn is_div38(&self) -> bool {
        *self == PLLN::Div38
    }
    ///pll_n_ck = vco_ck / 39
    #[inline(always)]
    pub fn is_div39(&self) -> bool {
        *self == PLLN::Div39
    }
    ///pll_n_ck = vco_ck / 40
    #[inline(always)]
    pub fn is_div40(&self) -> bool {
        *self == PLLN::Div40
    }
    ///pll_n_ck = vco_ck / 41
    #[inline(always)]
    pub fn is_div41(&self) -> bool {
        *self == PLLN::Div41
    }
    ///pll_n_ck = vco_ck / 42
    #[inline(always)]
    pub fn is_div42(&self) -> bool {
        *self == PLLN::Div42
    }
    ///pll_n_ck = vco_ck / 43
    #[inline(always)]
    pub fn is_div43(&self) -> bool {
        *self == PLLN::Div43
    }
    ///pll_n_ck = vco_ck / 44
    #[inline(always)]
    pub fn is_div44(&self) -> bool {
        *self == PLLN::Div44
    }
    ///pll_n_ck = vco_ck / 45
    #[inline(always)]
    pub fn is_div45(&self) -> bool {
        *self == PLLN::Div45
    }
    ///pll_n_ck = vco_ck / 46
    #[inline(always)]
    pub fn is_div46(&self) -> bool {
        *self == PLLN::Div46
    }
    ///pll_n_ck = vco_ck / 47
    #[inline(always)]
    pub fn is_div47(&self) -> bool {
        *self == PLLN::Div47
    }
    ///pll_n_ck = vco_ck / 48
    #[inline(always)]
    pub fn is_div48(&self) -> bool {
        *self == PLLN::Div48
    }
    ///pll_n_ck = vco_ck / 49
    #[inline(always)]
    pub fn is_div49(&self) -> bool {
        *self == PLLN::Div49
    }
    ///pll_n_ck = vco_ck / 50
    #[inline(always)]
    pub fn is_div50(&self) -> bool {
        *self == PLLN::Div50
    }
    ///pll_n_ck = vco_ck / 51
    #[inline(always)]
    pub fn is_div51(&self) -> bool {
        *self == PLLN::Div51
    }
    ///pll_n_ck = vco_ck / 52
    #[inline(always)]
    pub fn is_div52(&self) -> bool {
        *self == PLLN::Div52
    }
    ///pll_n_ck = vco_ck / 53
    #[inline(always)]
    pub fn is_div53(&self) -> bool {
        *self == PLLN::Div53
    }
    ///pll_n_ck = vco_ck / 54
    #[inline(always)]
    pub fn is_div54(&self) -> bool {
        *self == PLLN::Div54
    }
    ///pll_n_ck = vco_ck / 55
    #[inline(always)]
    pub fn is_div55(&self) -> bool {
        *self == PLLN::Div55
    }
    ///pll_n_ck = vco_ck / 56
    #[inline(always)]
    pub fn is_div56(&self) -> bool {
        *self == PLLN::Div56
    }
    ///pll_n_ck = vco_ck / 57
    #[inline(always)]
    pub fn is_div57(&self) -> bool {
        *self == PLLN::Div57
    }
    ///pll_n_ck = vco_ck / 58
    #[inline(always)]
    pub fn is_div58(&self) -> bool {
        *self == PLLN::Div58
    }
    ///pll_n_ck = vco_ck / 59
    #[inline(always)]
    pub fn is_div59(&self) -> bool {
        *self == PLLN::Div59
    }
    ///pll_n_ck = vco_ck / 60
    #[inline(always)]
    pub fn is_div60(&self) -> bool {
        *self == PLLN::Div60
    }
    ///pll_n_ck = vco_ck / 61
    #[inline(always)]
    pub fn is_div61(&self) -> bool {
        *self == PLLN::Div61
    }
    ///pll_n_ck = vco_ck / 62
    #[inline(always)]
    pub fn is_div62(&self) -> bool {
        *self == PLLN::Div62
    }
    ///pll_n_ck = vco_ck / 63
    #[inline(always)]
    pub fn is_div63(&self) -> bool {
        *self == PLLN::Div63
    }
    ///pll_n_ck = vco_ck / 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PLLN::Div64
    }
    ///pll_n_ck = vco_ck / 65
    #[inline(always)]
    pub fn is_div65(&self) -> bool {
        *self == PLLN::Div65
    }
    ///pll_n_ck = vco_ck / 66
    #[inline(always)]
    pub fn is_div66(&self) -> bool {
        *self == PLLN::Div66
    }
    ///pll_n_ck = vco_ck / 67
    #[inline(always)]
    pub fn is_div67(&self) -> bool {
        *self == PLLN::Div67
    }
    ///pll_n_ck = vco_ck / 68
    #[inline(always)]
    pub fn is_div68(&self) -> bool {
        *self == PLLN::Div68
    }
    ///pll_n_ck = vco_ck / 69
    #[inline(always)]
    pub fn is_div69(&self) -> bool {
        *self == PLLN::Div69
    }
    ///pll_n_ck = vco_ck / 70
    #[inline(always)]
    pub fn is_div70(&self) -> bool {
        *self == PLLN::Div70
    }
    ///pll_n_ck = vco_ck / 71
    #[inline(always)]
    pub fn is_div71(&self) -> bool {
        *self == PLLN::Div71
    }
    ///pll_n_ck = vco_ck / 72
    #[inline(always)]
    pub fn is_div72(&self) -> bool {
        *self == PLLN::Div72
    }
    ///pll_n_ck = vco_ck / 73
    #[inline(always)]
    pub fn is_div73(&self) -> bool {
        *self == PLLN::Div73
    }
    ///pll_n_ck = vco_ck / 74
    #[inline(always)]
    pub fn is_div74(&self) -> bool {
        *self == PLLN::Div74
    }
    ///pll_n_ck = vco_ck / 75
    #[inline(always)]
    pub fn is_div75(&self) -> bool {
        *self == PLLN::Div75
    }
    ///pll_n_ck = vco_ck / 76
    #[inline(always)]
    pub fn is_div76(&self) -> bool {
        *self == PLLN::Div76
    }
    ///pll_n_ck = vco_ck / 77
    #[inline(always)]
    pub fn is_div77(&self) -> bool {
        *self == PLLN::Div77
    }
    ///pll_n_ck = vco_ck / 78
    #[inline(always)]
    pub fn is_div78(&self) -> bool {
        *self == PLLN::Div78
    }
    ///pll_n_ck = vco_ck / 79
    #[inline(always)]
    pub fn is_div79(&self) -> bool {
        *self == PLLN::Div79
    }
    ///pll_n_ck = vco_ck / 80
    #[inline(always)]
    pub fn is_div80(&self) -> bool {
        *self == PLLN::Div80
    }
    ///pll_n_ck = vco_ck / 81
    #[inline(always)]
    pub fn is_div81(&self) -> bool {
        *self == PLLN::Div81
    }
    ///pll_n_ck = vco_ck / 82
    #[inline(always)]
    pub fn is_div82(&self) -> bool {
        *self == PLLN::Div82
    }
    ///pll_n_ck = vco_ck / 83
    #[inline(always)]
    pub fn is_div83(&self) -> bool {
        *self == PLLN::Div83
    }
    ///pll_n_ck = vco_ck / 84
    #[inline(always)]
    pub fn is_div84(&self) -> bool {
        *self == PLLN::Div84
    }
    ///pll_n_ck = vco_ck / 85
    #[inline(always)]
    pub fn is_div85(&self) -> bool {
        *self == PLLN::Div85
    }
    ///pll_n_ck = vco_ck / 86
    #[inline(always)]
    pub fn is_div86(&self) -> bool {
        *self == PLLN::Div86
    }
    ///pll_n_ck = vco_ck / 87
    #[inline(always)]
    pub fn is_div87(&self) -> bool {
        *self == PLLN::Div87
    }
    ///pll_n_ck = vco_ck / 88
    #[inline(always)]
    pub fn is_div88(&self) -> bool {
        *self == PLLN::Div88
    }
    ///pll_n_ck = vco_ck / 89
    #[inline(always)]
    pub fn is_div89(&self) -> bool {
        *self == PLLN::Div89
    }
    ///pll_n_ck = vco_ck / 90
    #[inline(always)]
    pub fn is_div90(&self) -> bool {
        *self == PLLN::Div90
    }
    ///pll_n_ck = vco_ck / 91
    #[inline(always)]
    pub fn is_div91(&self) -> bool {
        *self == PLLN::Div91
    }
    ///pll_n_ck = vco_ck / 92
    #[inline(always)]
    pub fn is_div92(&self) -> bool {
        *self == PLLN::Div92
    }
    ///pll_n_ck = vco_ck / 93
    #[inline(always)]
    pub fn is_div93(&self) -> bool {
        *self == PLLN::Div93
    }
    ///pll_n_ck = vco_ck / 94
    #[inline(always)]
    pub fn is_div94(&self) -> bool {
        *self == PLLN::Div94
    }
    ///pll_n_ck = vco_ck / 95
    #[inline(always)]
    pub fn is_div95(&self) -> bool {
        *self == PLLN::Div95
    }
    ///pll_n_ck = vco_ck / 96
    #[inline(always)]
    pub fn is_div96(&self) -> bool {
        *self == PLLN::Div96
    }
    ///pll_n_ck = vco_ck / 97
    #[inline(always)]
    pub fn is_div97(&self) -> bool {
        *self == PLLN::Div97
    }
    ///pll_n_ck = vco_ck / 98
    #[inline(always)]
    pub fn is_div98(&self) -> bool {
        *self == PLLN::Div98
    }
    ///pll_n_ck = vco_ck / 99
    #[inline(always)]
    pub fn is_div99(&self) -> bool {
        *self == PLLN::Div99
    }
    ///pll_n_ck = vco_ck / 100
    #[inline(always)]
    pub fn is_div100(&self) -> bool {
        *self == PLLN::Div100
    }
    ///pll_n_ck = vco_ck / 101
    #[inline(always)]
    pub fn is_div101(&self) -> bool {
        *self == PLLN::Div101
    }
    ///pll_n_ck = vco_ck / 102
    #[inline(always)]
    pub fn is_div102(&self) -> bool {
        *self == PLLN::Div102
    }
    ///pll_n_ck = vco_ck / 103
    #[inline(always)]
    pub fn is_div103(&self) -> bool {
        *self == PLLN::Div103
    }
    ///pll_n_ck = vco_ck / 104
    #[inline(always)]
    pub fn is_div104(&self) -> bool {
        *self == PLLN::Div104
    }
    ///pll_n_ck = vco_ck / 105
    #[inline(always)]
    pub fn is_div105(&self) -> bool {
        *self == PLLN::Div105
    }
    ///pll_n_ck = vco_ck / 106
    #[inline(always)]
    pub fn is_div106(&self) -> bool {
        *self == PLLN::Div106
    }
    ///pll_n_ck = vco_ck / 107
    #[inline(always)]
    pub fn is_div107(&self) -> bool {
        *self == PLLN::Div107
    }
    ///pll_n_ck = vco_ck / 108
    #[inline(always)]
    pub fn is_div108(&self) -> bool {
        *self == PLLN::Div108
    }
    ///pll_n_ck = vco_ck / 109
    #[inline(always)]
    pub fn is_div109(&self) -> bool {
        *self == PLLN::Div109
    }
    ///pll_n_ck = vco_ck / 110
    #[inline(always)]
    pub fn is_div110(&self) -> bool {
        *self == PLLN::Div110
    }
    ///pll_n_ck = vco_ck / 111
    #[inline(always)]
    pub fn is_div111(&self) -> bool {
        *self == PLLN::Div111
    }
    ///pll_n_ck = vco_ck / 112
    #[inline(always)]
    pub fn is_div112(&self) -> bool {
        *self == PLLN::Div112
    }
    ///pll_n_ck = vco_ck / 113
    #[inline(always)]
    pub fn is_div113(&self) -> bool {
        *self == PLLN::Div113
    }
    ///pll_n_ck = vco_ck / 114
    #[inline(always)]
    pub fn is_div114(&self) -> bool {
        *self == PLLN::Div114
    }
    ///pll_n_ck = vco_ck / 115
    #[inline(always)]
    pub fn is_div115(&self) -> bool {
        *self == PLLN::Div115
    }
    ///pll_n_ck = vco_ck / 116
    #[inline(always)]
    pub fn is_div116(&self) -> bool {
        *self == PLLN::Div116
    }
    ///pll_n_ck = vco_ck / 117
    #[inline(always)]
    pub fn is_div117(&self) -> bool {
        *self == PLLN::Div117
    }
    ///pll_n_ck = vco_ck / 118
    #[inline(always)]
    pub fn is_div118(&self) -> bool {
        *self == PLLN::Div118
    }
    ///pll_n_ck = vco_ck / 119
    #[inline(always)]
    pub fn is_div119(&self) -> bool {
        *self == PLLN::Div119
    }
    ///pll_n_ck = vco_ck / 120
    #[inline(always)]
    pub fn is_div120(&self) -> bool {
        *self == PLLN::Div120
    }
    ///pll_n_ck = vco_ck / 121
    #[inline(always)]
    pub fn is_div121(&self) -> bool {
        *self == PLLN::Div121
    }
    ///pll_n_ck = vco_ck / 122
    #[inline(always)]
    pub fn is_div122(&self) -> bool {
        *self == PLLN::Div122
    }
    ///pll_n_ck = vco_ck / 123
    #[inline(always)]
    pub fn is_div123(&self) -> bool {
        *self == PLLN::Div123
    }
    ///pll_n_ck = vco_ck / 124
    #[inline(always)]
    pub fn is_div124(&self) -> bool {
        *self == PLLN::Div124
    }
    ///pll_n_ck = vco_ck / 125
    #[inline(always)]
    pub fn is_div125(&self) -> bool {
        *self == PLLN::Div125
    }
    ///pll_n_ck = vco_ck / 126
    #[inline(always)]
    pub fn is_div126(&self) -> bool {
        *self == PLLN::Div126
    }
    ///pll_n_ck = vco_ck / 127
    #[inline(always)]
    pub fn is_div127(&self) -> bool {
        *self == PLLN::Div127
    }
}
///Field `PLLN` writer - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet.
pub type PLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7, PLLN>;
impl<'a, REG> PLLN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll_n_ck = vco_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div8)
    }
    ///pll_n_ck = vco_ck / 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div9)
    }
    ///pll_n_ck = vco_ck / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div10)
    }
    ///pll_n_ck = vco_ck / 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div11)
    }
    ///pll_n_ck = vco_ck / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div12)
    }
    ///pll_n_ck = vco_ck / 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div13)
    }
    ///pll_n_ck = vco_ck / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div14)
    }
    ///pll_n_ck = vco_ck / 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div15)
    }
    ///pll_n_ck = vco_ck / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div16)
    }
    ///pll_n_ck = vco_ck / 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div17)
    }
    ///pll_n_ck = vco_ck / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div18)
    }
    ///pll_n_ck = vco_ck / 19
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div19)
    }
    ///pll_n_ck = vco_ck / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div20)
    }
    ///pll_n_ck = vco_ck / 21
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div21)
    }
    ///pll_n_ck = vco_ck / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div22)
    }
    ///pll_n_ck = vco_ck / 23
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div23)
    }
    ///pll_n_ck = vco_ck / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div24)
    }
    ///pll_n_ck = vco_ck / 25
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div25)
    }
    ///pll_n_ck = vco_ck / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div26)
    }
    ///pll_n_ck = vco_ck / 27
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div27)
    }
    ///pll_n_ck = vco_ck / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div28)
    }
    ///pll_n_ck = vco_ck / 29
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div29)
    }
    ///pll_n_ck = vco_ck / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div30)
    }
    ///pll_n_ck = vco_ck / 31
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div31)
    }
    ///pll_n_ck = vco_ck / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div32)
    }
    ///pll_n_ck = vco_ck / 33
    #[inline(always)]
    pub fn div33(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div33)
    }
    ///pll_n_ck = vco_ck / 34
    #[inline(always)]
    pub fn div34(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div34)
    }
    ///pll_n_ck = vco_ck / 35
    #[inline(always)]
    pub fn div35(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div35)
    }
    ///pll_n_ck = vco_ck / 36
    #[inline(always)]
    pub fn div36(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div36)
    }
    ///pll_n_ck = vco_ck / 37
    #[inline(always)]
    pub fn div37(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div37)
    }
    ///pll_n_ck = vco_ck / 38
    #[inline(always)]
    pub fn div38(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div38)
    }
    ///pll_n_ck = vco_ck / 39
    #[inline(always)]
    pub fn div39(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div39)
    }
    ///pll_n_ck = vco_ck / 40
    #[inline(always)]
    pub fn div40(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div40)
    }
    ///pll_n_ck = vco_ck / 41
    #[inline(always)]
    pub fn div41(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div41)
    }
    ///pll_n_ck = vco_ck / 42
    #[inline(always)]
    pub fn div42(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div42)
    }
    ///pll_n_ck = vco_ck / 43
    #[inline(always)]
    pub fn div43(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div43)
    }
    ///pll_n_ck = vco_ck / 44
    #[inline(always)]
    pub fn div44(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div44)
    }
    ///pll_n_ck = vco_ck / 45
    #[inline(always)]
    pub fn div45(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div45)
    }
    ///pll_n_ck = vco_ck / 46
    #[inline(always)]
    pub fn div46(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div46)
    }
    ///pll_n_ck = vco_ck / 47
    #[inline(always)]
    pub fn div47(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div47)
    }
    ///pll_n_ck = vco_ck / 48
    #[inline(always)]
    pub fn div48(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div48)
    }
    ///pll_n_ck = vco_ck / 49
    #[inline(always)]
    pub fn div49(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div49)
    }
    ///pll_n_ck = vco_ck / 50
    #[inline(always)]
    pub fn div50(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div50)
    }
    ///pll_n_ck = vco_ck / 51
    #[inline(always)]
    pub fn div51(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div51)
    }
    ///pll_n_ck = vco_ck / 52
    #[inline(always)]
    pub fn div52(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div52)
    }
    ///pll_n_ck = vco_ck / 53
    #[inline(always)]
    pub fn div53(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div53)
    }
    ///pll_n_ck = vco_ck / 54
    #[inline(always)]
    pub fn div54(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div54)
    }
    ///pll_n_ck = vco_ck / 55
    #[inline(always)]
    pub fn div55(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div55)
    }
    ///pll_n_ck = vco_ck / 56
    #[inline(always)]
    pub fn div56(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div56)
    }
    ///pll_n_ck = vco_ck / 57
    #[inline(always)]
    pub fn div57(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div57)
    }
    ///pll_n_ck = vco_ck / 58
    #[inline(always)]
    pub fn div58(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div58)
    }
    ///pll_n_ck = vco_ck / 59
    #[inline(always)]
    pub fn div59(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div59)
    }
    ///pll_n_ck = vco_ck / 60
    #[inline(always)]
    pub fn div60(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div60)
    }
    ///pll_n_ck = vco_ck / 61
    #[inline(always)]
    pub fn div61(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div61)
    }
    ///pll_n_ck = vco_ck / 62
    #[inline(always)]
    pub fn div62(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div62)
    }
    ///pll_n_ck = vco_ck / 63
    #[inline(always)]
    pub fn div63(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div63)
    }
    ///pll_n_ck = vco_ck / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div64)
    }
    ///pll_n_ck = vco_ck / 65
    #[inline(always)]
    pub fn div65(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div65)
    }
    ///pll_n_ck = vco_ck / 66
    #[inline(always)]
    pub fn div66(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div66)
    }
    ///pll_n_ck = vco_ck / 67
    #[inline(always)]
    pub fn div67(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div67)
    }
    ///pll_n_ck = vco_ck / 68
    #[inline(always)]
    pub fn div68(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div68)
    }
    ///pll_n_ck = vco_ck / 69
    #[inline(always)]
    pub fn div69(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div69)
    }
    ///pll_n_ck = vco_ck / 70
    #[inline(always)]
    pub fn div70(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div70)
    }
    ///pll_n_ck = vco_ck / 71
    #[inline(always)]
    pub fn div71(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div71)
    }
    ///pll_n_ck = vco_ck / 72
    #[inline(always)]
    pub fn div72(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div72)
    }
    ///pll_n_ck = vco_ck / 73
    #[inline(always)]
    pub fn div73(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div73)
    }
    ///pll_n_ck = vco_ck / 74
    #[inline(always)]
    pub fn div74(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div74)
    }
    ///pll_n_ck = vco_ck / 75
    #[inline(always)]
    pub fn div75(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div75)
    }
    ///pll_n_ck = vco_ck / 76
    #[inline(always)]
    pub fn div76(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div76)
    }
    ///pll_n_ck = vco_ck / 77
    #[inline(always)]
    pub fn div77(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div77)
    }
    ///pll_n_ck = vco_ck / 78
    #[inline(always)]
    pub fn div78(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div78)
    }
    ///pll_n_ck = vco_ck / 79
    #[inline(always)]
    pub fn div79(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div79)
    }
    ///pll_n_ck = vco_ck / 80
    #[inline(always)]
    pub fn div80(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div80)
    }
    ///pll_n_ck = vco_ck / 81
    #[inline(always)]
    pub fn div81(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div81)
    }
    ///pll_n_ck = vco_ck / 82
    #[inline(always)]
    pub fn div82(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div82)
    }
    ///pll_n_ck = vco_ck / 83
    #[inline(always)]
    pub fn div83(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div83)
    }
    ///pll_n_ck = vco_ck / 84
    #[inline(always)]
    pub fn div84(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div84)
    }
    ///pll_n_ck = vco_ck / 85
    #[inline(always)]
    pub fn div85(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div85)
    }
    ///pll_n_ck = vco_ck / 86
    #[inline(always)]
    pub fn div86(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div86)
    }
    ///pll_n_ck = vco_ck / 87
    #[inline(always)]
    pub fn div87(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div87)
    }
    ///pll_n_ck = vco_ck / 88
    #[inline(always)]
    pub fn div88(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div88)
    }
    ///pll_n_ck = vco_ck / 89
    #[inline(always)]
    pub fn div89(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div89)
    }
    ///pll_n_ck = vco_ck / 90
    #[inline(always)]
    pub fn div90(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div90)
    }
    ///pll_n_ck = vco_ck / 91
    #[inline(always)]
    pub fn div91(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div91)
    }
    ///pll_n_ck = vco_ck / 92
    #[inline(always)]
    pub fn div92(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div92)
    }
    ///pll_n_ck = vco_ck / 93
    #[inline(always)]
    pub fn div93(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div93)
    }
    ///pll_n_ck = vco_ck / 94
    #[inline(always)]
    pub fn div94(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div94)
    }
    ///pll_n_ck = vco_ck / 95
    #[inline(always)]
    pub fn div95(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div95)
    }
    ///pll_n_ck = vco_ck / 96
    #[inline(always)]
    pub fn div96(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div96)
    }
    ///pll_n_ck = vco_ck / 97
    #[inline(always)]
    pub fn div97(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div97)
    }
    ///pll_n_ck = vco_ck / 98
    #[inline(always)]
    pub fn div98(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div98)
    }
    ///pll_n_ck = vco_ck / 99
    #[inline(always)]
    pub fn div99(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div99)
    }
    ///pll_n_ck = vco_ck / 100
    #[inline(always)]
    pub fn div100(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div100)
    }
    ///pll_n_ck = vco_ck / 101
    #[inline(always)]
    pub fn div101(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div101)
    }
    ///pll_n_ck = vco_ck / 102
    #[inline(always)]
    pub fn div102(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div102)
    }
    ///pll_n_ck = vco_ck / 103
    #[inline(always)]
    pub fn div103(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div103)
    }
    ///pll_n_ck = vco_ck / 104
    #[inline(always)]
    pub fn div104(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div104)
    }
    ///pll_n_ck = vco_ck / 105
    #[inline(always)]
    pub fn div105(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div105)
    }
    ///pll_n_ck = vco_ck / 106
    #[inline(always)]
    pub fn div106(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div106)
    }
    ///pll_n_ck = vco_ck / 107
    #[inline(always)]
    pub fn div107(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div107)
    }
    ///pll_n_ck = vco_ck / 108
    #[inline(always)]
    pub fn div108(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div108)
    }
    ///pll_n_ck = vco_ck / 109
    #[inline(always)]
    pub fn div109(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div109)
    }
    ///pll_n_ck = vco_ck / 110
    #[inline(always)]
    pub fn div110(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div110)
    }
    ///pll_n_ck = vco_ck / 111
    #[inline(always)]
    pub fn div111(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div111)
    }
    ///pll_n_ck = vco_ck / 112
    #[inline(always)]
    pub fn div112(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div112)
    }
    ///pll_n_ck = vco_ck / 113
    #[inline(always)]
    pub fn div113(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div113)
    }
    ///pll_n_ck = vco_ck / 114
    #[inline(always)]
    pub fn div114(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div114)
    }
    ///pll_n_ck = vco_ck / 115
    #[inline(always)]
    pub fn div115(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div115)
    }
    ///pll_n_ck = vco_ck / 116
    #[inline(always)]
    pub fn div116(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div116)
    }
    ///pll_n_ck = vco_ck / 117
    #[inline(always)]
    pub fn div117(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div117)
    }
    ///pll_n_ck = vco_ck / 118
    #[inline(always)]
    pub fn div118(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div118)
    }
    ///pll_n_ck = vco_ck / 119
    #[inline(always)]
    pub fn div119(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div119)
    }
    ///pll_n_ck = vco_ck / 120
    #[inline(always)]
    pub fn div120(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div120)
    }
    ///pll_n_ck = vco_ck / 121
    #[inline(always)]
    pub fn div121(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div121)
    }
    ///pll_n_ck = vco_ck / 122
    #[inline(always)]
    pub fn div122(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div122)
    }
    ///pll_n_ck = vco_ck / 123
    #[inline(always)]
    pub fn div123(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div123)
    }
    ///pll_n_ck = vco_ck / 124
    #[inline(always)]
    pub fn div124(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div124)
    }
    ///pll_n_ck = vco_ck / 125
    #[inline(always)]
    pub fn div125(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div125)
    }
    ///pll_n_ck = vco_ck / 126
    #[inline(always)]
    pub fn div126(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div126)
    }
    ///pll_n_ck = vco_ck / 127
    #[inline(always)]
    pub fn div127(self) -> &'a mut crate::W<REG> {
        self.variant(PLLN::Div127)
    }
}
///Field `PLLPEN` reader - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0.
pub type PLLPEN_R = crate::BitReader;
///Field `PLLPEN` writer - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0.
pub type PLLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\[4:0\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLP {
    ///0: pll_p_ck = vco_ck / 7
    Div7 = 0,
    ///1: pll_p_ck = vco_ck / 17
    Div17 = 1,
}
impl From<PLLP> for bool {
    #[inline(always)]
    fn from(variant: PLLP) -> Self {
        variant as u8 != 0
    }
}
///Field `PLLP` reader - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\[4:0\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
pub type PLLP_R = crate::BitReader<PLLP>;
impl PLLP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLP {
        match self.bits {
            false => PLLP::Div7,
            true => PLLP::Div17,
        }
    }
    ///pll_p_ck = vco_ck / 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLP::Div7
    }
    ///pll_p_ck = vco_ck / 17
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLP::Div17
    }
}
///Field `PLLP` writer - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\[4:0\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
pub type PLLP_W<'a, REG> = crate::BitWriter<'a, REG, PLLP>;
impl<'a, REG> PLLP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///pll_p_ck = vco_ck / 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div7)
    }
    ///pll_p_ck = vco_ck / 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLP::Div17)
    }
}
///Field `PLLQEN` reader - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0.
pub type PLLQEN_R = crate::BitReader;
///Field `PLLQEN` writer - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0.
pub type PLLQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLQ {
    ///0: pll_q_ck = vco_ck / 2
    Div2 = 0,
    ///1: pll_q_ck = vco_ck / 4
    Div4 = 1,
    ///2: pll_q_ck = vco_ck / 6
    Div6 = 2,
    ///3: pll_q_ck = vco_ck / 8
    Div8 = 3,
}
impl From<PLLQ> for u8 {
    #[inline(always)]
    fn from(variant: PLLQ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLQ {
    type Ux = u8;
}
impl crate::IsEnum for PLLQ {}
///Field `PLLQ` reader - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
pub type PLLQ_R = crate::FieldReader<PLLQ>;
impl PLLQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLQ {
        match self.bits {
            0 => PLLQ::Div2,
            1 => PLLQ::Div4,
            2 => PLLQ::Div6,
            3 => PLLQ::Div8,
            _ => unreachable!(),
        }
    }
    ///pll_q_ck = vco_ck / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLQ::Div2
    }
    ///pll_q_ck = vco_ck / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLQ::Div4
    }
    ///pll_q_ck = vco_ck / 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLQ::Div6
    }
    ///pll_q_ck = vco_ck / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLQ::Div8
    }
}
///Field `PLLQ` writer - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
pub type PLLQ_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLQ, crate::Safe>;
impl<'a, REG> PLLQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll_q_ck = vco_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div2)
    }
    ///pll_q_ck = vco_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div4)
    }
    ///pll_q_ck = vco_ck / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div6)
    }
    ///pll_q_ck = vco_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLQ::Div8)
    }
}
///Field `PLLREN` reader - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0.
pub type PLLREN_R = crate::BitReader;
///Field `PLLREN` writer - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0.
pub type PLLREN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLR {
    ///0: pll_r_ck = vco_ck / 2
    Div2 = 0,
    ///1: pll_r_ck = vco_ck / 4
    Div4 = 1,
    ///2: pll_r_ck = vco_ck / 6
    Div6 = 2,
    ///3: pll_r_ck = vco_ck / 8
    Div8 = 3,
}
impl From<PLLR> for u8 {
    #[inline(always)]
    fn from(variant: PLLR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLR {
    type Ux = u8;
}
impl crate::IsEnum for PLLR {}
///Field `PLLR` reader - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
pub type PLLR_R = crate::FieldReader<PLLR>;
impl PLLR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PLLR {
        match self.bits {
            0 => PLLR::Div2,
            1 => PLLR::Div4,
            2 => PLLR::Div6,
            3 => PLLR::Div8,
            _ => unreachable!(),
        }
    }
    ///pll_r_ck = vco_ck / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLR::Div2
    }
    ///pll_r_ck = vco_ck / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLR::Div4
    }
    ///pll_r_ck = vco_ck / 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLR::Div6
    }
    ///pll_r_ck = vco_ck / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLR::Div8
    }
}
///Field `PLLR` writer - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
pub type PLLR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PLLR, crate::Safe>;
impl<'a, REG> PLLR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll_r_ck = vco_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLR::Div2)
    }
    ///pll_r_ck = vco_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLR::Div4)
    }
    ///pll_r_ck = vco_ck / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLR::Div6)
    }
    ///pll_r_ck = vco_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLR::Div8)
    }
}
/**Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ....

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLPDIV {
    ///0: pll_p_ck is controlled by PLLP
    Pllp = 0,
    ///2: pll_p_ck = vco_ck / 2
    Div2 = 2,
    ///3: pll_p_ck = vco_ck / 3
    Div3 = 3,
    ///4: pll_p_ck = vco_ck / 4
    Div4 = 4,
    ///5: pll_p_ck = vco_ck / 5
    Div5 = 5,
    ///6: pll_p_ck = vco_ck / 6
    Div6 = 6,
    ///7: pll_p_ck = vco_ck / 7
    Div7 = 7,
    ///8: pll_p_ck = vco_ck / 8
    Div8 = 8,
    ///9: pll_p_ck = vco_ck / 9
    Div9 = 9,
    ///10: pll_p_ck = vco_ck / 10
    Div10 = 10,
    ///11: pll_p_ck = vco_ck / 11
    Div11 = 11,
    ///12: pll_p_ck = vco_ck / 12
    Div12 = 12,
    ///13: pll_p_ck = vco_ck / 13
    Div13 = 13,
    ///14: pll_p_ck = vco_ck / 14
    Div14 = 14,
    ///15: pll_p_ck = vco_ck / 15
    Div15 = 15,
    ///16: pll_p_ck = vco_ck / 16
    Div16 = 16,
    ///17: pll_p_ck = vco_ck / 17
    Div17 = 17,
    ///18: pll_p_ck = vco_ck / 18
    Div18 = 18,
    ///19: pll_p_ck = vco_ck / 19
    Div19 = 19,
    ///20: pll_p_ck = vco_ck / 20
    Div20 = 20,
    ///21: pll_p_ck = vco_ck / 21
    Div21 = 21,
    ///22: pll_p_ck = vco_ck / 22
    Div22 = 22,
    ///23: pll_p_ck = vco_ck / 23
    Div23 = 23,
    ///24: pll_p_ck = vco_ck / 24
    Div24 = 24,
    ///25: pll_p_ck = vco_ck / 25
    Div25 = 25,
    ///26: pll_p_ck = vco_ck / 26
    Div26 = 26,
    ///27: pll_p_ck = vco_ck / 27
    Div27 = 27,
    ///28: pll_p_ck = vco_ck / 28
    Div28 = 28,
    ///29: pll_p_ck = vco_ck / 29
    Div29 = 29,
    ///30: pll_p_ck = vco_ck / 30
    Div30 = 30,
    ///31: pll_p_ck = vco_ck / 31
    Div31 = 31,
}
impl From<PLLPDIV> for u8 {
    #[inline(always)]
    fn from(variant: PLLPDIV) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLPDIV {
    type Ux = u8;
}
impl crate::IsEnum for PLLPDIV {}
///Field `PLLPDIV` reader - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ....
pub type PLLPDIV_R = crate::FieldReader<PLLPDIV>;
impl PLLPDIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PLLPDIV> {
        match self.bits {
            0 => Some(PLLPDIV::Pllp),
            2 => Some(PLLPDIV::Div2),
            3 => Some(PLLPDIV::Div3),
            4 => Some(PLLPDIV::Div4),
            5 => Some(PLLPDIV::Div5),
            6 => Some(PLLPDIV::Div6),
            7 => Some(PLLPDIV::Div7),
            8 => Some(PLLPDIV::Div8),
            9 => Some(PLLPDIV::Div9),
            10 => Some(PLLPDIV::Div10),
            11 => Some(PLLPDIV::Div11),
            12 => Some(PLLPDIV::Div12),
            13 => Some(PLLPDIV::Div13),
            14 => Some(PLLPDIV::Div14),
            15 => Some(PLLPDIV::Div15),
            16 => Some(PLLPDIV::Div16),
            17 => Some(PLLPDIV::Div17),
            18 => Some(PLLPDIV::Div18),
            19 => Some(PLLPDIV::Div19),
            20 => Some(PLLPDIV::Div20),
            21 => Some(PLLPDIV::Div21),
            22 => Some(PLLPDIV::Div22),
            23 => Some(PLLPDIV::Div23),
            24 => Some(PLLPDIV::Div24),
            25 => Some(PLLPDIV::Div25),
            26 => Some(PLLPDIV::Div26),
            27 => Some(PLLPDIV::Div27),
            28 => Some(PLLPDIV::Div28),
            29 => Some(PLLPDIV::Div29),
            30 => Some(PLLPDIV::Div30),
            31 => Some(PLLPDIV::Div31),
            _ => None,
        }
    }
    ///pll_p_ck is controlled by PLLP
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        *self == PLLPDIV::Pllp
    }
    ///pll_p_ck = vco_ck / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLPDIV::Div2
    }
    ///pll_p_ck = vco_ck / 3
    #[inline(always)]
    pub fn is_div3(&self) -> bool {
        *self == PLLPDIV::Div3
    }
    ///pll_p_ck = vco_ck / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PLLPDIV::Div4
    }
    ///pll_p_ck = vco_ck / 5
    #[inline(always)]
    pub fn is_div5(&self) -> bool {
        *self == PLLPDIV::Div5
    }
    ///pll_p_ck = vco_ck / 6
    #[inline(always)]
    pub fn is_div6(&self) -> bool {
        *self == PLLPDIV::Div6
    }
    ///pll_p_ck = vco_ck / 7
    #[inline(always)]
    pub fn is_div7(&self) -> bool {
        *self == PLLPDIV::Div7
    }
    ///pll_p_ck = vco_ck / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PLLPDIV::Div8
    }
    ///pll_p_ck = vco_ck / 9
    #[inline(always)]
    pub fn is_div9(&self) -> bool {
        *self == PLLPDIV::Div9
    }
    ///pll_p_ck = vco_ck / 10
    #[inline(always)]
    pub fn is_div10(&self) -> bool {
        *self == PLLPDIV::Div10
    }
    ///pll_p_ck = vco_ck / 11
    #[inline(always)]
    pub fn is_div11(&self) -> bool {
        *self == PLLPDIV::Div11
    }
    ///pll_p_ck = vco_ck / 12
    #[inline(always)]
    pub fn is_div12(&self) -> bool {
        *self == PLLPDIV::Div12
    }
    ///pll_p_ck = vco_ck / 13
    #[inline(always)]
    pub fn is_div13(&self) -> bool {
        *self == PLLPDIV::Div13
    }
    ///pll_p_ck = vco_ck / 14
    #[inline(always)]
    pub fn is_div14(&self) -> bool {
        *self == PLLPDIV::Div14
    }
    ///pll_p_ck = vco_ck / 15
    #[inline(always)]
    pub fn is_div15(&self) -> bool {
        *self == PLLPDIV::Div15
    }
    ///pll_p_ck = vco_ck / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PLLPDIV::Div16
    }
    ///pll_p_ck = vco_ck / 17
    #[inline(always)]
    pub fn is_div17(&self) -> bool {
        *self == PLLPDIV::Div17
    }
    ///pll_p_ck = vco_ck / 18
    #[inline(always)]
    pub fn is_div18(&self) -> bool {
        *self == PLLPDIV::Div18
    }
    ///pll_p_ck = vco_ck / 19
    #[inline(always)]
    pub fn is_div19(&self) -> bool {
        *self == PLLPDIV::Div19
    }
    ///pll_p_ck = vco_ck / 20
    #[inline(always)]
    pub fn is_div20(&self) -> bool {
        *self == PLLPDIV::Div20
    }
    ///pll_p_ck = vco_ck / 21
    #[inline(always)]
    pub fn is_div21(&self) -> bool {
        *self == PLLPDIV::Div21
    }
    ///pll_p_ck = vco_ck / 22
    #[inline(always)]
    pub fn is_div22(&self) -> bool {
        *self == PLLPDIV::Div22
    }
    ///pll_p_ck = vco_ck / 23
    #[inline(always)]
    pub fn is_div23(&self) -> bool {
        *self == PLLPDIV::Div23
    }
    ///pll_p_ck = vco_ck / 24
    #[inline(always)]
    pub fn is_div24(&self) -> bool {
        *self == PLLPDIV::Div24
    }
    ///pll_p_ck = vco_ck / 25
    #[inline(always)]
    pub fn is_div25(&self) -> bool {
        *self == PLLPDIV::Div25
    }
    ///pll_p_ck = vco_ck / 26
    #[inline(always)]
    pub fn is_div26(&self) -> bool {
        *self == PLLPDIV::Div26
    }
    ///pll_p_ck = vco_ck / 27
    #[inline(always)]
    pub fn is_div27(&self) -> bool {
        *self == PLLPDIV::Div27
    }
    ///pll_p_ck = vco_ck / 28
    #[inline(always)]
    pub fn is_div28(&self) -> bool {
        *self == PLLPDIV::Div28
    }
    ///pll_p_ck = vco_ck / 29
    #[inline(always)]
    pub fn is_div29(&self) -> bool {
        *self == PLLPDIV::Div29
    }
    ///pll_p_ck = vco_ck / 30
    #[inline(always)]
    pub fn is_div30(&self) -> bool {
        *self == PLLPDIV::Div30
    }
    ///pll_p_ck = vco_ck / 31
    #[inline(always)]
    pub fn is_div31(&self) -> bool {
        *self == PLLPDIV::Div31
    }
}
///Field `PLLPDIV` writer - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ....
pub type PLLPDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5, PLLPDIV>;
impl<'a, REG> PLLPDIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///pll_p_ck is controlled by PLLP
    #[inline(always)]
    pub fn pllp(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Pllp)
    }
    ///pll_p_ck = vco_ck / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div2)
    }
    ///pll_p_ck = vco_ck / 3
    #[inline(always)]
    pub fn div3(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div3)
    }
    ///pll_p_ck = vco_ck / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div4)
    }
    ///pll_p_ck = vco_ck / 5
    #[inline(always)]
    pub fn div5(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div5)
    }
    ///pll_p_ck = vco_ck / 6
    #[inline(always)]
    pub fn div6(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div6)
    }
    ///pll_p_ck = vco_ck / 7
    #[inline(always)]
    pub fn div7(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div7)
    }
    ///pll_p_ck = vco_ck / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div8)
    }
    ///pll_p_ck = vco_ck / 9
    #[inline(always)]
    pub fn div9(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div9)
    }
    ///pll_p_ck = vco_ck / 10
    #[inline(always)]
    pub fn div10(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div10)
    }
    ///pll_p_ck = vco_ck / 11
    #[inline(always)]
    pub fn div11(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div11)
    }
    ///pll_p_ck = vco_ck / 12
    #[inline(always)]
    pub fn div12(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div12)
    }
    ///pll_p_ck = vco_ck / 13
    #[inline(always)]
    pub fn div13(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div13)
    }
    ///pll_p_ck = vco_ck / 14
    #[inline(always)]
    pub fn div14(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div14)
    }
    ///pll_p_ck = vco_ck / 15
    #[inline(always)]
    pub fn div15(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div15)
    }
    ///pll_p_ck = vco_ck / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div16)
    }
    ///pll_p_ck = vco_ck / 17
    #[inline(always)]
    pub fn div17(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div17)
    }
    ///pll_p_ck = vco_ck / 18
    #[inline(always)]
    pub fn div18(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div18)
    }
    ///pll_p_ck = vco_ck / 19
    #[inline(always)]
    pub fn div19(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div19)
    }
    ///pll_p_ck = vco_ck / 20
    #[inline(always)]
    pub fn div20(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div20)
    }
    ///pll_p_ck = vco_ck / 21
    #[inline(always)]
    pub fn div21(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div21)
    }
    ///pll_p_ck = vco_ck / 22
    #[inline(always)]
    pub fn div22(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div22)
    }
    ///pll_p_ck = vco_ck / 23
    #[inline(always)]
    pub fn div23(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div23)
    }
    ///pll_p_ck = vco_ck / 24
    #[inline(always)]
    pub fn div24(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div24)
    }
    ///pll_p_ck = vco_ck / 25
    #[inline(always)]
    pub fn div25(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div25)
    }
    ///pll_p_ck = vco_ck / 26
    #[inline(always)]
    pub fn div26(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div26)
    }
    ///pll_p_ck = vco_ck / 27
    #[inline(always)]
    pub fn div27(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div27)
    }
    ///pll_p_ck = vco_ck / 28
    #[inline(always)]
    pub fn div28(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div28)
    }
    ///pll_p_ck = vco_ck / 29
    #[inline(always)]
    pub fn div29(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div29)
    }
    ///pll_p_ck = vco_ck / 30
    #[inline(always)]
    pub fn div30(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div30)
    }
    ///pll_p_ck = vco_ck / 31
    #[inline(always)]
    pub fn div31(self) -> &'a mut crate::W<REG> {
        self.variant(PLLPDIV::Div31)
    }
}
impl R {
    ///Bits 0:1 - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00.
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new((self.bits & 3) as u8)
    }
    ///Bits 4:7 - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet.
    #[inline(always)]
    pub fn pllm(&self) -> PLLM_R {
        PLLM_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet.
    #[inline(always)]
    pub fn plln(&self) -> PLLN_R {
        PLLN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    ///Bit 16 - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0.
    #[inline(always)]
    pub fn pllpen(&self) -> PLLPEN_R {
        PLLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\[4:0\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
    #[inline(always)]
    pub fn pllp(&self) -> PLLP_R {
        PLLP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0.
    #[inline(always)]
    pub fn pllqen(&self) -> PLLQEN_R {
        PLLQEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
    #[inline(always)]
    pub fn pllq(&self) -> PLLQ_R {
        PLLQ_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 24 - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0.
    #[inline(always)]
    pub fn pllren(&self) -> PLLREN_R {
        PLLREN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
    #[inline(always)]
    pub fn pllr(&self) -> PLLR_R {
        PLLR_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bits 27:31 - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ....
    #[inline(always)]
    pub fn pllpdiv(&self) -> PLLPDIV_R {
        PLLPDIV_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLLCFGR")
            .field("pllsrc", &self.pllsrc())
            .field("pllm", &self.pllm())
            .field("plln", &self.plln())
            .field("pllpen", &self.pllpen())
            .field("pllp", &self.pllp())
            .field("pllqen", &self.pllqen())
            .field("pllq", &self.pllq())
            .field("pllren", &self.pllren())
            .field("pllr", &self.pllr())
            .field("pllpdiv", &self.pllpdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Main PLL entry clock source Set and cleared by software to select PLL clock source. These bits can be written only when PLL is disabled. In order to save power, when no PLL is used, the value of PLLSRC should be 00.
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W<PLLCFGRrs> {
        PLLSRC_W::new(self, 0)
    }
    ///Bits 4:7 - Division factor for the main PLL input clock Set and cleared by software to divide the PLL input clock before the VCO. These bits can be written only when all PLLs are disabled. VCO input frequency = PLL input clock frequency / PLLM with 1 <= PLLM <= 16 ... Note: The software has to set these bits correctly to ensure that the VCO input frequency is within the range defined in the device datasheet.
    #[inline(always)]
    pub fn pllm(&mut self) -> PLLM_W<PLLCFGRrs> {
        PLLM_W::new(self, 4)
    }
    ///Bits 8:14 - Main PLL multiplication factor for VCO Set and cleared by software to control the multiplication factor of the VCO. These bits can be written only when the PLL is disabled. VCO output frequency = VCO input frequency x PLLN with 8 =< PLLN =< 127 ... ... Note: The software has to set correctly these bits to assure that the VCO output frequency is within the range defined in the device datasheet.
    #[inline(always)]
    pub fn plln(&mut self) -> PLLN_W<PLLCFGRrs> {
        PLLN_W::new(self, 8)
    }
    ///Bit 16 - Main PLL PLL P clock output enable Set and reset by software to enable the PLL P clock output of the PLL. In order to save power, when the PLL P clock output of the PLL is not used, the value of PLLPEN should be 0.
    #[inline(always)]
    pub fn pllpen(&mut self) -> PLLPEN_W<PLLCFGRrs> {
        PLLPEN_W::new(self, 16)
    }
    ///Bit 17 - Main PLL division factor for PLL P clock. Set and cleared by software to control the frequency of the main PLL output clock PLL P clock. These bits can be written only if PLL is disabled. When the PLLPDIV\[4:0\] is set to 00000PLL P output clock frequency = VCO frequency / PLLP with PLLP =7, or 17 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
    #[inline(always)]
    pub fn pllp(&mut self) -> PLLP_W<PLLCFGRrs> {
        PLLP_W::new(self, 17)
    }
    ///Bit 20 - Main PLL Q clock output enable Set and reset by software to enable the PLL Q clock output of the PLL. In order to save power, when the PLL Q clock output of the PLL is not used, the value of PLLQEN should be 0.
    #[inline(always)]
    pub fn pllqen(&mut self) -> PLLQEN_W<PLLCFGRrs> {
        PLLQEN_W::new(self, 20)
    }
    ///Bits 21:22 - Main PLL division factor for PLL Q clock. Set and cleared by software to control the frequency of the main PLL output clock PLL Q clock. This output can be selected for USB, RNG, SAI (48 MHz clock). These bits can be written only if PLL is disabled. PLL Q output clock frequency = VCO frequency / PLLQ with PLLQ = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
    #[inline(always)]
    pub fn pllq(&mut self) -> PLLQ_W<PLLCFGRrs> {
        PLLQ_W::new(self, 21)
    }
    ///Bit 24 - PLL R clock output enable Set and reset by software to enable the PLL R clock output of the PLL (used as system clock). This bit cannot be written when PLL R clock output of the PLL is used as System Clock. In order to save power, when the PLL R clock output of the PLL is not used, the value of PLLREN should be 0.
    #[inline(always)]
    pub fn pllren(&mut self) -> PLLREN_W<PLLCFGRrs> {
        PLLREN_W::new(self, 24)
    }
    ///Bits 25:26 - Main PLL division factor for PLL R clock (system clock) Set and cleared by software to control the frequency of the main PLL output clock PLLCLK. This output can be selected as system clock. These bits can be written only if PLL is disabled. PLL R output clock frequency = VCO frequency / PLLR with PLLR = 2, 4, 6, or 8 Note: The software has to set these bits correctly not to exceed 170 MHz on this domain.
    #[inline(always)]
    pub fn pllr(&mut self) -> PLLR_W<PLLCFGRrs> {
        PLLR_W::new(self, 25)
    }
    ///Bits 27:31 - Main PLLP division factor Set and cleared by software to control the PLL P frequency. PLL P output clock frequency = VCO frequency / PLLPDIV. ....
    #[inline(always)]
    pub fn pllpdiv(&mut self) -> PLLPDIV_W<PLLCFGRrs> {
        PLLPDIV_W::new(self, 27)
    }
}
/**PLL configuration register

You can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G483.html#RCC:PLLCFGR)*/
pub struct PLLCFGRrs;
impl crate::RegisterSpec for PLLCFGRrs {
    type Ux = u32;
}
///`read()` method returns [`pllcfgr::R`](R) reader structure
impl crate::Readable for PLLCFGRrs {}
///`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure
impl crate::Writable for PLLCFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLLCFGR to value 0x1000
impl crate::Resettable for PLLCFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
