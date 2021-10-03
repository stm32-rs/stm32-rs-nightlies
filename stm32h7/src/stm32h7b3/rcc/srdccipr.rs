#[doc = "Register `SRDCCIPR` reader"]
pub struct R(crate::R<SRDCCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRDCCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRDCCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRDCCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRDCCIPR` writer"]
pub struct W(crate::W<SRDCCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRDCCIPR_SPEC>;
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
impl From<crate::W<SRDCCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRDCCIPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPUART1SEL_A {
    #[doc = "0: rcc_pclk_d3 selected as peripheral clock"]
    RCC_PCLK_D3 = 0,
    #[doc = "1: pll2_q selected as peripheral clock"]
    PLL2_Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    PLL3_Q = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HSI_KER = 3,
    #[doc = "4: csi_ker selected as peripheral clock"]
    CSI_KER = 4,
    #[doc = "5: LSE selected as peripheral clock"]
    LSE = 5,
}
impl From<LPUART1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPUART1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPUART1SEL` reader - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct LPUART1SEL_R(crate::FieldReader<u8, LPUART1SEL_A>);
impl LPUART1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPUART1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPUART1SEL_A> {
        match self.bits {
            0 => Some(LPUART1SEL_A::RCC_PCLK_D3),
            1 => Some(LPUART1SEL_A::PLL2_Q),
            2 => Some(LPUART1SEL_A::PLL3_Q),
            3 => Some(LPUART1SEL_A::HSI_KER),
            4 => Some(LPUART1SEL_A::CSI_KER),
            5 => Some(LPUART1SEL_A::LSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK_D3`"]
    #[inline(always)]
    pub fn is_rcc_pclk_d3(&self) -> bool {
        **self == LPUART1SEL_A::RCC_PCLK_D3
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        **self == LPUART1SEL_A::PLL2_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        **self == LPUART1SEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        **self == LPUART1SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        **self == LPUART1SEL_A::CSI_KER
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == LPUART1SEL_A::LSE
    }
}
impl core::ops::Deref for LPUART1SEL_R {
    type Target = crate::FieldReader<u8, LPUART1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPUART1SEL` writer - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct LPUART1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPUART1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPUART1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_pclk_d3 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk_d3(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::RCC_PCLK_D3)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::PLL2_Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::PLL3_Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::HSI_KER)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::CSI_KER)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPUART1SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "I2C4 kernel clock source selection Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C4SEL_A {
    #[doc = "0: rcc_pclk4 selected as peripheral clock"]
    RCC_PCLK4 = 0,
    #[doc = "1: pll3_r selected as peripheral clock"]
    PLL3_R = 1,
    #[doc = "2: hsi_ker selected as peripheral clock"]
    HSI_KER = 2,
    #[doc = "3: csi_ker selected as peripheral clock"]
    CSI_KER = 3,
}
impl From<I2C4SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C4SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2C4SEL` reader - I2C4 kernel clock source selection Set and reset by software."]
pub struct I2C4SEL_R(crate::FieldReader<u8, I2C4SEL_A>);
impl I2C4SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C4SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C4SEL_A {
        match self.bits {
            0 => I2C4SEL_A::RCC_PCLK4,
            1 => I2C4SEL_A::PLL3_R,
            2 => I2C4SEL_A::HSI_KER,
            3 => I2C4SEL_A::CSI_KER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK4`"]
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        **self == I2C4SEL_A::RCC_PCLK4
    }
    #[doc = "Checks if the value of the field is `PLL3_R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        **self == I2C4SEL_A::PLL3_R
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        **self == I2C4SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        **self == I2C4SEL_A::CSI_KER
    }
}
impl core::ops::Deref for I2C4SEL_R {
    type Target = crate::FieldReader<u8, I2C4SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C4SEL` writer - I2C4 kernel clock source selection Set and reset by software."]
pub struct I2C4SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C4SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C4SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut W {
        self.variant(I2C4SEL_A::RCC_PCLK4)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(I2C4SEL_A::PLL3_R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(I2C4SEL_A::HSI_KER)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(I2C4SEL_A::CSI_KER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM2SEL_A {
    #[doc = "0: rcc_pclk4 selected as peripheral clock"]
    RCC_PCLK4 = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    PLL2_P = 1,
    #[doc = "2: pll3_r selected as peripheral clock"]
    PLL3_R = 2,
    #[doc = "3: LSE selected as peripheral clock"]
    LSE = 3,
    #[doc = "4: LSI selected as peripheral clock"]
    LSI = 4,
    #[doc = "5: PER selected as peripheral clock"]
    PER = 5,
}
impl From<LPTIM2SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM2SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPTIM2SEL` reader - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct LPTIM2SEL_R(crate::FieldReader<u8, LPTIM2SEL_A>);
impl LPTIM2SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM2SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPTIM2SEL_A> {
        match self.bits {
            0 => Some(LPTIM2SEL_A::RCC_PCLK4),
            1 => Some(LPTIM2SEL_A::PLL2_P),
            2 => Some(LPTIM2SEL_A::PLL3_R),
            3 => Some(LPTIM2SEL_A::LSE),
            4 => Some(LPTIM2SEL_A::LSI),
            5 => Some(LPTIM2SEL_A::PER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK4`"]
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        **self == LPTIM2SEL_A::RCC_PCLK4
    }
    #[doc = "Checks if the value of the field is `PLL2_P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        **self == LPTIM2SEL_A::PLL2_P
    }
    #[doc = "Checks if the value of the field is `PLL3_R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        **self == LPTIM2SEL_A::PLL3_R
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == LPTIM2SEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == LPTIM2SEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `PER`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        **self == LPTIM2SEL_A::PER
    }
}
impl core::ops::Deref for LPTIM2SEL_R {
    type Target = crate::FieldReader<u8, LPTIM2SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM2SEL` writer - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct LPTIM2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM2SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM2SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::RCC_PCLK4)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::PLL2_P)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::PLL3_R)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::LSE)
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::LSI)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(LPTIM2SEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 10)) | ((value as u32 & 0x07) << 10);
        self.w
    }
}
#[doc = "Field `LPTIM3SEL` reader - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct LPTIM3SEL_R(crate::FieldReader<u8, u8>);
impl LPTIM3SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM3SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPTIM3SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM3SEL` writer - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct LPTIM3SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM3SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | ((value as u32 & 0x07) << 13);
        self.w
    }
}
#[doc = "SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum ADCSEL_A {
    #[doc = "0: pll2_p selected as peripheral clock"]
    PLL2_P = 0,
    #[doc = "1: pll3_r selected as peripheral clock"]
    PLL3_R = 1,
    #[doc = "2: PER selected as peripheral clock"]
    PER = 2,
}
impl From<ADCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `ADCSEL` reader - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct ADCSEL_R(crate::FieldReader<u8, ADCSEL_A>);
impl ADCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        ADCSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADCSEL_A> {
        match self.bits {
            0 => Some(ADCSEL_A::PLL2_P),
            1 => Some(ADCSEL_A::PLL3_R),
            2 => Some(ADCSEL_A::PER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PLL2_P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        **self == ADCSEL_A::PLL2_P
    }
    #[doc = "Checks if the value of the field is `PLL3_R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        **self == ADCSEL_A::PLL3_R
    }
    #[doc = "Checks if the value of the field is `PER`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        **self == ADCSEL_A::PER
    }
}
impl core::ops::Deref for ADCSEL_R {
    type Target = crate::FieldReader<u8, ADCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADCSEL` writer - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct ADCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> ADCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADCSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(ADCSEL_A::PLL2_P)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(ADCSEL_A::PLL3_R)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(ADCSEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | ((value as u32 & 0x03) << 16);
        self.w
    }
}
#[doc = "Field `DFSDM2SEL` reader - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and )."]
pub struct DFSDM2SEL_R(crate::FieldReader<bool, bool>);
impl DFSDM2SEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DFSDM2SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DFSDM2SEL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DFSDM2SEL` writer - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and )."]
pub struct DFSDM2SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM2SEL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 27)) | ((value as u32 & 0x01) << 27);
        self.w
    }
}
#[doc = "SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI6SEL_A {
    #[doc = "0: rcc_pclk4 selected as peripheral clock"]
    RCC_PCLK4 = 0,
    #[doc = "1: pll2_q selected as peripheral clock"]
    PLL2_Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    PLL3_Q = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HSI_KER = 3,
    #[doc = "4: csi_ker selected as peripheral clock"]
    CSI_KER = 4,
    #[doc = "5: HSE selected as peripheral clock"]
    HSE = 5,
}
impl From<SPI6SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI6SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SPI6SEL` reader - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct SPI6SEL_R(crate::FieldReader<u8, SPI6SEL_A>);
impl SPI6SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SPI6SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPI6SEL_A> {
        match self.bits {
            0 => Some(SPI6SEL_A::RCC_PCLK4),
            1 => Some(SPI6SEL_A::PLL2_Q),
            2 => Some(SPI6SEL_A::PLL3_Q),
            3 => Some(SPI6SEL_A::HSI_KER),
            4 => Some(SPI6SEL_A::CSI_KER),
            5 => Some(SPI6SEL_A::HSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK4`"]
    #[inline(always)]
    pub fn is_rcc_pclk4(&self) -> bool {
        **self == SPI6SEL_A::RCC_PCLK4
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        **self == SPI6SEL_A::PLL2_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        **self == SPI6SEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        **self == SPI6SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        **self == SPI6SEL_A::CSI_KER
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == SPI6SEL_A::HSE
    }
}
impl core::ops::Deref for SPI6SEL_R {
    type Target = crate::FieldReader<u8, SPI6SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPI6SEL` writer - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
pub struct SPI6SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI6SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI6SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_pclk4 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk4(self) -> &'a mut W {
        self.variant(SPI6SEL_A::RCC_PCLK4)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(SPI6SEL_A::PLL2_Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(SPI6SEL_A::PLL3_Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPI6SEL_A::HSI_KER)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(SPI6SEL_A::CSI_KER)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SPI6SEL_A::HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lpuart1sel(&self) -> LPUART1SEL_R {
        LPUART1SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn i2c4sel(&self) -> I2C4SEL_R {
        I2C4SEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim2sel(&self) -> LPTIM2SEL_R {
        LPTIM2SEL_R::new(((self.bits >> 10) & 0x07) as u8)
    }
    #[doc = "Bits 13:15 - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim3sel(&self) -> LPTIM3SEL_R {
        LPTIM3SEL_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn adcsel(&self) -> ADCSEL_R {
        ADCSEL_R::new(((self.bits >> 16) & 0x03) as u8)
    }
    #[doc = "Bit 27 - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and )."]
    #[inline(always)]
    pub fn dfsdm2sel(&self) -> DFSDM2SEL_R {
        DFSDM2SEL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi6sel(&self) -> SPI6SEL_R {
        SPI6SEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lpuart1sel(&mut self) -> LPUART1SEL_W {
        LPUART1SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection Set and reset by software."]
    #[inline(always)]
    pub fn i2c4sel(&mut self) -> I2C4SEL_W {
        I2C4SEL_W { w: self }
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim2sel(&mut self) -> LPTIM2SEL_W {
        LPTIM2SEL_W { w: self }
    }
    #[doc = "Bits 13:15 - LPTIM3 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn lptim3sel(&mut self) -> LPTIM3SEL_W {
        LPTIM3SEL_W { w: self }
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn adcsel(&mut self) -> ADCSEL_W {
        ADCSEL_W { w: self }
    }
    #[doc = "Bit 27 - DFSDM2 kernel Clk clock source selection Set and reset by software. Note: The DFSDM2 Aclk clock source selection is done by SPI6SEL (see and )."]
    #[inline(always)]
    pub fn dfsdm2sel(&mut self) -> DFSDM2SEL_W {
        DFSDM2SEL_W { w: self }
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection Set and reset by software. others: reserved, the kernel clock is disabled"]
    #[inline(always)]
    pub fn spi6sel(&mut self) -> SPI6SEL_W {
        SPI6SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC SmartRun domain kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srdccipr](index.html) module"]
pub struct SRDCCIPR_SPEC;
impl crate::RegisterSpec for SRDCCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srdccipr::R](R) reader structure"]
impl crate::Readable for SRDCCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srdccipr::W](W) writer structure"]
impl crate::Writable for SRDCCIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SRDCCIPR to value 0"]
impl crate::Resettable for SRDCCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
