///Register `PRIVCFGSR1` writer
pub type W = crate::W<PRIVCFGSR1rs>;
///Field `PLL1PVS` writer - Defines the privilege protection of the PLL1 configuration bits (enable, ready, divider).
pub type PLL1PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2PVS` writer - Defines the privilege protection of the PLL2 configuration bits (enable, ready, divider).
pub type PLL2PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3PVS` writer - Defines the privilege protection of the PLL3 configuration bits (enable, ready, divider).
pub type PLL3PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4PVS` writer - Defines the privilege protection of the PLL4 configuration bits (enable, ready, divider).
pub type PLL4PVS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGSR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the PLL1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll1pvs(&mut self) -> PLL1PVS_W<'_, PRIVCFGSR1rs> {
        PLL1PVS_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the PLL2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll2pvs(&mut self) -> PLL2PVS_W<'_, PRIVCFGSR1rs> {
        PLL2PVS_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the PLL3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll3pvs(&mut self) -> PLL3PVS_W<'_, PRIVCFGSR1rs> {
        PLL3PVS_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the PLL4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll4pvs(&mut self) -> PLL4PVS_W<'_, PRIVCFGSR1rs> {
        PLL4PVS_W::new(self, 3)
    }
}
/**RCC PLL privilege configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgsr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PRIVCFGSR1)*/
pub struct PRIVCFGSR1rs;
impl crate::RegisterSpec for PRIVCFGSR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgsr1::W`](W) writer structure
impl crate::Writable for PRIVCFGSR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGSR1 to value 0
impl crate::Resettable for PRIVCFGSR1rs {}
