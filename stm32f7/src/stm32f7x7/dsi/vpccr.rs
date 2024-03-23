#[doc = "Register `VPCCR` reader"]
pub type R = crate::R<VPCCRrs>;
#[doc = "Field `VPSIZE` reader - Video Packet Size"]
pub type VPSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - Video Packet Size"]
    #[inline(always)]
    pub fn vpsize(&self) -> VPSIZE_R {
        VPSIZE_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Host Video Packet Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VPCCRrs;
impl crate::RegisterSpec for VPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vpccr::R`](R) reader structure"]
impl crate::Readable for VPCCRrs {}
#[doc = "`reset()` method sets VPCCR to value 0"]
impl crate::Resettable for VPCCRrs {
    const RESET_VALUE: u32 = 0;
}
