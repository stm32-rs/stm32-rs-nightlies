///Register `GICD_SPISR3` reader
pub type R = crate::R<GICD_SPISR3rs>;
///Field `SPISR3` reader - SPISR3
pub type SPISR3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SPISR3
    #[inline(always)]
    pub fn spisr3(&self) -> SPISR3_R {
        SPISR3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_SPISR3")
            .field("spisr3", &self.spisr3())
            .finish()
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_spisr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_SPISR3)*/
pub struct GICD_SPISR3rs;
impl crate::RegisterSpec for GICD_SPISR3rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_spisr3::R`](R) reader structure
impl crate::Readable for GICD_SPISR3rs {}
///`reset()` method sets GICD_SPISR3 to value 0
impl crate::Resettable for GICD_SPISR3rs {
    const RESET_VALUE: u32 = 0;
}
