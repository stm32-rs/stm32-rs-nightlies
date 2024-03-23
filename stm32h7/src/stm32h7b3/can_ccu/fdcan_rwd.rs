#[doc = "Register `FDCAN_RWD` reader"]
pub type R = crate::R<FDCAN_RWDrs>;
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
#[doc = "FDCAN RAM Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rwd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RWDrs;
impl crate::RegisterSpec for FDCAN_RWDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rwd::R`](R) reader structure"]
impl crate::Readable for FDCAN_RWDrs {}
#[doc = "`reset()` method sets FDCAN_RWD to value 0"]
impl crate::Resettable for FDCAN_RWDrs {
    const RESET_VALUE: u32 = 0;
}
