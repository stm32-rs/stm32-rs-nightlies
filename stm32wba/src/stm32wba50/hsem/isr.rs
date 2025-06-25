///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `ISF` reader - Interrupt semaphore x status bit before enable (mask) This bit is set by hardware, and reset only by software. This bit is cleared by software writing the corresponding HSEM_ICR bit.
pub type ISF_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - Interrupt semaphore x status bit before enable (mask) This bit is set by hardware, and reset only by software. This bit is cleared by software writing the corresponding HSEM_ICR bit.
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR").field("isf", &self.isf()).finish()
    }
}
/**HSEM non-secure interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA50.html#HSEM:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
