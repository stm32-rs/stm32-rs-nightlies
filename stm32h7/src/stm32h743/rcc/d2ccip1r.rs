#[doc = "Reader of register D2CCIP1R"]
pub type R = crate::R<u32, super::D2CCIP1R>;
#[doc = "Writer for register D2CCIP1R"]
pub type W = crate::W<u32, super::D2CCIP1R>;
#[doc = "Register D2CCIP1R `reset()`'s with value 0"]
impl crate::ResetValue for super::D2CCIP1R {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "SAI1 and DFSDM1 kernel Aclk clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SAI1SEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    PLL1_Q = 0,
    #[doc = "1: pll2_p selected as peripheral clock"]
    PLL2_P = 1,
    #[doc = "2: pll3_p selected as peripheral clock"]
    PLL3_P = 2,
    #[doc = "3: I2S_CKIN selected as peripheral clock"]
    I2S_CKIN = 3,
    #[doc = "4: PER selected as peripheral clock"]
    PER = 4,
}
impl From<SAI1SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SAI1SEL`"]
pub type SAI1SEL_R = crate::R<u8, SAI1SEL_A>;
impl SAI1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SAI1SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SAI1SEL_A::PLL1_Q),
            1 => Val(SAI1SEL_A::PLL2_P),
            2 => Val(SAI1SEL_A::PLL3_P),
            3 => Val(SAI1SEL_A::I2S_CKIN),
            4 => Val(SAI1SEL_A::PER),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SAI1SEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_P`"]
    #[inline(always)]
    pub fn is_pll2_p(&self) -> bool {
        *self == SAI1SEL_A::PLL2_P
    }
    #[doc = "Checks if the value of the field is `PLL3_P`"]
    #[inline(always)]
    pub fn is_pll3_p(&self) -> bool {
        *self == SAI1SEL_A::PLL3_P
    }
    #[doc = "Checks if the value of the field is `I2S_CKIN`"]
    #[inline(always)]
    pub fn is_i2s_ckin(&self) -> bool {
        *self == SAI1SEL_A::I2S_CKIN
    }
    #[doc = "Checks if the value of the field is `PER`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == SAI1SEL_A::PER
    }
}
#[doc = "Write proxy for field `SAI1SEL`"]
pub struct SAI1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI1SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL1_Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL2_P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL3_P)
    }
    #[doc = "I2S_CKIN selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1SEL_A::I2S_CKIN)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "SAI2 and SAI3 kernel clock source selection"]
pub type SAI23SEL_A = SAI1SEL_A;
#[doc = "Reader of field `SAI23SEL`"]
pub type SAI23SEL_R = crate::R<u8, SAI1SEL_A>;
#[doc = "Write proxy for field `SAI23SEL`"]
pub struct SAI23SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAI23SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SAI23SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL1_Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL2_P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL3_P)
    }
    #[doc = "I2S_CKIN selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1SEL_A::I2S_CKIN)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 6)) | (((value as u32) & 0x07) << 6);
        self.w
    }
}
#[doc = "SPI/I2S1,2 and 3 kernel clock source selection"]
pub type SPI123SEL_A = SAI1SEL_A;
#[doc = "Reader of field `SPI123SEL`"]
pub type SPI123SEL_R = crate::R<u8, SAI1SEL_A>;
#[doc = "Write proxy for field `SPI123SEL`"]
pub struct SPI123SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI123SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI123SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL1_Q)
    }
    #[doc = "pll2_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL2_P)
    }
    #[doc = "pll3_p selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_p(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PLL3_P)
    }
    #[doc = "I2S_CKIN selected as peripheral clock"]
    #[inline(always)]
    pub fn i2s_ckin(self) -> &'a mut W {
        self.variant(SAI1SEL_A::I2S_CKIN)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(SAI1SEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | (((value as u32) & 0x07) << 12);
        self.w
    }
}
#[doc = "SPI4 and 5 kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPI45SEL_A {
    #[doc = "0: APB clock selected as peripheral clock"]
    APB = 0,
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
impl From<SPI45SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPI45SEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPI45SEL`"]
pub type SPI45SEL_R = crate::R<u8, SPI45SEL_A>;
impl SPI45SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, SPI45SEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(SPI45SEL_A::APB),
            1 => Val(SPI45SEL_A::PLL2_Q),
            2 => Val(SPI45SEL_A::PLL3_Q),
            3 => Val(SPI45SEL_A::HSI_KER),
            4 => Val(SPI45SEL_A::CSI_KER),
            5 => Val(SPI45SEL_A::HSE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `APB`"]
    #[inline(always)]
    pub fn is_apb(&self) -> bool {
        *self == SPI45SEL_A::APB
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == SPI45SEL_A::PLL2_Q
    }
    #[doc = "Checks if the value of the field is `PLL3_Q`"]
    #[inline(always)]
    pub fn is_pll3_q(&self) -> bool {
        *self == SPI45SEL_A::PLL3_Q
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPI45SEL_A::HSI_KER
    }
    #[doc = "Checks if the value of the field is `CSI_KER`"]
    #[inline(always)]
    pub fn is_csi_ker(&self) -> bool {
        *self == SPI45SEL_A::CSI_KER
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SPI45SEL_A::HSE
    }
}
#[doc = "Write proxy for field `SPI45SEL`"]
pub struct SPI45SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPI45SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPI45SEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "APB clock selected as peripheral clock"]
    #[inline(always)]
    pub fn apb(self) -> &'a mut W {
        self.variant(SPI45SEL_A::APB)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(SPI45SEL_A::PLL2_Q)
    }
    #[doc = "pll3_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_q(self) -> &'a mut W {
        self.variant(SPI45SEL_A::PLL3_Q)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPI45SEL_A::HSI_KER)
    }
    #[doc = "csi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn csi_ker(self) -> &'a mut W {
        self.variant(SPI45SEL_A::CSI_KER)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SPI45SEL_A::HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "SPDIFRX kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SPDIFSEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    PLL1_Q = 0,
    #[doc = "1: pll2_r selected as peripheral clock"]
    PLL2_R = 1,
    #[doc = "2: pll3_r selected as peripheral clock"]
    PLL3_R = 2,
    #[doc = "3: hsi_ker selected as peripheral clock"]
    HSI_KER = 3,
}
impl From<SPDIFSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIFSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `SPDIFSEL`"]
pub type SPDIFSEL_R = crate::R<u8, SPDIFSEL_A>;
impl SPDIFSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIFSEL_A {
        match self.bits {
            0 => SPDIFSEL_A::PLL1_Q,
            1 => SPDIFSEL_A::PLL2_R,
            2 => SPDIFSEL_A::PLL3_R,
            3 => SPDIFSEL_A::HSI_KER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SPDIFSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_R`"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SPDIFSEL_A::PLL2_R
    }
    #[doc = "Checks if the value of the field is `PLL3_R`"]
    #[inline(always)]
    pub fn is_pll3_r(&self) -> bool {
        *self == SPDIFSEL_A::PLL3_R
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SPDIFSEL_A::HSI_KER
    }
}
#[doc = "Write proxy for field `SPDIFSEL`"]
pub struct SPDIFSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SPDIFSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SPDIFSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SPDIFSEL_A::PLL1_Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(SPDIFSEL_A::PLL2_R)
    }
    #[doc = "pll3_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll3_r(self) -> &'a mut W {
        self.variant(SPDIFSEL_A::PLL3_R)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SPDIFSEL_A::HSI_KER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 20)) | (((value as u32) & 0x03) << 20);
        self.w
    }
}
#[doc = "DFSDM1 kernel Clk clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DFSDM1SEL_A {
    #[doc = "0: rcc_pclk2 selected as peripheral clock"]
    RCC_PCLK2 = 0,
    #[doc = "1: System clock selected as peripheral clock"]
    SYS = 1,
}
impl From<DFSDM1SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DFSDM1SEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DFSDM1SEL`"]
pub type DFSDM1SEL_R = crate::R<bool, DFSDM1SEL_A>;
impl DFSDM1SEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DFSDM1SEL_A {
        match self.bits {
            false => DFSDM1SEL_A::RCC_PCLK2,
            true => DFSDM1SEL_A::SYS,
        }
    }
    #[doc = "Checks if the value of the field is `RCC_PCLK2`"]
    #[inline(always)]
    pub fn is_rcc_pclk2(&self) -> bool {
        *self == DFSDM1SEL_A::RCC_PCLK2
    }
    #[doc = "Checks if the value of the field is `SYS`"]
    #[inline(always)]
    pub fn is_sys(&self) -> bool {
        *self == DFSDM1SEL_A::SYS
    }
}
#[doc = "Write proxy for field `DFSDM1SEL`"]
pub struct DFSDM1SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DFSDM1SEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DFSDM1SEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "rcc_pclk2 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_pclk2(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::RCC_PCLK2)
    }
    #[doc = "System clock selected as peripheral clock"]
    #[inline(always)]
    pub fn sys(self) -> &'a mut W {
        self.variant(DFSDM1SEL_A::SYS)
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
#[doc = "FDCAN kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FDCANSEL_A {
    #[doc = "0: HSE selected as peripheral clock"]
    HSE = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    PLL1_Q = 1,
    #[doc = "2: pll2_q selected as peripheral clock"]
    PLL2_Q = 2,
}
impl From<FDCANSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FDCANSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FDCANSEL`"]
pub type FDCANSEL_R = crate::R<u8, FDCANSEL_A>;
impl FDCANSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, FDCANSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(FDCANSEL_A::HSE),
            1 => Val(FDCANSEL_A::PLL1_Q),
            2 => Val(FDCANSEL_A::PLL2_Q),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == FDCANSEL_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FDCANSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_Q`"]
    #[inline(always)]
    pub fn is_pll2_q(&self) -> bool {
        *self == FDCANSEL_A::PLL2_Q
    }
}
#[doc = "Write proxy for field `FDCANSEL`"]
pub struct FDCANSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FDCANSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FDCANSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(FDCANSEL_A::HSE)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(FDCANSEL_A::PLL1_Q)
    }
    #[doc = "pll2_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_q(self) -> &'a mut W {
        self.variant(FDCANSEL_A::PLL2_Q)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
#[doc = "SWPMI kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SWPSEL_A {
    #[doc = "0: pclk selected as peripheral clock"]
    PCLK = 0,
    #[doc = "1: hsi_ker selected as peripheral clock"]
    HSI_KER = 1,
}
impl From<SWPSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SWPSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SWPSEL`"]
pub type SWPSEL_R = crate::R<bool, SWPSEL_A>;
impl SWPSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWPSEL_A {
        match self.bits {
            false => SWPSEL_A::PCLK,
            true => SWPSEL_A::HSI_KER,
        }
    }
    #[doc = "Checks if the value of the field is `PCLK`"]
    #[inline(always)]
    pub fn is_pclk(&self) -> bool {
        *self == SWPSEL_A::PCLK
    }
    #[doc = "Checks if the value of the field is `HSI_KER`"]
    #[inline(always)]
    pub fn is_hsi_ker(&self) -> bool {
        *self == SWPSEL_A::HSI_KER
    }
}
#[doc = "Write proxy for field `SWPSEL`"]
pub struct SWPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SWPSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "pclk selected as peripheral clock"]
    #[inline(always)]
    pub fn pclk(self) -> &'a mut W {
        self.variant(SWPSEL_A::PCLK)
    }
    #[doc = "hsi_ker selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi_ker(self) -> &'a mut W {
        self.variant(SWPSEL_A::HSI_KER)
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&self) -> SAI1SEL_R {
        SAI1SEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 6:8 - SAI2 and SAI3 kernel clock source selection"]
    #[inline(always)]
    pub fn sai23sel(&self) -> SAI23SEL_R {
        SAI23SEL_R::new(((self.bits >> 6) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection"]
    #[inline(always)]
    pub fn spi123sel(&self) -> SPI123SEL_R {
        SPI123SEL_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection"]
    #[inline(always)]
    pub fn spi45sel(&self) -> SPI45SEL_R {
        SPI45SEL_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    pub fn spdifsel(&self) -> SPDIFSEL_R {
        SPDIFSEL_R::new(((self.bits >> 20) & 0x03) as u8)
    }
    #[doc = "Bit 24 - DFSDM1 kernel Clk clock source selection"]
    #[inline(always)]
    pub fn dfsdm1sel(&self) -> DFSDM1SEL_R {
        DFSDM1SEL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection"]
    #[inline(always)]
    pub fn fdcansel(&self) -> FDCANSEL_R {
        FDCANSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection"]
    #[inline(always)]
    pub fn swpsel(&self) -> SWPSEL_R {
        SWPSEL_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection"]
    #[inline(always)]
    pub fn sai1sel(&mut self) -> SAI1SEL_W {
        SAI1SEL_W { w: self }
    }
    #[doc = "Bits 6:8 - SAI2 and SAI3 kernel clock source selection"]
    #[inline(always)]
    pub fn sai23sel(&mut self) -> SAI23SEL_W {
        SAI23SEL_W { w: self }
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection"]
    #[inline(always)]
    pub fn spi123sel(&mut self) -> SPI123SEL_W {
        SPI123SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection"]
    #[inline(always)]
    pub fn spi45sel(&mut self) -> SPI45SEL_W {
        SPI45SEL_W { w: self }
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    pub fn spdifsel(&mut self) -> SPDIFSEL_W {
        SPDIFSEL_W { w: self }
    }
    #[doc = "Bit 24 - DFSDM1 kernel Clk clock source selection"]
    #[inline(always)]
    pub fn dfsdm1sel(&mut self) -> DFSDM1SEL_W {
        DFSDM1SEL_W { w: self }
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection"]
    #[inline(always)]
    pub fn fdcansel(&mut self) -> FDCANSEL_W {
        FDCANSEL_W { w: self }
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection"]
    #[inline(always)]
    pub fn swpsel(&mut self) -> SWPSEL_W {
        SWPSEL_W { w: self }
    }
}
