///Register `MDF_DFLT3DR` reader
pub type R = crate::R<MDF_DFLT3DRrs>;
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
        f.debug_struct("MDF_DFLT3DR")
            .field("dr", &self.dr())
            .finish()
    }
}
/**This register is used to read the data processed by each digital filter.

You can [`read`](crate::Reg::read) this register and get [`mdf_dflt3dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A5.html#MDF1:MDF_DFLT3DR)*/
pub struct MDF_DFLT3DRrs;
impl crate::RegisterSpec for MDF_DFLT3DRrs {
    type Ux = u32;
}
///`read()` method returns [`mdf_dflt3dr::R`](R) reader structure
impl crate::Readable for MDF_DFLT3DRrs {}
///`reset()` method sets MDF_DFLT3DR to value 0
impl crate::Resettable for MDF_DFLT3DRrs {
    const RESET_VALUE: u32 = 0;
}
