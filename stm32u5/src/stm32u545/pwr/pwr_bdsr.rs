#[doc = "Register `PWR_BDSR` reader"]
pub type R = crate::R<PWR_BDSRrs>;
#[doc = "Field `VBATH` reader - Backup domain voltage level monitoring versus high threshold"]
pub type VBATH_R = crate::BitReader;
#[doc = "Field `TEMPL` reader - Temperature level monitoring versus low threshold"]
pub type TEMPL_R = crate::BitReader;
#[doc = "Field `TEMPH` reader - Temperature level monitoring versus high threshold"]
pub type TEMPH_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - Backup domain voltage level monitoring versus high threshold"]
    #[inline(always)]
    pub fn vbath(&self) -> VBATH_R {
        VBATH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Temperature level monitoring versus low threshold"]
    #[inline(always)]
    pub fn templ(&self) -> TEMPL_R {
        TEMPL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Temperature level monitoring versus high threshold"]
    #[inline(always)]
    pub fn temph(&self) -> TEMPH_R {
        TEMPH_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "PWR Backup domain status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_bdsr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_BDSRrs;
impl crate::RegisterSpec for PWR_BDSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_bdsr::R`](R) reader structure"]
impl crate::Readable for PWR_BDSRrs {}
#[doc = "`reset()` method sets PWR_BDSR to value 0"]
impl crate::Resettable for PWR_BDSRrs {
    const RESET_VALUE: u32 = 0;
}
