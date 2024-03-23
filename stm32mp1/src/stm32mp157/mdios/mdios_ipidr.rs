#[doc = "Register `MDIOS_IPIDR` reader"]
pub type R = crate::R<MDIOS_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "MDIOS identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdios_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDIOS_IPIDRrs;
impl crate::RegisterSpec for MDIOS_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_ipidr::R`](R) reader structure"]
impl crate::Readable for MDIOS_IPIDRrs {}
#[doc = "`reset()` method sets MDIOS_IPIDR to value 0x0018_0001"]
impl crate::Resettable for MDIOS_IPIDRrs {
    const RESET_VALUE: u32 = 0x0018_0001;
}
