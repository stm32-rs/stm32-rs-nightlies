///Register `GICD_SPISR2` reader
pub type R = crate::R<GICD_SPISR2rs>;
///Field `SPISR2` reader - SPISR2
pub type SPISR2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SPISR2
    #[inline(always)]
    pub fn spisr2(&self) -> SPISR2_R {
        SPISR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_SPISR2")
            .field("spisr2", &self.spisr2())
            .finish()
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_spisr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_SPISR2)*/
pub struct GICD_SPISR2rs;
impl crate::RegisterSpec for GICD_SPISR2rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_spisr2::R`](R) reader structure
impl crate::Readable for GICD_SPISR2rs {}
///`reset()` method sets GICD_SPISR2 to value 0
impl crate::Resettable for GICD_SPISR2rs {
    const RESET_VALUE: u32 = 0;
}
