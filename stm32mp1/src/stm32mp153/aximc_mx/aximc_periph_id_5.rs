#[doc = "Register `AXIMC_PERIPH_ID_5` reader"]
pub struct R(crate::R<AXIMC_PERIPH_ID_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_PERIPH_ID_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_PERIPH_ID_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_PERIPH_ID_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PERIPH_ID_5` reader - PERIPH_ID_5"]
pub struct PERIPH_ID_5_R(crate::FieldReader<u8, u8>);
impl PERIPH_ID_5_R {
    pub(crate) fn new(bits: u8) -> Self {
        PERIPH_ID_5_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERIPH_ID_5_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - PERIPH_ID_5"]
    #[inline(always)]
    pub fn periph_id_5(&self) -> PERIPH_ID_5_R {
        PERIPH_ID_5_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AXIMC peripheral ID5 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_5](index.html) module"]
pub struct AXIMC_PERIPH_ID_5_SPEC;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aximc_periph_id_5::R](R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_5_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AXIMC_PERIPH_ID_5 to value 0"]
impl crate::Resettable for AXIMC_PERIPH_ID_5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
