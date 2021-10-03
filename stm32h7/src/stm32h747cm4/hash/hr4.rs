#[doc = "Register `HR4` reader"]
pub struct R(crate::R<HR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H4` reader - H4"]
pub struct H4_R(crate::FieldReader<u32, u32>);
impl H4_R {
    pub(crate) fn new(bits: u32) -> Self {
        H4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for H4_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - H4"]
    #[inline(always)]
    pub fn h4(&self) -> H4_R {
        H4_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr4](index.html) module"]
pub struct HR4_SPEC;
impl crate::RegisterSpec for HR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hr4::R](R) reader structure"]
impl crate::Readable for HR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HR4 to value 0"]
impl crate::Resettable for HR4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
