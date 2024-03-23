#[doc = "Register `ENDN` reader"]
pub type R = crate::R<ENDNrs>;
#[doc = "Field `ETV` reader - Endiannes Test Value"]
pub type ETV_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Endiannes Test Value"]
    #[inline(always)]
    pub fn etv(&self) -> ETV_R {
        ETV_R::new(self.bits)
    }
}
#[doc = "FDCAN Core Release Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endn::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDNrs;
impl crate::RegisterSpec for ENDNrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endn::R`](R) reader structure"]
impl crate::Readable for ENDNrs {}
#[doc = "`reset()` method sets ENDN to value 0"]
impl crate::Resettable for ENDNrs {
    const RESET_VALUE: u32 = 0;
}
