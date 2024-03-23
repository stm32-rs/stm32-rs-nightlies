#[doc = "Register `VVBPCCR` reader"]
pub type R = crate::R<VVBPCCRrs>;
#[doc = "Field `VBP` reader - Vertical Back-Porch duration"]
pub type VBP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - Vertical Back-Porch duration"]
    #[inline(always)]
    pub fn vbp(&self) -> VBP_R {
        VBP_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host Video VBP Current Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvbpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVBPCCRrs;
impl crate::RegisterSpec for VVBPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvbpccr::R`](R) reader structure"]
impl crate::Readable for VVBPCCRrs {}
#[doc = "`reset()` method sets VVBPCCR to value 0"]
impl crate::Resettable for VVBPCCRrs {
    const RESET_VALUE: u32 = 0;
}
