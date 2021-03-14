#[doc = "Reader of register CDCCIP2R"]
pub type R = crate::R<u32, super::CDCCIP2R>;
#[doc = "Writer for register CDCCIP2R"]
pub type W = crate::W<u32, super::CDCCIP2R>;
#[doc = "Register CDCCIP2R `reset()`'s with value 0"]
impl crate::ResetValue for super::CDCCIP2R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
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
#[doc = "Reader of field `USART234578SEL`"]
pub type USART234578SEL_R = crate::R<u8, USART234578SEL_A>;
impl USART234578SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USART234578SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USART234578SEL_A::RCC_PCLK1),
            1 => Val(USART234578SEL_A::PLL2_Q),
            2 => Val(USART234578SEL_A::PLL3_Q),
            3 => Val(USART234578SEL_A::HSI_KER),
            4 => Val(USART234578SEL_A::CSI_KER),
            5 => Val(USART234578SEL_A::LSE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK1`"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == USART234578SEL_A::RCC_PCLK1
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART234578SEL_A::PLL2_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USART234578SEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART234578SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART234578SEL_A::CSI_KER
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART234578SEL_A::LSE
    }
}
#[doc = "Write proxy for field `USART234578SEL`"]
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
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "USART1, 6, 9 and 10 kernel clock source selection\n\nValue on reset: 0"]
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
#[doc = "Reader of field `USART16910SEL`"]
pub type USART16910SEL_R = crate::R<u8, USART16910SEL_A>;
impl USART16910SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, USART16910SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(USART16910SEL_A::RCC_PCLK2),
            1 => Val(USART16910SEL_A::PLL2_Q),
            2 => Val(USART16910SEL_A::PLL3_Q),
            3 => Val(USART16910SEL_A::HSI_KER),
            4 => Val(USART16910SEL_A::CSI_KER),
            5 => Val(USART16910SEL_A::LSE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK2`"]
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == USART16910SEL_A::RCC_PCLK2
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == USART16910SEL_A::PLL2_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USART16910SEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == USART16910SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == USART16910SEL_A::CSI_KER
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == USART16910SEL_A::LSE
    }
}
#[doc = "Write proxy for field `USART16910SEL`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
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
#[doc = "Reader of field `RNGSEL`"]
pub type RNGSEL_R = crate::R<u8, RNGSEL_A>;
impl RNGSEL_R {
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
        *self == RNGSEL_A::HSI48
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == RNGSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == RNGSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == RNGSEL_A::LSI
    }
}
#[doc = "Write proxy for field `RNGSEL`"]
pub struct RNGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> RNGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RNGSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "I2C1,2,3 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I2C123SEL_A {
    #[doc = "0: rcc_pclk1 selected as peripheral clock"]
    RCC_PCLK1 = 0,
    #[doc = "1: pll3_r selected as peripheral clock"]
    PLL3_R = 1,
    #[doc = "2: hsi_ker selected as peripheral clock"]
    HSI_KER = 2,
    #[doc = "3: csi_ker selected as peripheral clock"]
    CSI_KER = 3,
}
impl From<I2C123SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: I2C123SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `I2C123SEL`"]
pub type I2C123SEL_R = crate::R<u8, I2C123SEL_A>;
impl I2C123SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C123SEL_A {
        match self.bits {
            0 => I2C123SEL_A::RCC_PCLK1,
            1 => I2C123SEL_A::PLL3_R,
            2 => I2C123SEL_A::HSI_KER,
            3 => I2C123SEL_A::CSI_KER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK1`"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == I2C123SEL_A::RCC_PCLK1
    }
    #[doc = "Checks if the value of the field is `PLL3_R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == I2C123SEL_A::PLL3_R
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == I2C123SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == I2C123SEL_A::CSI_KER
    }
}
#[doc = "Write proxy for field `I2C123SEL`"]
pub struct I2C123SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C123SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C123SEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "rcc_pclk1 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk1(self) -> &'a mut W {
        self.variant(I2C123SEL_A::RCC_PCLK1)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(I2C123SEL_A::PLL3_R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(I2C123SEL_A::HSI_KER)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(I2C123SEL_A::CSI_KER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | (((value as u32) & 0x03) << 12);
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
#[doc = "Reader of field `USBSEL`"]
pub type USBSEL_R = crate::R<u8, USBSEL_A>;
impl USBSEL_R {
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
        *self == USBSEL_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == USBSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == USBSEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI48`"]
    #[inline(always)]
    pub fn is_hsi48(&self) -> bool {
        *self == USBSEL_A::HSI48
    }
}
#[doc = "Write proxy for field `USBSEL`"]
pub struct USBSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
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
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
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
#[doc = "Reader of field `CECSEL`"]
pub type CECSEL_R = crate::R<u8, CECSEL_A>;
impl CECSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CECSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CECSEL_A::LSE),
            1 => Val(CECSEL_A::LSI),
            2 => Val(CECSEL_A::CSI_KER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == CECSEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == CECSEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == CECSEL_A::CSI_KER
    }
}
#[doc = "Write proxy for field `CECSEL`"]
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
        self.w.bits = (self.w.bits & !(0x03 << 22)) | (((value as u32) & 0x03) << 22);
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
#[doc = "Reader of field `LPTIM1SEL`"]
pub type LPTIM1SEL_R = crate::R<u8, LPTIM1SEL_A>;
impl LPTIM1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, LPTIM1SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(LPTIM1SEL_A::RCC_PCLK1),
            1 => Val(LPTIM1SEL_A::PLL2_P),
            2 => Val(LPTIM1SEL_A::PLL3_R),
            3 => Val(LPTIM1SEL_A::LSE),
            4 => Val(LPTIM1SEL_A::LSI),
            5 => Val(LPTIM1SEL_A::PER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK1`"]
    #[inline(always)]
    pub fn is_rcc_pclk1(&self) -> bool {
        *self == LPTIM1SEL_A::RCC_PCLK1
    }
    #[doc = "Checks if the value of the field is `PLL2_P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == LPTIM1SEL_A::PLL2_P
    }
    #[doc = "Checks if the value of the field is `PLL3_R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == LPTIM1SEL_A::PLL3_R
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == LPTIM1SEL_A::LSE
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == LPTIM1SEL_A::LSI
    }
    #[doc = "Checks if the value of the field is `PER`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == LPTIM1SEL_A::PER
    }
}
#[doc = "Write proxy for field `LPTIM1SEL`"]
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
        self.w.bits = (self.w.bits & !(0x07 << 28)) | (((value as u32) & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
    #[inline(always)]
    pub fn usart234578sel(&self) -> USART234578SEL_R {
        USART234578SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - USART1, 6, 9 and 10 kernel clock source selection"]
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
    pub fn i2c123sel(&self) -> I2C123SEL_R {
        I2C123SEL_R::new(((self.bits >> 12) & 0x03) as u8)
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
    #[doc = "Bits 3:5 - USART1, 6, 9 and 10 kernel clock source selection"]
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
    pub fn i2c123sel(&mut self) -> I2C123SEL_W {
        I2C123SEL_W { w: self }
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
}
