#[doc = "Register `VVFPCCR` reader"]
pub type R = crate::R<VVFPCCRrs>;
#[doc = "Field `VFP` reader - VFP"]
pub type VFP_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - VFP"]
    #[inline(always)]
    pub fn vfp(&self) -> VFP_R {
        VFP_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host video VFP current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvfpccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVFPCCRrs;
impl crate::RegisterSpec for VVFPCCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvfpccr::R`](R) reader structure"]
impl crate::Readable for VVFPCCRrs {}
#[doc = "`reset()` method sets VVFPCCR to value 0"]
impl crate::Resettable for VVFPCCRrs {
    const RESET_VALUE: u32 = 0;
}
