#[doc = "Register `RWD` reader"]
pub type R = crate::R<RWDrs>;
#[doc = "Register `RWD` writer"]
pub type W = crate::W<RWDrs>;
#[doc = "Field `WDC` reader - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of FDCAN_CCCR register are set to 1."]
pub type WDC_R = crate::FieldReader;
#[doc = "Field `WDC` writer - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of FDCAN_CCCR register are set to 1."]
pub type WDC_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDV` reader - Watchdog value Actual message RAM watchdog counter value."]
pub type WDV_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of FDCAN_CCCR register are set to 1."]
    #[inline(always)]
    pub fn wdc(&self) -> WDC_R {
        WDC_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Watchdog value Actual message RAM watchdog counter value."]
    #[inline(always)]
    pub fn wdv(&self) -> WDV_R {
        WDV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Watchdog configuration Start value of the message RAM watchdog counter. With the reset value of 00, the counter is disabled. These are protected write (P) bits, write access is possible only when the bit 1 \\[CCE\\]
and bit 0 \\[INIT\\]
of FDCAN_CCCR register are set to 1."]
    #[inline(always)]
    #[must_use]
    pub fn wdc(&mut self) -> WDC_W<RWDrs> {
        WDC_W::new(self, 0)
    }
}
#[doc = "FDCAN RAM watchdog register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rwd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rwd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RWDrs;
impl crate::RegisterSpec for RWDrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rwd::R`](R) reader structure"]
impl crate::Readable for RWDrs {}
#[doc = "`write(|w| ..)` method takes [`rwd::W`](W) writer structure"]
impl crate::Writable for RWDrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RWD to value 0"]
impl crate::Resettable for RWDrs {
    const RESET_VALUE: u32 = 0;
}
