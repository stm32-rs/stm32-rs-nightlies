///Register `SWREG63` reader
pub type R = crate::R<SWREG63rs>;
///Field `SWREG_FIELD` reader - Synthesis configuration register encoder 0 (read only) (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Synthesis configuration register encoder 0 (read only) (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG63")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
/**VENC synthesis configuration register encoder 0 register

You can [`read`](crate::Reg::read) this register and get [`swreg63::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#VENC:SWREG63)*/
pub struct SWREG63rs;
impl crate::RegisterSpec for SWREG63rs {
    type Ux = u32;
}
///`read()` method returns [`swreg63::R`](R) reader structure
impl crate::Readable for SWREG63rs {}
///`reset()` method sets SWREG63 to value 0x1e62_2780
impl crate::Resettable for SWREG63rs {
    const RESET_VALUE: u32 = 0x1e62_2780;
}
