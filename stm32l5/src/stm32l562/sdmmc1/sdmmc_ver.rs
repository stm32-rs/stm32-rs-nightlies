#[doc = "Register `SDMMC_VER` reader"]
pub type R = crate::R<SDMMC_VERrs>;
#[doc = "Field `MINREV` reader - IP minor revision number."]
pub type MINREV_R = crate::FieldReader;
#[doc = "Field `MAJREV` reader - IP major revision number."]
pub type MAJREV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - IP minor revision number."]
    #[inline(always)]
    pub fn minrev(&self) -> MINREV_R {
        MINREV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - IP major revision number."]
    #[inline(always)]
    pub fn majrev(&self) -> MAJREV_R {
        MAJREV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "SDMMC IP version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_ver::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_VERrs;
impl crate::RegisterSpec for SDMMC_VERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_ver::R`](R) reader structure"]
impl crate::Readable for SDMMC_VERrs {}
#[doc = "`reset()` method sets SDMMC_VER to value 0x10"]
impl crate::Resettable for SDMMC_VERrs {
    const RESET_VALUE: u32 = 0x10;
}
