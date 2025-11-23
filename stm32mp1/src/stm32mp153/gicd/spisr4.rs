///Register `SPISR4` reader
pub type R = crate::R<SPISR4rs>;
///Field `SPISR4` reader - SPISR4
pub type SPISR4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SPISR4
    #[inline(always)]
    pub fn spisr4(&self) -> SPISR4_R {
        SPISR4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPISR4")
            .field("spisr4", &self.spisr4())
            .finish()
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:SPISR4)*/
pub struct SPISR4rs;
impl crate::RegisterSpec for SPISR4rs {
    type Ux = u32;
}
///`read()` method returns [`spisr4::R`](R) reader structure
impl crate::Readable for SPISR4rs {}
///`reset()` method sets SPISR4 to value 0
impl crate::Resettable for SPISR4rs {}
