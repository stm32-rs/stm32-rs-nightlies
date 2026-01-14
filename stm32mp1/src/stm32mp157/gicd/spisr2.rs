///Register `SPISR2` reader
pub type R = crate::R<SPISR2rs>;
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
        f.debug_struct("SPISR2")
            .field("spisr2", &self.spisr2())
            .finish()
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`spisr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:SPISR2)*/
pub struct SPISR2rs;
impl crate::RegisterSpec for SPISR2rs {
    type Ux = u32;
}
///`read()` method returns [`spisr2::R`](R) reader structure
impl crate::Readable for SPISR2rs {}
///`reset()` method sets SPISR2 to value 0
impl crate::Resettable for SPISR2rs {}
