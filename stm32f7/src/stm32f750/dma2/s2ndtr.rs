#[doc = "Register `S2NDTR` reader"]
pub struct R(crate::R<S2NDTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<S2NDTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<S2NDTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<S2NDTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `S2NDTR` writer"]
pub struct W(crate::W<S2NDTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<S2NDTR_SPEC>;
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
impl From<crate::W<S2NDTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<S2NDTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - Number of data items to transfer"]
pub struct NDT_R(crate::FieldReader<u16, u16>);
impl NDT_R {
    pub(crate) fn new(bits: u16) -> Self {
        NDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDT` writer - Number of data items to transfer"]
pub struct NDT_W<'a> {
    w: &'a mut W,
}
impl<'a> NDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data items to transfer"]
    #[inline(always)]
    pub fn ndt(&mut self) -> NDT_W {
        NDT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "stream x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [s2ndtr](index.html) module"]
pub struct S2NDTR_SPEC;
impl crate::RegisterSpec for S2NDTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [s2ndtr::R](R) reader structure"]
impl crate::Readable for S2NDTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [s2ndtr::W](W) writer structure"]
impl crate::Writable for S2NDTR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets S2NDTR to value 0"]
impl crate::Resettable for S2NDTR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
