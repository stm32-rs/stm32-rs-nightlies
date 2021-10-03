#[doc = "Register `STGENC_PIDR1` reader"]
pub struct R(crate::R<STGENC_PIDR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENC_PIDR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENC_PIDR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENC_PIDR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PART_1` reader - PART_1"]
pub struct PART_1_R(crate::FieldReader<u8, u8>);
impl PART_1_R {
    pub(crate) fn new(bits: u8) -> Self {
        PART_1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PART_1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DES_0` reader - DES_0"]
pub struct DES_0_R(crate::FieldReader<u8, u8>);
impl DES_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        DES_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DES_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - PART_1"]
    #[inline(always)]
    pub fn part_1(&self) -> PART_1_R {
        PART_1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DES_0"]
    #[inline(always)]
    pub fn des_0(&self) -> DES_0_R {
        DES_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "STGENC peripheral ID1 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenc_pidr1](index.html) module"]
pub struct STGENC_PIDR1_SPEC;
impl crate::RegisterSpec for STGENC_PIDR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenc_pidr1::R](R) reader structure"]
impl crate::Readable for STGENC_PIDR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENC_PIDR1 to value 0xb1"]
impl crate::Resettable for STGENC_PIDR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xb1
    }
}
