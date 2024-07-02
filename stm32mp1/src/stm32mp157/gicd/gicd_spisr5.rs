///Register `GICD_SPISR5` reader
pub type R = crate::R<GICD_SPISR5rs>;
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
        f.debug_struct("GICD_SPISR5")
            .field("spisr5", &self.spisr5())
            .finish()
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_spisr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_SPISR5)*/
pub struct GICD_SPISR5rs;
impl crate::RegisterSpec for GICD_SPISR5rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_spisr5::R`](R) reader structure
impl crate::Readable for GICD_SPISR5rs {}
///`reset()` method sets GICD_SPISR5 to value 0
impl crate::Resettable for GICD_SPISR5rs {
    const RESET_VALUE: u32 = 0;
}
