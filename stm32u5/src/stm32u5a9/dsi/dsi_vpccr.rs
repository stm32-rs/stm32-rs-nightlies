///Register `DSI_VPCCR` reader
pub type R = crate::R<DSI_VPCCRrs>;
///Field `VPSIZE` reader - Video packet size This field returns the number of pixels in a single video packet.
pub type VPSIZE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:13 - Video packet size This field returns the number of pixels in a single video packet.
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSI_VPCCR")
            .field("vpsize", &self.vpsize())
            .finish()
    }
}
/**DSI Host video packet current configuration register

You can [`read`](crate::Reg::read) this register and get [`dsi_vpccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#DSI:DSI_VPCCR)*/
pub struct DSI_VPCCRrs;
impl crate::RegisterSpec for DSI_VPCCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsi_vpccr::R`](R) reader structure
impl crate::Readable for DSI_VPCCRrs {}
///`reset()` method sets DSI_VPCCR to value 0
impl crate::Resettable for DSI_VPCCRrs {
    const RESET_VALUE: u32 = 0;
}
