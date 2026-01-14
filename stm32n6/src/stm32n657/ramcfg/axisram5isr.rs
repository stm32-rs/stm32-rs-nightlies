///Register `AXISRAM5ISR` reader
pub type R = crate::R<AXISRAM5ISRrs>;
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
        f.debug_struct("AXISRAM5ISR")
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG AXISRAM5 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`axisram5isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:AXISRAM5ISR)*/
pub struct AXISRAM5ISRrs;
impl crate::RegisterSpec for AXISRAM5ISRrs {
    type Ux = u32;
}
///`read()` method returns [`axisram5isr::R`](R) reader structure
impl crate::Readable for AXISRAM5ISRrs {}
///`reset()` method sets AXISRAM5ISR to value 0
impl crate::Resettable for AXISRAM5ISRrs {}
