///Register `PLL4CFGR2` reader
pub type R = crate::R<PLL4CFGR2rs>;
///Register `PLL4CFGR2` writer
pub type W = crate::W<PLL4CFGR2rs>;
///Field `PLL4DIVNFRAC` reader - PLL4 Fractional part of the VCO multiplication factor
pub type PLL4DIVNFRAC_R = crate::FieldReader<u32>;
///Field `PLL4DIVNFRAC` writer - PLL4 Fractional part of the VCO multiplication factor
pub type PLL4DIVNFRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - PLL4 Fractional part of the VCO multiplication factor
    #[inline(always)]
    pub fn pll4divnfrac(&self) -> PLL4DIVNFRAC_R {
        PLL4DIVNFRAC_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL4CFGR2")
            .field("pll4divnfrac", &self.pll4divnfrac())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - PLL4 Fractional part of the VCO multiplication factor
    #[inline(always)]
    pub fn pll4divnfrac(&mut self) -> PLL4DIVNFRAC_W<'_, PLL4CFGR2rs> {
        PLL4DIVNFRAC_W::new(self, 0)
    }
}
/**RCC PLL4 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll4cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PLL4CFGR2)*/
pub struct PLL4CFGR2rs;
impl crate::RegisterSpec for PLL4CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`pll4cfgr2::R`](R) reader structure
impl crate::Readable for PLL4CFGR2rs {}
///`write(|w| ..)` method takes [`pll4cfgr2::W`](W) writer structure
impl crate::Writable for PLL4CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL4CFGR2 to value 0
impl crate::Resettable for PLL4CFGR2rs {}
