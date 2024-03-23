#[doc = "Register `WRFR` reader"]
pub type R = crate::R<WRFRrs>;
#[doc = "Field `WRF` reader - Write flags for MDIO registers 0 to 31"]
pub type WRF_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Write flags for MDIO registers 0 to 31"]
    #[inline(always)]
    pub fn wrf(&self) -> WRF_R {
        WRF_R::new(self.bits)
    }
}
#[doc = "MDIOS write flag register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wrfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WRFRrs;
impl crate::RegisterSpec for WRFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrfr::R`](R) reader structure"]
impl crate::Readable for WRFRrs {}
#[doc = "`reset()` method sets WRFR to value 0"]
impl crate::Resettable for WRFRrs {
    const RESET_VALUE: u32 = 0;
}
