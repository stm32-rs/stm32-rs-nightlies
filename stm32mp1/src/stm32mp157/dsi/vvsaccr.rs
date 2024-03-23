#[doc = "Register `VVSACCR` reader"]
pub type R = crate::R<VVSACCRrs>;
#[doc = "Field `VSA` reader - VSA"]
pub type VSA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    pub fn vsa(&self) -> VSA_R {
        VSA_R::new((self.bits & 0x03ff) as u16)
    }
}
#[doc = "DSI Host video VSA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvsaccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVSACCRrs;
impl crate::RegisterSpec for VVSACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvsaccr::R`](R) reader structure"]
impl crate::Readable for VVSACCRrs {}
#[doc = "`reset()` method sets VVSACCR to value 0"]
impl crate::Resettable for VVSACCRrs {
    const RESET_VALUE: u32 = 0;
}
