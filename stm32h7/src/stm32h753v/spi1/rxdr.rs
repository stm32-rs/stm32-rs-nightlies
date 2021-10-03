#[doc = "Register `RXDR` reader"]
pub struct R(crate::R<RXDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDR` reader - Receive data register"]
pub struct RXDR_R(crate::FieldReader<u32, u32>);
impl RXDR_R {
    pub(crate) fn new(bits: u32) -> Self {
        RXDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDR_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Receive data register"]
    #[inline(always)]
    pub fn rxdr(&self) -> RXDR_R {
        RXDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Receive Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdr](index.html) module"]
pub struct RXDR_SPEC;
impl crate::RegisterSpec for RXDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdr::R](R) reader structure"]
impl crate::Readable for RXDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDR to value 0"]
impl crate::Resettable for RXDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
