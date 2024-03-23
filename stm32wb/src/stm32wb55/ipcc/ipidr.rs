#[doc = "Register `IPIDR` reader"]
pub type R = crate::R<IPIDRrs>;
#[doc = "Field `IPID` reader - Identification Code"]
pub type IPID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Identification Code"]
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
#[doc = "IPCC indentification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IPIDRrs;
impl crate::RegisterSpec for IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ipidr::R`](R) reader structure"]
impl crate::Readable for IPIDRrs {}
#[doc = "`reset()` method sets IPIDR to value 0x0010_0071"]
impl crate::Resettable for IPIDRrs {
    const RESET_VALUE: u32 = 0x0010_0071;
}
