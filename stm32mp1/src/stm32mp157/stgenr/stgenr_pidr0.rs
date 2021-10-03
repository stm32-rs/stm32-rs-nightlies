#[doc = "Register `STGENR_PIDR0` reader"]
pub struct R(crate::R<STGENR_PIDR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STGENR_PIDR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STGENR_PIDR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STGENR_PIDR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PART_0` reader - PART_0"]
pub struct PART_0_R(crate::FieldReader<u8, u8>);
impl PART_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        PART_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PART_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PART_0"]
    #[inline(always)]
    pub fn part_0(&self) -> PART_0_R {
        PART_0_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "STGENR peripheral ID0 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stgenr_pidr0](index.html) module"]
pub struct STGENR_PIDR0_SPEC;
impl crate::RegisterSpec for STGENR_PIDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stgenr_pidr0::R](R) reader structure"]
impl crate::Readable for STGENR_PIDR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STGENR_PIDR0 to value 0x01"]
impl crate::Resettable for STGENR_PIDR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x01
    }
}
