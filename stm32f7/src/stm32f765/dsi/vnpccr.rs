#[doc = "Register `VNPCCR` reader"]
pub type R = crate::R<VNPCCRrs>;
#[doc = "Field `NPSIZE` reader - Null Packet Size"]
pub type NPSIZE_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:12 - Null Packet Size"]
    #[inline(always)]
    pub fn npsize(&self) -> NPSIZE_R {
        NPSIZE_R::new((self.bits & 0x1fff) as u16)
    }
}
#[doc = "DSI Host Video Null Packet Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vnpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VNPCCRrs;
impl crate::RegisterSpec for VNPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vnpccr::R`](R) reader structure"]
impl crate::Readable for VNPCCRrs {}
#[doc = "`reset()` method sets VNPCCR to value 0"]
impl crate::Resettable for VNPCCRrs {
    const RESET_VALUE: u32 = 0;
}
