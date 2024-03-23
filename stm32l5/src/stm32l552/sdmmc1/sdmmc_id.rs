#[doc = "Register `SDMMC_ID` reader"]
pub type R = crate::R<SDMMC_IDrs>;
#[doc = "Field `IP_ID` reader - SDMMC IP identification."]
pub type IP_ID_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - SDMMC IP identification."]
    #[inline(always)]
    pub fn ip_id(&self) -> IP_ID_R {
        IP_ID_R::new(self.bits)
    }
}
#[doc = "SDMMC IP identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_IDrs;
impl crate::RegisterSpec for SDMMC_IDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_id::R`](R) reader structure"]
impl crate::Readable for SDMMC_IDrs {}
#[doc = "`reset()` method sets SDMMC_ID to value 0x0014_0022"]
impl crate::Resettable for SDMMC_IDrs {
    const RESET_VALUE: u32 = 0x0014_0022;
}
