///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::BitReader;
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PRFTEN` reader - Prefetch enable
pub type PRFTEN_R = crate::BitReader;
///Field `PRFTEN` writer - Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACC64` reader - 64-bit access
pub type ACC64_R = crate::BitReader;
///Field `ACC64` writer - 64-bit access
pub type ACC64_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLEEP_PD` reader - Flash mode during Sleep
pub type SLEEP_PD_R = crate::BitReader;
///Field `SLEEP_PD` writer - Flash mode during Sleep
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RUN_PD` reader - Flash mode during Run
pub type RUN_PD_R = crate::BitReader;
///Field `RUN_PD` writer - Flash mode during Run
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - 64-bit access
    #[inline(always)]
    pub fn acc64(&self) -> ACC64_R {
        ACC64_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Flash mode during Sleep
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Flash mode during Run
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("prften", &self.prften())
            .field("acc64", &self.acc64())
            .field("sleep_pd", &self.sleep_pd())
            .field("run_pd", &self.run_pd())
            .finish()
    }
}
impl W {
    ///Bit 0 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 1 - Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, ACRrs> {
        PRFTEN_W::new(self, 1)
    }
    ///Bit 2 - 64-bit access
    #[inline(always)]
    pub fn acc64(&mut self) -> ACC64_W<'_, ACRrs> {
        ACC64_W::new(self, 2)
    }
    ///Bit 3 - Flash mode during Sleep
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<'_, ACRrs> {
        SLEEP_PD_W::new(self, 3)
    }
    ///Bit 4 - Flash mode during Run
    #[inline(always)]
    pub fn run_pd(&mut self) -> RUN_PD_W<'_, ACRrs> {
        RUN_PD_W::new(self, 4)
    }
}
/**Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L100.html#Flash:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0
impl crate::Resettable for ACRrs {}
