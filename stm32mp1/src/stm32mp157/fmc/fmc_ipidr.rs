#[doc = "Register `FMC_IPIDR` reader"]
pub type R = crate::R<FMC_IPIDRrs>;
#[doc = "Field `ID` reader - ID"]
pub type ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - ID"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(self.bits)
    }
}
#[doc = "FMC Identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_IPIDRrs;
impl crate::RegisterSpec for FMC_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_ipidr::R`](R) reader structure"]
impl crate::Readable for FMC_IPIDRrs {}
#[doc = "`reset()` method sets FMC_IPIDR to value 0x0014_0001"]
impl crate::Resettable for FMC_IPIDRrs {
    const RESET_VALUE: u32 = 0x0014_0001;
}
