#[doc = "Register `CALFACT` reader"]
pub struct R(crate::R<CALFACT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALFACT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALFACT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALFACT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALFACT` writer"]
pub struct W(crate::W<CALFACT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALFACT_SPEC>;
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
impl From<crate::W<CALFACT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALFACT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CALFACT` reader - ADC calibration factor in single-ended mode"]
pub struct CALFACT_R(crate::FieldReader<u8, u8>);
impl CALFACT_R {
    pub(crate) fn new(bits: u8) -> Self {
        CALFACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CALFACT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CALFACT` writer - ADC calibration factor in single-ended mode"]
pub struct CALFACT_W<'a> {
    w: &'a mut W,
}
impl<'a> CALFACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact(&self) -> CALFACT_R {
        CALFACT_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact(&mut self) -> CALFACT_W {
        CALFACT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC calibration factors register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calfact](index.html) module"]
pub struct CALFACT_SPEC;
impl crate::RegisterSpec for CALFACT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calfact::R](R) reader structure"]
impl crate::Readable for CALFACT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calfact::W](W) writer structure"]
impl crate::Writable for CALFACT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALFACT to value 0"]
impl crate::Resettable for CALFACT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
