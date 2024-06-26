///Register `GICD_SPISR7` reader
pub type R = crate::R<GICD_SPISR7rs>;
///Field `SPISR7` reader - SPISR7
pub type SPISR7_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SPISR7
    #[inline(always)]
    pub fn spisr7(&self) -> SPISR7_R {
        SPISR7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_SPISR7")
            .field("spisr7", &self.spisr7())
            .finish()
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_spisr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_SPISR7)*/
pub struct GICD_SPISR7rs;
impl crate::RegisterSpec for GICD_SPISR7rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_spisr7::R`](R) reader structure
impl crate::Readable for GICD_SPISR7rs {}
///`reset()` method sets GICD_SPISR7 to value 0
impl crate::Resettable for GICD_SPISR7rs {
    const RESET_VALUE: u32 = 0;
}
