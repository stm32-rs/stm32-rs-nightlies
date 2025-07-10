///Register `BKPSRAMISR` reader
pub type R = crate::R<BKPSRAMISRrs>;
///Field `SEC` reader - ECC single error detected
pub type SEC_R = crate::BitReader;
///Field `DED` reader - ECC double-error interrupt enable
pub type DED_R = crate::BitReader;
///Field `SRAMBUSY` reader - SRAM busy with erase operation
pub type SRAMBUSY_R = crate::BitReader;
impl R {
    ///Bit 0 - ECC single error detected
    #[inline(always)]
    pub fn sec(&self) -> SEC_R {
        SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ECC double-error interrupt enable
    #[inline(always)]
    pub fn ded(&self) -> DED_R {
        DED_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SRAM busy with erase operation
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BKPSRAMISR")
            .field("sec", &self.sec())
            .field("ded", &self.ded())
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG BKPSRAM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`bkpsramisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:BKPSRAMISR)*/
pub struct BKPSRAMISRrs;
impl crate::RegisterSpec for BKPSRAMISRrs {
    type Ux = u32;
}
///`read()` method returns [`bkpsramisr::R`](R) reader structure
impl crate::Readable for BKPSRAMISRrs {}
///`reset()` method sets BKPSRAMISR to value 0
impl crate::Resettable for BKPSRAMISRrs {}
