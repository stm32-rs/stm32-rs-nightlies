///Register `FLEXRAMISR` reader
pub type R = crate::R<FLEXRAMISRrs>;
///Field `SRAMBUSY` reader - SRAM busy with erase operation
pub type SRAMBUSY_R = crate::BitReader;
impl R {
    ///Bit 8 - SRAM busy with erase operation
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXRAMISR")
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG FLEXRAM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`flexramisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:FLEXRAMISR)*/
pub struct FLEXRAMISRrs;
impl crate::RegisterSpec for FLEXRAMISRrs {
    type Ux = u32;
}
///`read()` method returns [`flexramisr::R`](R) reader structure
impl crate::Readable for FLEXRAMISRrs {}
///`reset()` method sets FLEXRAMISR to value 0
impl crate::Resettable for FLEXRAMISRrs {}
