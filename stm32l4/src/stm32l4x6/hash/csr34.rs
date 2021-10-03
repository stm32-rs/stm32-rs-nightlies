#[doc = "Register `CSR34` reader"]
pub struct R(crate::R<CSR34_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR34_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR34_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR34_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR34` writer"]
pub struct W(crate::W<CSR34_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR34_SPEC>;
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
impl From<crate::W<CSR34_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR34_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR34` reader - CSR34"]
pub struct CSR34_R(crate::FieldReader<u32, u32>);
impl CSR34_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSR34_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR34_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR34` writer - CSR34"]
pub struct CSR34_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR34_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR34"]
    #[inline(always)]
    pub fn csr34(&self) -> CSR34_R {
        CSR34_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR34"]
    #[inline(always)]
    pub fn csr34(&mut self) -> CSR34_W {
        CSR34_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr34](index.html) module"]
pub struct CSR34_SPEC;
impl crate::RegisterSpec for CSR34_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr34::R](R) reader structure"]
impl crate::Readable for CSR34_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr34::W](W) writer structure"]
impl crate::Writable for CSR34_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR34 to value 0"]
impl crate::Resettable for CSR34_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
