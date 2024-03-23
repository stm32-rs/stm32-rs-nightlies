#[doc = "Register `VVACCR` reader"]
pub type R = crate::R<VVACCRrs>;
#[doc = "Field `VA` reader - VA"]
pub type VA_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - VA"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
#[doc = "DSI Host video VA current configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvaccr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVACCRrs;
impl crate::RegisterSpec for VVACCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvaccr::R`](R) reader structure"]
impl crate::Readable for VVACCRrs {}
#[doc = "`reset()` method sets VVACCR to value 0"]
impl crate::Resettable for VVACCRrs {
    const RESET_VALUE: u32 = 0;
}
