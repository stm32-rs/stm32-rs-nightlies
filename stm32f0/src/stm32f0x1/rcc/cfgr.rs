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
#[doc = "System clock Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "3: HSI48 selected as system clock (when available)"]
    HSI48 = 3,
    #[doc = "0: HSI selected as system clock"]
    HSI = 0,
    #[doc = "1: HSE selected as system clock"]
    HSE = 1,
    #[doc = "2: PLL selected as system clock"]
    PLL = 2,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SW` reader - System clock Switch"]
pub struct SW_R(crate::FieldReader<u8, SW_A>);
impl SW_R {
    pub(crate) fn new(bits: u8) -> Self {
        SW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SW_A {
        match self.bits {
            3 => SW_A::HSI48,
            0 => SW_A::HSI,
            1 => SW_A::HSE,
            2 => SW_A::PLL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        **self == SW_A::HSI48
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == SW_A::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == SW_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == SW_A::PLL
    }
}
impl core::ops::Deref for SW_R {
    type Target = crate::FieldReader<u8, SW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SW` writer - System clock Switch"]
pub struct SW_W<'a> {
    w: &'a mut W,
}
impl<'a> SW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "HSI48 selected as system clock (when available)"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(SW_A::HSI48)
    }
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SW_A::HSI)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::HSE)
    }
    #[doc = "PLL selected as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SW_A::PLL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "System Clock Switch Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SWS_A {
    #[doc = "3: HSI48 used as system clock (when avaiable)"]
    HSI48 = 3,
    #[doc = "0: HSI oscillator used as system clock"]
    HSI = 0,
    #[doc = "1: HSE oscillator used as system clock"]
    HSE = 1,
    #[doc = "2: PLL used as system clock"]
    PLL = 2,
}
impl From<SWS_A> for u8 {
    #[inline(always)]
    fn from(variant: SWS_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SWS` reader - System Clock Switch Status"]
pub struct SWS_R(crate::FieldReader<u8, SWS_A>);
impl SWS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SWS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWS_A {
        match self.bits {
            3 => SWS_A::HSI48,
            0 => SWS_A::HSI,
            1 => SWS_A::HSE,
            2 => SWS_A::PLL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        **self == SWS_A::HSI48
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == SWS_A::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == SWS_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == SWS_A::PLL
    }
}
impl core::ops::Deref for SWS_R {
    type Target = crate::FieldReader<u8, SWS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "0: SYSCLK not divided"]
    DIV1 = 0,
    #[doc = "8: SYSCLK divided by 2"]
    DIV2 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    DIV4 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    DIV8 = 10,
    #[doc = "11: SYSCLK divided by 16"]
    DIV16 = 11,
    #[doc = "12: SYSCLK divided by 64"]
    DIV64 = 12,
    #[doc = "13: SYSCLK divided by 128"]
    DIV128 = 13,
    #[doc = "14: SYSCLK divided by 256"]
    DIV256 = 14,
    #[doc = "15: SYSCLK divided by 512"]
    DIV512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub struct HPRE_R(crate::FieldReader<u8, HPRE_A>);
impl HPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        HPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::DIV1),
            8 => Some(HPRE_A::DIV2),
            9 => Some(HPRE_A::DIV4),
            10 => Some(HPRE_A::DIV8),
            11 => Some(HPRE_A::DIV16),
            12 => Some(HPRE_A::DIV64),
            13 => Some(HPRE_A::DIV128),
            14 => Some(HPRE_A::DIV256),
            15 => Some(HPRE_A::DIV512),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == HPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == HPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == HPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == HPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == HPRE_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == HPRE_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == HPRE_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        **self == HPRE_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        **self == HPRE_A::DIV512
    }
}
impl core::ops::Deref for HPRE_R {
    type Target = crate::FieldReader<u8, HPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub struct HPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> HPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::DIV1)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::DIV2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::DIV4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::DIV8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::DIV16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::DIV64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::DIV128)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::DIV256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::DIV512)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "APB Low speed prescaler (APB1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PPRE_A {
    #[doc = "0: HCLK not divided"]
    DIV1 = 0,
    #[doc = "4: HCLK divided by 2"]
    DIV2 = 4,
    #[doc = "5: HCLK divided by 4"]
    DIV4 = 5,
    #[doc = "6: HCLK divided by 8"]
    DIV8 = 6,
    #[doc = "7: HCLK divided by 16"]
    DIV16 = 7,
}
impl From<PPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PPRE` reader - APB Low speed prescaler (APB1)"]
pub struct PPRE_R(crate::FieldReader<u8, PPRE_A>);
impl PPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        PPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE_A> {
        match self.bits {
            0 => Some(PPRE_A::DIV1),
            4 => Some(PPRE_A::DIV2),
            5 => Some(PPRE_A::DIV4),
            6 => Some(PPRE_A::DIV8),
            7 => Some(PPRE_A::DIV16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == PPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == PPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == PPRE_A::DIV16
    }
}
impl core::ops::Deref for PPRE_R {
    type Target = crate::FieldReader<u8, PPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PPRE` writer - APB Low speed prescaler (APB1)"]
pub struct PPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PPRE_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE_A::DIV1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE_A::DIV2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE_A::DIV4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE_A::DIV8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE_A::DIV16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "Field `ADCPRE` reader - APCPRE is deprecated. See ADC field in CFGR2 register."]
pub struct ADCPRE_R(crate::FieldReader<bool, bool>);
impl ADCPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ADCPRE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADCPRE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCPRE` writer - APCPRE is deprecated. See ADC field in CFGR2 register."]
pub struct ADCPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCPRE_W<'a> {
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
#[doc = "PLL input clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "3: HSI48 divided by PREDIV selected as PLL input clock"]
    HSI48_DIV_PREDIV = 3,
    #[doc = "0: HSI divided by 2 selected as PLL input clock"]
    HSI_DIV2 = 0,
    #[doc = "1: HSI divided by PREDIV selected as PLL input clock"]
    HSI_DIV_PREDIV = 1,
    #[doc = "2: HSE divided by PREDIV selected as PLL input clock"]
    HSE_DIV_PREDIV = 2,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLSRC` reader - PLL input clock source"]
pub struct PLLSRC_R(crate::FieldReader<u8, PLLSRC_A>);
impl PLLSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLSRC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLSRC_A {
        match self.bits {
            3 => PLLSRC_A::HSI48_DIV_PREDIV,
            0 => PLLSRC_A::HSI_DIV2,
            1 => PLLSRC_A::HSI_DIV_PREDIV,
            2 => PLLSRC_A::HSE_DIV_PREDIV,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HSI48_DIV_PREDIV`"]
    #[inline(always)]
    pub fn is_hsi48_div_prediv(&self) -> bool {
        **self == PLLSRC_A::HSI48_DIV_PREDIV
    }
    #[doc = "Checks if the value of the field is `HSI_DIV2`"]
    #[inline(always)]
    pub fn is_hsi_div2(&self) -> bool {
        **self == PLLSRC_A::HSI_DIV2
    }
    #[doc = "Checks if the value of the field is `HSI_DIV_PREDIV`"]
    #[inline(always)]
    pub fn is_hsi_div_prediv(&self) -> bool {
        **self == PLLSRC_A::HSI_DIV_PREDIV
    }
    #[doc = "Checks if the value of the field is `HSE_DIV_PREDIV`"]
    #[inline(always)]
    pub fn is_hse_div_prediv(&self) -> bool {
        **self == PLLSRC_A::HSE_DIV_PREDIV
    }
}
impl core::ops::Deref for PLLSRC_R {
    type Target = crate::FieldReader<u8, PLLSRC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLSRC` writer - PLL input clock source"]
pub struct PLLSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLSRC_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "HSI48 divided by PREDIV selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi48_div_prediv(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI48_DIV_PREDIV)
    }
    #[doc = "HSI divided by 2 selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi_div2(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI_DIV2)
    }
    #[doc = "HSI divided by PREDIV selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi_div_prediv(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI_DIV_PREDIV)
    }
    #[doc = "HSE divided by PREDIV selected as PLL input clock"]
    #[inline(always)]
    pub fn hse_div_prediv(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE_DIV_PREDIV)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 15)) | ((value as u32 & 0x03) << 15);
        self.w
    }
}
#[doc = "HSE divider for PLL entry. Same bit as PREDIC\\[0\\]
from CFGR2 register. Refer to it for its meaning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLXTPRE_A {
    #[doc = "0: HSE clock not divided"]
    DIV1 = 0,
    #[doc = "1: HSE clock divided by 2"]
    DIV2 = 1,
}
impl From<PLLXTPRE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLXTPRE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry. Same bit as PREDIC\\[0\\]
from CFGR2 register. Refer to it for its meaning"]
pub struct PLLXTPRE_R(crate::FieldReader<bool, PLLXTPRE_A>);
impl PLLXTPRE_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLXTPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLXTPRE_A {
        match self.bits {
            false => PLLXTPRE_A::DIV1,
            true => PLLXTPRE_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PLLXTPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLXTPRE_A::DIV2
    }
}
impl core::ops::Deref for PLLXTPRE_R {
    type Target = crate::FieldReader<bool, PLLXTPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry. Same bit as PREDIC\\[0\\]
from CFGR2 register. Refer to it for its meaning"]
pub struct PLLXTPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLXTPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLXTPRE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "HSE clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLXTPRE_A::DIV1)
    }
    #[doc = "HSE clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLXTPRE_A::DIV2)
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "PLL Multiplication Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PLLMUL_A {
    #[doc = "0: PLL input clock x2"]
    MUL2 = 0,
    #[doc = "1: PLL input clock x3"]
    MUL3 = 1,
    #[doc = "2: PLL input clock x4"]
    MUL4 = 2,
    #[doc = "3: PLL input clock x5"]
    MUL5 = 3,
    #[doc = "4: PLL input clock x6"]
    MUL6 = 4,
    #[doc = "5: PLL input clock x7"]
    MUL7 = 5,
    #[doc = "6: PLL input clock x8"]
    MUL8 = 6,
    #[doc = "7: PLL input clock x9"]
    MUL9 = 7,
    #[doc = "8: PLL input clock x10"]
    MUL10 = 8,
    #[doc = "9: PLL input clock x11"]
    MUL11 = 9,
    #[doc = "10: PLL input clock x12"]
    MUL12 = 10,
    #[doc = "11: PLL input clock x13"]
    MUL13 = 11,
    #[doc = "12: PLL input clock x14"]
    MUL14 = 12,
    #[doc = "13: PLL input clock x15"]
    MUL15 = 13,
    #[doc = "14: PLL input clock x16"]
    MUL16 = 14,
    #[doc = "15: PLL input clock x16"]
    MUL16X = 15,
}
impl From<PLLMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLMUL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PLLMUL` reader - PLL Multiplication Factor"]
pub struct PLLMUL_R(crate::FieldReader<u8, PLLMUL_A>);
impl PLLMUL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLLMUL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLMUL_A {
        match self.bits {
            0 => PLLMUL_A::MUL2,
            1 => PLLMUL_A::MUL3,
            2 => PLLMUL_A::MUL4,
            3 => PLLMUL_A::MUL5,
            4 => PLLMUL_A::MUL6,
            5 => PLLMUL_A::MUL7,
            6 => PLLMUL_A::MUL8,
            7 => PLLMUL_A::MUL9,
            8 => PLLMUL_A::MUL10,
            9 => PLLMUL_A::MUL11,
            10 => PLLMUL_A::MUL12,
            11 => PLLMUL_A::MUL13,
            12 => PLLMUL_A::MUL14,
            13 => PLLMUL_A::MUL15,
            14 => PLLMUL_A::MUL16,
            15 => PLLMUL_A::MUL16X,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        **self == PLLMUL_A::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL3`"]
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        **self == PLLMUL_A::MUL3
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        **self == PLLMUL_A::MUL4
    }
    #[doc = "Checks if the value of the field is `MUL5`"]
    #[inline(always)]
    pub fn is_mul5(&self) -> bool {
        **self == PLLMUL_A::MUL5
    }
    #[doc = "Checks if the value of the field is `MUL6`"]
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        **self == PLLMUL_A::MUL6
    }
    #[doc = "Checks if the value of the field is `MUL7`"]
    #[inline(always)]
    pub fn is_mul7(&self) -> bool {
        **self == PLLMUL_A::MUL7
    }
    #[doc = "Checks if the value of the field is `MUL8`"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        **self == PLLMUL_A::MUL8
    }
    #[doc = "Checks if the value of the field is `MUL9`"]
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        **self == PLLMUL_A::MUL9
    }
    #[doc = "Checks if the value of the field is `MUL10`"]
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        **self == PLLMUL_A::MUL10
    }
    #[doc = "Checks if the value of the field is `MUL11`"]
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        **self == PLLMUL_A::MUL11
    }
    #[doc = "Checks if the value of the field is `MUL12`"]
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        **self == PLLMUL_A::MUL12
    }
    #[doc = "Checks if the value of the field is `MUL13`"]
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        **self == PLLMUL_A::MUL13
    }
    #[doc = "Checks if the value of the field is `MUL14`"]
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        **self == PLLMUL_A::MUL14
    }
    #[doc = "Checks if the value of the field is `MUL15`"]
    #[inline(always)]
    pub fn is_mul15(&self) -> bool {
        **self == PLLMUL_A::MUL15
    }
    #[doc = "Checks if the value of the field is `MUL16`"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        **self == PLLMUL_A::MUL16
    }
    #[doc = "Checks if the value of the field is `MUL16X`"]
    #[inline(always)]
    pub fn is_mul16x(&self) -> bool {
        **self == PLLMUL_A::MUL16X
    }
}
impl core::ops::Deref for PLLMUL_R {
    type Target = crate::FieldReader<u8, PLLMUL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLMUL` writer - PLL Multiplication Factor"]
pub struct PLLMUL_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLMUL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLMUL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PLL input clock x2"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL2)
    }
    #[doc = "PLL input clock x3"]
    #[inline(always)]
    pub fn mul3(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL3)
    }
    #[doc = "PLL input clock x4"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL4)
    }
    #[doc = "PLL input clock x5"]
    #[inline(always)]
    pub fn mul5(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL5)
    }
    #[doc = "PLL input clock x6"]
    #[inline(always)]
    pub fn mul6(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL6)
    }
    #[doc = "PLL input clock x7"]
    #[inline(always)]
    pub fn mul7(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL7)
    }
    #[doc = "PLL input clock x8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL8)
    }
    #[doc = "PLL input clock x9"]
    #[inline(always)]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL9)
    }
    #[doc = "PLL input clock x10"]
    #[inline(always)]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL10)
    }
    #[doc = "PLL input clock x11"]
    #[inline(always)]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL11)
    }
    #[doc = "PLL input clock x12"]
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL12)
    }
    #[doc = "PLL input clock x13"]
    #[inline(always)]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL13)
    }
    #[doc = "PLL input clock x14"]
    #[inline(always)]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL14)
    }
    #[doc = "PLL input clock x15"]
    #[inline(always)]
    pub fn mul15(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL15)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL16)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16x(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL16X)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 18)) | ((value as u32 & 0x0f) << 18);
        self.w
    }
}
#[doc = "Microcontroller clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCO_A {
    #[doc = "1: Internal RC 14 MHz (HSI14) oscillator clock selected"]
    HSI14 = 1,
    #[doc = "8: Internal RC 48 MHz (HSI48) oscillator clock selected"]
    HSI48 = 8,
    #[doc = "0: MCO output disabled, no clock on MCO"]
    NOMCO = 0,
    #[doc = "2: Internal low speed (LSI) oscillator clock selected"]
    LSI = 2,
    #[doc = "3: External low speed (LSE) oscillator clock selected"]
    LSE = 3,
    #[doc = "4: System clock selected"]
    SYSCLK = 4,
    #[doc = "5: Internal RC 8 MHz (HSI) oscillator clock selected"]
    HSI = 5,
    #[doc = "6: External 4-32 MHz (HSE) oscillator clock selected"]
    HSE = 6,
    #[doc = "7: PLL clock selected (divided by 1 or 2, depending en PLLNODIV)"]
    PLL = 7,
}
impl From<MCO_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MCO` reader - Microcontroller clock output"]
pub struct MCO_R(crate::FieldReader<u8, MCO_A>);
impl MCO_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCO_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCO_A> {
        match self.bits {
            1 => Some(MCO_A::HSI14),
            8 => Some(MCO_A::HSI48),
            0 => Some(MCO_A::NOMCO),
            2 => Some(MCO_A::LSI),
            3 => Some(MCO_A::LSE),
            4 => Some(MCO_A::SYSCLK),
            5 => Some(MCO_A::HSI),
            6 => Some(MCO_A::HSE),
            7 => Some(MCO_A::PLL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSI14`"]
    #[inline(always)]
    pub fn is_hsi14(&self) -> bool {
        **self == MCO_A::HSI14
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        **self == MCO_A::HSI48
    }
    #[doc = "Checks if the value of the field is `NOMCO`"]
    #[inline(always)]
    pub fn is_no_mco(&self) -> bool {
        **self == MCO_A::NOMCO
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == MCO_A::LSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == MCO_A::LSE
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        **self == MCO_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == MCO_A::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == MCO_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        **self == MCO_A::PLL
    }
}
impl core::ops::Deref for MCO_R {
    type Target = crate::FieldReader<u8, MCO_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCO` writer - Microcontroller clock output"]
pub struct MCO_W<'a> {
    w: &'a mut W,
}
impl<'a> MCO_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCO_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Internal RC 14 MHz (HSI14) oscillator clock selected"]
    #[inline(always)]
    pub fn hsi14(self) -> &'a mut W {
        self.variant(MCO_A::HSI14)
    }
    #[doc = "Internal RC 48 MHz (HSI48) oscillator clock selected"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(MCO_A::HSI48)
    }
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn no_mco(self) -> &'a mut W {
        self.variant(MCO_A::NOMCO)
    }
    #[doc = "Internal low speed (LSI) oscillator clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCO_A::LSI)
    }
    #[doc = "External low speed (LSE) oscillator clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCO_A::LSE)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO_A::SYSCLK)
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO_A::HSI)
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO_A::HSE)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending en PLLNODIV)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCO_A::PLL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Microcontroller Clock Output Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MCOPRE_A {
    #[doc = "0: MCO is divided by 1"]
    DIV1 = 0,
    #[doc = "1: MCO is divided by 2"]
    DIV2 = 1,
    #[doc = "2: MCO is divided by 4"]
    DIV4 = 2,
    #[doc = "3: MCO is divided by 8"]
    DIV8 = 3,
    #[doc = "4: MCO is divided by 16"]
    DIV16 = 4,
    #[doc = "5: MCO is divided by 32"]
    DIV32 = 5,
    #[doc = "6: MCO is divided by 64"]
    DIV64 = 6,
    #[doc = "7: MCO is divided by 128"]
    DIV128 = 7,
}
impl From<MCOPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MCOPRE` reader - Microcontroller Clock Output Prescaler"]
pub struct MCOPRE_R(crate::FieldReader<u8, MCOPRE_A>);
impl MCOPRE_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCOPRE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCOPRE_A {
        match self.bits {
            0 => MCOPRE_A::DIV1,
            1 => MCOPRE_A::DIV2,
            2 => MCOPRE_A::DIV4,
            3 => MCOPRE_A::DIV8,
            4 => MCOPRE_A::DIV16,
            5 => MCOPRE_A::DIV32,
            6 => MCOPRE_A::DIV64,
            7 => MCOPRE_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == MCOPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == MCOPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        **self == MCOPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        **self == MCOPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        **self == MCOPRE_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        **self == MCOPRE_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        **self == MCOPRE_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        **self == MCOPRE_A::DIV128
    }
}
impl core::ops::Deref for MCOPRE_R {
    type Target = crate::FieldReader<u8, MCOPRE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCOPRE` writer - Microcontroller Clock Output Prescaler"]
pub struct MCOPRE_W<'a> {
    w: &'a mut W,
}
impl<'a> MCOPRE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MCOPRE_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "MCO is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV1)
    }
    #[doc = "MCO is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV2)
    }
    #[doc = "MCO is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV4)
    }
    #[doc = "MCO is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV8)
    }
    #[doc = "MCO is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV16)
    }
    #[doc = "MCO is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV32)
    }
    #[doc = "MCO is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV64)
    }
    #[doc = "MCO is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV128)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
#[doc = "PLL clock not divided for MCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PLLNODIV_A {
    #[doc = "0: PLL is divided by 2 for MCO"]
    DIV2 = 0,
    #[doc = "1: PLL is not divided for MCO"]
    DIV1 = 1,
}
impl From<PLLNODIV_A> for bool {
    #[inline(always)]
    fn from(variant: PLLNODIV_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PLLNODIV` reader - PLL clock not divided for MCO"]
pub struct PLLNODIV_R(crate::FieldReader<bool, PLLNODIV_A>);
impl PLLNODIV_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLLNODIV_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLNODIV_A {
        match self.bits {
            false => PLLNODIV_A::DIV2,
            true => PLLNODIV_A::DIV1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        **self == PLLNODIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        **self == PLLNODIV_A::DIV1
    }
}
impl core::ops::Deref for PLLNODIV_R {
    type Target = crate::FieldReader<bool, PLLNODIV_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLLNODIV` writer - PLL clock not divided for MCO"]
pub struct PLLNODIV_W<'a> {
    w: &'a mut W,
}
impl<'a> PLLNODIV_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PLLNODIV_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "PLL is divided by 2 for MCO"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLNODIV_A::DIV2)
    }
    #[doc = "PLL is not divided for MCO"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLNODIV_A::DIV1)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - System Clock Switch Status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre(&self) -> PPRE_R {
        PPRE_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bit 14 - APCPRE is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    pub fn adcpre(&self) -> ADCPRE_R {
        ADCPRE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bits 15:16 - PLL input clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 15) & 0x03) as u8)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry. Same bit as PREDIC\\[0\\]
from CFGR2 register. Refer to it for its meaning"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller Clock Output Prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 0x07) as u8)
    }
    #[doc = "Bit 31 - PLL clock not divided for MCO"]
    #[inline(always)]
    pub fn pllnodiv(&self) -> PLLNODIV_R {
        PLLNODIV_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W {
        SW_W { w: self }
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&mut self) -> HPRE_W {
        HPRE_W { w: self }
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre(&mut self) -> PPRE_W {
        PPRE_W { w: self }
    }
    #[doc = "Bit 14 - APCPRE is deprecated. See ADC field in CFGR2 register."]
    #[inline(always)]
    pub fn adcpre(&mut self) -> ADCPRE_W {
        ADCPRE_W { w: self }
    }
    #[doc = "Bits 15:16 - PLL input clock source"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PLLSRC_W {
        PLLSRC_W { w: self }
    }
    #[doc = "Bit 17 - HSE divider for PLL entry. Same bit as PREDIC\\[0\\]
from CFGR2 register. Refer to it for its meaning"]
    #[inline(always)]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W {
        PLLXTPRE_W { w: self }
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&mut self) -> PLLMUL_W {
        PLLMUL_W { w: self }
    }
    #[doc = "Bits 24:27 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&mut self) -> MCO_W {
        MCO_W { w: self }
    }
    #[doc = "Bits 28:30 - Microcontroller Clock Output Prescaler"]
    #[inline(always)]
    pub fn mcopre(&mut self) -> MCOPRE_W {
        MCOPRE_W { w: self }
    }
    #[doc = "Bit 31 - PLL clock not divided for MCO"]
    #[inline(always)]
    pub fn pllnodiv(&mut self) -> PLLNODIV_W {
        PLLNODIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register (RCC_CFGR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
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
