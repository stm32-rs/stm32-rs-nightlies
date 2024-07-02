///Register `DSI_VVFPCCR` reader
pub type R = crate::R<DSI_VVFPCCRrs>;
///Field `VFP` reader - Vertical front-porch duration This field returns the current vertical front-porch period measured in number of horizontal lines.
pub type VFP_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - Vertical front-porch duration This field returns the current vertical front-porch period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VVFPCCR")
            .field("vfp", &self.vfp())
            .finish()
    }
}
/**DSI Host video VFP current configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vvfpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_VVFPCCR)*/
pub struct DSI_VVFPCCRrs;
impl crate::RegisterSpec for DSI_VVFPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vvfpccr::R`](R) reader structure
impl crate::Readable for DSI_VVFPCCRrs {}
///`reset()` method sets DSI_VVFPCCR to value 0
impl crate::Resettable for DSI_VVFPCCRrs {
    const RESET_VALUE: u32 = 0;
}
