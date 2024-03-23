#[doc = "Register `SYSCFG_ITLINE2` reader"]
pub type R = crate::R<SYSCFG_ITLINE2rs>;
#[doc = "Field `RTC` reader - RTC interrupt request pending (EXTI line 19)"]
pub type RTC_R = crate::BitReader;
impl R {
    #[doc = "Bit 1 - RTC interrupt request pending (EXTI line 19)"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "SYSCFG interrupt line 2 status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg_itline2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYSCFG_ITLINE2rs;
impl crate::RegisterSpec for SYSCFG_ITLINE2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg_itline2::R`](R) reader structure"]
impl crate::Readable for SYSCFG_ITLINE2rs {}
#[doc = "`reset()` method sets SYSCFG_ITLINE2 to value 0"]
impl crate::Resettable for SYSCFG_ITLINE2rs {
    const RESET_VALUE: u32 = 0;
}
