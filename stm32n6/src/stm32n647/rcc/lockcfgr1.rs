///Register `LOCKCFGR1` writer
pub type W = crate::W<LOCKCFGR1rs>;
///Field `PLL1LOCK` writer - Defines the lock protection of the PLL1 PLL configuration bits.
pub type PLL1LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2LOCK` writer - Defines the lock protection of the PLL2 PLL configuration bits.
pub type PLL2LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3LOCK` writer - Defines the lock protection of the PLL3 PLL configuration bits.
pub type PLL3LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4LOCK` writer - Defines the lock protection of the PLL4 PLL configuration bits.
pub type PLL4LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<LOCKCFGR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the lock protection of the PLL1 PLL configuration bits.
    #[inline(always)]
    pub fn pll1lock(&mut self) -> PLL1LOCK_W<'_, LOCKCFGR1rs> {
        PLL1LOCK_W::new(self, 0)
    }
    ///Bit 1 - Defines the lock protection of the PLL2 PLL configuration bits.
    #[inline(always)]
    pub fn pll2lock(&mut self) -> PLL2LOCK_W<'_, LOCKCFGR1rs> {
        PLL2LOCK_W::new(self, 1)
    }
    ///Bit 2 - Defines the lock protection of the PLL3 PLL configuration bits.
    #[inline(always)]
    pub fn pll3lock(&mut self) -> PLL3LOCK_W<'_, LOCKCFGR1rs> {
        PLL3LOCK_W::new(self, 2)
    }
    ///Bit 3 - Defines the lock protection of the PLL4 PLL configuration bits.
    #[inline(always)]
    pub fn pll4lock(&mut self) -> PLL4LOCK_W<'_, LOCKCFGR1rs> {
        PLL4LOCK_W::new(self, 3)
    }
}
/**RCC PLL lock configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lockcfgr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:LOCKCFGR1)*/
pub struct LOCKCFGR1rs;
impl crate::RegisterSpec for LOCKCFGR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lockcfgr1::W`](W) writer structure
impl crate::Writable for LOCKCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LOCKCFGR1 to value 0
impl crate::Resettable for LOCKCFGR1rs {}
