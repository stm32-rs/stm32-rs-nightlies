///Register `PRIVCFGCR1` writer
pub type W = crate::W<PRIVCFGCR1rs>;
///Field `PLL1PVC` writer - Defines the privilege protection of the PLL1 configuration bits (enable, ready, divider).
pub type PLL1PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2PVC` writer - Defines the privilege protection of the PLL2 configuration bits (enable, ready, divider).
pub type PLL2PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3PVC` writer - Defines the privilege protection of the PLL3 configuration bits (enable, ready, divider).
pub type PLL3PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4PVC` writer - Defines the privilege protection of the PLL4 configuration bits (enable, ready, divider).
pub type PLL4PVC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<PRIVCFGCR1rs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the PLL1 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll1pvc(&mut self) -> PLL1PVC_W<'_, PRIVCFGCR1rs> {
        PLL1PVC_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the PLL2 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll2pvc(&mut self) -> PLL2PVC_W<'_, PRIVCFGCR1rs> {
        PLL2PVC_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the PLL3 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll3pvc(&mut self) -> PLL3PVC_W<'_, PRIVCFGCR1rs> {
        PLL3PVC_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the PLL4 configuration bits (enable, ready, divider).
    #[inline(always)]
    pub fn pll4pvc(&mut self) -> PLL4PVC_W<'_, PRIVCFGCR1rs> {
        PLL4PVC_W::new(self, 3)
    }
}
/**RCC PLL privilege configuration register1

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgcr1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PRIVCFGCR1)*/
pub struct PRIVCFGCR1rs;
impl crate::RegisterSpec for PRIVCFGCR1rs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`privcfgcr1::W`](W) writer structure
impl crate::Writable for PRIVCFGCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGCR1 to value 0
impl crate::Resettable for PRIVCFGCR1rs {}
