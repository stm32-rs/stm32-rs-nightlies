#[doc = "Register `TZSC_MPCWM1_NSWMR2` reader"]
pub struct R(crate::R<TZSC_MPCWM1_NSWMR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_MPCWM1_NSWMR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_MPCWM1_NSWMR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_MPCWM1_NSWMR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZSC_MPCWM1_NSWMR2` writer"]
pub struct W(crate::W<TZSC_MPCWM1_NSWMR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_MPCWM1_NSWMR2_SPEC>;
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
impl From<crate::W<TZSC_MPCWM1_NSWMR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_MPCWM1_NSWMR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NSWM2STRT` reader - NSWM2STRT"]
pub struct NSWM2STRT_R(crate::FieldReader<u16, u16>);
impl NSWM2STRT_R {
    pub(crate) fn new(bits: u16) -> Self {
        NSWM2STRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSWM2STRT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSWM2STRT` writer - NSWM2STRT"]
pub struct NSWM2STRT_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWM2STRT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff) | (value as u32 & 0x07ff);
        self.w
    }
}
#[doc = "Field `NSWM2LGTH` reader - NSWM2LGTH"]
pub struct NSWM2LGTH_R(crate::FieldReader<u16, u16>);
impl NSWM2LGTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        NSWM2LGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NSWM2LGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NSWM2LGTH` writer - NSWM2LGTH"]
pub struct NSWM2LGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> NSWM2LGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:10 - NSWM2STRT"]
    #[inline(always)]
    pub fn nswm2strt(&self) -> NSWM2STRT_R {
        NSWM2STRT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:27 - NSWM2LGTH"]
    #[inline(always)]
    pub fn nswm2lgth(&self) -> NSWM2LGTH_R {
        NSWM2LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - NSWM2STRT"]
    #[inline(always)]
    pub fn nswm2strt(&mut self) -> NSWM2STRT_W {
        NSWM2STRT_W { w: self }
    }
    #[doc = "Bits 16:27 - NSWM2LGTH"]
    #[inline(always)]
    pub fn nswm2lgth(&mut self) -> NSWM2LGTH_W {
        NSWM2LGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TZSC external memory non-secure watermark register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm1_nswmr2](index.html) module"]
pub struct TZSC_MPCWM1_NSWMR2_SPEC;
impl crate::RegisterSpec for TZSC_MPCWM1_NSWMR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzsc_mpcwm1_nswmr2::R](R) reader structure"]
impl crate::Readable for TZSC_MPCWM1_NSWMR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm1_nswmr2::W](W) writer structure"]
impl crate::Writable for TZSC_MPCWM1_NSWMR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZSC_MPCWM1_NSWMR2 to value 0"]
impl crate::Resettable for TZSC_MPCWM1_NSWMR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
