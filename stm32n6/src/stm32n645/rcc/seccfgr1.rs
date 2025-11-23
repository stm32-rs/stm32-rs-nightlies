///Register `SECCFGR1` reader
pub type R = crate::R<SECCFGR1rs>;
///Register `SECCFGR1` writer
pub type W = crate::W<SECCFGR1rs>;
///Field `PLL1SEC` reader - Defines the secure protection of the PLL1 PLL configuration bits.
pub type PLL1SEC_R = crate::BitReader;
///Field `PLL1SEC` writer - Defines the secure protection of the PLL1 PLL configuration bits.
pub type PLL1SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2SEC` reader - Defines the secure protection of the PLL2 PLL configuration bits.
pub type PLL2SEC_R = crate::BitReader;
///Field `PLL2SEC` writer - Defines the secure protection of the PLL2 PLL configuration bits.
pub type PLL2SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3SEC` reader - Defines the secure protection of the PLL3 PLL configuration bits.
pub type PLL3SEC_R = crate::BitReader;
///Field `PLL3SEC` writer - Defines the secure protection of the PLL3 PLL configuration bits.
pub type PLL3SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4SEC` reader - Defines the secure protection of the PLL4 PLL configuration bits.
pub type PLL4SEC_R = crate::BitReader;
///Field `PLL4SEC` writer - Defines the secure protection of the PLL4 PLL configuration bits.
pub type PLL4SEC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Defines the secure protection of the PLL1 PLL configuration bits.
    #[inline(always)]
    pub fn pll1sec(&self) -> PLL1SEC_R {
        PLL1SEC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Defines the secure protection of the PLL2 PLL configuration bits.
    #[inline(always)]
    pub fn pll2sec(&self) -> PLL2SEC_R {
        PLL2SEC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Defines the secure protection of the PLL3 PLL configuration bits.
    #[inline(always)]
    pub fn pll3sec(&self) -> PLL3SEC_R {
        PLL3SEC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Defines the secure protection of the PLL4 PLL configuration bits.
    #[inline(always)]
    pub fn pll4sec(&self) -> PLL4SEC_R {
        PLL4SEC_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECCFGR1")
            .field("pll1sec", &self.pll1sec())
            .field("pll2sec", &self.pll2sec())
            .field("pll3sec", &self.pll3sec())
            .field("pll4sec", &self.pll4sec())
            .finish()
    }
}
impl W {
    ///Bit 0 - Defines the secure protection of the PLL1 PLL configuration bits.
    #[inline(always)]
    pub fn pll1sec(&mut self) -> PLL1SEC_W<'_, SECCFGR1rs> {
        PLL1SEC_W::new(self, 0)
    }
    ///Bit 1 - Defines the secure protection of the PLL2 PLL configuration bits.
    #[inline(always)]
    pub fn pll2sec(&mut self) -> PLL2SEC_W<'_, SECCFGR1rs> {
        PLL2SEC_W::new(self, 1)
    }
    ///Bit 2 - Defines the secure protection of the PLL3 PLL configuration bits.
    #[inline(always)]
    pub fn pll3sec(&mut self) -> PLL3SEC_W<'_, SECCFGR1rs> {
        PLL3SEC_W::new(self, 2)
    }
    ///Bit 3 - Defines the secure protection of the PLL4 PLL configuration bits.
    #[inline(always)]
    pub fn pll4sec(&mut self) -> PLL4SEC_W<'_, SECCFGR1rs> {
        PLL4SEC_W::new(self, 3)
    }
}
/**RCC PLL secure configuration register1

You can [`read`](crate::Reg::read) this register and get [`seccfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`seccfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:SECCFGR1)*/
pub struct SECCFGR1rs;
impl crate::RegisterSpec for SECCFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`seccfgr1::R`](R) reader structure
impl crate::Readable for SECCFGR1rs {}
///`write(|w| ..)` method takes [`seccfgr1::W`](W) writer structure
impl crate::Writable for SECCFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SECCFGR1 to value 0
impl crate::Resettable for SECCFGR1rs {}
