#[doc = "Register `CSR20` reader"]
pub struct R(crate::R<CSR20_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR20_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR20_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR20_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR20` writer"]
pub struct W(crate::W<CSR20_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR20_SPEC>;
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
impl From<crate::W<CSR20_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR20_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR20` reader - CSR20"]
pub struct CSR20_R(crate::FieldReader<u32, u32>);
impl CSR20_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSR20_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR20_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR20` writer - CSR20"]
pub struct CSR20_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR20_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR20"]
    #[inline(always)]
    pub fn csr20(&self) -> CSR20_R {
        CSR20_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR20"]
    #[inline(always)]
    pub fn csr20(&mut self) -> CSR20_W {
        CSR20_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr20](index.html) module"]
pub struct CSR20_SPEC;
impl crate::RegisterSpec for CSR20_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr20::R](R) reader structure"]
impl crate::Readable for CSR20_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr20::W](W) writer structure"]
impl crate::Writable for CSR20_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR20 to value 0"]
impl crate::Resettable for CSR20_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
