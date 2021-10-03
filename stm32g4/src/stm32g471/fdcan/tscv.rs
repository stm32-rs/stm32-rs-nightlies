#[doc = "Register `TSCV` reader"]
pub struct R(crate::R<TSCV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TSCV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TSCV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TSCV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TSCV` writer"]
pub struct W(crate::W<TSCV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TSCV_SPEC>;
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
impl From<crate::W<TSCV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TSCV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSC` reader - TSC"]
pub struct TSC_R(crate::FieldReader<u16, u16>);
impl TSC_R {
    pub(crate) fn new(bits: u16) -> Self {
        TSC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSC` writer - TSC"]
pub struct TSC_W<'a> {
    w: &'a mut W,
}
impl<'a> TSC_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - TSC"]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TSC"]
    #[inline(always)]
    pub fn tsc(&mut self) -> TSC_W {
        TSC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FDCAN Timestamp Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tscv](index.html) module"]
pub struct TSCV_SPEC;
impl crate::RegisterSpec for TSCV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tscv::R](R) reader structure"]
impl crate::Readable for TSCV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tscv::W](W) writer structure"]
impl crate::Writable for TSCV_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TSCV_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
