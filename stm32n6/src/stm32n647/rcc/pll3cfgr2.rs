///Register `PLL3CFGR2` reader
pub type R = crate::R<PLL3CFGR2rs>;
///Register `PLL3CFGR2` writer
pub type W = crate::W<PLL3CFGR2rs>;
///Field `PLL3DIVNFRAC` reader - PLL3 Fractional part of the VCO multiplication factor
pub type PLL3DIVNFRAC_R = crate::FieldReader<u32>;
///Field `PLL3DIVNFRAC` writer - PLL3 Fractional part of the VCO multiplication factor
pub type PLL3DIVNFRAC_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    ///Bits 0:23 - PLL3 Fractional part of the VCO multiplication factor
    #[inline(always)]
    pub fn pll3divnfrac(&self) -> PLL3DIVNFRAC_R {
        PLL3DIVNFRAC_R::new(self.bits & 0x00ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3CFGR2")
            .field("pll3divnfrac", &self.pll3divnfrac())
            .finish()
    }
}
impl W {
    ///Bits 0:23 - PLL3 Fractional part of the VCO multiplication factor
    #[inline(always)]
    pub fn pll3divnfrac(&mut self) -> PLL3DIVNFRAC_W<'_, PLL3CFGR2rs> {
        PLL3DIVNFRAC_W::new(self, 0)
    }
}
/**RCC PLL3 configuration register 2

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL3CFGR2)*/
pub struct PLL3CFGR2rs;
impl crate::RegisterSpec for PLL3CFGR2rs {
    type Ux = u32;
}
///`read()` method returns [`pll3cfgr2::R`](R) reader structure
impl crate::Readable for PLL3CFGR2rs {}
///`write(|w| ..)` method takes [`pll3cfgr2::W`](W) writer structure
impl crate::Writable for PLL3CFGR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3CFGR2 to value 0
impl crate::Resettable for PLL3CFGR2rs {}
