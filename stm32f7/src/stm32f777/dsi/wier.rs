///Register `WIER` reader
pub type R = crate::R<WIERrs>;
///Register `WIER` writer
pub type W = crate::W<WIERrs>;
///Field `TEIE` reader - Tearing Effect Interrupt Enable
pub type TEIE_R = crate::BitReader;
///Field `TEIE` writer - Tearing Effect Interrupt Enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERIE` reader - End of Refresh Interrupt Enable
pub type ERIE_R = crate::BitReader;
///Field `ERIE` writer - End of Refresh Interrupt Enable
pub type ERIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLLIE` reader - PLL Lock Interrupt Enable
pub type PLLLIE_R = crate::BitReader;
///Field `PLLLIE` writer - PLL Lock Interrupt Enable
pub type PLLLIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLLUIE` reader - PLL Unlock Interrupt Enable
pub type PLLUIE_R = crate::BitReader;
///Field `PLLUIE` writer - PLL Unlock Interrupt Enable
pub type PLLUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRIE` reader - Regulator Ready Interrupt Enable
pub type RRIE_R = crate::BitReader;
///Field `RRIE` writer - Regulator Ready Interrupt Enable
pub type RRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tearing Effect Interrupt Enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of Refresh Interrupt Enable
    #[inline(always)]
    pub fn erie(&self) -> ERIE_R {
        ERIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - PLL Lock Interrupt Enable
    #[inline(always)]
    pub fn plllie(&self) -> PLLLIE_R {
        PLLLIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - PLL Unlock Interrupt Enable
    #[inline(always)]
    pub fn plluie(&self) -> PLLUIE_R {
        PLLUIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - Regulator Ready Interrupt Enable
    #[inline(always)]
    pub fn rrie(&self) -> RRIE_R {
        RRIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIER")
            .field("rrie", &self.rrie())
            .field("plluie", &self.plluie())
            .field("plllie", &self.plllie())
            .field("erie", &self.erie())
            .field("teie", &self.teie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tearing Effect Interrupt Enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, WIERrs> {
        TEIE_W::new(self, 0)
    }
    ///Bit 1 - End of Refresh Interrupt Enable
    #[inline(always)]
    pub fn erie(&mut self) -> ERIE_W<'_, WIERrs> {
        ERIE_W::new(self, 1)
    }
    ///Bit 9 - PLL Lock Interrupt Enable
    #[inline(always)]
    pub fn plllie(&mut self) -> PLLLIE_W<'_, WIERrs> {
        PLLLIE_W::new(self, 9)
    }
    ///Bit 10 - PLL Unlock Interrupt Enable
    #[inline(always)]
    pub fn plluie(&mut self) -> PLLUIE_W<'_, WIERrs> {
        PLLUIE_W::new(self, 10)
    }
    ///Bit 13 - Regulator Ready Interrupt Enable
    #[inline(always)]
    pub fn rrie(&mut self) -> RRIE_W<'_, WIERrs> {
        RRIE_W::new(self, 13)
    }
}
/**DSI Wrapper Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`wier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#DSI:WIER)*/
pub struct WIERrs;
impl crate::RegisterSpec for WIERrs {
    type Ux = u32;
}
///`read()` method returns [`wier::R`](R) reader structure
impl crate::Readable for WIERrs {}
///`write(|w| ..)` method takes [`wier::W`](W) writer structure
impl crate::Writable for WIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets WIER to value 0
impl crate::Resettable for WIERrs {}
