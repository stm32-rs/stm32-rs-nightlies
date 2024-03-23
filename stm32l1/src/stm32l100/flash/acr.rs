#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::BitReader;
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader;
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACC64` reader - 64-bit access"]
pub type ACC64_R = crate::BitReader;
#[doc = "Field `ACC64` writer - 64-bit access"]
pub type ACC64_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_PD` reader - Flash mode during Sleep"]
pub type SLEEP_PD_R = crate::BitReader;
#[doc = "Field `SLEEP_PD` writer - Flash mode during Sleep"]
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN_PD` reader - Flash mode during Run"]
pub type RUN_PD_R = crate::BitReader;
#[doc = "Field `RUN_PD` writer - Flash mode during Run"]
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 64-bit access"]
    #[inline(always)]
    pub fn acc64(&self) -> ACC64_R {
        ACC64_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash mode during Sleep"]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash mode during Run"]
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<ACRrs> {
        PRFTEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - 64-bit access"]
    #[inline(always)]
    #[must_use]
    pub fn acc64(&mut self) -> ACC64_W<ACRrs> {
        ACC64_W::new(self, 2)
    }
    #[doc = "Bit 3 - Flash mode during Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<ACRrs> {
        SLEEP_PD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Flash mode during Run"]
    #[inline(always)]
    #[must_use]
    pub fn run_pd(&mut self) -> RUN_PD_W<ACRrs> {
        RUN_PD_W::new(self, 4)
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACRrs {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0;
}
