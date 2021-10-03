#[doc = "Register `AXIMC_PERIPH_ID_3` reader"]
pub struct R(crate::R<AXIMC_PERIPH_ID_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AXIMC_PERIPH_ID_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AXIMC_PERIPH_ID_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AXIMC_PERIPH_ID_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CUST_MOD_NUM` reader - CUST_MOD_NUM"]
pub struct CUST_MOD_NUM_R(crate::FieldReader<u8, u8>);
impl CUST_MOD_NUM_R {
    pub(crate) fn new(bits: u8) -> Self {
        CUST_MOD_NUM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CUST_MOD_NUM_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REV_AND` reader - REV_AND"]
pub struct REV_AND_R(crate::FieldReader<u8, u8>);
impl REV_AND_R {
    pub(crate) fn new(bits: u8) -> Self {
        REV_AND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REV_AND_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:3 - CUST_MOD_NUM"]
    #[inline(always)]
    pub fn cust_mod_num(&self) -> CUST_MOD_NUM_R {
        CUST_MOD_NUM_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - REV_AND"]
    #[inline(always)]
    pub fn rev_and(&self) -> REV_AND_R {
        REV_AND_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "AXIMC peripheral ID3 register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [aximc_periph_id_3](index.html) module"]
pub struct AXIMC_PERIPH_ID_3_SPEC;
impl crate::RegisterSpec for AXIMC_PERIPH_ID_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [aximc_periph_id_3::R](R) reader structure"]
impl crate::Readable for AXIMC_PERIPH_ID_3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets AXIMC_PERIPH_ID_3 to value 0"]
impl crate::Resettable for AXIMC_PERIPH_ID_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
