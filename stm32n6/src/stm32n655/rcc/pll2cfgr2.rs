///Register `PLL2CFGR2` reader
pub type R = crate::R<PLL2CFGR2rs>;
///Register `PLL2CFGR2` writer
pub type W = crate::W<PLL2CFGR2rs>;
///Field `PLL2DIVNFRAC` reader - PLL2 Fractional part of the VCO multiplication factor
pub type PLL2DIVNFRAC_R = crate::FieldReader<u32>;
///Field `PLL2DIVNFRAC` writer - PLL2 Fractional part of the VCO multiplication factor
pub type PLL2DIVNFRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - PLL2 Fractional part of the VCO multiplication factor
    #[inline(always)]
    pub fn pll2divnfrac(&self) -> PLL2DIVNFRAC_R {
        PLL2DIVNFRAC_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2CFGR2")
            .field("pll2divnfrac", &self.pll2divnfrac())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - PLL2 Fractional part of the VCO multiplication factor
    #[inline(always)]
    pub fn pll2divnfrac(&mut self) -> PLL2DIVNFRAC_W<'_, PLL2CFGR2rs> {
        PLL2DIVNFRAC_W::new(self, 0)
    }
}
/**RCC PLL2 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:PLL2CFGR2)*/
pub struct PLL2CFGR2rs;
impl crate::RegisterSpec for PLL2CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`pll2cfgr2::R`](R) reader structure
impl crate::Readable for PLL2CFGR2rs {}
///`write(|w| ..)` method takes [`pll2cfgr2::W`](W) writer structure
impl crate::Writable for PLL2CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2CFGR2 to value 0
impl crate::Resettable for PLL2CFGR2rs {}
