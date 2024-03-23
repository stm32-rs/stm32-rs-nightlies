#[doc = "Register `ITLINE2` reader"]
pub type R = crate::R<ITLINE2rs>;
#[doc = "Field `TAMP` reader - TAMP"]
pub type TAMP_R = crate::BitReader;
#[doc = "Field `RTC` reader - RTC"]
pub type RTC_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - TAMP"]
    #[inline(always)]
    pub fn tamp(&self) -> TAMP_R {
        TAMP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "interrupt line 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itline2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ITLINE2rs;
impl crate::RegisterSpec for ITLINE2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itline2::R`](R) reader structure"]
impl crate::Readable for ITLINE2rs {}
#[doc = "`reset()` method sets ITLINE2 to value 0"]
impl crate::Resettable for ITLINE2rs {
    const RESET_VALUE: u32 = 0;
}
