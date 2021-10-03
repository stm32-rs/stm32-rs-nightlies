#[doc = "Register `NVDSL` reader"]
pub struct R(crate::R<NVDSL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NVDSL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NVDSL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NVDSL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NVDSL` writer"]
pub struct W(crate::W<NVDSL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NVDSL_SPEC>;
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
impl From<crate::W<NVDSL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NVDSL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LENG` reader - Non-volatile data segment length"]
pub struct LENG_R(crate::FieldReader<u16, u16>);
impl LENG_R {
    pub(crate) fn new(bits: u16) -> Self {
        LENG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LENG_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LENG` writer - Non-volatile data segment length"]
pub struct LENG_W<'a> {
    w: &'a mut W,
}
impl<'a> LENG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3fff << 8)) | ((value as u32 & 0x3fff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bits 8:21 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:21 - Non-volatile data segment length"]
    #[inline(always)]
    pub fn leng(&mut self) -> LENG_W {
        LENG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-volatile data segment length\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nvdsl](index.html) module"]
pub struct NVDSL_SPEC;
impl crate::RegisterSpec for NVDSL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nvdsl::R](R) reader structure"]
impl crate::Readable for NVDSL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nvdsl::W](W) writer structure"]
impl crate::Writable for NVDSL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets NVDSL to value 0"]
impl crate::Resettable for NVDSL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
