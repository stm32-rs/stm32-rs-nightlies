///Register `STGENC_PIDR5` reader
pub type R = crate::R<STGENC_PIDR5rs>;
///Field `PIDR5` reader - PIDR5
pub type PIDR5_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR5
    #[inline(always)]
    pub fn pidr5(&self) -> PIDR5_R {
        PIDR5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENC_PIDR5")
            .field("pidr5", &self.pidr5())
            .finish()
    }
}
/**STGENC peripheral ID5 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:STGENC_PIDR5)*/
pub struct STGENC_PIDR5rs;
impl crate::RegisterSpec for STGENC_PIDR5rs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_pidr5::R`](R) reader structure
impl crate::Readable for STGENC_PIDR5rs {}
///`reset()` method sets STGENC_PIDR5 to value 0
impl crate::Resettable for STGENC_PIDR5rs {
    const RESET_VALUE: u32 = 0;
}
