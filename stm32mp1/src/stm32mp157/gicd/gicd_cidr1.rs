#[doc = "Register `GICD_CIDR1` reader"]
pub struct R(crate::R<GICD_CIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIDR1` reader - CIDR1"]
pub struct CIDR1_R(crate::FieldReader<u32, u32>);
impl CIDR1_R {
    pub(crate) fn new(bits: u32) -> Self {
        CIDR1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIDR1_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - CIDR1"]
    #[inline(always)]
    pub fn cidr1(&self) -> CIDR1_R {
        CIDR1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GICD component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr1](index.html) module"]
pub struct GICD_CIDR1_SPEC;
impl crate::RegisterSpec for GICD_CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_cidr1::R](R) reader structure"]
impl crate::Readable for GICD_CIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_CIDR1 to value 0xf0"]
impl crate::Resettable for GICD_CIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
