///Register `C2ISR` reader
pub type R = crate::R<C2ISRrs>;
///Field `ISFm` reader - CPU(2) semaphore m status bit before enable (mask).
pub type ISFM_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CPU(2) semaphore m status bit before enable (mask).
    #[inline(always)]
    pub fn isfm(&self) -> ISFM_R {
        ISFM_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C2ISR").field("isfm", &self.isfm()).finish()
    }
}
/**HSEM Interrupt status register

You can [`read`](crate::Reg::read) this register and get [`c2isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#HSEM:C2ISR)*/
pub struct C2ISRrs;
impl crate::RegisterSpec for C2ISRrs {
    type Ux = u32;
}
///`read()` method returns [`c2isr::R`](R) reader structure
impl crate::Readable for C2ISRrs {}
///`reset()` method sets C2ISR to value 0
impl crate::Resettable for C2ISRrs {
    const RESET_VALUE: u32 = 0;
}
