///Register `PLL3CFGR3` reader
pub type R = crate::R<PLL3CFGR3rs>;
///Register `PLL3CFGR3` writer
pub type W = crate::W<PLL3CFGR3rs>;
///Field `PLL3MODSSRST` reader - PLL3 Modulation Spread Spectrum reset
pub type PLL3MODSSRST_R = crate::BitReader;
///Field `PLL3MODSSRST` writer - PLL3 Modulation Spread Spectrum reset
pub type PLL3MODSSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3DACEN` reader - PLL3 noise canceling DAC enable in fractional mode.
pub type PLL3DACEN_R = crate::BitReader;
///Field `PLL3DACEN` writer - PLL3 noise canceling DAC enable in fractional mode.
pub type PLL3DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3MODSSDIS` reader - PLL3 Modulation Spread-Spectrum Disable
pub type PLL3MODSSDIS_R = crate::BitReader;
///Field `PLL3MODSSDIS` writer - PLL3 Modulation Spread-Spectrum Disable
pub type PLL3MODSSDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3MODDSEN` reader - PLL3 Modulation Spread-Spectrum (and Fractional Divide) enable
pub type PLL3MODDSEN_R = crate::BitReader;
///Field `PLL3MODDSEN` writer - PLL3 Modulation Spread-Spectrum (and Fractional Divide) enable
pub type PLL3MODDSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3MODSPRDW` reader - PLL3 Modulation Down Spread
pub type PLL3MODSPRDW_R = crate::BitReader;
///Field `PLL3MODSPRDW` writer - PLL3 Modulation Down Spread
pub type PLL3MODSPRDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL3MODDIV` reader - PLL3 Modulation Division frequency adjustment
pub type PLL3MODDIV_R = crate::FieldReader;
///Field `PLL3MODDIV` writer - PLL3 Modulation Division frequency adjustment
pub type PLL3MODDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLL3MODSPR` reader - PLL3 Modulation Spread depth adjustment
pub type PLL3MODSPR_R = crate::FieldReader;
///Field `PLL3MODSPR` writer - PLL3 Modulation Spread depth adjustment
pub type PLL3MODSPR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PLL3PDIV2` reader - PLL3 VCO frequency divider level 2
pub type PLL3PDIV2_R = crate::FieldReader;
///Field `PLL3PDIV2` writer - PLL3 VCO frequency divider level 2
pub type PLL3PDIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLL3PDIV1` reader - PLL3 VCO frequency divider level 1
pub type PLL3PDIV1_R = crate::FieldReader;
///Field `PLL3PDIV1` writer - PLL3 VCO frequency divider level 1
pub type PLL3PDIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLL3PDIVEN` reader - PLL3 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
pub type PLL3PDIVEN_R = crate::BitReader;
///Field `PLL3PDIVEN` writer - PLL3 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
pub type PLL3PDIVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PLL3 Modulation Spread Spectrum reset
    #[inline(always)]
    pub fn pll3modssrst(&self) -> PLL3MODSSRST_R {
        PLL3MODSSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL3 noise canceling DAC enable in fractional mode.
    #[inline(always)]
    pub fn pll3dacen(&self) -> PLL3DACEN_R {
        PLL3DACEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PLL3 Modulation Spread-Spectrum Disable
    #[inline(always)]
    pub fn pll3modssdis(&self) -> PLL3MODSSDIS_R {
        PLL3MODSSDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PLL3 Modulation Spread-Spectrum (and Fractional Divide) enable
    #[inline(always)]
    pub fn pll3moddsen(&self) -> PLL3MODDSEN_R {
        PLL3MODDSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL3 Modulation Down Spread
    #[inline(always)]
    pub fn pll3modsprdw(&self) -> PLL3MODSPRDW_R {
        PLL3MODSPRDW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - PLL3 Modulation Division frequency adjustment
    #[inline(always)]
    pub fn pll3moddiv(&self) -> PLL3MODDIV_R {
        PLL3MODDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:20 - PLL3 Modulation Spread depth adjustment
    #[inline(always)]
    pub fn pll3modspr(&self) -> PLL3MODSPR_R {
        PLL3MODSPR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:26 - PLL3 VCO frequency divider level 2
    #[inline(always)]
    pub fn pll3pdiv2(&self) -> PLL3PDIV2_R {
        PLL3PDIV2_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - PLL3 VCO frequency divider level 1
    #[inline(always)]
    pub fn pll3pdiv1(&self) -> PLL3PDIV1_R {
        PLL3PDIV1_R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bit 30 - PLL3 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
    #[inline(always)]
    pub fn pll3pdiven(&self) -> PLL3PDIVEN_R {
        PLL3PDIVEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL3CFGR3")
            .field("pll3modssrst", &self.pll3modssrst())
            .field("pll3dacen", &self.pll3dacen())
            .field("pll3modssdis", &self.pll3modssdis())
            .field("pll3moddsen", &self.pll3moddsen())
            .field("pll3modsprdw", &self.pll3modsprdw())
            .field("pll3moddiv", &self.pll3moddiv())
            .field("pll3modspr", &self.pll3modspr())
            .field("pll3pdiv2", &self.pll3pdiv2())
            .field("pll3pdiv1", &self.pll3pdiv1())
            .field("pll3pdiven", &self.pll3pdiven())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLL3 Modulation Spread Spectrum reset
    #[inline(always)]
    pub fn pll3modssrst(&mut self) -> PLL3MODSSRST_W<'_, PLL3CFGR3rs> {
        PLL3MODSSRST_W::new(self, 0)
    }
    ///Bit 1 - PLL3 noise canceling DAC enable in fractional mode.
    #[inline(always)]
    pub fn pll3dacen(&mut self) -> PLL3DACEN_W<'_, PLL3CFGR3rs> {
        PLL3DACEN_W::new(self, 1)
    }
    ///Bit 2 - PLL3 Modulation Spread-Spectrum Disable
    #[inline(always)]
    pub fn pll3modssdis(&mut self) -> PLL3MODSSDIS_W<'_, PLL3CFGR3rs> {
        PLL3MODSSDIS_W::new(self, 2)
    }
    ///Bit 3 - PLL3 Modulation Spread-Spectrum (and Fractional Divide) enable
    #[inline(always)]
    pub fn pll3moddsen(&mut self) -> PLL3MODDSEN_W<'_, PLL3CFGR3rs> {
        PLL3MODDSEN_W::new(self, 3)
    }
    ///Bit 4 - PLL3 Modulation Down Spread
    #[inline(always)]
    pub fn pll3modsprdw(&mut self) -> PLL3MODSPRDW_W<'_, PLL3CFGR3rs> {
        PLL3MODSPRDW_W::new(self, 4)
    }
    ///Bits 8:11 - PLL3 Modulation Division frequency adjustment
    #[inline(always)]
    pub fn pll3moddiv(&mut self) -> PLL3MODDIV_W<'_, PLL3CFGR3rs> {
        PLL3MODDIV_W::new(self, 8)
    }
    ///Bits 16:20 - PLL3 Modulation Spread depth adjustment
    #[inline(always)]
    pub fn pll3modspr(&mut self) -> PLL3MODSPR_W<'_, PLL3CFGR3rs> {
        PLL3MODSPR_W::new(self, 16)
    }
    ///Bits 24:26 - PLL3 VCO frequency divider level 2
    #[inline(always)]
    pub fn pll3pdiv2(&mut self) -> PLL3PDIV2_W<'_, PLL3CFGR3rs> {
        PLL3PDIV2_W::new(self, 24)
    }
    ///Bits 27:29 - PLL3 VCO frequency divider level 1
    #[inline(always)]
    pub fn pll3pdiv1(&mut self) -> PLL3PDIV1_W<'_, PLL3CFGR3rs> {
        PLL3PDIV1_W::new(self, 27)
    }
    ///Bit 30 - PLL3 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
    #[inline(always)]
    pub fn pll3pdiven(&mut self) -> PLL3PDIVEN_W<'_, PLL3CFGR3rs> {
        PLL3PDIVEN_W::new(self, 30)
    }
}
/**RCC PLL3 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`pll3cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL3CFGR3)*/
pub struct PLL3CFGR3rs;
impl crate::RegisterSpec for PLL3CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`pll3cfgr3::R`](R) reader structure
impl crate::Readable for PLL3CFGR3rs {}
///`write(|w| ..)` method takes [`pll3cfgr3::W`](W) writer structure
impl crate::Writable for PLL3CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL3CFGR3 to value 0x4900_0005
impl crate::Resettable for PLL3CFGR3rs {
    const RESET_VALUE: u32 = 0x4900_0005;
}
