///Register `STGENC_PIDR7` reader
pub type R = crate::R<STGENC_PIDR7rs>;
///Field `PIDR7` reader - PIDR7
pub type PIDR7_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - PIDR7
    #[inline(always)]
    pub fn pidr7(&self) -> PIDR7_R {
        PIDR7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENC_PIDR7")
            .field("pidr7", &self.pidr7())
            .finish()
    }
}
/**STGENC peripheral ID7 register

You can [`read`](crate::Reg::read) this register and get [`stgenc_pidr7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:STGENC_PIDR7)*/
pub struct STGENC_PIDR7rs;
impl crate::RegisterSpec for STGENC_PIDR7rs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_pidr7::R`](R) reader structure
impl crate::Readable for STGENC_PIDR7rs {}
///`reset()` method sets STGENC_PIDR7 to value 0
impl crate::Resettable for STGENC_PIDR7rs {
    const RESET_VALUE: u32 = 0;
}
