#[doc = "Register `HR3` reader"]
pub struct R(crate::R<HR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `H3` reader - H3"]
pub struct H3_R(crate::FieldReader<u32, u32>);
impl H3_R {
    pub(crate) fn new(bits: u32) -> Self {
        H3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for H3_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - H3"]
    #[inline(always)]
    pub fn h3(&self) -> H3_R {
        H3_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "digest registers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hr3](index.html) module"]
pub struct HR3_SPEC;
impl crate::RegisterSpec for HR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hr3::R](R) reader structure"]
impl crate::Readable for HR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HR3 to value 0"]
impl crate::Resettable for HR3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
