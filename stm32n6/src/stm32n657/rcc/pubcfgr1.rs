///Register `PUBCFGR1` reader
pub type R = crate::R<PUBCFGR1rs>;
///Register `PUBCFGR1` writer
pub type W = crate::W<PUBCFGR1rs>;
///Field `PLL1PUB` reader - Defines the public protection of the PLL1 PLL configuration bits.
pub type PLL1PUB_R = crate::BitReader;
///Field `PLL1PUB` writer - Defines the public protection of the PLL1 PLL configuration bits.
pub type PLL1PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2PUB` reader - Defines the public protection of the PLL2 PLL configuration bits.
pub type PLL2PUB_R = crate::BitReader;
///Field `PLL2PUB` writer - Defines the public protection of the PLL2 PLL configuration bits.
pub type PLL2PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3PUB` reader - Defines the public protection of the PLL3 PLL configuration bits.
pub type PLL3PUB_R = crate::BitReader;
///Field `PLL3PUB` writer - Defines the public protection of the PLL3 PLL configuration bits.
pub type PLL3PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4PUB` reader - Defines the public protection of the PLL4 PLL configuration bits.
pub type PLL4PUB_R = crate::BitReader;
///Field `PLL4PUB` writer - Defines the public protection of the PLL4 PLL configuration bits.
pub type PLL4PUB_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the public protection of the PLL1 PLL configuration bits.
    #[inline(always)]
    pub fn pll1pub(&self) -> PLL1PUB_R {
        PLL1PUB_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the public protection of the PLL2 PLL configuration bits.
    #[inline(always)]
    pub fn pll2pub(&self) -> PLL2PUB_R {
        PLL2PUB_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the public protection of the PLL3 PLL configuration bits.
    #[inline(always)]
    pub fn pll3pub(&self) -> PLL3PUB_R {
        PLL3PUB_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the public protection of the PLL4 PLL configuration bits.
    #[inline(always)]
    pub fn pll4pub(&self) -> PLL4PUB_R {
        PLL4PUB_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUBCFGR1")
            .field("pll1pub", &self.pll1pub())
            .field("pll2pub", &self.pll2pub())
            .field("pll3pub", &self.pll3pub())
            .field("pll4pub", &self.pll4pub())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the public protection of the PLL1 PLL configuration bits.
    #[inline(always)]
    pub fn pll1pub(&mut self) -> PLL1PUB_W<'_, PUBCFGR1rs> {
        PLL1PUB_W::new(self, 0)
    }
    ///Bit 1 - Defines the public protection of the PLL2 PLL configuration bits.
    #[inline(always)]
    pub fn pll2pub(&mut self) -> PLL2PUB_W<'_, PUBCFGR1rs> {
        PLL2PUB_W::new(self, 1)
    }
    ///Bit 2 - Defines the public protection of the PLL3 PLL configuration bits.
    #[inline(always)]
    pub fn pll3pub(&mut self) -> PLL3PUB_W<'_, PUBCFGR1rs> {
        PLL3PUB_W::new(self, 2)
    }
    ///Bit 3 - Defines the public protection of the PLL4 PLL configuration bits.
    #[inline(always)]
    pub fn pll4pub(&mut self) -> PLL4PUB_W<'_, PUBCFGR1rs> {
        PLL4PUB_W::new(self, 3)
    }
}
/**RCC PLL public configuration register1

You can [`read`](crate::Reg::read) this register and get [`pubcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pubcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PUBCFGR1)*/
pub struct PUBCFGR1rs;
impl crate::RegisterSpec for PUBCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`pubcfgr1::R`](R) reader structure
impl crate::Readable for PUBCFGR1rs {}
///`write(|w| ..)` method takes [`pubcfgr1::W`](W) writer structure
impl crate::Writable for PUBCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUBCFGR1 to value 0
impl crate::Resettable for PUBCFGR1rs {}
