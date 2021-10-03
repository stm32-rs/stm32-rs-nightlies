#[doc = "Register `CSR5` reader"]
pub struct R(crate::R<CSR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR5` writer"]
pub struct W(crate::W<CSR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR5_SPEC>;
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
impl From<crate::W<CSR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR5` reader - CSR5"]
pub struct CSR5_R(crate::FieldReader<u32, u32>);
impl CSR5_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSR5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR5_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR5` writer - CSR5"]
pub struct CSR5_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR5_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR5"]
    #[inline(always)]
    pub fn csr5(&self) -> CSR5_R {
        CSR5_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR5"]
    #[inline(always)]
    pub fn csr5(&mut self) -> CSR5_W {
        CSR5_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr5](index.html) module"]
pub struct CSR5_SPEC;
impl crate::RegisterSpec for CSR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr5::R](R) reader structure"]
impl crate::Readable for CSR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr5::W](W) writer structure"]
impl crate::Writable for CSR5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR5 to value 0"]
impl crate::Resettable for CSR5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
