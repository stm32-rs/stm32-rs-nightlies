///Register `AHBSRAM1ISR` reader
pub type R = crate::R<AHBSRAM1ISRrs>;
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
        f.debug_struct("AHBSRAM1ISR")
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG AHBSRAM1 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ahbsram1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RAMCFG:AHBSRAM1ISR)*/
pub struct AHBSRAM1ISRrs;
impl crate::RegisterSpec for AHBSRAM1ISRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbsram1isr::R`](R) reader structure
impl crate::Readable for AHBSRAM1ISRrs {}
///`reset()` method sets AHBSRAM1ISR to value 0
impl crate::Resettable for AHBSRAM1ISRrs {}
