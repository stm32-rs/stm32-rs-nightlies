#[doc = "Register `SDMMC_IPIDR` reader"]
pub type R = crate::R<SDMMC_IPIDRrs>;
#[doc = "Field `IP_ID` reader - IP_ID"]
pub type IP_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - IP_ID"]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(self.bits)
    }
}
#[doc = "SDMMC identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_ipidr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_IPIDRrs;
impl crate::RegisterSpec for SDMMC_IPIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_ipidr::R`](R) reader structure"]
impl crate::Readable for SDMMC_IPIDRrs {}
#[doc = "`reset()` method sets SDMMC_IPIDR to value 0x0014_0022"]
impl crate::Resettable for SDMMC_IPIDRrs {
    const RESET_VALUE: u32 = 0x0014_0022;
}
