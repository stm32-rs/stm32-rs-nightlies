///Register `SPISR6` reader
pub type R = crate::R<SPISR6rs>;
///Field `SPISR6` reader - SPISR6
pub type SPISR6_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - SPISR6
    #[inline(always)]
    pub fn spisr6(&self) -> SPISR6_R {
        SPISR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPISR6")
            .field("spisr6", &self.spisr6())
            .finish()
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:SPISR6)*/
pub struct SPISR6rs;
impl crate::RegisterSpec for SPISR6rs {
    type Ux = u32;
}
///`read()` method returns [`spisr6::R`](R) reader structure
impl crate::Readable for SPISR6rs {}
///`reset()` method sets SPISR6 to value 0
impl crate::Resettable for SPISR6rs {}
