///Register `DSI_VHBPCCR` reader
pub type R = crate::R<DSI_VHBPCCRrs>;
///Field `HBP` reader - Horizontal back-porch duration This field returns the horizontal back-porch period in lane byte clock cycles.
pub type HBP_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:11 - Horizontal back-porch duration This field returns the horizontal back-porch period in lane byte clock cycles.
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VHBPCCR")
            .field("hbp", &self.hbp())
            .finish()
    }
}
/**DSI Host video HBP current configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vhbpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#DSI:DSI_VHBPCCR)*/
pub struct DSI_VHBPCCRrs;
impl crate::RegisterSpec for DSI_VHBPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vhbpccr::R`](R) reader structure
impl crate::Readable for DSI_VHBPCCRrs {}
///`reset()` method sets DSI_VHBPCCR to value 0
impl crate::Resettable for DSI_VHBPCCRrs {
    const RESET_VALUE: u32 = 0;
}
