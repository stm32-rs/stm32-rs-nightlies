#[doc = "Register `RCC_QSPICKSELR` reader"]
pub struct R(crate::R<RCC_QSPICKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_QSPICKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_QSPICKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_QSPICKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_QSPICKSELR` writer"]
pub struct W(crate::W<RCC_QSPICKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_QSPICKSELR_SPEC>;
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
impl From<crate::W<RCC_QSPICKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_QSPICKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `QSPISRC` reader - QSPISRC"]
pub struct QSPISRC_R(crate::FieldReader<u8, u8>);
impl QSPISRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        QSPISRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for QSPISRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `QSPISRC` writer - QSPISRC"]
pub struct QSPISRC_W<'a> {
    w: &'a mut W,
}
impl<'a> QSPISRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - QSPISRC"]
    #[inline(always)]
    pub fn qspisrc(&self) -> QSPISRC_R {
        QSPISRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - QSPISRC"]
    #[inline(always)]
    pub fn qspisrc(&mut self) -> QSPISRC_W {
        QSPISRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to control the selection of the kernel clock for the QUADSPI. Note that changing the clock source on-the-fly is allowed, and will not generate any timing violation, however the user has to ensure that both the previous and the new clock sources are present during the switching, and for the whole transition time. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_qspickselr](index.html) module"]
pub struct RCC_QSPICKSELR_SPEC;
impl crate::RegisterSpec for RCC_QSPICKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_qspickselr::R](R) reader structure"]
impl crate::Readable for RCC_QSPICKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_qspickselr::W](W) writer structure"]
impl crate::Writable for RCC_QSPICKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_QSPICKSELR to value 0"]
impl crate::Resettable for RCC_QSPICKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
