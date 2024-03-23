#[doc = "Register `IPID` reader"]
pub type R = crate::R<IPIDrs>;
#[doc = "Field `IPID` reader - IPID"]
pub type IPID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IPID"]
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
#[doc = "UCPD IP ID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipid::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPIDrs;
impl crate::RegisterSpec for IPIDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipid::R`](R) reader structure"]
impl crate::Readable for IPIDrs {}
#[doc = "`reset()` method sets IPID to value 0x0015_0021"]
impl crate::Resettable for IPIDrs {
    const RESET_VALUE: u32 = 0x0015_0021;
}
