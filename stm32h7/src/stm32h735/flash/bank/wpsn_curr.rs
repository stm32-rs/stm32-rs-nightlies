#[doc = "Register `WPSN_CURR` reader"]
pub type R = crate::R<WPSN_CURRrs>;
#[doc = "Field `WRPSn` reader - Bank 1 sector write protection option status byte"]
pub type WRPSN_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Bank 1 sector write protection option status byte"]
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "FLASH write sector protection for bank 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpsn_curr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPSN_CURRrs;
impl crate::RegisterSpec for WPSN_CURRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpsn_curr::R`](R) reader structure"]
impl crate::Readable for WPSN_CURRrs {}
#[doc = "`reset()` method sets WPSN_CURR to value 0"]
impl crate::Resettable for WPSN_CURRrs {
    const RESET_VALUE: u32 = 0;
}
