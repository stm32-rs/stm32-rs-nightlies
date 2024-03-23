#[doc = "Register `IPVER` reader"]
pub type R = crate::R<IPVERrs>;
#[doc = "Field `IPVER` reader - IPVER"]
pub type IPVER_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IPVER"]
    #[inline(always)]
    pub fn ipver(&self) -> IPVER_R {
        IPVER_R::new(self.bits)
    }
}
#[doc = "UCPD IP ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPVERrs;
impl crate::RegisterSpec for IPVERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipver::R`](R) reader structure"]
impl crate::Readable for IPVERrs {}
#[doc = "`reset()` method sets IPVER to value 0x10"]
impl crate::Resettable for IPVERrs {
    const RESET_VALUE: u32 = 0x10;
}
