#[doc = "Register `CMAR6` reader"]
pub struct R(crate::R<CMAR6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMAR6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMAR6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMAR6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMAR6` writer"]
pub struct W(crate::W<CMAR6_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMAR6_SPEC>;
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
impl From<crate::W<CMAR6_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMAR6_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MA` reader - Memory address"]
pub struct MA_R(crate::FieldReader<u32, u32>);
impl MA_R {
    pub(crate) fn new(bits: u32) -> Self {
        MA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MA` writer - Memory address"]
pub struct MA_W<'a> {
    w: &'a mut W,
}
impl<'a> MA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory address"]
    #[inline(always)]
    pub fn ma(&mut self) -> MA_W {
        MA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel x memory address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmar6](index.html) module"]
pub struct CMAR6_SPEC;
impl crate::RegisterSpec for CMAR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmar6::R](R) reader structure"]
impl crate::Readable for CMAR6_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmar6::W](W) writer structure"]
impl crate::Writable for CMAR6_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMAR6 to value 0"]
impl crate::Resettable for CMAR6_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
