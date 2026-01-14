///Register `PUBCFGSR1` writer
pub type W = crate::W<PUBCFGSR1rs>;
///Field `PLL1PUBS` writer - Defines the public protection of the PLL1 configuration bits (enable, ready, divider).
pub type PLL1PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2PUBS` writer - Defines the public protection of the PLL2 configuration bits (enable, ready, divider).
pub type PLL2PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3PUBS` writer - Defines the public protection of the PLL3 configuration bits (enable, ready, divider).
pub type PLL3PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4PUBS` writer - Defines the public protection of the PLL4 configuration bits (enable, ready, divider).
pub type PLL4PUBS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGSR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the PLL1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll1pubs(&mut self) -> PLL1PUBS_W<'_, PUBCFGSR1rs> {
        PLL1PUBS_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the PLL2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll2pubs(&mut self) -> PLL2PUBS_W<'_, PUBCFGSR1rs> {
        PLL2PUBS_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the PLL3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll3pubs(&mut self) -> PLL3PUBS_W<'_, PUBCFGSR1rs> {
        PLL3PUBS_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the PLL4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll4pubs(&mut self) -> PLL4PUBS_W<'_, PUBCFGSR1rs> {
        PLL4PUBS_W::new(self, 3)
    }
}
/**RCC PLL public configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgsr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PUBCFGSR1)*/
pub struct PUBCFGSR1rs;
impl crate::RegisterSpec for PUBCFGSR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgsr1::W`](W) writer structure
impl crate::Writable for PUBCFGSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGSR1 to value 0
impl crate::Resettable for PUBCFGSR1rs {}
