#[doc = "Register `VHBPCCR` reader"]
pub type R = crate::R<VHBPCCRrs>;
#[doc = "Field `HBP` reader - HBP"]
pub type HBP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    pub fn hbp(&self) -> HBP_R {
        HBP_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HBP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhbpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VHBPCCRrs;
impl crate::RegisterSpec for VHBPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vhbpccr::R`](R) reader structure"]
impl crate::Readable for VHBPCCRrs {}
#[doc = "`reset()` method sets VHBPCCR to value 0"]
impl crate::Resettable for VHBPCCRrs {
    const RESET_VALUE: u32 = 0;
}
