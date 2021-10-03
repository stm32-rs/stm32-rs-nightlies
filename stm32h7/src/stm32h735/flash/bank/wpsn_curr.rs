#[doc = "Register `WPSN_CURR` reader"]
pub struct R(crate::R<WPSN_CURR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WPSN_CURR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WPSN_CURR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WPSN_CURR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `WRPSn` reader - Bank 1 sector write protection option status byte"]
pub struct WRPSN_R(crate::FieldReader<u8, u8>);
impl WRPSN_R {
    pub(crate) fn new(bits: u8) -> Self {
        WRPSN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WRPSN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FLASH write sector protection for bank 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wpsn_curr](index.html) module"]
pub struct WPSN_CURR_SPEC;
impl crate::RegisterSpec for WPSN_CURR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wpsn_curr::R](R) reader structure"]
impl crate::Readable for WPSN_CURR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WPSN_CURR to value 0"]
impl crate::Resettable for WPSN_CURR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
