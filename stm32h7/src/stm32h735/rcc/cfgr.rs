#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `SW` reader - System clock switch"]
pub struct SW_R(crate::FieldReader<u8, SW_A>);
impl SW_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SW_A> {
        match self.bits {
            0 => Some(SW_A::HSI),
            1 => Some(SW_A::CSI),
            2 => Some(SW_A::HSE),
            3 => Some(SW_A::PLL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == SW_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        **self == SW_A::CSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == SW_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1`"]
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        **self == SW_A::PLL1
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<u8, SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW` writer - System clock switch"]
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
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
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
#[doc = "Field `SWS` reader - System clock switch status"]
pub struct SWS_R(crate::FieldReader<u8, SWS_A>);
impl SWS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWS_A> {
        match self.bits {
            0 => Some(SWS_A::HSI),
            1 => Some(SWS_A::CSI),
            2 => Some(SWS_A::HSE),
            3 => Some(SWS_A::PLL1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == SWS_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        **self == SWS_A::CSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == SWS_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1`"]
    #[inline(always)]
    pub fn is_pll1(&self) -> bool {
        **self == SWS_A::PLL1
    }
}
impl core::ops::Deref for SWS_R {
    type Target = crate::FieldReader<u8, SWS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWS` writer - System clock switch status"]
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
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
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
#[doc = "Field `STOPWUCK` reader - System clock selection after a wake up from system Stop"]
pub struct STOPWUCK_R(crate::FieldReader<bool, STOPWUCK_A>);
impl STOPWUCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        STOPWUCK_R(crate::FieldReader::new(bits))
    }
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
        **self == STOPWUCK_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        **self == STOPWUCK_A::CSI
    }
}
impl core::ops::Deref for STOPWUCK_R {
    type Target = crate::FieldReader<bool, STOPWUCK_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STOPWUCK` writer - System clock selection after a wake up from system Stop"]
pub struct STOPWUCK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPWUCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPWUCK_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Kernel clock selection after a wake up from system Stop"]
pub type STOPKERWUCK_A = STOPWUCK_A;
#[doc = "Field `STOPKERWUCK` reader - Kernel clock selection after a wake up from system Stop"]
pub type STOPKERWUCK_R = STOPWUCK_R;
#[doc = "Field `STOPKERWUCK` writer - Kernel clock selection after a wake up from system Stop"]
pub struct STOPKERWUCK_W<'a> {
    w: &'a mut W,
}
impl<'a> STOPKERWUCK_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: STOPKERWUCK_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(STOPKERWUCK_A::HSI)
    }
    #[doc = "CSI selected as wake up clock from system Stop"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(STOPKERWUCK_A::CSI)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `RTCPRE` reader - HSE division factor for RTC clock"]
pub struct RTCPRE_R(crate::FieldReader<u8, u8>);
impl RTCPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        RTCPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RTCPRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RTCPRE` writer - HSE division factor for RTC clock"]
pub struct RTCPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCPRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `HRTIMSEL` reader - High Resolution Timer clock prescaler selection"]
pub struct HRTIMSEL_R(crate::FieldReader<bool, bool>);
impl HRTIMSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        HRTIMSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HRTIMSEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HRTIMSEL` writer - High Resolution Timer clock prescaler selection"]
pub struct HRTIMSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HRTIMSEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
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
#[doc = "Field `TIMPRE` reader - Timers clocks prescaler selection"]
pub struct TIMPRE_R(crate::FieldReader<bool, TIMPRE_A>);
impl TIMPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TIMPRE_R(crate::FieldReader::new(bits))
    }
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
        **self == TIMPRE_A::DEFAULTX2
    }
    #[doc = "Checks if the value of the field is `DEFAULTX4`"]
    #[inline(always)]
    pub fn is_default_x4(&self) -> bool {
        **self == TIMPRE_A::DEFAULTX4
    }
}
impl core::ops::Deref for TIMPRE_R {
    type Target = crate::FieldReader<bool, TIMPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TIMPRE` writer - Timers clocks prescaler selection"]
pub struct TIMPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> TIMPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: TIMPRE_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
#[doc = "Field `MCO1PRE` reader - MCO1 prescaler"]
pub struct MCO1PRE_R(crate::FieldReader<u8, u8>);
impl MCO1PRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCO1PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCO1PRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO1PRE` writer - MCO1 prescaler"]
pub struct MCO1PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO1PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
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
#[doc = "Field `MCO1` reader - Micro-controller clock output 1"]
pub struct MCO1_R(crate::FieldReader<u8, MCO1_A>);
impl MCO1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCO1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCO1_A> {
        match self.bits {
            0 => Some(MCO1_A::HSI),
            1 => Some(MCO1_A::LSE),
            2 => Some(MCO1_A::HSE),
            3 => Some(MCO1_A::PLL1_Q),
            4 => Some(MCO1_A::HSI48),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == MCO1_A::HSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == MCO1_A::LSE
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == MCO1_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        **self == MCO1_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        **self == MCO1_A::HSI48
    }
}
impl core::ops::Deref for MCO1_R {
    type Target = crate::FieldReader<u8, MCO1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO1` writer - Micro-controller clock output 1"]
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
        self.w.bits = (self.w.bits & !(0x07 << 22)) | ((value as u32 & 0x07) << 22);
        self.w
    }
}
#[doc = "Field `MCO2PRE` reader - MCO2 prescaler"]
pub struct MCO2PRE_R(crate::FieldReader<u8, u8>);
impl MCO2PRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCO2PRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCO2PRE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO2PRE` writer - MCO2 prescaler"]
pub struct MCO2PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO2PRE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 25)) | ((value as u32 & 0x0f) << 25);
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
#[doc = "Field `MCO2` reader - Micro-controller clock output 2"]
pub struct MCO2_R(crate::FieldReader<u8, MCO2_A>);
impl MCO2_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCO2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCO2_A> {
        match self.bits {
            0 => Some(MCO2_A::SYSCLK),
            1 => Some(MCO2_A::PLL2_P),
            2 => Some(MCO2_A::HSE),
            3 => Some(MCO2_A::PLL1_P),
            4 => Some(MCO2_A::CSI),
            5 => Some(MCO2_A::LSI),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == MCO2_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `PLL2_P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        **self == MCO2_A::PLL2_P
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == MCO2_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1_P`"]
    #[inline(always)]
    pub fn is_pll1_p(&self) -> bool {
        **self == MCO2_A::PLL1_P
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        **self == MCO2_A::CSI
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == MCO2_A::LSI
    }
}
impl core::ops::Deref for MCO2_R {
    type Target = crate::FieldReader<u8, MCO2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO2` writer - Micro-controller clock output 2"]
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
        self.w.bits = (self.w.bits & !(0x07 << 29)) | ((value as u32 & 0x07) << 29);
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
