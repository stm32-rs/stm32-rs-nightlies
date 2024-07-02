///Register `ADF_DFLT0DR` reader
pub type R = crate::R<ADF_DFLT0DRrs>;
///Field `DR` reader - DR
pub type DR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 8:31 - DR
    #[inline(always)]
    pub fn dr(&self) -> DR_R {
        DR_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADF_DFLT0DR")
            .field("dr", &self.dr())
            .finish()
    }
}
/**ADF digital filter data register 0

You can [`read`](crate::Reg::read) this register and get [`adf_dflt0dr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#ADF1:ADF_DFLT0DR)*/
pub struct ADF_DFLT0DRrs;
impl crate::RegisterSpec for ADF_DFLT0DRrs {
    type Ux = u32;
}
///`read()` method returns [`adf_dflt0dr::R`](R) reader structure
impl crate::Readable for ADF_DFLT0DRrs {}
///`reset()` method sets ADF_DFLT0DR to value 0
impl crate::Resettable for ADF_DFLT0DRrs {
    const RESET_VALUE: u32 = 0;
}
