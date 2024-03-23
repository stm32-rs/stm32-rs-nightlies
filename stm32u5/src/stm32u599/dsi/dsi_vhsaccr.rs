#[doc = "Register `DSI_VHSACCR` reader"]
pub type R = crate::R<DSI_VHSACCRrs>;
#[doc = "Field `HSA` reader - Horizontal synchronism active duration This fields returns the horizontal synchronism active period in lane byte clock cycles."]
pub type HSA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Horizontal synchronism active duration This fields returns the horizontal synchronism active period in lane byte clock cycles."]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vhsaccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VHSACCRrs;
impl crate::RegisterSpec for DSI_VHSACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhsaccr::R`](R) reader structure"]
impl crate::Readable for DSI_VHSACCRrs {}
#[doc = "`reset()` method sets DSI_VHSACCR to value 0"]
impl crate::Resettable for DSI_VHSACCRrs {
    const RESET_VALUE: u32 = 0;
}
