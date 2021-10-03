#[doc = "Register `CSR17` reader"]
pub struct R(crate::R<CSR17_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR17_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR17_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR17_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR17` writer"]
pub struct W(crate::W<CSR17_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR17_SPEC>;
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
impl From<crate::W<CSR17_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR17_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR17` reader - CSR17"]
pub struct CSR17_R(crate::FieldReader<u32, u32>);
impl CSR17_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSR17_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR17_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR17` writer - CSR17"]
pub struct CSR17_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR17_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR17"]
    #[inline(always)]
    pub fn csr17(&self) -> CSR17_R {
        CSR17_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR17"]
    #[inline(always)]
    pub fn csr17(&mut self) -> CSR17_W {
        CSR17_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr17](index.html) module"]
pub struct CSR17_SPEC;
impl crate::RegisterSpec for CSR17_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr17::R](R) reader structure"]
impl crate::Readable for CSR17_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr17::W](W) writer structure"]
impl crate::Writable for CSR17_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR17 to value 0"]
impl crate::Resettable for CSR17_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
