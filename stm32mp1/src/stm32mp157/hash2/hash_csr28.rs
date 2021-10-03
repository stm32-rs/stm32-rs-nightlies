#[doc = "Register `HASH_CSR28` reader"]
pub struct R(crate::R<HASH_CSR28_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HASH_CSR28_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HASH_CSR28_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HASH_CSR28_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HASH_CSR28` writer"]
pub struct W(crate::W<HASH_CSR28_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HASH_CSR28_SPEC>;
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
impl From<crate::W<HASH_CSR28_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HASH_CSR28_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CS28` reader - CS28"]
pub struct CS28_R(crate::FieldReader<u32, u32>);
impl CS28_R {
    pub(crate) fn new(bits: u32) -> Self {
        CS28_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CS28_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS28` writer - CS28"]
pub struct CS28_W<'a> {
    w: &'a mut W,
}
impl<'a> CS28_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CS28"]
    #[inline(always)]
    pub fn cs28(&self) -> CS28_R {
        CS28_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CS28"]
    #[inline(always)]
    pub fn cs28(&mut self) -> CS28_W {
        CS28_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HASH context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hash_csr28](index.html) module"]
pub struct HASH_CSR28_SPEC;
impl crate::RegisterSpec for HASH_CSR28_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hash_csr28::R](R) reader structure"]
impl crate::Readable for HASH_CSR28_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hash_csr28::W](W) writer structure"]
impl crate::Writable for HASH_CSR28_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HASH_CSR28 to value 0"]
impl crate::Resettable for HASH_CSR28_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
