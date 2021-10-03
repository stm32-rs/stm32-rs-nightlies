#[doc = "Register `RCC_RCK3SELR` reader"]
pub struct R(crate::R<RCC_RCK3SELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_RCK3SELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_RCK3SELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_RCK3SELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_RCK3SELR` writer"]
pub struct W(crate::W<RCC_RCK3SELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_RCK3SELR_SPEC>;
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
impl From<crate::W<RCC_RCK3SELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_RCK3SELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PLL3SRC` reader - PLL3SRC"]
pub struct PLL3SRC_R(crate::FieldReader<u8, u8>);
impl PLL3SRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        PLL3SRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL3SRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PLL3SRC` writer - PLL3SRC"]
pub struct PLL3SRC_W<'a> {
    w: &'a mut W,
}
impl<'a> PLL3SRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `PLL3SRCRDY` reader - PLL3SRCRDY"]
pub struct PLL3SRCRDY_R(crate::FieldReader<bool, bool>);
impl PLL3SRCRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        PLL3SRCRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PLL3SRCRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - PLL3SRC"]
    #[inline(always)]
    pub fn pll3src(&self) -> PLL3SRC_R {
        PLL3SRC_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 31 - PLL3SRCRDY"]
    #[inline(always)]
    pub fn pll3srcrdy(&self) -> PLL3SRCRDY_R {
        PLL3SRCRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - PLL3SRC"]
    #[inline(always)]
    pub fn pll3src(&mut self) -> PLL3SRC_W {
        PLL3SRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select the reference clock for PLL3. If TZEN = MCKPROT = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_rck3selr](index.html) module"]
pub struct RCC_RCK3SELR_SPEC;
impl crate::RegisterSpec for RCC_RCK3SELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_rck3selr::R](R) reader structure"]
impl crate::Readable for RCC_RCK3SELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_rck3selr::W](W) writer structure"]
impl crate::Writable for RCC_RCK3SELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_RCK3SELR to value 0x8000_0000"]
impl crate::Resettable for RCC_RCK3SELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
