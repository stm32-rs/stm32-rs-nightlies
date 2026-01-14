///Register `SPISR5` reader
pub type R = crate::R<SPISR5rs>;
///Field `SPISR5` reader - SPISR5
pub type SPISR5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SPISR5
    #[inline(always)]
    pub fn spisr5(&self) -> SPISR5_R {
        SPISR5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPISR5")
            .field("spisr5", &self.spisr5())
            .finish()
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:SPISR5)*/
pub struct SPISR5rs;
impl crate::RegisterSpec for SPISR5rs {
    type Ux = u32;
}
///`read()` method returns [`spisr5::R`](R) reader structure
impl crate::Readable for SPISR5rs {}
///`reset()` method sets SPISR5 to value 0
impl crate::Resettable for SPISR5rs {}
