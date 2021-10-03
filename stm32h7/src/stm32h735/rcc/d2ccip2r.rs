#[doc = "Register `D2CCIP2R` reader"]
pub struct R(crate::R<D2CCIP2R_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<D2CCIP2R_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<D2CCIP2R_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<D2CCIP2R_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `D2CCIP2R` writer"]
pub struct W(crate::W<D2CCIP2R_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<D2CCIP2R_SPEC>;
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
impl From<crate::W<D2CCIP2R_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<D2CCIP2R_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART234578SEL_A {
    #[doc = "0: rcc_pclk1 selected as peripheral clock"]
    RCC_PCLK1 = 0,
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
impl From<USART234578SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART234578SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USART234578SEL` reader - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
pub struct USART234578SEL_R(crate::FieldReader<u8, USART234578SEL_A>);
impl USART234578SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART234578SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USART234578SEL_A> {
        match self.bits {
            0 => Some(USART234578SEL_A::RCC_PCLK1),
            1 => Some(USART234578SEL_A::PLL2_Q),
            2 => Some(USART234578SEL_A::PLL3_Q),
            3 => Some(USART234578SEL_A::HSI_KER),
            4 => Some(USART234578SEL_A::CSI_KER),
            5 => Some(USART234578SEL_A::LSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK1`"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        **self == USART234578SEL_A::RCC_PCLK1
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        **self == USART234578SEL_A::PLL2_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        **self == USART234578SEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        **self == USART234578SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        **self == USART234578SEL_A::CSI_KER
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == USART234578SEL_A::LSE
    }
}
impl core::ops::Deref for USART234578SEL_R {
    type Target = crate::FieldReader<u8, USART234578SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART234578SEL` writer - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
pub struct USART234578SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART234578SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART234578SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut W {
        self.variant(USART234578SEL_A::RCC_PCLK1)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(USART234578SEL_A::PLL2_Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(USART234578SEL_A::PLL3_Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(USART234578SEL_A::HSI_KER)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(USART234578SEL_A::CSI_KER)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART234578SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "USART1 and 6 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USART16910SEL_A {
    #[doc = "0: rcc_pclk2 selected as peripheral clock"]
    RCC_PCLK2 = 0,
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
impl From<USART16910SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USART16910SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USART16910SEL` reader - USART1 and 6 kernel clock source selection"]
pub struct USART16910SEL_R(crate::FieldReader<u8, USART16910SEL_A>);
impl USART16910SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USART16910SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<USART16910SEL_A> {
        match self.bits {
            0 => Some(USART16910SEL_A::RCC_PCLK2),
            1 => Some(USART16910SEL_A::PLL2_Q),
            2 => Some(USART16910SEL_A::PLL3_Q),
            3 => Some(USART16910SEL_A::HSI_KER),
            4 => Some(USART16910SEL_A::CSI_KER),
            5 => Some(USART16910SEL_A::LSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK2`"]
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        **self == USART16910SEL_A::RCC_PCLK2
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        **self == USART16910SEL_A::PLL2_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        **self == USART16910SEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        **self == USART16910SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        **self == USART16910SEL_A::CSI_KER
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == USART16910SEL_A::LSE
    }
}
impl core::ops::Deref for USART16910SEL_R {
    type Target = crate::FieldReader<u8, USART16910SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USART16910SEL` writer - USART1 and 6 kernel clock source selection"]
pub struct USART16910SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USART16910SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USART16910SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_pclk2 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut W {
        self.variant(USART16910SEL_A::RCC_PCLK2)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(USART16910SEL_A::PLL2_Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(USART16910SEL_A::PLL3_Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(USART16910SEL_A::HSI_KER)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(USART16910SEL_A::CSI_KER)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(USART16910SEL_A::LSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "RNG kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum RNGSEL_A {
    #[doc = "0: HSI48 selected as peripheral clock"]
    HSI48 = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    PLL1_Q = 1,
    #[doc = "2: LSE selected as peripheral clock"]
    LSE = 2,
    #[doc = "3: LSI selected as peripheral clock"]
    LSI = 3,
}
impl From<RNGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: RNGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `RNGSEL` reader - RNG kernel clock source selection"]
pub struct RNGSEL_R(crate::FieldReader<u8, RNGSEL_A>);
impl RNGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        RNGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RNGSEL_A {
        match self.bits {
            0 => RNGSEL_A::HSI48,
            1 => RNGSEL_A::PLL1_Q,
            2 => RNGSEL_A::LSE,
            3 => RNGSEL_A::LSI,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        **self == RNGSEL_A::HSI48
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        **self == RNGSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == RNGSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == RNGSEL_A::LSI
    }
}
impl core::ops::Deref for RNGSEL_R {
    type Target = crate::FieldReader<u8, RNGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RNGSEL` writer - RNG kernel clock source selection"]
pub struct RNGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "HSI48 selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(RNGSEL_A::HSI48)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(RNGSEL_A::PLL1_Q)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(RNGSEL_A::LSE)
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(RNGSEL_A::LSI)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | ((value as u32 & 0x03) << 8);
        self.w
    }
}
#[doc = "I2C1,2,3 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C1235SEL_A {
    #[doc = "0: rcc_pclk1 selected as peripheral clock"]
    RCC_PCLK1 = 0,
    #[doc = "1: pll3_r selected as peripheral clock"]
    PLL3_R = 1,
    #[doc = "2: hsi_ker selected as peripheral clock"]
    HSI_KER = 2,
    #[doc = "3: csi_ker selected as peripheral clock"]
    CSI_KER = 3,
}
impl From<I2C1235SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C1235SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I2C1235SEL` reader - I2C1,2,3 kernel clock source selection"]
pub struct I2C1235SEL_R(crate::FieldReader<u8, I2C1235SEL_A>);
impl I2C1235SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        I2C1235SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C1235SEL_A {
        match self.bits {
            0 => I2C1235SEL_A::RCC_PCLK1,
            1 => I2C1235SEL_A::PLL3_R,
            2 => I2C1235SEL_A::HSI_KER,
            3 => I2C1235SEL_A::CSI_KER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK1`"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        **self == I2C1235SEL_A::RCC_PCLK1
    }
    #[doc = "Checks if the value of the field is `PLL3_R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        **self == I2C1235SEL_A::PLL3_R
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        **self == I2C1235SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        **self == I2C1235SEL_A::CSI_KER
    }
}
impl core::ops::Deref for I2C1235SEL_R {
    type Target = crate::FieldReader<u8, I2C1235SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C1235SEL` writer - I2C1,2,3 kernel clock source selection"]
pub struct I2C1235SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C1235SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C1235SEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut W {
        self.variant(I2C1235SEL_A::RCC_PCLK1)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(I2C1235SEL_A::PLL3_R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(I2C1235SEL_A::HSI_KER)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(I2C1235SEL_A::CSI_KER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "USBOTG 1 and 2 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBSEL_A {
    #[doc = "0: Disable the kernel clock"]
    DISABLE = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    PLL1_Q = 1,
    #[doc = "2: pll3_q selected as peripheral clock"]
    PLL3_Q = 2,
    #[doc = "3: HSI48 selected as peripheral clock"]
    HSI48 = 3,
}
impl From<USBSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: USBSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `USBSEL` reader - USBOTG 1 and 2 kernel clock source selection"]
pub struct USBSEL_R(crate::FieldReader<u8, USBSEL_A>);
impl USBSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        USBSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBSEL_A {
        match self.bits {
            0 => USBSEL_A::DISABLE,
            1 => USBSEL_A::PLL1_Q,
            2 => USBSEL_A::PLL3_Q,
            3 => USBSEL_A::HSI48,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == USBSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        **self == USBSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        **self == USBSEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        **self == USBSEL_A::HSI48
    }
}
impl core::ops::Deref for USBSEL_R {
    type Target = crate::FieldReader<u8, USBSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USBSEL` writer - USBOTG 1 and 2 kernel clock source selection"]
pub struct USBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Disable the kernel clock"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(USBSEL_A::DISABLE)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(USBSEL_A::PLL1_Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(USBSEL_A::PLL3_Q)
    }
    #[doc = "HSI48 selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi48(self) -> &'a mut W {
        self.variant(USBSEL_A::HSI48)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | ((value as u32 & 0x03) << 20);
        self.w
    }
}
#[doc = "HDMI-CEC kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CECSEL_A {
    #[doc = "0: LSE selected as peripheral clock"]
    LSE = 0,
    #[doc = "1: LSI selected as peripheral clock"]
    LSI = 1,
    #[doc = "2: csi_ker selected as peripheral clock"]
    CSI_KER = 2,
}
impl From<CECSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CECSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CECSEL` reader - HDMI-CEC kernel clock source selection"]
pub struct CECSEL_R(crate::FieldReader<u8, CECSEL_A>);
impl CECSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CECSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CECSEL_A> {
        match self.bits {
            0 => Some(CECSEL_A::LSE),
            1 => Some(CECSEL_A::LSI),
            2 => Some(CECSEL_A::CSI_KER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == CECSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == CECSEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        **self == CECSEL_A::CSI_KER
    }
}
impl core::ops::Deref for CECSEL_R {
    type Target = crate::FieldReader<u8, CECSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CECSEL` writer - HDMI-CEC kernel clock source selection"]
pub struct CECSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CECSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CECSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(CECSEL_A::LSE)
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(CECSEL_A::LSI)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(CECSEL_A::CSI_KER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 22)) | ((value as u32 & 0x03) << 22);
        self.w
    }
}
#[doc = "LPTIM1 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum LPTIM1SEL_A {
    #[doc = "0: rcc_pclk1 selected as peripheral clock"]
    RCC_PCLK1 = 0,
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
impl From<LPTIM1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: LPTIM1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `LPTIM1SEL` reader - LPTIM1 kernel clock source selection"]
pub struct LPTIM1SEL_R(crate::FieldReader<u8, LPTIM1SEL_A>);
impl LPTIM1SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        LPTIM1SEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LPTIM1SEL_A> {
        match self.bits {
            0 => Some(LPTIM1SEL_A::RCC_PCLK1),
            1 => Some(LPTIM1SEL_A::PLL2_P),
            2 => Some(LPTIM1SEL_A::PLL3_R),
            3 => Some(LPTIM1SEL_A::LSE),
            4 => Some(LPTIM1SEL_A::LSI),
            5 => Some(LPTIM1SEL_A::PER),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK1`"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        **self == LPTIM1SEL_A::RCC_PCLK1
    }
    #[doc = "Checks if the value of the field is `PLL2_P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        **self == LPTIM1SEL_A::PLL2_P
    }
    #[doc = "Checks if the value of the field is `PLL3_R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        **self == LPTIM1SEL_A::PLL3_R
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        **self == LPTIM1SEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        **self == LPTIM1SEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `PER`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        **self == LPTIM1SEL_A::PER
    }
}
impl core::ops::Deref for LPTIM1SEL_R {
    type Target = crate::FieldReader<u8, LPTIM1SEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPTIM1SEL` writer - LPTIM1 kernel clock source selection"]
pub struct LPTIM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> LPTIM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LPTIM1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::RCC_PCLK1)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::PLL2_P)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::PLL3_R)
    }
    #[doc = "LSE selected as peripheral clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSE)
    }
    #[doc = "LSI selected as peripheral clock"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::LSI)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(LPTIM1SEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
    #[inline(always)]
    pub fn usart234578sel(&self) -> USART234578SEL_R {
        USART234578SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - USART1 and 6 kernel clock source selection"]
    #[inline(always)]
    pub fn usart16910sel(&self) -> USART16910SEL_R {
        USART16910SEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 8:9 - RNG kernel clock source selection"]
    #[inline(always)]
    pub fn rngsel(&self) -> RNGSEL_R {
        RNGSEL_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 12:13 - I2C1,2,3 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c1235sel(&self) -> I2C1235SEL_R {
        I2C1235SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 20:21 - USBOTG 1 and 2 kernel clock source selection"]
    #[inline(always)]
    pub fn usbsel(&self) -> USBSEL_R {
        USBSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bits 22:23 - HDMI-CEC kernel clock source selection"]
    #[inline(always)]
    pub fn cecsel(&self) -> CECSEL_R {
        CECSEL_R::new(((self.bits >> 22) & 0x03) as u8)
    }
    #[doc = "Bits 28:30 - LPTIM1 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&self) -> LPTIM1SEL_R {
        LPTIM1SEL_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
    #[inline(always)]
    pub fn usart234578sel(&mut self) -> USART234578SEL_W {
        USART234578SEL_W { w: self }
    }
    #[doc = "Bits 3:5 - USART1 and 6 kernel clock source selection"]
    #[inline(always)]
    pub fn usart16910sel(&mut self) -> USART16910SEL_W {
        USART16910SEL_W { w: self }
    }
    #[doc = "Bits 8:9 - RNG kernel clock source selection"]
    #[inline(always)]
    pub fn rngsel(&mut self) -> RNGSEL_W {
        RNGSEL_W { w: self }
    }
    #[doc = "Bits 12:13 - I2C1,2,3 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c1235sel(&mut self) -> I2C1235SEL_W {
        I2C1235SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - USBOTG 1 and 2 kernel clock source selection"]
    #[inline(always)]
    pub fn usbsel(&mut self) -> USBSEL_W {
        USBSEL_W { w: self }
    }
    #[doc = "Bits 22:23 - HDMI-CEC kernel clock source selection"]
    #[inline(always)]
    pub fn cecsel(&mut self) -> CECSEL_W {
        CECSEL_W { w: self }
    }
    #[doc = "Bits 28:30 - LPTIM1 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim1sel(&mut self) -> LPTIM1SEL_W {
        LPTIM1SEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC Domain 2 Kernel Clock Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [d2ccip2r](index.html) module"]
pub struct D2CCIP2R_SPEC;
impl crate::RegisterSpec for D2CCIP2R_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [d2ccip2r::R](R) reader structure"]
impl crate::Readable for D2CCIP2R_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [d2ccip2r::W](W) writer structure"]
impl crate::Writable for D2CCIP2R_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets D2CCIP2R to value 0"]
impl crate::Resettable for D2CCIP2R_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
