///Register `WPSN_CURR` reader
pub type R = crate::R<WPSN_CURRrs>;
///Field `WRPSGn` reader - Bank 1 sector group protection option status byte Each FLASH_WPSGN_CUR1R bit reflects the write protection status of the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127 Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
pub type WRPSGN_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Bank 1 sector group protection option status byte Each FLASH_WPSGN_CUR1R bit reflects the write protection status of the corresponding group of four consecutive sectors in bank 1 (0: the group is write protected; 1: the group is not write protected) Bit 0: Group embedding sectors 0 to 3 Bit 1: Group embedding sectors 4 to 7 Bit N: Group embedding sectors 4 x N to 4 x N + 3 Bit 31: Group embedding sectors 124 to 127 Note: Bank 1 is limited to 16 and 64 sectors on STM32H7B0 and STM32H7A3xG devices, respectively.
    #[inline(always)]
    pub fn wrpsgn(&self) -> WRPSGN_R {
        WRPSGN_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WPSN_CURR")
            .field("wrpsgn", &self.wrpsgn())
            .finish()
    }
}
/**FLASH write sector group protection for bank 1

You can [`read`](crate::Reg::read) this register and get [`wpsn_curr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WPSN_CURRrs;
impl crate::RegisterSpec for WPSN_CURRrs {
    type Ux = u32;
}
///`read()` method returns [`wpsn_curr::R`](R) reader structure
impl crate::Readable for WPSN_CURRrs {}
///`reset()` method sets WPSN_CURR to value 0
impl crate::Resettable for WPSN_CURRrs {}
