#[doc = "Register `HSEM_IPIDR` reader"]
pub type R = crate::R<HSEM_IPIDRrs>;
#[doc = "Field `IPID` reader - IPID"]
pub type IPID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IPID"]
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
#[doc = "HSEM IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hsem_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSEM_IPIDRrs;
impl crate::RegisterSpec for HSEM_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsem_ipidr::R`](R) reader structure"]
impl crate::Readable for HSEM_IPIDRrs {}
#[doc = "`reset()` method sets HSEM_IPIDR to value 0x0010_0072"]
impl crate::Resettable for HSEM_IPIDRrs {
    const RESET_VALUE: u32 = 0x0010_0072;
}
