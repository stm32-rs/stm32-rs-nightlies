#[doc = "Reader of register D1CCIPR"]
pub type R = crate::R<u32, super::D1CCIPR>;
#[doc = "Writer for register D1CCIPR"]
pub type W = crate::W<u32, super::D1CCIPR>;
#[doc = "Register D1CCIPR `reset()`'s with value 0"]
impl crate::ResetValue for super::D1CCIPR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "FMC kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum FMCSEL_A {
    #[doc = "0: rcc_hclk3 selected as peripheral clock"]
    RCC_HCLK3 = 0,
    #[doc = "1: pll1_q selected as peripheral clock"]
    PLL1_Q = 1,
    #[doc = "2: pll2_r selected as peripheral clock"]
    PLL2_R = 2,
    #[doc = "3: PER selected as peripheral clock"]
    PER = 3,
}
impl From<FMCSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: FMCSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `FMCSEL`"]
pub type FMCSEL_R = crate::R<u8, FMCSEL_A>;
impl FMCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FMCSEL_A {
        match self.bits {
            0 => FMCSEL_A::RCC_HCLK3,
            1 => FMCSEL_A::PLL1_Q,
            2 => FMCSEL_A::PLL2_R,
            3 => FMCSEL_A::PER,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RCC_HCLK3`"]
    #[inline(always)]
    pub fn is_rcc_hclk3(&self) -> bool {
        *self == FMCSEL_A::RCC_HCLK3
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == FMCSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_R`"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == FMCSEL_A::PLL2_R
    }
    #[doc = "Checks if the value of the field is `PER`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        *self == FMCSEL_A::PER
    }
}
#[doc = "Write proxy for field `FMCSEL`"]
pub struct FMCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCSEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "rcc_hclk3 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_hclk3(self) -> &'a mut W {
        self.variant(FMCSEL_A::RCC_HCLK3)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(FMCSEL_A::PLL1_Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(FMCSEL_A::PLL2_R)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(FMCSEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
#[doc = "QUADSPI kernel clock source selection"]
pub type QSPISEL_A = FMCSEL_A;
#[doc = "Reader of field `QSPISEL`"]
pub type QSPISEL_R = crate::R<u8, FMCSEL_A>;
#[doc = "Write proxy for field `QSPISEL`"]
pub struct QSPISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: QSPISEL_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "rcc_hclk3 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_hclk3(self) -> &'a mut W {
        self.variant(FMCSEL_A::RCC_HCLK3)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(FMCSEL_A::PLL1_Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(FMCSEL_A::PLL2_R)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(FMCSEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | (((value as u32) & 0x03) << 4);
        self.w
    }
}
#[doc = "SDMMC kernel clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SDMMCSEL_A {
    #[doc = "0: pll1_q selected as peripheral clock"]
    PLL1_Q = 0,
    #[doc = "1: pll2_r selected as peripheral clock"]
    PLL2_R = 1,
}
impl From<SDMMCSEL_A> for bool {
    #[inline(always)]
    fn from(variant: SDMMCSEL_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SDMMCSEL`"]
pub type SDMMCSEL_R = crate::R<bool, SDMMCSEL_A>;
impl SDMMCSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDMMCSEL_A {
        match self.bits {
            false => SDMMCSEL_A::PLL1_Q,
            true => SDMMCSEL_A::PLL2_R,
        }
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        *self == SDMMCSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_R`"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        *self == SDMMCSEL_A::PLL2_R
    }
}
#[doc = "Write proxy for field `SDMMCSEL`"]
pub struct SDMMCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMCSEL_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(SDMMCSEL_A::PLL1_Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(SDMMCSEL_A::PLL2_R)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "per_ck clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CKPERSEL_A {
    #[doc = "0: HSI selected as peripheral clock"]
    HSI = 0,
    #[doc = "1: CSI selected as peripheral clock"]
    CSI = 1,
    #[doc = "2: HSE selected as peripheral clock"]
    HSE = 2,
}
impl From<CKPERSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CKPERSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `CKPERSEL`"]
pub type CKPERSEL_R = crate::R<u8, CKPERSEL_A>;
impl CKPERSEL_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, CKPERSEL_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(CKPERSEL_A::HSI),
            1 => Val(CKPERSEL_A::CSI),
            2 => Val(CKPERSEL_A::HSE),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == CKPERSEL_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        *self == CKPERSEL_A::CSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == CKPERSEL_A::HSE
    }
}
#[doc = "Write proxy for field `CKPERSEL`"]
pub struct CKPERSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPERSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CKPERSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "HSI selected as peripheral clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(CKPERSEL_A::HSI)
    }
    #[doc = "CSI selected as peripheral clock"]
    #[inline(always)]
    pub fn csi(self) -> &'a mut W {
        self.variant(CKPERSEL_A::CSI)
    }
    #[doc = "HSE selected as peripheral clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(CKPERSEL_A::HSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | (((value as u32) & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    pub fn fmcsel(&self) -> FMCSEL_R {
        FMCSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - QUADSPI kernel clock source selection"]
    #[inline(always)]
    pub fn qspisel(&self) -> QSPISEL_R {
        QSPISEL_R::new(((self.bits >> 4) & 0x03) as u8)
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmcsel(&self) -> SDMMCSEL_R {
        SDMMCSEL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersel(&self) -> CKPERSEL_R {
        CKPERSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    pub fn fmcsel(&mut self) -> FMCSEL_W {
        FMCSEL_W { w: self }
    }
    #[doc = "Bits 4:5 - QUADSPI kernel clock source selection"]
    #[inline(always)]
    pub fn qspisel(&mut self) -> QSPISEL_W {
        QSPISEL_W { w: self }
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmcsel(&mut self) -> SDMMCSEL_W {
        SDMMCSEL_W { w: self }
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersel(&mut self) -> CKPERSEL_W {
        CKPERSEL_W { w: self }
    }
}
