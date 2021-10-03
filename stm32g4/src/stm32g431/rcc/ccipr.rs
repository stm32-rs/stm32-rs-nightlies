#[doc = "Register `CCIPR` reader"]
pub struct R(crate::R<CCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCIPR` writer"]
pub struct W(crate::W<CCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCIPR_SPEC>;
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
impl From<crate::W<CCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "ADC3/4/5 clock source selection"]
pub type ADC345SEL_A = ADC12SEL_A;
#[doc = "Field `ADC345SEL` reader - ADC3/4/5 clock source selection"]
pub type ADC345SEL_R = ADC12SEL_R;
#[doc = "Field `ADC345SEL` writer - ADC3/4/5 clock source selection"]
pub struct ADC345SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC345SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC345SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No clock selected for ADC"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ADC345SEL_A::NONE)
    }
    #[doc = "PLL 'P' clock selected for ADC"]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(ADC345SEL_A::PLLP)
    }
    #[doc = "System clock selected for ADC"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(ADC345SEL_A::SYSTEM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 30)) | ((value as u32 & 0x03) << 30);
        self.w
    }
}
#[doc = "ADCs clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADC12SEL_A {
    #[doc = "0: No clock selected for ADC"]
    NONE = 0,
    #[doc = "1: PLL 'P' clock selected for ADC"]
    PLLP = 1,
    #[doc = "2: System clock selected for ADC"]
    SYSTEM = 2,
}
impl From<ADC12SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADC12SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADC12SEL` reader - ADCs clock source selection"]
pub struct ADC12SEL_R(crate::FieldReader<u8, ADC12SEL_A>);
impl ADC12SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADC12SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADC12SEL_A> {
        match self.bits {
            0 => Some(ADC12SEL_A::NONE),
            1 => Some(ADC12SEL_A::PLLP),
            2 => Some(ADC12SEL_A::SYSTEM),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == ADC12SEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `PLLP`"]
    #[inline(always)]
    pub fn is_pllp(&self) -> bool {
        **self == ADC12SEL_A::PLLP
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        **self == ADC12SEL_A::SYSTEM
    }
}
impl core::ops::Deref for ADC12SEL_R {
    type Target = crate::FieldReader<u8, ADC12SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADC12SEL` writer - ADCs clock source selection"]
pub struct ADC12SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC12SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC12SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No clock selected for ADC"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(ADC12SEL_A::NONE)
    }
    #[doc = "PLL 'P' clock selected for ADC"]
    #[inline(always)]
    pub fn pllp(self) -> &'a mut W {
        self.variant(ADC12SEL_A::PLLP)
    }
    #[doc = "System clock selected for ADC"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(ADC12SEL_A::SYSTEM)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
#[doc = "48 MHz clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CLK48SEL_A {
    #[doc = "0: HSI48 clock selected as 48MHz clock"]
    HSI48 = 0,
    #[doc = "2: PLL 'Q' (PLL48M1CLK) clock selected as 48MHz clock"]
    PLLQ = 2,
}
impl From<CLK48SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK48SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CLK48SEL` reader - 48 MHz clock source selection"]
pub struct CLK48SEL_R(crate::FieldReader<u8, CLK48SEL_A>);
impl CLK48SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLK48SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK48SEL_A> {
        match self.bits {
            0 => Some(CLK48SEL_A::HSI48),
            2 => Some(CLK48SEL_A::PLLQ),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        **self == CLK48SEL_A::HSI48
    }
    #[doc = "Checks if the value of the field is `PLLQ`"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        **self == CLK48SEL_A::PLLQ
    }
}
impl core::ops::Deref for CLK48SEL_R {
    type Target = crate::FieldReader<u8, CLK48SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLK48SEL` writer - 48 MHz clock source selection"]
pub struct CLK48SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK48SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CLK48SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI48 clock selected as 48MHz clock"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(CLK48SEL_A::HSI48)
    }
    #[doc = "PLL 'Q' (PLL48M1CLK) clock selected as 48MHz clock"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(CLK48SEL_A::PLLQ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 26)) | ((value as u32 & 0x03) << 26);
        self.w
    }
}
#[doc = "SAI2 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FDCANSEL_A {
    #[doc = "0: HSE clock selected as FDCAN clock"]
    HSE = 0,
    #[doc = "1: PLL 'Q' clock selected as FDCAN clock"]
    PLLQ = 1,
    #[doc = "2: PCLK clock selected as FDCAN clock"]
    PCLK = 2,
}
impl From<FDCANSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `FDCANSEL` reader - SAI2 clock source selection"]
pub struct FDCANSEL_R(crate::FieldReader<u8, FDCANSEL_A>);
impl FDCANSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FDCANSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FDCANSEL_A> {
        match self.bits {
            0 => Some(FDCANSEL_A::HSE),
            1 => Some(FDCANSEL_A::PLLQ),
            2 => Some(FDCANSEL_A::PCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == FDCANSEL_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLLQ`"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        **self == FDCANSEL_A::PLLQ
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == FDCANSEL_A::PCLK
    }
}
impl core::ops::Deref for FDCANSEL_R {
    type Target = crate::FieldReader<u8, FDCANSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FDCANSEL` writer - SAI2 clock source selection"]
pub struct FDCANSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSE clock selected as FDCAN clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(FDCANSEL_A::HSE)
    }
    #[doc = "PLL 'Q' clock selected as FDCAN clock"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(FDCANSEL_A::PLLQ)
    }
    #[doc = "PCLK clock selected as FDCAN clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(FDCANSEL_A::PCLK)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "SAI1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2S23SEL_A {
    #[doc = "0: System clock selected as I2S23 clock"]
    SYSTEM = 0,
    #[doc = "1: PLL 'Q' clock selected as I2S23 clock"]
    PLLQ = 1,
    #[doc = "2: Clock provided on I2S_CKIN pin is selected as I2S23 clock"]
    I2S_CKIN = 2,
    #[doc = "3: HSI16 clock selected as I2S23 clock"]
    HSI16 = 3,
}
impl From<I2S23SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2S23SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2S23SEL` reader - SAI1 clock source selection"]
pub struct I2S23SEL_R(crate::FieldReader<u8, I2S23SEL_A>);
impl I2S23SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2S23SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2S23SEL_A {
        match self.bits {
            0 => I2S23SEL_A::SYSTEM,
            1 => I2S23SEL_A::PLLQ,
            2 => I2S23SEL_A::I2S_CKIN,
            3 => I2S23SEL_A::HSI16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        **self == I2S23SEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `PLLQ`"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        **self == I2S23SEL_A::PLLQ
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        **self == I2S23SEL_A::I2S_CKIN
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == I2S23SEL_A::HSI16
    }
}
impl core::ops::Deref for I2S23SEL_R {
    type Target = crate::FieldReader<u8, I2S23SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2S23SEL` writer - SAI1 clock source selection"]
pub struct I2S23SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S23SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2S23SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "System clock selected as I2S23 clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2S23SEL_A::SYSTEM)
    }
    #[doc = "PLL 'Q' clock selected as I2S23 clock"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(I2S23SEL_A::PLLQ)
    }
    #[doc = "Clock provided on I2S_CKIN pin is selected as I2S23 clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(I2S23SEL_A::I2S_CKIN)
    }
    #[doc = "HSI16 clock selected as I2S23 clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2S23SEL_A::HSI16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "Low power timer 2 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    #[doc = "0: System clock selected as SAI clock"]
    SYSTEM = 0,
    #[doc = "1: PLL 'Q' clock selected as SAI clock"]
    PLLQ = 1,
    #[doc = "2: Clock provided on I2S_CKIN pin is selected as SAI clock"]
    I2S_CKIN = 2,
    #[doc = "3: HSI16 clock selected as SAI clock"]
    HSI16 = 3,
}
impl From<SAI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SAI1SEL` reader - Low power timer 2 clock source selection"]
pub struct SAI1SEL_R(crate::FieldReader<u8, SAI1SEL_A>);
impl SAI1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAI1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1SEL_A {
        match self.bits {
            0 => SAI1SEL_A::SYSTEM,
            1 => SAI1SEL_A::PLLQ,
            2 => SAI1SEL_A::I2S_CKIN,
            3 => SAI1SEL_A::HSI16,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        **self == SAI1SEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `PLLQ`"]
    #[inline(always)]
    pub fn is_pllq(&self) -> bool {
        **self == SAI1SEL_A::PLLQ
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        **self == SAI1SEL_A::I2S_CKIN
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == SAI1SEL_A::HSI16
    }
}
impl core::ops::Deref for SAI1SEL_R {
    type Target = crate::FieldReader<u8, SAI1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAI1SEL` writer - Low power timer 2 clock source selection"]
pub struct SAI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "System clock selected as SAI clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(SAI1SEL_A::SYSTEM)
    }
    #[doc = "PLL 'Q' clock selected as SAI clock"]
    #[inline(always)]
    pub fn pllq(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLLQ)
    }
    #[doc = "Clock provided on I2S_CKIN pin is selected as SAI clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1SEL_A::I2S_CKIN)
    }
    #[doc = "HSI16 clock selected as SAI clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(SAI1SEL_A::HSI16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "Low power timer 1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: PCLK clock selected as LPTIM1 clock"]
    PCLK = 0,
    #[doc = "1: LSI clock selected as LPTIM1 clock"]
    LSI = 1,
    #[doc = "2: HSI16 clock selected as LPTIM1 clock"]
    HSI16 = 2,
    #[doc = "3: LSE clock selected as LPTIM1 clock"]
    LSE = 3,
}
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPTIM1SEL` reader - Low power timer 1 clock source selection"]
pub struct LPTIM1SEL_R(crate::FieldReader<u8, LPTIM1SEL_A>);
impl LPTIM1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPTIM1SEL_A {
        match self.bits {
            0 => LPTIM1SEL_A::PCLK,
            1 => LPTIM1SEL_A::LSI,
            2 => LPTIM1SEL_A::HSI16,
            3 => LPTIM1SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == LPTIM1SEL_A::PCLK
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == LPTIM1SEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == LPTIM1SEL_A::HSI16
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == LPTIM1SEL_A::LSE
    }
}
impl core::ops::Deref for LPTIM1SEL_R {
    type Target = crate::FieldReader<u8, LPTIM1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1SEL` writer - Low power timer 1 clock source selection"]
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PCLK clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::PCLK)
    }
    #[doc = "LSI clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSI)
    }
    #[doc = "HSI16 clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::HSI16)
    }
    #[doc = "LSE clock selected as LPTIM1 clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 18)) | ((value as u32 & 0x03) << 18);
        self.w
    }
}
#[doc = "I2C3 clock source selection"]
pub type I2C3SEL_A = I2C1SEL_A;
#[doc = "Field `I2C3SEL` reader - I2C3 clock source selection"]
pub type I2C3SEL_R = I2C1SEL_R;
#[doc = "Field `I2C3SEL` writer - I2C3 clock source selection"]
pub struct I2C3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C3SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C3SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PCLK clock selected as I2C clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C3SEL_A::PCLK)
    }
    #[doc = "System clock (SYSCLK) selected as I2C clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C3SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C3SEL_A::HSI16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "I2C2 clock source selection"]
pub type I2C2SEL_A = I2C1SEL_A;
#[doc = "Field `I2C2SEL` reader - I2C2 clock source selection"]
pub type I2C2SEL_R = I2C1SEL_R;
#[doc = "Field `I2C2SEL` writer - I2C2 clock source selection"]
pub struct I2C2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PCLK clock selected as I2C clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C2SEL_A::PCLK)
    }
    #[doc = "System clock (SYSCLK) selected as I2C clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C2SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C2SEL_A::HSI16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | ((value as u32 & 0x03) << 14);
        self.w
    }
}
#[doc = "I2C1 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C1SEL_A {
    #[doc = "0: PCLK clock selected as I2C clock"]
    PCLK = 0,
    #[doc = "1: System clock (SYSCLK) selected as I2C clock"]
    SYSTEM = 1,
    #[doc = "2: HSI16 clock selected as I2C clock"]
    HSI16 = 2,
}
impl From<I2C1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2C1SEL` reader - I2C1 clock source selection"]
pub struct I2C1SEL_R(crate::FieldReader<u8, I2C1SEL_A>);
impl I2C1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<I2C1SEL_A> {
        match self.bits {
            0 => Some(I2C1SEL_A::PCLK),
            1 => Some(I2C1SEL_A::SYSTEM),
            2 => Some(I2C1SEL_A::HSI16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == I2C1SEL_A::PCLK
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        **self == I2C1SEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == I2C1SEL_A::HSI16
    }
}
impl core::ops::Deref for I2C1SEL_R {
    type Target = crate::FieldReader<u8, I2C1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1SEL` writer - I2C1 clock source selection"]
pub struct I2C1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PCLK clock selected as I2C clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(I2C1SEL_A::PCLK)
    }
    #[doc = "System clock (SYSCLK) selected as I2C clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(I2C1SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as I2C clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(I2C1SEL_A::HSI16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "LPUART1 clock source selection"]
pub type LPUART1SEL_A = UART4SEL_A;
#[doc = "Field `LPUART1SEL` reader - LPUART1 clock source selection"]
pub type LPUART1SEL_R = UART4SEL_R;
#[doc = "Field `LPUART1SEL` writer - LPUART1 clock source selection"]
pub struct LPUART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PCLK clock selected as UART clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::PCLK)
    }
    #[doc = "System clock (SYSCLK) selected as UART clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as UART clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::HSI16)
    }
    #[doc = "LSE clock selected as UART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 10)) | ((value as u32 & 0x03) << 10);
        self.w
    }
}
#[doc = "UART5 clock source selection"]
pub type UART5SEL_A = UART4SEL_A;
#[doc = "Field `UART5SEL` reader - UART5 clock source selection"]
pub type UART5SEL_R = UART4SEL_R;
#[doc = "Field `UART5SEL` writer - UART5 clock source selection"]
pub struct UART5SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART5SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART5SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PCLK clock selected as UART clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(UART5SEL_A::PCLK)
    }
    #[doc = "System clock (SYSCLK) selected as UART clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(UART5SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as UART clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(UART5SEL_A::HSI16)
    }
    #[doc = "LSE clock selected as UART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(UART5SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "UART4 clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UART4SEL_A {
    #[doc = "0: PCLK clock selected as UART clock"]
    PCLK = 0,
    #[doc = "1: System clock (SYSCLK) selected as UART clock"]
    SYSTEM = 1,
    #[doc = "2: HSI16 clock selected as UART clock"]
    HSI16 = 2,
    #[doc = "3: LSE clock selected as UART clock"]
    LSE = 3,
}
impl From<UART4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UART4SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UART4SEL` reader - UART4 clock source selection"]
pub struct UART4SEL_R(crate::FieldReader<u8, UART4SEL_A>);
impl UART4SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UART4SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART4SEL_A {
        match self.bits {
            0 => UART4SEL_A::PCLK,
            1 => UART4SEL_A::SYSTEM,
            2 => UART4SEL_A::HSI16,
            3 => UART4SEL_A::LSE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        **self == UART4SEL_A::PCLK
    }
    #[doc = "Checks if the value of the field is `SYSTEM`"]
    #[inline(always)]
    pub fn is_system(&self) -> bool {
        **self == UART4SEL_A::SYSTEM
    }
    #[doc = "Checks if the value of the field is `HSI16`"]
    #[inline(always)]
    pub fn is_hsi16(&self) -> bool {
        **self == UART4SEL_A::HSI16
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == UART4SEL_A::LSE
    }
}
impl core::ops::Deref for UART4SEL_R {
    type Target = crate::FieldReader<u8, UART4SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UART4SEL` writer - UART4 clock source selection"]
pub struct UART4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UART4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART4SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "PCLK clock selected as UART clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(UART4SEL_A::PCLK)
    }
    #[doc = "System clock (SYSCLK) selected as UART clock"]
    #[inline(always)]
    pub fn system(self) -> &'a mut W {
        self.variant(UART4SEL_A::SYSTEM)
    }
    #[doc = "HSI16 clock selected as UART clock"]
    #[inline(always)]
    pub fn hsi16(self) -> &'a mut W {
        self.variant(UART4SEL_A::HSI16)
    }
    #[doc = "LSE clock selected as UART clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(UART4SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `USART3SEL` reader - USART3 clock source selection"]
pub struct USART3SEL_R(crate::FieldReader<u8, u8>);
impl USART3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART3SEL` writer - USART3 clock source selection"]
pub struct USART3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
        self.w
    }
}
#[doc = "Field `USART2SEL` reader - USART2 clock source selection"]
pub struct USART2SEL_R(crate::FieldReader<u8, u8>);
impl USART2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART2SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART2SEL` writer - USART2 clock source selection"]
pub struct USART2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART2SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `USART1SEL` reader - USART1 clock source selection"]
pub struct USART1SEL_R(crate::FieldReader<u8, u8>);
impl USART1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART1SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USART1SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART1SEL` writer - USART1 clock source selection"]
pub struct USART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART1SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 30:31 - ADC3/4/5 clock source selection"]
    #[inline(always)]
    pub fn adc345sel(&self) -> ADC345SEL_R {
        ADC345SEL_R::new(((self.bits >> 30) & 0x03) as u8)
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adc12sel(&self) -> ADC12SEL_R {
        ADC12SEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48sel(&self) -> CLK48SEL_R {
        CLK48SEL_R::new(((self.bits >> 26) & 0x03) as u8)
    }
    #[doc = "Bits 24:25 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn i2s23sel(&self) -> I2S23SEL_R {
        I2S23SEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 18) & 0x03) as u8)
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&self) -> I2C3SEL_R {
        I2C3SEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&self) -> I2C2SEL_R {
        I2C2SEL_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&self) -> I2C1SEL_R {
        I2C1SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new(((self.bits >> 10) & 0x03) as u8)
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&self) -> UART5SEL_R {
        UART5SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&self) -> UART4SEL_R {
        UART4SEL_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&self) -> USART3SEL_R {
        USART3SEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&self) -> USART2SEL_R {
        USART2SEL_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&self) -> USART1SEL_R {
        USART1SEL_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 30:31 - ADC3/4/5 clock source selection"]
    #[inline(always)]
    pub fn adc345sel(&mut self) -> ADC345SEL_W {
        ADC345SEL_W { w: self }
    }
    #[doc = "Bits 28:29 - ADCs clock source selection"]
    #[inline(always)]
    pub fn adc12sel(&mut self) -> ADC12SEL_W {
        ADC12SEL_W { w: self }
    }
    #[doc = "Bits 26:27 - 48 MHz clock source selection"]
    #[inline(always)]
    pub fn clk48sel(&mut self) -> CLK48SEL_W {
        CLK48SEL_W { w: self }
    }
    #[doc = "Bits 24:25 - SAI2 clock source selection"]
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W {
        FDCANSEL_W { w: self }
    }
    #[doc = "Bits 22:23 - SAI1 clock source selection"]
    #[inline(always)]
    pub fn i2s23sel(&mut self) -> I2S23SEL_W {
        I2S23SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - Low power timer 2 clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W {
        SAI1SEL_W { w: self }
    }
    #[doc = "Bits 18:19 - Low power timer 1 clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - I2C3 clock source selection"]
    #[inline(always)]
    pub fn i2c3sel(&mut self) -> I2C3SEL_W {
        I2C3SEL_W { w: self }
    }
    #[doc = "Bits 14:15 - I2C2 clock source selection"]
    #[inline(always)]
    pub fn i2c2sel(&mut self) -> I2C2SEL_W {
        I2C2SEL_W { w: self }
    }
    #[doc = "Bits 12:13 - I2C1 clock source selection"]
    #[inline(always)]
    pub fn i2c1sel(&mut self) -> I2C1SEL_W {
        I2C1SEL_W { w: self }
    }
    #[doc = "Bits 10:11 - LPUART1 clock source selection"]
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W {
        LPUART1SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - UART5 clock source selection"]
    #[inline(always)]
    pub fn uart5sel(&mut self) -> UART5SEL_W {
        UART5SEL_W { w: self }
    }
    #[doc = "Bits 6:7 - UART4 clock source selection"]
    #[inline(always)]
    pub fn uart4sel(&mut self) -> UART4SEL_W {
        UART4SEL_W { w: self }
    }
    #[doc = "Bits 4:5 - USART3 clock source selection"]
    #[inline(always)]
    pub fn usart3sel(&mut self) -> USART3SEL_W {
        USART3SEL_W { w: self }
    }
    #[doc = "Bits 2:3 - USART2 clock source selection"]
    #[inline(always)]
    pub fn usart2sel(&mut self) -> USART2SEL_W {
        USART2SEL_W { w: self }
    }
    #[doc = "Bits 0:1 - USART1 clock source selection"]
    #[inline(always)]
    pub fn usart1sel(&mut self) -> USART1SEL_W {
        USART1SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCIPR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccipr](index.html) module"]
pub struct CCIPR_SPEC;
impl crate::RegisterSpec for CCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccipr::R](R) reader structure"]
impl crate::Readable for CCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccipr::W](W) writer structure"]
impl crate::Writable for CCIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CCIPR to value 0"]
impl crate::Resettable for CCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
