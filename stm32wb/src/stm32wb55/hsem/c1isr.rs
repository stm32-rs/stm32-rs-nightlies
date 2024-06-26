///Register `C1ISR` reader
pub type R = crate::R<C1ISRrs>;
///Field `ISFm` reader - CPU(n) semaphore m status bit before enable (mask)
pub type ISFM_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CPU(n) semaphore m status bit before enable (mask)
    #[inline(always)]
    pub fn isfm(&self) -> ISFM_R {
        ISFM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1ISR").field("isfm", &self.isfm()).finish()
    }
}
/**HSEM Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#HSEM:C1ISR)*/
pub struct C1ISRrs;
impl crate::RegisterSpec for C1ISRrs {
    type Ux = u32;
}
///`read()` method returns [`c1isr::R`](R) reader structure
impl crate::Readable for C1ISRrs {}
///`reset()` method sets C1ISR to value 0
impl crate::Resettable for C1ISRrs {
    const RESET_VALUE: u32 = 0;
}
