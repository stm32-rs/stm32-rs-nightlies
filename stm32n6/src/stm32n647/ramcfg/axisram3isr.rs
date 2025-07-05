///Register `AXISRAM3ISR` reader
pub type R = crate::R<AXISRAM3ISRrs>;
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
        f.debug_struct("AXISRAM3ISR")
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG AXISRAM3 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`axisram3isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RAMCFG:AXISRAM3ISR)*/
pub struct AXISRAM3ISRrs;
impl crate::RegisterSpec for AXISRAM3ISRrs {
    type Ux = u32;
}
///`read()` method returns [`axisram3isr::R`](R) reader structure
impl crate::Readable for AXISRAM3ISRrs {}
///`reset()` method sets AXISRAM3ISR to value 0
impl crate::Resettable for AXISRAM3ISRrs {}
