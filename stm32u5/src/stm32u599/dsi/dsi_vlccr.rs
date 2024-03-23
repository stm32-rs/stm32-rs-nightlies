#[doc = "Register `DSI_VLCCR` reader"]
pub type R = crate::R<DSI_VLCCRrs>;
#[doc = "Field `HLINE` reader - Horizontal line duration This field returns the current total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
pub type HLINE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Horizontal line duration This field returns the current total of the horizontal line period (HSA+HBP+HACT+HFP) counted in lane byte clock cycles."]
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "DSI Host video line current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vlccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VLCCRrs;
impl crate::RegisterSpec for DSI_VLCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vlccr::R`](R) reader structure"]
impl crate::Readable for DSI_VLCCRrs {}
#[doc = "`reset()` method sets DSI_VLCCR to value 0"]
impl crate::Resettable for DSI_VLCCRrs {
    const RESET_VALUE: u32 = 0;
}
