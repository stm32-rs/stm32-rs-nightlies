#[doc = "Register `CALFACT2` reader"]
pub struct R(crate::R<CALFACT2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALFACT2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALFACT2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALFACT2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALFACT2` writer"]
pub struct W(crate::W<CALFACT2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALFACT2_SPEC>;
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
impl From<crate::W<CALFACT2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALFACT2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LINCALFACT` reader - Linearity Calibration Factor"]
pub struct LINCALFACT_R(crate::FieldReader<u32, u32>);
impl LINCALFACT_R {
    pub(crate) fn new(bits: u32) -> Self {
        LINCALFACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LINCALFACT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LINCALFACT` writer - Linearity Calibration Factor"]
pub struct LINCALFACT_W<'a> {
    w: &'a mut W,
}
impl<'a> LINCALFACT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3fff_ffff) | (value as u32 & 0x3fff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    pub fn lincalfact(&self) -> LINCALFACT_R {
        LINCALFACT_R::new((self.bits & 0x3fff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:29 - Linearity Calibration Factor"]
    #[inline(always)]
    pub fn lincalfact(&mut self) -> LINCALFACT_W {
        LINCALFACT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Calibration Factor register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calfact2](index.html) module"]
pub struct CALFACT2_SPEC;
impl crate::RegisterSpec for CALFACT2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calfact2::R](R) reader structure"]
impl crate::Readable for CALFACT2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calfact2::W](W) writer structure"]
impl crate::Writable for CALFACT2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALFACT2 to value 0"]
impl crate::Resettable for CALFACT2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
