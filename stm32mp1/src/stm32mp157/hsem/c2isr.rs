///Register `C2ISR` reader
pub type R = crate::R<C2ISRrs>;
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
        f.debug_struct("C2ISR").field("isf", &self.isf()).finish()
    }
}
/**HSEM i2terrupt status register

You can [`read`](crate::Reg::read) this register and get [`c2isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HSEM:C2ISR)*/
pub struct C2ISRrs;
impl crate::RegisterSpec for C2ISRrs {
    type Ux = u32;
}
///`read()` method returns [`c2isr::R`](R) reader structure
impl crate::Readable for C2ISRrs {}
///`reset()` method sets C2ISR to value 0
impl crate::Resettable for C2ISRrs {}
