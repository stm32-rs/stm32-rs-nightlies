///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
///Field `EN` reader - enable
pub type EN_R = crate::BitReader;
///Field `EN` writer - enable
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CACHEINV` writer - full cache invalidation
pub type CACHEINV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RHITMEN` reader - read-hit monitor enable
pub type RHITMEN_R = crate::BitReader;
///Field `RHITMEN` writer - read-hit monitor enable
pub type RHITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMISSMEN` reader - read-miss monitor enable
pub type RMISSMEN_R = crate::BitReader;
///Field `RMISSMEN` writer - read-miss monitor enable
pub type RMISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RHITMRST` writer - read-hit monitor reset
pub type RHITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMISSMRST` writer - read-miss monitor reset
pub type RMISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WHITMEN` reader - write-hit monitor enable
pub type WHITMEN_R = crate::BitReader;
///Field `WHITMEN` writer - write-hit monitor enable
pub type WHITMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WMISSMEN` reader - write-miss monitor enable
pub type WMISSMEN_R = crate::BitReader;
///Field `WMISSMEN` writer - write-miss monitor enable
pub type WMISSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WHITMRST` writer - write-hit monitor reset
pub type WHITMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WMISSMRST` writer - write-miss monitor reset
pub type WMISSMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMMEN` reader - read-allocate miss monitor enable
pub type RAMMEN_R = crate::BitReader;
///Field `RAMMEN` writer - read-allocate miss monitor enable
pub type RAMMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAMMEN` reader - write-allocate miss monitor enable
pub type WAMMEN_R = crate::BitReader;
///Field `WAMMEN` writer - write-allocate miss monitor enable
pub type WAMMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMMRST` writer - read-allocate miss monitor reset
pub type RAMMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WAMMRST` writer - write-allocate miss monitor reset
pub type WAMMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WTMEN` reader - write-through monitor enable
pub type WTMEN_R = crate::BitReader;
///Field `WTMEN` writer - write-through monitor enable
pub type WTMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVIMEN` reader - eviction monitor enable
pub type EVIMEN_R = crate::BitReader;
///Field `EVIMEN` writer - eviction monitor enable
pub type EVIMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WTMRST` writer - write-through monitor reset
pub type WTMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EVIMRST` writer - eviction monitor reset
pub type EVIMRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - read-hit monitor enable
    #[inline(always)]
    pub fn rhitmen(&self) -> RHITMEN_R {
        RHITMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - read-miss monitor enable
    #[inline(always)]
    pub fn rmissmen(&self) -> RMISSMEN_R {
        RMISSMEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - write-hit monitor enable
    #[inline(always)]
    pub fn whitmen(&self) -> WHITMEN_R {
        WHITMEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - write-miss monitor enable
    #[inline(always)]
    pub fn wmissmen(&self) -> WMISSMEN_R {
        WMISSMEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 24 - read-allocate miss monitor enable
    #[inline(always)]
    pub fn rammen(&self) -> RAMMEN_R {
        RAMMEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - write-allocate miss monitor enable
    #[inline(always)]
    pub fn wammen(&self) -> WAMMEN_R {
        WAMMEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - write-through monitor enable
    #[inline(always)]
    pub fn wtmen(&self) -> WTMEN_R {
        WTMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - eviction monitor enable
    #[inline(always)]
    pub fn evimen(&self) -> EVIMEN_R {
        EVIMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("en", &self.en())
            .field("rhitmen", &self.rhitmen())
            .field("rmissmen", &self.rmissmen())
            .field("whitmen", &self.whitmen())
            .field("wmissmen", &self.wmissmen())
            .field("rammen", &self.rammen())
            .field("wammen", &self.wammen())
            .field("wtmen", &self.wtmen())
            .field("evimen", &self.evimen())
            .finish()
    }
}
impl W {
    ///Bit 0 - enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CR1rs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - full cache invalidation
    #[inline(always)]
    pub fn cacheinv(&mut self) -> CACHEINV_W<'_, CR1rs> {
        CACHEINV_W::new(self, 1)
    }
    ///Bit 16 - read-hit monitor enable
    #[inline(always)]
    pub fn rhitmen(&mut self) -> RHITMEN_W<'_, CR1rs> {
        RHITMEN_W::new(self, 16)
    }
    ///Bit 17 - read-miss monitor enable
    #[inline(always)]
    pub fn rmissmen(&mut self) -> RMISSMEN_W<'_, CR1rs> {
        RMISSMEN_W::new(self, 17)
    }
    ///Bit 18 - read-hit monitor reset
    #[inline(always)]
    pub fn rhitmrst(&mut self) -> RHITMRST_W<'_, CR1rs> {
        RHITMRST_W::new(self, 18)
    }
    ///Bit 19 - read-miss monitor reset
    #[inline(always)]
    pub fn rmissmrst(&mut self) -> RMISSMRST_W<'_, CR1rs> {
        RMISSMRST_W::new(self, 19)
    }
    ///Bit 20 - write-hit monitor enable
    #[inline(always)]
    pub fn whitmen(&mut self) -> WHITMEN_W<'_, CR1rs> {
        WHITMEN_W::new(self, 20)
    }
    ///Bit 21 - write-miss monitor enable
    #[inline(always)]
    pub fn wmissmen(&mut self) -> WMISSMEN_W<'_, CR1rs> {
        WMISSMEN_W::new(self, 21)
    }
    ///Bit 22 - write-hit monitor reset
    #[inline(always)]
    pub fn whitmrst(&mut self) -> WHITMRST_W<'_, CR1rs> {
        WHITMRST_W::new(self, 22)
    }
    ///Bit 23 - write-miss monitor reset
    #[inline(always)]
    pub fn wmissmrst(&mut self) -> WMISSMRST_W<'_, CR1rs> {
        WMISSMRST_W::new(self, 23)
    }
    ///Bit 24 - read-allocate miss monitor enable
    #[inline(always)]
    pub fn rammen(&mut self) -> RAMMEN_W<'_, CR1rs> {
        RAMMEN_W::new(self, 24)
    }
    ///Bit 25 - write-allocate miss monitor enable
    #[inline(always)]
    pub fn wammen(&mut self) -> WAMMEN_W<'_, CR1rs> {
        WAMMEN_W::new(self, 25)
    }
    ///Bit 26 - read-allocate miss monitor reset
    #[inline(always)]
    pub fn rammrst(&mut self) -> RAMMRST_W<'_, CR1rs> {
        RAMMRST_W::new(self, 26)
    }
    ///Bit 27 - write-allocate miss monitor reset
    #[inline(always)]
    pub fn wammrst(&mut self) -> WAMMRST_W<'_, CR1rs> {
        WAMMRST_W::new(self, 27)
    }
    ///Bit 28 - write-through monitor enable
    #[inline(always)]
    pub fn wtmen(&mut self) -> WTMEN_W<'_, CR1rs> {
        WTMEN_W::new(self, 28)
    }
    ///Bit 29 - eviction monitor enable
    #[inline(always)]
    pub fn evimen(&mut self) -> EVIMEN_W<'_, CR1rs> {
        EVIMEN_W::new(self, 29)
    }
    ///Bit 30 - write-through monitor reset
    #[inline(always)]
    pub fn wtmrst(&mut self) -> WTMRST_W<'_, CR1rs> {
        WTMRST_W::new(self, 30)
    }
    ///Bit 31 - eviction monitor reset
    #[inline(always)]
    pub fn evimrst(&mut self) -> EVIMRST_W<'_, CR1rs> {
        EVIMRST_W::new(self, 31)
    }
}
/**CACHEAXI control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#CACHEAXI:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
