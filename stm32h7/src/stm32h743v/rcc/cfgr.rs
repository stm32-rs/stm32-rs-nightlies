#[doc = "Reader of register CFGR"]
pub type R = crate::R<u32, super::CFGR>;
#[doc = "Writer for register CFGR"]
pub type W = crate::W<u32, super::CFGR>;
#[doc = "Register CFGR `reset()`'s with value 0"]
impl crate::ResetValue for super::CFGR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "System clock switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "0: HSI selected as system clock"]
    HSI = 0,
    #[doc = "1: CSI selected as system clock"]
    CSI = 1,
    #[doc = "2: HSE selected as system clock"]
    HSE = 2,
    #[doc = "3: PLL1 selected as system clock"]
    PLL1 = 3,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SW`"]
pub type SW_R = crate::R<u8, SW_A>;
impl SW_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SW_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SW_A::HSI),
            1 => Val(SW_A::CSI),
            2 => Val(SW_A::HSE),
            3 => Val(SW_A::PLL1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SW_A::CSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1`"]
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SW_A::PLL1
    }
}
#[doc = "Write proxy for field `SW`"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SW_A::HSI)
    }
    #[doc = "CSI selected as system clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(SW_A::CSI)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::HSE)
    }
    #[doc = "PLL1 selected as system clock"]
    #[inline(always)]
    pub fn pll1(self) -> &'a mut W {
        self.variant(SW_A::PLL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "System clock switch status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWS_A {
    #[doc = "0: HSI oscillator used as system clock"]
    HSI = 0,
    #[doc = "1: CSI oscillator used as system clock"]
    CSI = 1,
    #[doc = "2: HSE oscillator used as system clock"]
    HSE = 2,
    #[doc = "3: PLL1 used as system clock"]
    PLL1 = 3,
}
impl From<SWS_A> for u8 {
    #[inline(always)]
    fn from(variant: SWS_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SWS`"]
pub type SWS_R = crate::R<u8, SWS_A>;
impl SWS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SWS_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SWS_A::HSI),
            1 => Val(SWS_A::CSI),
            2 => Val(SWS_A::HSE),
            3 => Val(SWS_A::PLL1),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWS_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == SWS_A::CSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWS_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1`"]
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        *self == SWS_A::PLL1
    }
}
#[doc = "Write proxy for field `SWS`"]
pub struct SWS_W<'a> {
    w: &'a mut W,
}
impl<'a> SWS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWS_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI oscillator used as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SWS_A::HSI)
    }
    #[doc = "CSI oscillator used as system clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(SWS_A::CSI)
    }
    #[doc = "HSE oscillator used as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SWS_A::HSE)
    }
    #[doc = "PLL1 used as system clock"]
    #[inline(always)]
    pub fn pll1(self) -> &'a mut W {
        self.variant(SWS_A::PLL1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "System clock selection after a wake up from system Stop\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum STOPWUCK_A {
    #[doc = "0: HSI selected as wake up clock from system Stop"]
    HSI = 0,
    #[doc = "1: CSI selected as wake up clock from system Stop"]
    CSI = 1,
}
impl From<STOPWUCK_A> for bool {
    #[inline(always)]
    fn from(variant: STOPWUCK_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `STOPWUCK`"]
pub type STOPWUCK_R = crate::R<bool, STOPWUCK_A>;
impl STOPWUCK_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPWUCK_A {
        match self.bits {
            false => STOPWUCK_A::HSI,
            true => STOPWUCK_A::CSI,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == STOPWUCK_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == STOPWUCK_A::CSI
    }
}
#[doc = "Write proxy for field `STOPWUCK`"]
pub struct STOPWUCK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPWUCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPWUCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::HSI)
    }
    #[doc = "CSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::CSI)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Kernel clock selection after a wake up from system Stop"]
pub type STOPKERWUCK_A = STOPWUCK_A;
#[doc = "Reader of field `STOPKERWUCK`"]
pub type STOPKERWUCK_R = crate::R<bool, STOPWUCK_A>;
#[doc = "Write proxy for field `STOPKERWUCK`"]
pub struct STOPKERWUCK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPKERWUCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPKERWUCK_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "HSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::HSI)
    }
    #[doc = "CSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(STOPWUCK_A::CSI)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RTCPRE`"]
pub type RTCPRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RTCPRE`"]
pub struct RTCPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | (((value as u32) & 0x3f) << 8);
        self.w
    }
}
#[doc = "High Resolution Timer clock prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HRTIMSEL_A {
    #[doc = "0: The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)"]
    TIMY_KER = 0,
    #[doc = "1: The HRTIM prescaler clock source is the CPU clock (c_ck)"]
    C_CK = 1,
}
impl From<HRTIMSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HRTIMSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `HRTIMSEL`"]
pub type HRTIMSEL_R = crate::R<bool, HRTIMSEL_A>;
impl HRTIMSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRTIMSEL_A {
        match self.bits {
            false => HRTIMSEL_A::TIMY_KER,
            true => HRTIMSEL_A::C_CK,
        }
    }
    #[doc = "Checks if the value of the field is `TIMY_KER`"]
    #[inline(always)]
    pub fn is_timy_ker(&self) -> bool {
        *self == HRTIMSEL_A::TIMY_KER
    }
    #[doc = "Checks if the value of the field is `C_CK`"]
    #[inline(always)]
    pub fn is_c_ck(&self) -> bool {
        *self == HRTIMSEL_A::C_CK
    }
}
#[doc = "Write proxy for field `HRTIMSEL`"]
pub struct HRTIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HRTIMSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HRTIMSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "The HRTIM prescaler clock source is the same as other timers (rcc_timy_ker_ck)"]
    #[inline(always)]
    pub fn timy_ker(self) -> &'a mut W {
        self.variant(HRTIMSEL_A::TIMY_KER)
    }
    #[doc = "The HRTIM prescaler clock source is the CPU clock (c_ck)"]
    #[inline(always)]
    pub fn c_ck(self) -> &'a mut W {
        self.variant(HRTIMSEL_A::C_CK)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Timers clocks prescaler selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TIMPRE_A {
    #[doc = "0: Timer kernel clock equal to 2x pclk by default"]
    DEFAULTX2 = 0,
    #[doc = "1: Timer kernel clock equal to 4x pclk by default"]
    DEFAULTX4 = 1,
}
impl From<TIMPRE_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `TIMPRE`"]
pub type TIMPRE_R = crate::R<bool, TIMPRE_A>;
impl TIMPRE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPRE_A {
        match self.bits {
            false => TIMPRE_A::DEFAULTX2,
            true => TIMPRE_A::DEFAULTX4,
        }
    }
    #[doc = "Checks if the value of the field is `DEFAULTX2`"]
    #[inline(always)]
    pub fn is_default_x2(&self) -> bool {
        *self == TIMPRE_A::DEFAULTX2
    }
    #[doc = "Checks if the value of the field is `DEFAULTX4`"]
    #[inline(always)]
    pub fn is_default_x4(&self) -> bool {
        *self == TIMPRE_A::DEFAULTX4
    }
}
#[doc = "Write proxy for field `TIMPRE`"]
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMPRE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Timer kernel clock equal to 2x pclk by default"]
    #[inline(always)]
    pub fn default_x2(self) -> &'a mut W {
        self.variant(TIMPRE_A::DEFAULTX2)
    }
    #[doc = "Timer kernel clock equal to 4x pclk by default"]
    #[inline(always)]
    pub fn default_x4(self) -> &'a mut W {
        self.variant(TIMPRE_A::DEFAULTX4)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `MCO1PRE`"]
pub type MCO1PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCO1PRE`"]
pub struct MCO1PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | (((value as u32) & 0x0f) << 18);
        self.w
    }
}
#[doc = "Micro-controller clock output 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO1_A {
    #[doc = "0: HSI selected for micro-controller clock output"]
    HSI = 0,
    #[doc = "1: LSE selected for micro-controller clock output"]
    LSE = 1,
    #[doc = "2: HSE selected for micro-controller clock output"]
    HSE = 2,
    #[doc = "3: pll1_q selected for micro-controller clock output"]
    PLL1_Q = 3,
    #[doc = "4: HSI48 selected for micro-controller clock output"]
    HSI48 = 4,
}
impl From<MCO1_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO1_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCO1`"]
pub type MCO1_R = crate::R<u8, MCO1_A>;
impl MCO1_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCO1_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCO1_A::HSI),
            1 => Val(MCO1_A::LSE),
            2 => Val(MCO1_A::HSE),
            3 => Val(MCO1_A::PLL1_Q),
            4 => Val(MCO1_A::HSI48),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO1_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO1_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO1_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == MCO1_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == MCO1_A::HSI48
    }
}
#[doc = "Write proxy for field `MCO1`"]
pub struct MCO1_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI selected for micro-controller clock output"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO1_A::HSI)
    }
    #[doc = "LSE selected for micro-controller clock output"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCO1_A::LSE)
    }
    #[doc = "HSE selected for micro-controller clock output"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO1_A::HSE)
    }
    #[doc = "pll1_q selected for micro-controller clock output"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(MCO1_A::PLL1_Q)
    }
    #[doc = "HSI48 selected for micro-controller clock output"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(MCO1_A::HSI48)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 22)) | (((value as u32) & 0x07) << 22);
        self.w
    }
}
#[doc = "Reader of field `MCO2PRE`"]
pub type MCO2PRE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `MCO2PRE`"]
pub struct MCO2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | (((value as u32) & 0x0f) << 25);
        self.w
    }
}
#[doc = "Micro-controller clock output 2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO2_A {
    #[doc = "0: System clock selected for micro-controller clock output"]
    SYSCLK = 0,
    #[doc = "1: pll2_p selected for micro-controller clock output"]
    PLL2_P = 1,
    #[doc = "2: HSE selected for micro-controller clock output"]
    HSE = 2,
    #[doc = "3: pll1_p selected for micro-controller clock output"]
    PLL1_P = 3,
    #[doc = "4: CSI selected for micro-controller clock output"]
    CSI = 4,
    #[doc = "5: LSI selected for micro-controller clock output"]
    LSI = 5,
}
impl From<MCO2_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO2_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MCO2`"]
pub type MCO2_R = crate::R<u8, MCO2_A>;
impl MCO2_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, MCO2_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(MCO2_A::SYSCLK),
            1 => Val(MCO2_A::PLL2_P),
            2 => Val(MCO2_A::HSE),
            3 => Val(MCO2_A::PLL1_P),
            4 => Val(MCO2_A::CSI),
            5 => Val(MCO2_A::LSI),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO2_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `PLL2_P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == MCO2_A::PLL2_P
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO2_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1_P`"]
    #[inline(always)]
    pub fn is_pll1_p(&self) -> bool {
        *self == MCO2_A::PLL1_P
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == MCO2_A::CSI
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO2_A::LSI
    }
}
#[doc = "Write proxy for field `MCO2`"]
pub struct MCO2_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "System clock selected for micro-controller clock output"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO2_A::SYSCLK)
    }
    #[doc = "pll2_p selected for micro-controller clock output"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(MCO2_A::PLL2_P)
    }
    #[doc = "HSE selected for micro-controller clock output"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO2_A::HSE)
    }
    #[doc = "pll1_p selected for micro-controller clock output"]
    #[inline(always)]
    pub fn pll1_p(self) -> &'a mut W {
        self.variant(MCO2_A::PLL1_P)
    }
    #[doc = "CSI selected for micro-controller clock output"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(MCO2_A::CSI)
    }
    #[doc = "LSI selected for micro-controller clock output"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCO2_A::LSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 29)) | (((value as u32) & 0x07) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - System clock selection after a wake up from system Stop"]
    #[inline(always)]
    pub fn stopwuck(&self) -> STOPWUCK_R {
        STOPWUCK_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Kernel clock selection after a wake up from system Stop"]
    #[inline(always)]
    pub fn stopkerwuck(&self) -> STOPKERWUCK_R {
        STOPKERWUCK_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RTCPRE_R {
        RTCPRE_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - High Resolution Timer clock prescaler selection"]
    #[inline(always)]
    pub fn hrtimsel(&self) -> HRTIMSEL_R {
        HRTIMSEL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Timers clocks prescaler selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TIMPRE_R {
        TIMPRE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&self) -> MCO1PRE_R {
        MCO1PRE_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:24 - Micro-controller clock output 1"]
    #[inline(always)]
    pub fn mco1(&self) -> MCO1_R {
        MCO1_R::new(((self.bits >> 22) & 0x07) as u8)
    }
    #[doc = "Bits 25:28 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&self) -> MCO2PRE_R {
        MCO2PRE_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - Micro-controller clock output 2"]
    #[inline(always)]
    pub fn mco2(&self) -> MCO2_R {
        MCO2_R::new(((self.bits >> 29) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    #[doc = "Bits 3:5 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&mut self) -> SWS_W {
        SWS_W { w: self }
    }
    #[doc = "Bit 6 - System clock selection after a wake up from system Stop"]
    #[inline(always)]
    pub fn stopwuck(&mut self) -> STOPWUCK_W {
        STOPWUCK_W { w: self }
    }
    #[doc = "Bit 7 - Kernel clock selection after a wake up from system Stop"]
    #[inline(always)]
    pub fn stopkerwuck(&mut self) -> STOPKERWUCK_W {
        STOPKERWUCK_W { w: self }
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RTCPRE_W {
        RTCPRE_W { w: self }
    }
    #[doc = "Bit 14 - High Resolution Timer clock prescaler selection"]
    #[inline(always)]
    pub fn hrtimsel(&mut self) -> HRTIMSEL_W {
        HRTIMSEL_W { w: self }
    }
    #[doc = "Bit 15 - Timers clocks prescaler selection"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TIMPRE_W {
        TIMPRE_W { w: self }
    }
    #[doc = "Bits 18:21 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&mut self) -> MCO1PRE_W {
        MCO1PRE_W { w: self }
    }
    #[doc = "Bits 22:24 - Micro-controller clock output 1"]
    #[inline(always)]
    pub fn mco1(&mut self) -> MCO1_W {
        MCO1_W { w: self }
    }
    #[doc = "Bits 25:28 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&mut self) -> MCO2PRE_W {
        MCO2PRE_W { w: self }
    }
    #[doc = "Bits 29:31 - Micro-controller clock output 2"]
    #[inline(always)]
    pub fn mco2(&mut self) -> MCO2_W {
        MCO2_W { w: self }
    }
}
