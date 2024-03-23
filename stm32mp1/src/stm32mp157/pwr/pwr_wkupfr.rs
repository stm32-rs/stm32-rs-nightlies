#[doc = "Register `PWR_WKUPFR` reader"]
pub type R = crate::R<PWR_WKUPFRrs>;
#[doc = "Field `WKUPF1` reader - WKUPF1"]
pub type WKUPF1_R = crate::BitReader;
#[doc = "Field `WKUPF2` reader - WKUPF2"]
pub type WKUPF2_R = crate::BitReader;
#[doc = "Field `WKUPF3` reader - WKUPF3"]
pub type WKUPF3_R = crate::BitReader;
#[doc = "Field `WKUPF4` reader - WKUPF4"]
pub type WKUPF4_R = crate::BitReader;
#[doc = "Field `WKUPF5` reader - WKUPF5"]
pub type WKUPF5_R = crate::BitReader;
#[doc = "Field `WKUPF6` reader - WKUPF6"]
pub type WKUPF6_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - WKUPF1"]
    #[inline(always)]
    pub fn wkupf1(&self) -> WKUPF1_R {
        WKUPF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WKUPF2"]
    #[inline(always)]
    pub fn wkupf2(&self) -> WKUPF2_R {
        WKUPF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WKUPF3"]
    #[inline(always)]
    pub fn wkupf3(&self) -> WKUPF3_R {
        WKUPF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WKUPF4"]
    #[inline(always)]
    pub fn wkupf4(&self) -> WKUPF4_R {
        WKUPF4_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WKUPF5"]
    #[inline(always)]
    pub fn wkupf5(&self) -> WKUPF5_R {
        WKUPF5_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WKUPF6"]
    #[inline(always)]
    pub fn wkupf6(&self) -> WKUPF6_R {
        WKUPF6_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "Not reset by wakeup from Standby mode but by any Application reset (NRST, IWDG, ...)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwr_wkupfr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PWR_WKUPFRrs;
impl crate::RegisterSpec for PWR_WKUPFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwr_wkupfr::R`](R) reader structure"]
impl crate::Readable for PWR_WKUPFRrs {}
#[doc = "`reset()` method sets PWR_WKUPFR to value 0"]
impl crate::Resettable for PWR_WKUPFRrs {
    const RESET_VALUE: u32 = 0;
}
