///Register `M2ISR` reader
pub type R = crate::R<M2ISRrs>;
///Field `PED` reader - Parity error detected
pub type PED_R = crate::BitReader;
///Field `SRAMBUSY` reader - SRAM2 busy with erase operation. Note: Depending on the SRAM2, the erase operation can be performed due to software request, system reset if the enabled by user option, tamper detection or RDP regression. Refer to Table38.
pub type SRAMBUSY_R = crate::BitReader;
impl R {
    ///Bit 1 - Parity error detected
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SRAM2 busy with erase operation. Note: Depending on the SRAM2, the erase operation can be performed due to software request, system reset if the enabled by user option, tamper detection or RDP regression. Refer to Table38.
    #[inline(always)]
    pub fn srambusy(&self) -> SRAMBUSY_R {
        SRAMBUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2ISR")
            .field("ped", &self.ped())
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG SRAM2 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`m2isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA55.html#RAMCFG:M2ISR)*/
pub struct M2ISRrs;
impl crate::RegisterSpec for M2ISRrs {
    type Ux = u32;
}
///`read()` method returns [`m2isr::R`](R) reader structure
impl crate::Readable for M2ISRrs {}
///`reset()` method sets M2ISR to value 0
impl crate::Resettable for M2ISRrs {}
