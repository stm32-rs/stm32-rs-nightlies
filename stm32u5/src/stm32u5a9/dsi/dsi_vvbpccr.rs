#[doc = "Register `DSI_VVBPCCR` reader"]
pub type R = crate::R<DSI_VVBPCCRrs>;
#[doc = "Field `VBP` reader - Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines."]
pub type VBP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical back-porch duration This field returns the current vertical back-porch period measured in number of horizontal lines."]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host video VBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvbpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVBPCCRrs;
impl crate::RegisterSpec for DSI_VVBPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvbpccr::R`](R) reader structure"]
impl crate::Readable for DSI_VVBPCCRrs {}
#[doc = "`reset()` method sets DSI_VVBPCCR to value 0"]
impl crate::Resettable for DSI_VVBPCCRrs {
    const RESET_VALUE: u32 = 0;
}
