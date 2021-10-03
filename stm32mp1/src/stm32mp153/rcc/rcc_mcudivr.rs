#[doc = "Register `RCC_MCUDIVR` reader"]
pub struct R(crate::R<RCC_MCUDIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_MCUDIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_MCUDIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_MCUDIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_MCUDIVR` writer"]
pub struct W(crate::W<RCC_MCUDIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_MCUDIVR_SPEC>;
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
impl From<crate::W<RCC_MCUDIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_MCUDIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MCUDIV` reader - MCUDIV"]
pub struct MCUDIV_R(crate::FieldReader<u8, u8>);
impl MCUDIV_R {
    pub(crate) fn new(bits: u8) -> Self {
        MCUDIV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCUDIV_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MCUDIV` writer - MCUDIV"]
pub struct MCUDIV_W<'a> {
    w: &'a mut W,
}
impl<'a> MCUDIV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `MCUDIVRDY` reader - MCUDIVRDY"]
pub struct MCUDIVRDY_R(crate::FieldReader<bool, bool>);
impl MCUDIVRDY_R {
    pub(crate) fn new(bits: bool) -> Self {
        MCUDIVRDY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MCUDIVRDY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - MCUDIV"]
    #[inline(always)]
    pub fn mcudiv(&self) -> MCUDIV_R {
        MCUDIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 31 - MCUDIVRDY"]
    #[inline(always)]
    pub fn mcudivrdy(&self) -> MCUDIVRDY_R {
        MCUDIVRDY_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MCUDIV"]
    #[inline(always)]
    pub fn mcudiv(&mut self) -> MCUDIV_W {
        MCUDIV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the MCU sub-system clock prescaler. Refer to Section: Sub-system clock generation for additional information. If TZEN = , this register can only be modified in secure mode.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_mcudivr](index.html) module"]
pub struct RCC_MCUDIVR_SPEC;
impl crate::RegisterSpec for RCC_MCUDIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_mcudivr::R](R) reader structure"]
impl crate::Readable for RCC_MCUDIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_mcudivr::W](W) writer structure"]
impl crate::Writable for RCC_MCUDIVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_MCUDIVR to value 0x8000_0000"]
impl crate::Resettable for RCC_MCUDIVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
