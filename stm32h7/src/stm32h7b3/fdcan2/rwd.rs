#[doc = "Register `RWD` reader"]
pub type R = crate::R<RWDrs>;
#[doc = "Field `WDC` reader - Watchdog configuration"]
pub type WDC_R = crate::FieldReader;
#[doc = "Field `WDV` reader - Watchdog value"]
pub type WDV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog configuration"]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog value"]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "FDCAN RAM Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RWDrs;
impl crate::RegisterSpec for RWDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwd::R`](R) reader structure"]
impl crate::Readable for RWDrs {}
#[doc = "`reset()` method sets RWD to value 0"]
impl crate::Resettable for RWDrs {
    const RESET_VALUE: u32 = 0;
}
