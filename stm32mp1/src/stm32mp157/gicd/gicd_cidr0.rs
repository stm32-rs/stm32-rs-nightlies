#[doc = "Register `GICD_CIDR0` reader"]
pub struct R(crate::R<GICD_CIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GICD_CIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GICD_CIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GICD_CIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CIDR0` reader - CIDR0"]
pub struct CIDR0_R(crate::FieldReader<u32, u32>);
impl CIDR0_R {
    pub(crate) fn new(bits: u32) -> Self {
        CIDR0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CIDR0_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - CIDR0"]
    #[inline(always)]
    pub fn cidr0(&self) -> CIDR0_R {
        CIDR0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "GICD component ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gicd_cidr0](index.html) module"]
pub struct GICD_CIDR0_SPEC;
impl crate::RegisterSpec for GICD_CIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gicd_cidr0::R](R) reader structure"]
impl crate::Readable for GICD_CIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets GICD_CIDR0 to value 0x0d"]
impl crate::Resettable for GICD_CIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0d
    }
}
