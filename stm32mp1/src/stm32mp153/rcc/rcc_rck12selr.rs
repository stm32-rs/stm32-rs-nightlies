#[doc = "Register `RCC_RCK12SELR` reader"]
pub struct R(crate::R<RCC_RCK12SELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_RCK12SELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_RCK12SELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_RCK12SELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_RCK12SELR` writer"]
pub struct W(crate::W<RCC_RCK12SELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_RCK12SELR_SPEC>;
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
impl From<crate::W<RCC_RCK12SELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_RCK12SELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL12SRC` reader - PLL12SRC"]
pub struct PLL12SRC_R(crate::FieldReader<u8, u8>);
impl PLL12SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLL12SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL12SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL12SRC` writer - PLL12SRC"]
pub struct PLL12SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL12SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PLL12SRCRDY` reader - PLL12SRCRDY"]
pub struct PLL12SRCRDY_R(crate::FieldReader<bool, bool>);
impl PLL12SRCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL12SRCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL12SRCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - PLL12SRC"]
    #[inline(always)]
    pub fn pll12src(&self) -> PLL12SRC_R {
        PLL12SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - PLL12SRCRDY"]
    #[inline(always)]
    pub fn pll12srcrdy(&self) -> PLL12SRCRDY_R {
        PLL12SRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL12SRC"]
    #[inline(always)]
    pub fn pll12src(&mut self) -> PLL12SRC_W {
        PLL12SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the reference clock for PLL1 and PLL2. If TZEN = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See Section: The clock restore sequence description for details.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rck12selr](index.html) module"]
pub struct RCC_RCK12SELR_SPEC;
impl crate::RegisterSpec for RCC_RCK12SELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_rck12selr::R](R) reader structure"]
impl crate::Readable for RCC_RCK12SELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_rck12selr::W](W) writer structure"]
impl crate::Writable for RCC_RCK12SELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_RCK12SELR to value 0x8000_0000"]
impl crate::Resettable for RCC_RCK12SELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
