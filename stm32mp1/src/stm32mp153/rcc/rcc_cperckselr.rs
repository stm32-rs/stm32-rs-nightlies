#[doc = "Register `RCC_CPERCKSELR` reader"]
pub struct R(crate::R<RCC_CPERCKSELR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCC_CPERCKSELR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCC_CPERCKSELR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCC_CPERCKSELR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCC_CPERCKSELR` writer"]
pub struct W(crate::W<RCC_CPERCKSELR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCC_CPERCKSELR_SPEC>;
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
impl From<crate::W<RCC_CPERCKSELR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCC_CPERCKSELR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CKPERSRC` reader - CKPERSRC"]
pub struct CKPERSRC_R(crate::FieldReader<u8, u8>);
impl CKPERSRC_R {
    pub(crate) fn new(bits: u8) -> Self {
        CKPERSRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CKPERSRC_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CKPERSRC` writer - CKPERSRC"]
pub struct CKPERSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> CKPERSRC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    pub fn ckpersrc(&self) -> CKPERSRC_R {
        CKPERSRC_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - CKPERSRC"]
    #[inline(always)]
    pub fn ckpersrc(&mut self) -> CKPERSRC_W {
        CKPERSRC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "This register is used to select an oscillator source as kernel clock for the per_ck clock. The per_ck clock is distributed to several peripherals. Refer to Section: Clock enabling delays.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcc_cperckselr](index.html) module"]
pub struct RCC_CPERCKSELR_SPEC;
impl crate::RegisterSpec for RCC_CPERCKSELR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcc_cperckselr::R](R) reader structure"]
impl crate::Readable for RCC_CPERCKSELR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcc_cperckselr::W](W) writer structure"]
impl crate::Writable for RCC_CPERCKSELR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RCC_CPERCKSELR to value 0"]
impl crate::Resettable for RCC_CPERCKSELR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
