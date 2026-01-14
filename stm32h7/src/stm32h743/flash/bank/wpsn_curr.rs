///Register `WPSN_CURR` reader
pub type R = crate::R<WPSN_CURRrs>;
///Field `WRPSn` reader - Bank 1 sector write protection option status byte
pub type WRPSN_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Bank 1 sector write protection option status byte
    #[inline(always)]
    pub fn wrpsn(&self) -> WRPSN_R {
        WRPSN_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPSN_CURR")
            .field("wrpsn", &self.wrpsn())
            .finish()
    }
}
/**FLASH write sector protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_curr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WPSN_CURRrs;
impl crate::RegisterSpec for WPSN_CURRrs {
    type Ux = u32;
}
///`read()` method returns [`wpsn_curr::R`](R) reader structure
impl crate::Readable for WPSN_CURRrs {}
///`reset()` method sets WPSN_CURR to value 0
impl crate::Resettable for WPSN_CURRrs {}
