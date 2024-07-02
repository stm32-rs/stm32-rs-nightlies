///Register `DSI_VVBPCCR` reader
pub type R = crate::R<DSI_VVBPCCRrs>;
///Field `VBP` reader - Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines.
pub type VBP_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:9 - Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines.
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VVBPCCR")
            .field("vbp", &self.vbp())
            .finish()
    }
}
/**DSI Host video VBP current configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vvbpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VVBPCCR)*/
pub struct DSI_VVBPCCRrs;
impl crate::RegisterSpec for DSI_VVBPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vvbpccr::R`](R) reader structure
impl crate::Readable for DSI_VVBPCCRrs {}
///`reset()` method sets DSI_VVBPCCR to value 0
impl crate::Resettable for DSI_VVBPCCRrs {
    const RESET_VALUE: u32 = 0;
}
