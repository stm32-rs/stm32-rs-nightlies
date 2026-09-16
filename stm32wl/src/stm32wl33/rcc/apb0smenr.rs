///Register `APB0SMENR` reader
pub type R = crate::R<APB0SMENRrs>;
///Register `APB0SMENR` writer
pub type W = crate::W<APB0SMENRrs>;
///Field `TIM2SMEN` reader - TIM2 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM2 bus clock disabled in Sleep mode - 1: TIM2 bus clock enabled in Sleep mode (if enabled in TIM2EN)
pub type TIM2SMEN_R = crate::BitReader;
///Field `TIM2SMEN` writer - TIM2 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM2 bus clock disabled in Sleep mode - 1: TIM2 bus clock enabled in Sleep mode (if enabled in TIM2EN)
pub type TIM2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16SMEN` reader - TIM16 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM16 bus clock disabled in Sleep mode - 1: TIM16 bus clock enabled in Sleep mode (if enabled in TIM16EN)
pub type TIM16SMEN_R = crate::BitReader;
///Field `TIM16SMEN` writer - TIM16 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM16 bus clock disabled in Sleep mode - 1: TIM16 bus clock enabled in Sleep mode (if enabled in TIM16EN)
pub type TIM16SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGSMEN` reader - SYSCFG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SYSCFG bus clock disabled in Sleep mode - 1: SYSCFG bus clock enabled in Sleep mode (if enabled in SYSCFGEN)
pub type SYSCFGSMEN_R = crate::BitReader;
///Field `SYSCFGSMEN` writer - SYSCFG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SYSCFG bus clock disabled in Sleep mode - 1: SYSCFG bus clock enabled in Sleep mode (if enabled in SYSCFGEN)
pub type SYSCFGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDCSMEN` reader - LCDC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCDC bus clock disabled in Sleep mode - 1: LCDC bus clock enabled in Sleep mode (if enabled in LCDCEN)
pub type LCDCSMEN_R = crate::BitReader;
///Field `LCDCSMEN` writer - LCDC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCDC bus clock disabled in Sleep mode - 1: LCDC bus clock enabled in Sleep mode (if enabled in LCDCEN)
pub type LCDCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPSMEN` reader - COMP bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: COMP bus clock disabled in Sleep mode - 1: COMP bus clock enabled in Sleep mode (if enabled in COMPEN)
pub type COMPSMEN_R = crate::BitReader;
///Field `COMPSMEN` writer - COMP bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: COMP bus clock disabled in Sleep mode - 1: COMP bus clock enabled in Sleep mode (if enabled in COMPEN)
pub type COMPSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DACSMEN` reader - DAC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: DAC bus clock disabled in Sleep mode - 1: DAC bus clock enabled in Sleep mode (if enabled in DACEN)
pub type DACSMEN_R = crate::BitReader;
///Field `DACSMEN` writer - DAC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: DAC bus clock disabled in Sleep mode - 1: DAC bus clock enabled in Sleep mode (if enabled in DACEN)
pub type DACSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTCSMEN` reader - RTC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RTC bus clock disabled in Sleep mode - 1: RTC bus clock enabled in Sleep mode (if enabled in RTCEN)
pub type RTCSMEN_R = crate::BitReader;
///Field `RTCSMEN` writer - RTC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RTC bus clock disabled in Sleep mode - 1: RTC bus clock enabled in Sleep mode (if enabled in RTCEN)
pub type RTCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCSCSMEN` reader - LCSC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCSC bus clock disabled in Sleep mode - 1: LCSC bus clock enabled in Sleep mode (if enabled in LCSCEN)
pub type LCSCSMEN_R = crate::BitReader;
///Field `LCSCSMEN` writer - LCSC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCSC bus clock disabled in Sleep mode - 1: LCSC bus clock enabled in Sleep mode (if enabled in LCSCEN)
pub type LCSCSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WDGSMEN` reader - WDG clock enable during Sleep mode bit This bit is set and reset by software. - 0: WDG clock disabled in Sleep mode - 1: WDG clock enabled in Sleep mode (if enabled in WDGEN)
pub type WDGSMEN_R = crate::BitReader;
///Field `WDGSMEN` writer - WDG clock enable during Sleep mode bit This bit is set and reset by software. - 0: WDG clock disabled in Sleep mode - 1: WDG clock enabled in Sleep mode (if enabled in WDGEN)
pub type WDGSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DBGMCUSMEN` reader - DBGMCU clock enable during Sleep mode bit This bit is set and reset by software. - 0: DBGMCU clock disabled in Sleep mode - 1: DBGMCU clock enabled in Sleep mode (if enabled in DBGMCUEN)
pub type DBGMCUSMEN_R = crate::BitReader;
///Field `DBGMCUSMEN` writer - DBGMCU clock enable during Sleep mode bit This bit is set and reset by software. - 0: DBGMCU clock disabled in Sleep mode - 1: DBGMCU clock enabled in Sleep mode (if enabled in DBGMCUEN)
pub type DBGMCUSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM2 bus clock disabled in Sleep mode - 1: TIM2 bus clock enabled in Sleep mode (if enabled in TIM2EN)
    #[inline(always)]
    pub fn tim2smen(&self) -> TIM2SMEN_R {
        TIM2SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM16 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM16 bus clock disabled in Sleep mode - 1: TIM16 bus clock enabled in Sleep mode (if enabled in TIM16EN)
    #[inline(always)]
    pub fn tim16smen(&self) -> TIM16SMEN_R {
        TIM16SMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 8 - SYSCFG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SYSCFG bus clock disabled in Sleep mode - 1: SYSCFG bus clock enabled in Sleep mode (if enabled in SYSCFGEN)
    #[inline(always)]
    pub fn syscfgsmen(&self) -> SYSCFGSMEN_R {
        SYSCFGSMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - LCDC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCDC bus clock disabled in Sleep mode - 1: LCDC bus clock enabled in Sleep mode (if enabled in LCDCEN)
    #[inline(always)]
    pub fn lcdcsmen(&self) -> LCDCSMEN_R {
        LCDCSMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - COMP bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: COMP bus clock disabled in Sleep mode - 1: COMP bus clock enabled in Sleep mode (if enabled in COMPEN)
    #[inline(always)]
    pub fn compsmen(&self) -> COMPSMEN_R {
        COMPSMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - DAC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: DAC bus clock disabled in Sleep mode - 1: DAC bus clock enabled in Sleep mode (if enabled in DACEN)
    #[inline(always)]
    pub fn dacsmen(&self) -> DACSMEN_R {
        DACSMEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - RTC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RTC bus clock disabled in Sleep mode - 1: RTC bus clock enabled in Sleep mode (if enabled in RTCEN)
    #[inline(always)]
    pub fn rtcsmen(&self) -> RTCSMEN_R {
        RTCSMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - LCSC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCSC bus clock disabled in Sleep mode - 1: LCSC bus clock enabled in Sleep mode (if enabled in LCSCEN)
    #[inline(always)]
    pub fn lcscsmen(&self) -> LCSCSMEN_R {
        LCSCSMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - WDG clock enable during Sleep mode bit This bit is set and reset by software. - 0: WDG clock disabled in Sleep mode - 1: WDG clock enabled in Sleep mode (if enabled in WDGEN)
    #[inline(always)]
    pub fn wdgsmen(&self) -> WDGSMEN_R {
        WDGSMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DBGMCU clock enable during Sleep mode bit This bit is set and reset by software. - 0: DBGMCU clock disabled in Sleep mode - 1: DBGMCU clock enabled in Sleep mode (if enabled in DBGMCUEN)
    #[inline(always)]
    pub fn dbgmcusmen(&self) -> DBGMCUSMEN_R {
        DBGMCUSMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB0SMENR")
            .field("tim2smen", &self.tim2smen())
            .field("tim16smen", &self.tim16smen())
            .field("syscfgsmen", &self.syscfgsmen())
            .field("lcdcsmen", &self.lcdcsmen())
            .field("compsmen", &self.compsmen())
            .field("dacsmen", &self.dacsmen())
            .field("rtcsmen", &self.rtcsmen())
            .field("lcscsmen", &self.lcscsmen())
            .field("wdgsmen", &self.wdgsmen())
            .field("dbgmcusmen", &self.dbgmcusmen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM2 bus clock disabled in Sleep mode - 1: TIM2 bus clock enabled in Sleep mode (if enabled in TIM2EN)
    #[inline(always)]
    pub fn tim2smen(&mut self) -> TIM2SMEN_W<'_, APB0SMENRrs> {
        TIM2SMEN_W::new(self, 0)
    }
    ///Bit 1 - TIM16 bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: TIM16 bus clock disabled in Sleep mode - 1: TIM16 bus clock enabled in Sleep mode (if enabled in TIM16EN)
    #[inline(always)]
    pub fn tim16smen(&mut self) -> TIM16SMEN_W<'_, APB0SMENRrs> {
        TIM16SMEN_W::new(self, 1)
    }
    ///Bit 8 - SYSCFG bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: SYSCFG bus clock disabled in Sleep mode - 1: SYSCFG bus clock enabled in Sleep mode (if enabled in SYSCFGEN)
    #[inline(always)]
    pub fn syscfgsmen(&mut self) -> SYSCFGSMEN_W<'_, APB0SMENRrs> {
        SYSCFGSMEN_W::new(self, 8)
    }
    ///Bit 9 - LCDC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCDC bus clock disabled in Sleep mode - 1: LCDC bus clock enabled in Sleep mode (if enabled in LCDCEN)
    #[inline(always)]
    pub fn lcdcsmen(&mut self) -> LCDCSMEN_W<'_, APB0SMENRrs> {
        LCDCSMEN_W::new(self, 9)
    }
    ///Bit 10 - COMP bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: COMP bus clock disabled in Sleep mode - 1: COMP bus clock enabled in Sleep mode (if enabled in COMPEN)
    #[inline(always)]
    pub fn compsmen(&mut self) -> COMPSMEN_W<'_, APB0SMENRrs> {
        COMPSMEN_W::new(self, 10)
    }
    ///Bit 11 - DAC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: DAC bus clock disabled in Sleep mode - 1: DAC bus clock enabled in Sleep mode (if enabled in DACEN)
    #[inline(always)]
    pub fn dacsmen(&mut self) -> DACSMEN_W<'_, APB0SMENRrs> {
        DACSMEN_W::new(self, 11)
    }
    ///Bit 12 - RTC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: RTC bus clock disabled in Sleep mode - 1: RTC bus clock enabled in Sleep mode (if enabled in RTCEN)
    #[inline(always)]
    pub fn rtcsmen(&mut self) -> RTCSMEN_W<'_, APB0SMENRrs> {
        RTCSMEN_W::new(self, 12)
    }
    ///Bit 13 - LCSC bus clock enable during Sleep mode bit This bit is set and reset by software. - 0: LCSC bus clock disabled in Sleep mode - 1: LCSC bus clock enabled in Sleep mode (if enabled in LCSCEN)
    #[inline(always)]
    pub fn lcscsmen(&mut self) -> LCSCSMEN_W<'_, APB0SMENRrs> {
        LCSCSMEN_W::new(self, 13)
    }
    ///Bit 14 - WDG clock enable during Sleep mode bit This bit is set and reset by software. - 0: WDG clock disabled in Sleep mode - 1: WDG clock enabled in Sleep mode (if enabled in WDGEN)
    #[inline(always)]
    pub fn wdgsmen(&mut self) -> WDGSMEN_W<'_, APB0SMENRrs> {
        WDGSMEN_W::new(self, 14)
    }
    ///Bit 15 - DBGMCU clock enable during Sleep mode bit This bit is set and reset by software. - 0: DBGMCU clock disabled in Sleep mode - 1: DBGMCU clock enabled in Sleep mode (if enabled in DBGMCUEN)
    #[inline(always)]
    pub fn dbgmcusmen(&mut self) -> DBGMCUSMEN_W<'_, APB0SMENRrs> {
        DBGMCUSMEN_W::new(self, 15)
    }
}
/**APB0SMENR register

You can [`read`](crate::Reg::read) this register and get [`apb0smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb0smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#RCC:APB0SMENR)*/
pub struct APB0SMENRrs;
impl crate::RegisterSpec for APB0SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb0smenr::R`](R) reader structure
impl crate::Readable for APB0SMENRrs {}
///`write(|w| ..)` method takes [`apb0smenr::W`](W) writer structure
impl crate::Writable for APB0SMENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB0SMENR to value 0xff03
impl crate::Resettable for APB0SMENRrs {
    const RESET_VALUE: u32 = 0xff03;
}
