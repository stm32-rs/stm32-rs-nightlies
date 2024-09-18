///Register `MDF_DFLT2DR` reader
pub type R = crate::R<MDF_DFLT2DRrs>;
///Field `DR` reader - Data processed by digital filter.
pub type DR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 8:31 - Data processed by digital filter.
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDF_DFLT2DR")
            .field("dr", &self.dr())
            .finish()
    }
}
/**This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt2dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#MDF1:MDF_DFLT2DR)*/
pub struct MDF_DFLT2DRrs;
impl crate::RegisterSpec for MDF_DFLT2DRrs {
    type Ux = u32;
}
///`read()` method returns [`mdf_dflt2dr::R`](R) reader structure
impl crate::Readable for MDF_DFLT2DRrs {}
///`reset()` method sets MDF_DFLT2DR to value 0
impl crate::Resettable for MDF_DFLT2DRrs {
    const RESET_VALUE: u32 = 0;
}
