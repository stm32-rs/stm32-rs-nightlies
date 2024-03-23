#[doc = "Register `FDCAN_RWD` reader"]
pub type R = crate::R<FDCAN_RWDrs>;
#[doc = "Register `FDCAN_RWD` writer"]
pub type W = crate::W<FDCAN_RWDrs>;
#[doc = "Field `WDC` reader - Watchdog configuration"]
pub type WDC_R = crate::FieldReader;
#[doc = "Field `WDC` writer - Watchdog configuration"]
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
impl W {
    #[doc = "Bits 0:7 - Watchdog configuration"]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<FDCAN_RWDrs> {
        WDC_W::new(self, 0)
    }
}
#[doc = "FDCAN RAM Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fdcan_rwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fdcan_rwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FDCAN_RWDrs;
impl crate::RegisterSpec for FDCAN_RWDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rwd::R`](R) reader structure"]
impl crate::Readable for FDCAN_RWDrs {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rwd::W`](W) writer structure"]
impl crate::Writable for FDCAN_RWDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FDCAN_RWD to value 0"]
impl crate::Resettable for FDCAN_RWDrs {
    const RESET_VALUE: u32 = 0;
}
