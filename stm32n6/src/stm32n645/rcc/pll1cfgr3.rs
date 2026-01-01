///Register `PLL1CFGR3` reader
pub type R = crate::R<PLL1CFGR3rs>;
///Register `PLL1CFGR3` writer
pub type W = crate::W<PLL1CFGR3rs>;
///Field `PLL1MODSSRST` reader - PLL1 Modulation Spread Spectrum reset
pub type PLL1MODSSRST_R = crate::BitReader;
///Field `PLL1MODSSRST` writer - PLL1 Modulation Spread Spectrum reset
pub type PLL1MODSSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1DACEN` reader - PLL1 noise canceling DAC enable in fractional mode.
pub type PLL1DACEN_R = crate::BitReader;
///Field `PLL1DACEN` writer - PLL1 noise canceling DAC enable in fractional mode.
pub type PLL1DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1MODSSDIS` reader - PLL1 Modulation Spread-Spectrum Disable
pub type PLL1MODSSDIS_R = crate::BitReader;
///Field `PLL1MODSSDIS` writer - PLL1 Modulation Spread-Spectrum Disable
pub type PLL1MODSSDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1MODDSEN` reader - PLL1 Modulation Spread-Spectrum (and Fractional Divide) enable
pub type PLL1MODDSEN_R = crate::BitReader;
///Field `PLL1MODDSEN` writer - PLL1 Modulation Spread-Spectrum (and Fractional Divide) enable
pub type PLL1MODDSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1MODSPRDW` reader - PLL1 Modulation Spread-Spectrum Down
pub type PLL1MODSPRDW_R = crate::BitReader;
///Field `PLL1MODSPRDW` writer - PLL1 Modulation Spread-Spectrum Down
pub type PLL1MODSPRDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL1MODDIV` reader - PLL1 Modulation Division frequency adjustment
pub type PLL1MODDIV_R = crate::FieldReader;
///Field `PLL1MODDIV` writer - PLL1 Modulation Division frequency adjustment
pub type PLL1MODDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLL1MODSPR` reader - PLL1 Modulation Spread depth adjustment
pub type PLL1MODSPR_R = crate::FieldReader;
///Field `PLL1MODSPR` writer - PLL1 Modulation Spread depth adjustment
pub type PLL1MODSPR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PLL1PDIV2` reader - PLL1 VCO frequency divider level 2
pub type PLL1PDIV2_R = crate::FieldReader;
///Field `PLL1PDIV2` writer - PLL1 VCO frequency divider level 2
pub type PLL1PDIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLL1PDIV1` reader - PLL1 VCO frequency divider level 1
pub type PLL1PDIV1_R = crate::FieldReader;
///Field `PLL1PDIV1` writer - PLL1 VCO frequency divider level 1
pub type PLL1PDIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLL1PDIVEN` reader - PLL1 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
pub type PLL1PDIVEN_R = crate::BitReader;
///Field `PLL1PDIVEN` writer - PLL1 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
pub type PLL1PDIVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PLL1 Modulation Spread Spectrum reset
    #[inline(always)]
    pub fn pll1modssrst(&self) -> PLL1MODSSRST_R {
        PLL1MODSSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL1 noise canceling DAC enable in fractional mode.
    #[inline(always)]
    pub fn pll1dacen(&self) -> PLL1DACEN_R {
        PLL1DACEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PLL1 Modulation Spread-Spectrum Disable
    #[inline(always)]
    pub fn pll1modssdis(&self) -> PLL1MODSSDIS_R {
        PLL1MODSSDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PLL1 Modulation Spread-Spectrum (and Fractional Divide) enable
    #[inline(always)]
    pub fn pll1moddsen(&self) -> PLL1MODDSEN_R {
        PLL1MODDSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL1 Modulation Spread-Spectrum Down
    #[inline(always)]
    pub fn pll1modsprdw(&self) -> PLL1MODSPRDW_R {
        PLL1MODSPRDW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - PLL1 Modulation Division frequency adjustment
    #[inline(always)]
    pub fn pll1moddiv(&self) -> PLL1MODDIV_R {
        PLL1MODDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:20 - PLL1 Modulation Spread depth adjustment
    #[inline(always)]
    pub fn pll1modspr(&self) -> PLL1MODSPR_R {
        PLL1MODSPR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:26 - PLL1 VCO frequency divider level 2
    #[inline(always)]
    pub fn pll1pdiv2(&self) -> PLL1PDIV2_R {
        PLL1PDIV2_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - PLL1 VCO frequency divider level 1
    #[inline(always)]
    pub fn pll1pdiv1(&self) -> PLL1PDIV1_R {
        PLL1PDIV1_R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bit 30 - PLL1 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
    #[inline(always)]
    pub fn pll1pdiven(&self) -> PLL1PDIVEN_R {
        PLL1PDIVEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL1CFGR3")
            .field("pll1modssrst", &self.pll1modssrst())
            .field("pll1dacen", &self.pll1dacen())
            .field("pll1modssdis", &self.pll1modssdis())
            .field("pll1moddsen", &self.pll1moddsen())
            .field("pll1modsprdw", &self.pll1modsprdw())
            .field("pll1moddiv", &self.pll1moddiv())
            .field("pll1modspr", &self.pll1modspr())
            .field("pll1pdiv2", &self.pll1pdiv2())
            .field("pll1pdiv1", &self.pll1pdiv1())
            .field("pll1pdiven", &self.pll1pdiven())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLL1 Modulation Spread Spectrum reset
    #[inline(always)]
    pub fn pll1modssrst(&mut self) -> PLL1MODSSRST_W<'_, PLL1CFGR3rs> {
        PLL1MODSSRST_W::new(self, 0)
    }
    ///Bit 1 - PLL1 noise canceling DAC enable in fractional mode.
    #[inline(always)]
    pub fn pll1dacen(&mut self) -> PLL1DACEN_W<'_, PLL1CFGR3rs> {
        PLL1DACEN_W::new(self, 1)
    }
    ///Bit 2 - PLL1 Modulation Spread-Spectrum Disable
    #[inline(always)]
    pub fn pll1modssdis(&mut self) -> PLL1MODSSDIS_W<'_, PLL1CFGR3rs> {
        PLL1MODSSDIS_W::new(self, 2)
    }
    ///Bit 3 - PLL1 Modulation Spread-Spectrum (and Fractional Divide) enable
    #[inline(always)]
    pub fn pll1moddsen(&mut self) -> PLL1MODDSEN_W<'_, PLL1CFGR3rs> {
        PLL1MODDSEN_W::new(self, 3)
    }
    ///Bit 4 - PLL1 Modulation Spread-Spectrum Down
    #[inline(always)]
    pub fn pll1modsprdw(&mut self) -> PLL1MODSPRDW_W<'_, PLL1CFGR3rs> {
        PLL1MODSPRDW_W::new(self, 4)
    }
    ///Bits 8:11 - PLL1 Modulation Division frequency adjustment
    #[inline(always)]
    pub fn pll1moddiv(&mut self) -> PLL1MODDIV_W<'_, PLL1CFGR3rs> {
        PLL1MODDIV_W::new(self, 8)
    }
    ///Bits 16:20 - PLL1 Modulation Spread depth adjustment
    #[inline(always)]
    pub fn pll1modspr(&mut self) -> PLL1MODSPR_W<'_, PLL1CFGR3rs> {
        PLL1MODSPR_W::new(self, 16)
    }
    ///Bits 24:26 - PLL1 VCO frequency divider level 2
    #[inline(always)]
    pub fn pll1pdiv2(&mut self) -> PLL1PDIV2_W<'_, PLL1CFGR3rs> {
        PLL1PDIV2_W::new(self, 24)
    }
    ///Bits 27:29 - PLL1 VCO frequency divider level 1
    #[inline(always)]
    pub fn pll1pdiv1(&mut self) -> PLL1PDIV1_W<'_, PLL1CFGR3rs> {
        PLL1PDIV1_W::new(self, 27)
    }
    ///Bit 30 - PLL1 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
    #[inline(always)]
    pub fn pll1pdiven(&mut self) -> PLL1PDIVEN_W<'_, PLL1CFGR3rs> {
        PLL1PDIVEN_W::new(self, 30)
    }
}
/**RCC PLL1 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`pll1cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:PLL1CFGR3)*/
pub struct PLL1CFGR3rs;
impl crate::RegisterSpec for PLL1CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`pll1cfgr3::R`](R) reader structure
impl crate::Readable for PLL1CFGR3rs {}
///`write(|w| ..)` method takes [`pll1cfgr3::W`](W) writer structure
impl crate::Writable for PLL1CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL1CFGR3 to value 0x4900_000d
impl crate::Resettable for PLL1CFGR3rs {
    const RESET_VALUE: u32 = 0x4900_000d;
}
