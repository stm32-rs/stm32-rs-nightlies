///Register `C1ISR` reader
pub type R = crate::R<C1ISRrs>;
///Field `ISF` reader - ISF
pub type ISF_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - ISF
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1ISR").field("isf", &self.isf()).finish()
    }
}
/**HSEM i1terrupt status register

You can [`read`](crate::Reg::read) this register and get [`c1isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#HSEM:C1ISR)*/
pub struct C1ISRrs;
impl crate::RegisterSpec for C1ISRrs {
    type Ux = u32;
}
///`read()` method returns [`c1isr::R`](R) reader structure
impl crate::Readable for C1ISRrs {}
///`reset()` method sets C1ISR to value 0
impl crate::Resettable for C1ISRrs {}
