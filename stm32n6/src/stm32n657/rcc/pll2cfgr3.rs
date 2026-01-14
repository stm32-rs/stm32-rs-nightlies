///Register `PLL2CFGR3` reader
pub type R = crate::R<PLL2CFGR3rs>;
///Register `PLL2CFGR3` writer
pub type W = crate::W<PLL2CFGR3rs>;
///Field `PLL2MODSSRST` reader - PLL2 Modulation Spread Spectrum reset
pub type PLL2MODSSRST_R = crate::BitReader;
///Field `PLL2MODSSRST` writer - PLL2 Modulation Spread Spectrum reset
pub type PLL2MODSSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2DACEN` reader - PLL2 noise canceling DAC enable in fractional mode.
pub type PLL2DACEN_R = crate::BitReader;
///Field `PLL2DACEN` writer - PLL2 noise canceling DAC enable in fractional mode.
pub type PLL2DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2MODSSDIS` reader - PLL2 Modulation Spread-Spectrum Disable
pub type PLL2MODSSDIS_R = crate::BitReader;
///Field `PLL2MODSSDIS` writer - PLL2 Modulation Spread-Spectrum Disable
pub type PLL2MODSSDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2MODDSEN` reader - PLL2 Modulation Spread-Spectrum (and Fractional Divide) enable
pub type PLL2MODDSEN_R = crate::BitReader;
///Field `PLL2MODDSEN` writer - PLL2 Modulation Spread-Spectrum (and Fractional Divide) enable
pub type PLL2MODDSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2MODSPRDW` reader - PLL2 Modulation Down Spread
pub type PLL2MODSPRDW_R = crate::BitReader;
///Field `PLL2MODSPRDW` writer - PLL2 Modulation Down Spread
pub type PLL2MODSPRDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL2MODDIV` reader - PLL2 Modulation Division frequency adjustment
pub type PLL2MODDIV_R = crate::FieldReader;
///Field `PLL2MODDIV` writer - PLL2 Modulation Division frequency adjustment
pub type PLL2MODDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLL2MODSPR` reader - PLL2 Modulation Spread depth adjustment
pub type PLL2MODSPR_R = crate::FieldReader;
///Field `PLL2MODSPR` writer - PLL2 Modulation Spread depth adjustment
pub type PLL2MODSPR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PLL2PDIV2` reader - PLL2 VCO frequency divider level 2
pub type PLL2PDIV2_R = crate::FieldReader;
///Field `PLL2PDIV2` writer - PLL2 VCO frequency divider level 2
pub type PLL2PDIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLL2PDIV1` reader - PLL2 VCO frequency divider level 1
pub type PLL2PDIV1_R = crate::FieldReader;
///Field `PLL2PDIV1` writer - PLL2 VCO frequency divider level 1
pub type PLL2PDIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLL2PDIVEN` reader - PLL2 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
pub type PLL2PDIVEN_R = crate::BitReader;
///Field `PLL2PDIVEN` writer - PLL2 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
pub type PLL2PDIVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PLL2 Modulation Spread Spectrum reset
    #[inline(always)]
    pub fn pll2modssrst(&self) -> PLL2MODSSRST_R {
        PLL2MODSSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL2 noise canceling DAC enable in fractional mode.
    #[inline(always)]
    pub fn pll2dacen(&self) -> PLL2DACEN_R {
        PLL2DACEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PLL2 Modulation Spread-Spectrum Disable
    #[inline(always)]
    pub fn pll2modssdis(&self) -> PLL2MODSSDIS_R {
        PLL2MODSSDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PLL2 Modulation Spread-Spectrum (and Fractional Divide) enable
    #[inline(always)]
    pub fn pll2moddsen(&self) -> PLL2MODDSEN_R {
        PLL2MODDSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL2 Modulation Down Spread
    #[inline(always)]
    pub fn pll2modsprdw(&self) -> PLL2MODSPRDW_R {
        PLL2MODSPRDW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - PLL2 Modulation Division frequency adjustment
    #[inline(always)]
    pub fn pll2moddiv(&self) -> PLL2MODDIV_R {
        PLL2MODDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:20 - PLL2 Modulation Spread depth adjustment
    #[inline(always)]
    pub fn pll2modspr(&self) -> PLL2MODSPR_R {
        PLL2MODSPR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:26 - PLL2 VCO frequency divider level 2
    #[inline(always)]
    pub fn pll2pdiv2(&self) -> PLL2PDIV2_R {
        PLL2PDIV2_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - PLL2 VCO frequency divider level 1
    #[inline(always)]
    pub fn pll2pdiv1(&self) -> PLL2PDIV1_R {
        PLL2PDIV1_R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bit 30 - PLL2 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
    #[inline(always)]
    pub fn pll2pdiven(&self) -> PLL2PDIVEN_R {
        PLL2PDIVEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL2CFGR3")
            .field("pll2modssrst", &self.pll2modssrst())
            .field("pll2dacen", &self.pll2dacen())
            .field("pll2modssdis", &self.pll2modssdis())
            .field("pll2moddsen", &self.pll2moddsen())
            .field("pll2modsprdw", &self.pll2modsprdw())
            .field("pll2moddiv", &self.pll2moddiv())
            .field("pll2modspr", &self.pll2modspr())
            .field("pll2pdiv2", &self.pll2pdiv2())
            .field("pll2pdiv1", &self.pll2pdiv1())
            .field("pll2pdiven", &self.pll2pdiven())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLL2 Modulation Spread Spectrum reset
    #[inline(always)]
    pub fn pll2modssrst(&mut self) -> PLL2MODSSRST_W<'_, PLL2CFGR3rs> {
        PLL2MODSSRST_W::new(self, 0)
    }
    ///Bit 1 - PLL2 noise canceling DAC enable in fractional mode.
    #[inline(always)]
    pub fn pll2dacen(&mut self) -> PLL2DACEN_W<'_, PLL2CFGR3rs> {
        PLL2DACEN_W::new(self, 1)
    }
    ///Bit 2 - PLL2 Modulation Spread-Spectrum Disable
    #[inline(always)]
    pub fn pll2modssdis(&mut self) -> PLL2MODSSDIS_W<'_, PLL2CFGR3rs> {
        PLL2MODSSDIS_W::new(self, 2)
    }
    ///Bit 3 - PLL2 Modulation Spread-Spectrum (and Fractional Divide) enable
    #[inline(always)]
    pub fn pll2moddsen(&mut self) -> PLL2MODDSEN_W<'_, PLL2CFGR3rs> {
        PLL2MODDSEN_W::new(self, 3)
    }
    ///Bit 4 - PLL2 Modulation Down Spread
    #[inline(always)]
    pub fn pll2modsprdw(&mut self) -> PLL2MODSPRDW_W<'_, PLL2CFGR3rs> {
        PLL2MODSPRDW_W::new(self, 4)
    }
    ///Bits 8:11 - PLL2 Modulation Division frequency adjustment
    #[inline(always)]
    pub fn pll2moddiv(&mut self) -> PLL2MODDIV_W<'_, PLL2CFGR3rs> {
        PLL2MODDIV_W::new(self, 8)
    }
    ///Bits 16:20 - PLL2 Modulation Spread depth adjustment
    #[inline(always)]
    pub fn pll2modspr(&mut self) -> PLL2MODSPR_W<'_, PLL2CFGR3rs> {
        PLL2MODSPR_W::new(self, 16)
    }
    ///Bits 24:26 - PLL2 VCO frequency divider level 2
    #[inline(always)]
    pub fn pll2pdiv2(&mut self) -> PLL2PDIV2_W<'_, PLL2CFGR3rs> {
        PLL2PDIV2_W::new(self, 24)
    }
    ///Bits 27:29 - PLL2 VCO frequency divider level 1
    #[inline(always)]
    pub fn pll2pdiv1(&mut self) -> PLL2PDIV1_W<'_, PLL2CFGR3rs> {
        PLL2PDIV1_W::new(self, 27)
    }
    ///Bit 30 - PLL2 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
    #[inline(always)]
    pub fn pll2pdiven(&mut self) -> PLL2PDIVEN_W<'_, PLL2CFGR3rs> {
        PLL2PDIVEN_W::new(self, 30)
    }
}
/**RCC PLL2 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`pll2cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:PLL2CFGR3)*/
pub struct PLL2CFGR3rs;
impl crate::RegisterSpec for PLL2CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`pll2cfgr3::R`](R) reader structure
impl crate::Readable for PLL2CFGR3rs {}
///`write(|w| ..)` method takes [`pll2cfgr3::W`](W) writer structure
impl crate::Writable for PLL2CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL2CFGR3 to value 0x4900_0005
impl crate::Resettable for PLL2CFGR3rs {
    const RESET_VALUE: u32 = 0x4900_0005;
}
