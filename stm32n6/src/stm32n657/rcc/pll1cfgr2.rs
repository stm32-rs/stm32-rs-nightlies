///Register `PLL1CFGR2` reader
pub type R = crate::R<PLL1CFGR2rs>;
///Register `PLL1CFGR2` writer
pub type W = crate::W<PLL1CFGR2rs>;
///Field `PLL1DIVNFRAC` reader - PLL1 Fractional part of the VCO multiplication factor
pub type PLL1DIVNFRAC_R = crate::FieldReader<u32>;
///Field `PLL1DIVNFRAC` writer - PLL1 Fractional part of the VCO multiplication factor
pub type PLL1DIVNFRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - PLL1 Fractional part of the VCO multiplication factor
    #[inline(always)]
    pub fn pll1divnfrac(&self) -> PLL1DIVNFRAC_R {
        PLL1DIVNFRAC_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CFGR2")
            .field("pll1divnfrac", &self.pll1divnfrac())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - PLL1 Fractional part of the VCO multiplication factor
    #[inline(always)]
    pub fn pll1divnfrac(&mut self) -> PLL1DIVNFRAC_W<'_, PLL1CFGR2rs> {
        PLL1DIVNFRAC_W::new(self, 0)
    }
}
/**RCC PLL1 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PLL1CFGR2)*/
pub struct PLL1CFGR2rs;
impl crate::RegisterSpec for PLL1CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`pll1cfgr2::R`](R) reader structure
impl crate::Readable for PLL1CFGR2rs {}
///`write(|w| ..)` method takes [`pll1cfgr2::W`](W) writer structure
impl crate::Writable for PLL1CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1CFGR2 to value 0x0080_0000
impl crate::Resettable for PLL1CFGR2rs {
    const RESET_VALUE: u32 = 0x0080_0000;
}
