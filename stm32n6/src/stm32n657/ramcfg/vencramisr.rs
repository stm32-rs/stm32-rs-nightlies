///Register `VENCRAMISR` reader
pub type R = crate::R<VENCRAMISRrs>;
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
        f.debug_struct("VENCRAMISR")
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG VENCRAM interrupt status register

You can [`read`](crate::Reg::read) this register and get [`vencramisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:VENCRAMISR)*/
pub struct VENCRAMISRrs;
impl crate::RegisterSpec for VENCRAMISRrs {
    type Ux = u32;
}
///`read()` method returns [`vencramisr::R`](R) reader structure
impl crate::Readable for VENCRAMISRrs {}
///`reset()` method sets VENCRAMISR to value 0
impl crate::Resettable for VENCRAMISRrs {}
