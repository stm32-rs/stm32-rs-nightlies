#[doc = "Register `STGENR_CIDR1` reader"]
pub struct R(crate::R<STGENR_CIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_CIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_CIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_CIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PRMBL_1` reader - PRMBL_1"]
pub struct PRMBL_1_R(crate::FieldReader<u8, u8>);
impl PRMBL_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PRMBL_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PRMBL_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CLASS` reader - CLASS"]
pub struct CLASS_R(crate::FieldReader<u8, u8>);
impl CLASS_R {
    pub(crate) fn new(bits: u8) -> Self {
        CLASS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CLASS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - PRMBL_1"]
    #[inline(always)]
    pub fn prmbl_1(&self) -> PRMBL_1_R {
        PRMBL_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CLASS"]
    #[inline(always)]
    pub fn class(&self) -> CLASS_R {
        CLASS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "STGENR component ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_cidr1](index.html) module"]
pub struct STGENR_CIDR1_SPEC;
impl crate::RegisterSpec for STGENR_CIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenr_cidr1::R](R) reader structure"]
impl crate::Readable for STGENR_CIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENR_CIDR1 to value 0xf0"]
impl crate::Resettable for STGENR_CIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xf0
    }
}
