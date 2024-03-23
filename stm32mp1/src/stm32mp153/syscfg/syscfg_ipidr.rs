#[doc = "Register `SYSCFG_IPIDR` reader"]
pub type R = crate::R<SYSCFG_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "SYSCFG identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_IPIDRrs;
impl crate::RegisterSpec for SYSCFG_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_ipidr::R`](R) reader structure"]
impl crate::Readable for SYSCFG_IPIDRrs {}
#[doc = "`reset()` method sets SYSCFG_IPIDR to value 0x0003_0001"]
impl crate::Resettable for SYSCFG_IPIDRrs {
    const RESET_VALUE: u32 = 0x0003_0001;
}
