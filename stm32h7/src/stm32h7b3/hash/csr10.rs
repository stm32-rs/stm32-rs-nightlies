#[doc = "Register `CSR10` reader"]
pub struct R(crate::R<CSR10_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR10_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR10_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR10_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSR10` writer"]
pub struct W(crate::W<CSR10_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSR10_SPEC>;
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
impl From<crate::W<CSR10_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSR10_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CSR10` reader - CSR10"]
pub struct CSR10_R(crate::FieldReader<u32, u32>);
impl CSR10_R {
    pub(crate) fn new(bits: u32) -> Self {
        CSR10_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSR10_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSR10` writer - CSR10"]
pub struct CSR10_W<'a> {
    w: &'a mut W,
}
impl<'a> CSR10_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - CSR10"]
    #[inline(always)]
    pub fn csr10(&self) -> CSR10_R {
        CSR10_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - CSR10"]
    #[inline(always)]
    pub fn csr10(&mut self) -> CSR10_W {
        CSR10_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "context swap registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr10](index.html) module"]
pub struct CSR10_SPEC;
impl crate::RegisterSpec for CSR10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr10::R](R) reader structure"]
impl crate::Readable for CSR10_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csr10::W](W) writer structure"]
impl crate::Writable for CSR10_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSR10 to value 0"]
impl crate::Resettable for CSR10_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
