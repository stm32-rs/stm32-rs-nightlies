///Register `PUBCFGCR1` writer
pub type W = crate::W<PUBCFGCR1rs>;
///Field `PLL1PUBC` writer - Defines the public protection of the PLL1 configuration bits (enable, ready, divider).
pub type PLL1PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2PUBC` writer - Defines the public protection of the PLL2 configuration bits (enable, ready, divider).
pub type PLL2PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3PUBC` writer - Defines the public protection of the PLL3 configuration bits (enable, ready, divider).
pub type PLL3PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4PUBC` writer - Defines the public protection of the PLL4 configuration bits (enable, ready, divider).
pub type PLL4PUBC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PUBCFGCR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the PLL1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll1pubc(&mut self) -> PLL1PUBC_W<'_, PUBCFGCR1rs> {
        PLL1PUBC_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the PLL2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll2pubc(&mut self) -> PLL2PUBC_W<'_, PUBCFGCR1rs> {
        PLL2PUBC_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the PLL3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll3pubc(&mut self) -> PLL3PUBC_W<'_, PUBCFGCR1rs> {
        PLL3PUBC_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the PLL4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll4pubc(&mut self) -> PLL4PUBC_W<'_, PUBCFGCR1rs> {
        PLL4PUBC_W::new(self, 3)
    }
}
/**RCC PLL public configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PUBCFGCR1)*/
pub struct PUBCFGCR1rs;
impl crate::RegisterSpec for PUBCFGCR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`pubcfgcr1::W`](W) writer structure
impl crate::Writable for PUBCFGCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGCR1 to value 0
impl crate::Resettable for PUBCFGCR1rs {}
