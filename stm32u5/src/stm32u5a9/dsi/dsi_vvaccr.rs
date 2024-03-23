#[doc = "Register `DSI_VVACCR` reader"]
pub type R = crate::R<DSI_VVACCRrs>;
#[doc = "Field `VA` reader - Vertical active duration This field returns the current vertical active period measured in number of horizontal lines."]
pub type VA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Vertical active duration This field returns the current vertical active period measured in number of horizontal lines."]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Host video VA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vvaccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VVACCRrs;
impl crate::RegisterSpec for DSI_VVACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvaccr::R`](R) reader structure"]
impl crate::Readable for DSI_VVACCRrs {}
#[doc = "`reset()` method sets DSI_VVACCR to value 0"]
impl crate::Resettable for DSI_VVACCRrs {
    const RESET_VALUE: u32 = 0;
}
