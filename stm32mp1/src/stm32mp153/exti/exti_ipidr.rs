#[doc = "Register `EXTI_IPIDR` reader"]
pub type R = crate::R<EXTI_IPIDRrs>;
#[doc = "Field `IPID` reader - IPID"]
pub type IPID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IPID"]
    #[inline(always)]
    pub fn ipid(&self) -> IPID_R {
        IPID_R::new(self.bits)
    }
}
#[doc = "EXTI identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exti_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTI_IPIDRrs;
impl crate::RegisterSpec for EXTI_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exti_ipidr::R`](R) reader structure"]
impl crate::Readable for EXTI_IPIDRrs {}
#[doc = "`reset()` method sets EXTI_IPIDR to value 0x000e_0001"]
impl crate::Resettable for EXTI_IPIDRrs {
    const RESET_VALUE: u32 = 0x000e_0001;
}
