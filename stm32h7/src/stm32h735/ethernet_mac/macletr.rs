#[doc = "Register `MACLETR` reader"]
pub struct R(crate::R<MACLETR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MACLETR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MACLETR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MACLETR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MACLETR` writer"]
pub struct W(crate::W<MACLETR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MACLETR_SPEC>;
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
impl From<crate::W<MACLETR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MACLETR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPIET` reader - LPI Entry Timer"]
pub struct LPIET_R(crate::FieldReader<u32, u32>);
impl LPIET_R {
    pub(crate) fn new(bits: u32) -> Self {
        LPIET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPIET_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPIET` writer - LPI Entry Timer"]
pub struct LPIET_W<'a> {
    w: &'a mut W,
}
impl<'a> LPIET_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0001_ffff << 3)) | ((value as u32 & 0x0001_ffff) << 3);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:19 - LPI Entry Timer"]
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new(((self.bits >> 3) & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 3:19 - LPI Entry Timer"]
    #[inline(always)]
    pub fn lpiet(&mut self) -> LPIET_W {
        LPIET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPI entry timer register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [macletr](index.html) module"]
pub struct MACLETR_SPEC;
impl crate::RegisterSpec for MACLETR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [macletr::R](R) reader structure"]
impl crate::Readable for MACLETR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [macletr::W](W) writer structure"]
impl crate::Writable for MACLETR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MACLETR to value 0"]
impl crate::Resettable for MACLETR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
