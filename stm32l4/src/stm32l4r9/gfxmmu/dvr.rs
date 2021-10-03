#[doc = "Register `DVR` reader"]
pub struct R(crate::R<DVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DVR` writer"]
pub struct W(crate::W<DVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DVR_SPEC>;
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
impl From<crate::W<DVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DV` reader - Default value"]
pub struct DV_R(crate::FieldReader<u32, u32>);
impl DV_R {
    pub(crate) fn new(bits: u32) -> Self {
        DV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DV_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DV` writer - Default value"]
pub struct DV_W<'a> {
    w: &'a mut W,
}
impl<'a> DV_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Default value"]
    #[inline(always)]
    pub fn dv(&self) -> DV_R {
        DV_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Default value"]
    #[inline(always)]
    pub fn dv(&mut self) -> DV_W {
        DV_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Graphic MMU default value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dvr](index.html) module"]
pub struct DVR_SPEC;
impl crate::RegisterSpec for DVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dvr::R](R) reader structure"]
impl crate::Readable for DVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dvr::W](W) writer structure"]
impl crate::Writable for DVR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DVR to value 0"]
impl crate::Resettable for DVR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
