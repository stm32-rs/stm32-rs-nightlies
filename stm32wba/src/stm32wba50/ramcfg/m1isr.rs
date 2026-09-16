///Register `M1ISR` reader
pub type R = crate::R<M1ISRrs>;
///Field `SRAMBUSY` reader - SRAM busy with erase operation. Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the enabled by user option, tamper detection or RDP regression. Refer to Table38.
pub type SRAMBUSY_R = crate::BitReader;
impl R {
    ///Bit 8 - SRAM busy with erase operation. Note: Depending on the SRAM, the erase operation can be performed due to software request, system reset if the enabled by user option, tamper detection or RDP regression. Refer to Table38.
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M1ISR")
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG SRAM1 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#RAMCFG:M1ISR)*/
pub struct M1ISRrs;
impl crate::RegisterSpec for M1ISRrs {
    type Ux = u32;
}
///`read()` method returns [`m1isr::R`](R) reader structure
impl crate::Readable for M1ISRrs {}
///`reset()` method sets M1ISR to value 0
impl crate::Resettable for M1ISRrs {}
