#[doc = "Register `TZSC_MPCWM2_UPWMR` reader"]
pub struct R(crate::R<TZSC_MPCWM2_UPWMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TZSC_MPCWM2_UPWMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TZSC_MPCWM2_UPWMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TZSC_MPCWM2_UPWMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TZSC_MPCWM2_UPWMR` writer"]
pub struct W(crate::W<TZSC_MPCWM2_UPWMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TZSC_MPCWM2_UPWMR_SPEC>;
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
impl From<crate::W<TZSC_MPCWM2_UPWMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TZSC_MPCWM2_UPWMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LGTH` reader - LGTH"]
pub struct LGTH_R(crate::FieldReader<u16, u16>);
impl LGTH_R {
    pub(crate) fn new(bits: u16) -> Self {
        LGTH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LGTH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LGTH` writer - LGTH"]
pub struct LGTH_W<'a> {
    w: &'a mut W,
}
impl<'a> LGTH_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 16)) | ((value as u32 & 0x0fff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:27 - LGTH"]
    #[inline(always)]
    pub fn lgth(&self) -> LGTH_R {
        LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:27 - LGTH"]
    #[inline(always)]
    pub fn lgth(&mut self) -> LGTH_W {
        LGTH_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Unprivileged Water Mark 2 register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tzsc_mpcwm2_upwmr](index.html) module"]
pub struct TZSC_MPCWM2_UPWMR_SPEC;
impl crate::RegisterSpec for TZSC_MPCWM2_UPWMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tzsc_mpcwm2_upwmr::R](R) reader structure"]
impl crate::Readable for TZSC_MPCWM2_UPWMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tzsc_mpcwm2_upwmr::W](W) writer structure"]
impl crate::Writable for TZSC_MPCWM2_UPWMR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TZSC_MPCWM2_UPWMR to value 0x0fff_0000"]
impl crate::Resettable for TZSC_MPCWM2_UPWMR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0fff_0000
    }
}
