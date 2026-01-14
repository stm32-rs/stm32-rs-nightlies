///Register `AHBSRAM2ISR` reader
pub type R = crate::R<AHBSRAM2ISRrs>;
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
        f.debug_struct("AHBSRAM2ISR")
            .field("srambusy", &self.srambusy())
            .finish()
    }
}
/**RAMCFG AHBSRAM2 interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ahbsram2isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RAMCFG:AHBSRAM2ISR)*/
pub struct AHBSRAM2ISRrs;
impl crate::RegisterSpec for AHBSRAM2ISRrs {
    type Ux = u32;
}
///`read()` method returns [`ahbsram2isr::R`](R) reader structure
impl crate::Readable for AHBSRAM2ISRrs {}
///`reset()` method sets AHBSRAM2ISR to value 0
impl crate::Resettable for AHBSRAM2ISRrs {}
