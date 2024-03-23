#[doc = "Register `FMC_CSQEMSR` reader"]
pub type R = crate::R<FMC_CSQEMSRrs>;
#[doc = "Field `SEM` reader - SEM"]
pub type SEM_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - SEM"]
    #[inline(always)]
    pub fn sem(&self) -> SEM_R {
        SEM_R::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "This register holds a sector error mapping status when the whole transfer is complete.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_csqemsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_CSQEMSRrs;
impl crate::RegisterSpec for FMC_CSQEMSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_csqemsr::R`](R) reader structure"]
impl crate::Readable for FMC_CSQEMSRrs {}
#[doc = "`reset()` method sets FMC_CSQEMSR to value 0"]
impl crate::Resettable for FMC_CSQEMSRrs {
    const RESET_VALUE: u32 = 0;
}
