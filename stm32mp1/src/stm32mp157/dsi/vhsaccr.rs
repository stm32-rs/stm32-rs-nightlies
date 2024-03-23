#[doc = "Register `VHSACCR` reader"]
pub type R = crate::R<VHSACCRrs>;
#[doc = "Field `HSA` reader - HSA"]
pub type HSA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    pub fn hsa(&self) -> HSA_R {
        HSA_R::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "DSI Host video HSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vhsaccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VHSACCRrs;
impl crate::RegisterSpec for VHSACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vhsaccr::R`](R) reader structure"]
impl crate::Readable for VHSACCRrs {}
#[doc = "`reset()` method sets VHSACCR to value 0"]
impl crate::Resettable for VHSACCRrs {
    const RESET_VALUE: u32 = 0;
}
