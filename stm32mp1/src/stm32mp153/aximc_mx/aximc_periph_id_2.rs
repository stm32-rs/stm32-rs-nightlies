#[doc = "Register `AXIMC_PERIPH_ID_2` reader"]
pub struct R(crate::R<AXIMC_PERIPH_ID_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_PERIPH_ID_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_PERIPH_ID_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_PERIPH_ID_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERIPH_ID_2` reader - PERIPH_ID_2"]
pub struct PERIPH_ID_2_R(crate::FieldReader<u8, u8>);
impl PERIPH_ID_2_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERIPH_ID_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPH_ID_2_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PERIPH_ID_2"]
    #[inline(always)]
    pub fn periph_id_2(&self) -> PERIPH_ID_2_R {
        PERIPH_ID_2_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXIMC peripheral ID2 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_2](index.html) module"]
pub struct AXIMC_PERIPH_ID_2_SPEC;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aximc_periph_id_2::R](R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AXIMC_PERIPH_ID_2 to value 0x3b"]
impl crate::Resettable for AXIMC_PERIPH_ID_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x3b
    }
}
