///Register `PRIVCFGR1` reader
pub type R = crate::R<PRIVCFGR1rs>;
///Register `PRIVCFGR1` writer
pub type W = crate::W<PRIVCFGR1rs>;
///Field `PLL1PV` reader - Defines the privilege protection of the PLL1 PLL configuration bits.
pub type PLL1PV_R = crate::BitReader;
///Field `PLL1PV` writer - Defines the privilege protection of the PLL1 PLL configuration bits.
pub type PLL1PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2PV` reader - Defines the privilege protection of the PLL2 PLL configuration bits.
pub type PLL2PV_R = crate::BitReader;
///Field `PLL2PV` writer - Defines the privilege protection of the PLL2 PLL configuration bits.
pub type PLL2PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3PV` reader - Defines the privilege protection of the PLL3 PLL configuration bits.
pub type PLL3PV_R = crate::BitReader;
///Field `PLL3PV` writer - Defines the privilege protection of the PLL3 PLL configuration bits.
pub type PLL3PV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4PV` reader - Defines the privilege protection of the PLL4 PLL configuration bits.
pub type PLL4PV_R = crate::BitReader;
///Field `PLL4PV` writer - Defines the privilege protection of the PLL4 PLL configuration bits.
pub type PLL4PV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the privilege protection of the PLL1 PLL configuration bits.
    #[inline(always)]
    pub fn pll1pv(&self) -> PLL1PV_R {
        PLL1PV_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the privilege protection of the PLL2 PLL configuration bits.
    #[inline(always)]
    pub fn pll2pv(&self) -> PLL2PV_R {
        PLL2PV_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the privilege protection of the PLL3 PLL configuration bits.
    #[inline(always)]
    pub fn pll3pv(&self) -> PLL3PV_R {
        PLL3PV_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the privilege protection of the PLL4 PLL configuration bits.
    #[inline(always)]
    pub fn pll4pv(&self) -> PLL4PV_R {
        PLL4PV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVCFGR1")
            .field("pll1pv", &self.pll1pv())
            .field("pll2pv", &self.pll2pv())
            .field("pll3pv", &self.pll3pv())
            .field("pll4pv", &self.pll4pv())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the privilege protection of the PLL1 PLL configuration bits.
    #[inline(always)]
    pub fn pll1pv(&mut self) -> PLL1PV_W<'_, PRIVCFGR1rs> {
        PLL1PV_W::new(self, 0)
    }
    ///Bit 1 - Defines the privilege protection of the PLL2 PLL configuration bits.
    #[inline(always)]
    pub fn pll2pv(&mut self) -> PLL2PV_W<'_, PRIVCFGR1rs> {
        PLL2PV_W::new(self, 1)
    }
    ///Bit 2 - Defines the privilege protection of the PLL3 PLL configuration bits.
    #[inline(always)]
    pub fn pll3pv(&mut self) -> PLL3PV_W<'_, PRIVCFGR1rs> {
        PLL3PV_W::new(self, 2)
    }
    ///Bit 3 - Defines the privilege protection of the PLL4 PLL configuration bits.
    #[inline(always)]
    pub fn pll4pv(&mut self) -> PLL4PV_W<'_, PRIVCFGR1rs> {
        PLL4PV_W::new(self, 3)
    }
}
/**RCC PLL privilege configuration register1

You can [`read`](crate::Reg::read) this register and get [`privcfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`privcfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PRIVCFGR1)*/
pub struct PRIVCFGR1rs;
impl crate::RegisterSpec for PRIVCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`privcfgr1::R`](R) reader structure
impl crate::Readable for PRIVCFGR1rs {}
///`write(|w| ..)` method takes [`privcfgr1::W`](W) writer structure
impl crate::Writable for PRIVCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PRIVCFGR1 to value 0
impl crate::Resettable for PRIVCFGR1rs {}
