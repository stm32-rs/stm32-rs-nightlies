#[doc = "Register `CNDTR1` reader"]
pub struct R(crate::R<CNDTR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CNDTR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CNDTR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CNDTR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CNDTR1` writer"]
pub struct W(crate::W<CNDTR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CNDTR1_SPEC>;
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
impl From<crate::W<CNDTR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CNDTR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDT` reader - Number of data to transfer"]
pub struct NDT_R(crate::FieldReader<u32, u32>);
impl NDT_R {
    pub(crate) fn new(bits: u32) -> Self {
        NDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NDT_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NDT` writer - Number of data to transfer"]
pub struct NDT_W<'a> {
    w: &'a mut W,
}
impl<'a> NDT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0003_ffff) | (value as u32 & 0x0003_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:17 - Number of data to transfer"]
    #[inline(always)]
    pub fn ndt(&self) -> NDT_R {
        NDT_R::new((self.bits & 0x0003_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:17 - Number of data to transfer"]
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
#[doc = "channel x number of data register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cndtr1](index.html) module"]
pub struct CNDTR1_SPEC;
impl crate::RegisterSpec for CNDTR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cndtr1::R](R) reader structure"]
impl crate::Readable for CNDTR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cndtr1::W](W) writer structure"]
impl crate::Writable for CNDTR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CNDTR1 to value 0"]
impl crate::Resettable for CNDTR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
