///Register `CIER` reader
pub type R = crate::R<CIERrs>;
///Register `CIER` writer
pub type W = crate::W<CIERrs>;
///Field `LSIRDYIE` reader - LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization.
pub type LSIRDYIE_R = crate::BitReader;
///Field `LSIRDYIE` writer - LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization.
pub type LSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LSERDYIE` reader - LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization.
pub type LSERDYIE_R = crate::BitReader;
///Field `LSERDYIE` writer - LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization.
pub type LSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIRDYIE` reader - HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization.
pub type HSIRDYIE_R = crate::BitReader;
///Field `HSIRDYIE` writer - HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization.
pub type HSIRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSERDYIE` reader - HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization.
pub type HSERDYIE_R = crate::BitReader;
///Field `HSERDYIE` writer - HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization.
pub type HSERDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPLLRDYIE` reader - HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE.
pub type HSIPLLRDYIE_R = crate::BitReader;
///Field `HSIPLLRDYIE` writer - HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE.
pub type HSIPLLRDYIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSIPLLUNLOCKDETIE` reader - HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock.
pub type HSIPLLUNLOCKDETIE_R = crate::BitReader;
///Field `HSIPLLUNLOCKDETIE` writer - HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock.
pub type HSIPLLUNLOCKDETIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCRSTIE` reader - RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end.
pub type RTCRSTIE_R = crate::BitReader;
///Field `RTCRSTIE` writer - RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end.
pub type RTCRSTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGRSTIE` reader - WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end.
pub type WDGRSTIE_R = crate::BitReader;
///Field `WDGRSTIE` writer - WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end.
pub type WDGRSTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPURSTIE` reader - LPURSTIE: LPUART reset release interrupt enable.
pub type LPURSTIE_R = crate::BitReader;
///Field `LPURSTIE` writer - LPURSTIE: LPUART reset release interrupt enable.
pub type LPURSTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization.
    #[inline(always)]
    pub fn lsirdyie(&self) -> LSIRDYIE_R {
        LSIRDYIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization.
    #[inline(always)]
    pub fn lserdyie(&self) -> LSERDYIE_R {
        LSERDYIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization.
    #[inline(always)]
    pub fn hsirdyie(&self) -> HSIRDYIE_R {
        HSIRDYIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization.
    #[inline(always)]
    pub fn hserdyie(&self) -> HSERDYIE_R {
        HSERDYIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE.
    #[inline(always)]
    pub fn hsipllrdyie(&self) -> HSIPLLRDYIE_R {
        HSIPLLRDYIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock.
    #[inline(always)]
    pub fn hsipllunlockdetie(&self) -> HSIPLLUNLOCKDETIE_R {
        HSIPLLUNLOCKDETIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end.
    #[inline(always)]
    pub fn rtcrstie(&self) -> RTCRSTIE_R {
        RTCRSTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end.
    #[inline(always)]
    pub fn wdgrstie(&self) -> WDGRSTIE_R {
        WDGRSTIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LPURSTIE: LPUART reset release interrupt enable.
    #[inline(always)]
    pub fn lpurstie(&self) -> LPURSTIE_R {
        LPURSTIE_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIER")
            .field("lsirdyie", &self.lsirdyie())
            .field("lserdyie", &self.lserdyie())
            .field("hsirdyie", &self.hsirdyie())
            .field("hserdyie", &self.hserdyie())
            .field("hsipllrdyie", &self.hsipllrdyie())
            .field("hsipllunlockdetie", &self.hsipllunlockdetie())
            .field("rtcrstie", &self.rtcrstie())
            .field("wdgrstie", &self.wdgrstie())
            .field("lpurstie", &self.lpurstie())
            .finish()
    }
}
impl W {
    ///Bit 0 - LSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by internal RC 32 kHz oscillator stabilization.
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LSIRDYIE_W<'_, CIERrs> {
        LSIRDYIE_W::new(self, 0)
    }
    ///Bit 1 - LSE Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the external 32 kHz oscillator stabilization.
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LSERDYIE_W<'_, CIERrs> {
        LSERDYIE_W::new(self, 1)
    }
    ///Bit 3 - HSI Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the internal RC 64MHz oscillator stabilization.
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HSIRDYIE_W<'_, CIERrs> {
        HSIRDYIE_W::new(self, 3)
    }
    ///Bit 4 - HSE Ready Interrupt Enable Set and reset by software to enable/disable interrupt caused by the external HSE oscillator stabilization.
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HSERDYIE_W<'_, CIERrs> {
        HSERDYIE_W::new(self, 4)
    }
    ///Bit 5 - HSI PLL Ready Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL locked on HSE.
    #[inline(always)]
    pub fn hsipllrdyie(&mut self) -> HSIPLLRDYIE_W<'_, CIERrs> {
        HSIPLLRDYIE_W::new(self, 5)
    }
    ///Bit 6 - HSIPLLUNLOCKDETIE: HSI PLL unlock detection Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the HSI 64MHz PLL unlock.
    #[inline(always)]
    pub fn hsipllunlockdetie(&mut self) -> HSIPLLUNLOCKDETIE_W<'_, CIERrs> {
        HSIPLLUNLOCKDETIE_W::new(self, 6)
    }
    ///Bit 7 - RTCRSTIE: RTC reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the RTC reset end.
    #[inline(always)]
    pub fn rtcrstie(&mut self) -> RTCRSTIE_W<'_, CIERrs> {
        RTCRSTIE_W::new(self, 7)
    }
    ///Bit 8 - WDGRSTIE: Watchdog reset end Interrupt Enable. Set and reset by software to enable/disable interrupt caused by the watchdog reset end.
    #[inline(always)]
    pub fn wdgrstie(&mut self) -> WDGRSTIE_W<'_, CIERrs> {
        WDGRSTIE_W::new(self, 8)
    }
    ///Bit 9 - LPURSTIE: LPUART reset release interrupt enable.
    #[inline(always)]
    pub fn lpurstie(&mut self) -> LPURSTIE_W<'_, CIERrs> {
        LPURSTIE_W::new(self, 9)
    }
}
/**CIER register

You can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#RCC:CIER)*/
pub struct CIERrs;
impl crate::RegisterSpec for CIERrs {
    type Ux = u32;
}
///`read()` method returns [`cier::R`](R) reader structure
impl crate::Readable for CIERrs {}
///`write(|w| ..)` method takes [`cier::W`](W) writer structure
impl crate::Writable for CIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CIER to value 0
impl crate::Resettable for CIERrs {}
