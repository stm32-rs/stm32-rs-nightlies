#[doc = "Register `DSI_VHBPCCR` reader"]
pub type R = crate::R<DSI_VHBPCCRrs>;
#[doc = "Field `HBP` reader - Horizontal back-porch duration This field returns the horizontal back-porch period in lane byte clock cycles."]
pub type HBP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal back-porch duration This field returns the horizontal back-porch period in lane byte clock cycles."]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhbpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VHBPCCRrs;
impl crate::RegisterSpec for DSI_VHBPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhbpccr::R`](R) reader structure"]
impl crate::Readable for DSI_VHBPCCRrs {}
#[doc = "`reset()` method sets DSI_VHBPCCR to value 0"]
impl crate::Resettable for DSI_VHBPCCRrs {
    const RESET_VALUE: u32 = 0;
}
