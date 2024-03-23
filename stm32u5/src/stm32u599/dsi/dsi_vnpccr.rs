#[doc = "Register `DSI_VNPCCR` reader"]
pub type R = crate::R<DSI_VNPCCRrs>;
#[doc = "Field `NPSIZE` reader - Null packet size This field returns the number of bytes inside a null packet."]
pub type NPSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Null packet size This field returns the number of bytes inside a null packet."]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host video null packet current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dsi_vnpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DSI_VNPCCRrs;
impl crate::RegisterSpec for DSI_VNPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vnpccr::R`](R) reader structure"]
impl crate::Readable for DSI_VNPCCRrs {}
#[doc = "`reset()` method sets DSI_VNPCCR to value 0"]
impl crate::Resettable for DSI_VNPCCRrs {
    const RESET_VALUE: u32 = 0;
}
