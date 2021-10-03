#[doc = "Register `CDCCIPR` reader"]
pub struct R(crate::R<CDCCIPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDCCIPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDCCIPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDCCIPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDCCIPR` writer"]
pub struct W(crate::W<CDCCIPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDCCIPR_SPEC>;
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
impl From<crate::W<CDCCIPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDCCIPR_SPEC>) -> Self {
        W(writer)
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
#[doc = "Field `FMCSEL` reader - FMC kernel clock source selection"]
pub struct FMCSEL_R(crate::FieldReader<u8, FMCSEL_A>);
impl FMCSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        FMCSEL_R(crate::FieldReader::new(bits))
    }
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
        **self == FMCSEL_A::RCC_HCLK3
    }
    #[doc = "Checks if the value of the field is `PLL1_Q`"]
    #[inline(always)]
    pub fn is_pll1_q(&self) -> bool {
        **self == FMCSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_R`"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        **self == FMCSEL_A::PLL2_R
    }
    #[doc = "Checks if the value of the field is `PER`"]
    #[inline(always)]
    pub fn is_per(&self) -> bool {
        **self == FMCSEL_A::PER
    }
}
impl core::ops::Deref for FMCSEL_R {
    type Target = crate::FieldReader<u8, FMCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FMCSEL` writer - FMC kernel clock source selection"]
pub struct FMCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> FMCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FMCSEL_A) -> &'a mut W {
        self.bits(variant.into())
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
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "OCTOSPI kernel clock source selection"]
pub type OCTOSPISEL_A = FMCSEL_A;
#[doc = "Field `OCTOSPISEL` reader - OCTOSPI kernel clock source selection"]
pub type OCTOSPISEL_R = FMCSEL_R;
#[doc = "Field `OCTOSPISEL` writer - OCTOSPI kernel clock source selection"]
pub struct OCTOSPISEL_W<'a> {
    w: &'a mut W,
}
impl<'a> OCTOSPISEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OCTOSPISEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "rcc_hclk3 selected as peripheral clock"]
    #[inline(always)]
    pub fn rcc_hclk3(self) -> &'a mut W {
        self.variant(OCTOSPISEL_A::RCC_HCLK3)
    }
    #[doc = "pll1_q selected as peripheral clock"]
    #[inline(always)]
    pub fn pll1_q(self) -> &'a mut W {
        self.variant(OCTOSPISEL_A::PLL1_Q)
    }
    #[doc = "pll2_r selected as peripheral clock"]
    #[inline(always)]
    pub fn pll2_r(self) -> &'a mut W {
        self.variant(OCTOSPISEL_A::PLL2_R)
    }
    #[doc = "PER selected as peripheral clock"]
    #[inline(always)]
    pub fn per(self) -> &'a mut W {
        self.variant(OCTOSPISEL_A::PER)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 4)) | ((value as u32 & 0x03) << 4);
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
#[doc = "Field `SDMMCSEL` reader - SDMMC kernel clock source selection"]
pub struct SDMMCSEL_R(crate::FieldReader<bool, SDMMCSEL_A>);
impl SDMMCSEL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SDMMCSEL_R(crate::FieldReader::new(bits))
    }
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
        **self == SDMMCSEL_A::PLL1_Q
    }
    #[doc = "Checks if the value of the field is `PLL2_R`"]
    #[inline(always)]
    pub fn is_pll2_r(&self) -> bool {
        **self == SDMMCSEL_A::PLL2_R
    }
}
impl core::ops::Deref for SDMMCSEL_R {
    type Target = crate::FieldReader<bool, SDMMCSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SDMMCSEL` writer - SDMMC kernel clock source selection"]
pub struct SDMMCSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SDMMCSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SDMMCSEL_A) -> &'a mut W {
        self.bit(variant.into())
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
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
#[doc = "Field `CKPERSEL` reader - per_ck clock source selection"]
pub struct CKPERSEL_R(crate::FieldReader<u8, CKPERSEL_A>);
impl CKPERSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKPERSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CKPERSEL_A> {
        match self.bits {
            0 => Some(CKPERSEL_A::HSI),
            1 => Some(CKPERSEL_A::CSI),
            2 => Some(CKPERSEL_A::HSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        **self == CKPERSEL_A::HSI
    }
    #[doc = "Checks if the value of the field is `CSI`"]
    #[inline(always)]
    pub fn is_csi(&self) -> bool {
        **self == CKPERSEL_A::CSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        **self == CKPERSEL_A::HSE
    }
}
impl core::ops::Deref for CKPERSEL_R {
    type Target = crate::FieldReader<u8, CKPERSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPERSEL` writer - per_ck clock source selection"]
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
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    pub fn fmcsel(&self) -> FMCSEL_R {
        FMCSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 4:5 - OCTOSPI kernel clock source selection"]
    #[inline(always)]
    pub fn octospisel(&self) -> OCTOSPISEL_R {
        OCTOSPISEL_R::new(((self.bits >> 4) & 0x03) as u8)
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
    #[doc = "Bits 4:5 - OCTOSPI kernel clock source selection"]
    #[inline(always)]
    pub fn octospisel(&mut self) -> OCTOSPISEL_W {
        OCTOSPISEL_W { w: self }
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
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCC CPU domain kernel clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdccipr](index.html) module"]
pub struct CDCCIPR_SPEC;
impl crate::RegisterSpec for CDCCIPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdccipr::R](R) reader structure"]
impl crate::Readable for CDCCIPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdccipr::W](W) writer structure"]
impl crate::Writable for CDCCIPR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CDCCIPR to value 0"]
impl crate::Resettable for CDCCIPR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
