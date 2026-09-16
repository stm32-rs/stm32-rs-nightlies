///Register `PLL4CFGR3` reader
pub type R = crate::R<PLL4CFGR3rs>;
///Register `PLL4CFGR3` writer
pub type W = crate::W<PLL4CFGR3rs>;
///Field `PLL4MODSSRST` reader - PLL4 Modulation Spread Spectrum reset
pub type PLL4MODSSRST_R = crate::BitReader;
///Field `PLL4MODSSRST` writer - PLL4 Modulation Spread Spectrum reset
pub type PLL4MODSSRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4DACEN` reader - PLL4 noise canceling DAC enable in fractional mode.
pub type PLL4DACEN_R = crate::BitReader;
///Field `PLL4DACEN` writer - PLL4 noise canceling DAC enable in fractional mode.
pub type PLL4DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4MODSSDIS` reader - PLL4 Modulation Spread-Spectrum Disable
pub type PLL4MODSSDIS_R = crate::BitReader;
///Field `PLL4MODSSDIS` writer - PLL4 Modulation Spread-Spectrum Disable
pub type PLL4MODSSDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4MODDSEN` reader - PLL4 Modulation Spread-Spectrum (and Fractional Divide) enable
pub type PLL4MODDSEN_R = crate::BitReader;
///Field `PLL4MODDSEN` writer - PLL4 Modulation Spread-Spectrum (and Fractional Divide) enable
pub type PLL4MODDSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4MODSPRDW` reader - PLL4 Modulation Down Spread
pub type PLL4MODSPRDW_R = crate::BitReader;
///Field `PLL4MODSPRDW` writer - PLL4 Modulation Down Spread
pub type PLL4MODSPRDW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PLL4MODDIV` reader - PLL4 Modulation Division frequency adjustment
pub type PLL4MODDIV_R = crate::FieldReader;
///Field `PLL4MODDIV` writer - PLL4 Modulation Division frequency adjustment
pub type PLL4MODDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `PLL4MODSPR` reader - PLL4 Modulation Spread depth adjustment
pub type PLL4MODSPR_R = crate::FieldReader;
///Field `PLL4MODSPR` writer - PLL4 Modulation Spread depth adjustment
pub type PLL4MODSPR_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `PLL4PDIV2` reader - PLL4 VCO frequency divider level 2
pub type PLL4PDIV2_R = crate::FieldReader;
///Field `PLL4PDIV2` writer - PLL4 VCO frequency divider level 2
pub type PLL4PDIV2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLL4PDIV1` reader - PLL4 VCO frequency divider level 1
pub type PLL4PDIV1_R = crate::FieldReader;
///Field `PLL4PDIV1` writer - PLL4 VCO frequency divider level 1
pub type PLL4PDIV1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `PLL4PDIVEN` reader - PLL4 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
pub type PLL4PDIVEN_R = crate::BitReader;
///Field `PLL4PDIVEN` writer - PLL4 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
pub type PLL4PDIVEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PLL4 Modulation Spread Spectrum reset
    #[inline(always)]
    pub fn pll4modssrst(&self) -> PLL4MODSSRST_R {
        PLL4MODSSRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - PLL4 noise canceling DAC enable in fractional mode.
    #[inline(always)]
    pub fn pll4dacen(&self) -> PLL4DACEN_R {
        PLL4DACEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - PLL4 Modulation Spread-Spectrum Disable
    #[inline(always)]
    pub fn pll4modssdis(&self) -> PLL4MODSSDIS_R {
        PLL4MODSSDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - PLL4 Modulation Spread-Spectrum (and Fractional Divide) enable
    #[inline(always)]
    pub fn pll4moddsen(&self) -> PLL4MODDSEN_R {
        PLL4MODDSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - PLL4 Modulation Down Spread
    #[inline(always)]
    pub fn pll4modsprdw(&self) -> PLL4MODSPRDW_R {
        PLL4MODSPRDW_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 8:11 - PLL4 Modulation Division frequency adjustment
    #[inline(always)]
    pub fn pll4moddiv(&self) -> PLL4MODDIV_R {
        PLL4MODDIV_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:20 - PLL4 Modulation Spread depth adjustment
    #[inline(always)]
    pub fn pll4modspr(&self) -> PLL4MODSPR_R {
        PLL4MODSPR_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    ///Bits 24:26 - PLL4 VCO frequency divider level 2
    #[inline(always)]
    pub fn pll4pdiv2(&self) -> PLL4PDIV2_R {
        PLL4PDIV2_R::new(((self.bits >> 24) & 7) as u8)
    }
    ///Bits 27:29 - PLL4 VCO frequency divider level 1
    #[inline(always)]
    pub fn pll4pdiv1(&self) -> PLL4PDIV1_R {
        PLL4PDIV1_R::new(((self.bits >> 27) & 7) as u8)
    }
    ///Bit 30 - PLL4 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
    #[inline(always)]
    pub fn pll4pdiven(&self) -> PLL4PDIVEN_R {
        PLL4PDIVEN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PLL4CFGR3")
            .field("pll4modssrst", &self.pll4modssrst())
            .field("pll4dacen", &self.pll4dacen())
            .field("pll4modssdis", &self.pll4modssdis())
            .field("pll4moddsen", &self.pll4moddsen())
            .field("pll4modsprdw", &self.pll4modsprdw())
            .field("pll4moddiv", &self.pll4moddiv())
            .field("pll4modspr", &self.pll4modspr())
            .field("pll4pdiv2", &self.pll4pdiv2())
            .field("pll4pdiv1", &self.pll4pdiv1())
            .field("pll4pdiven", &self.pll4pdiven())
            .finish()
    }
}
impl W {
    ///Bit 0 - PLL4 Modulation Spread Spectrum reset
    #[inline(always)]
    pub fn pll4modssrst(&mut self) -> PLL4MODSSRST_W<'_, PLL4CFGR3rs> {
        PLL4MODSSRST_W::new(self, 0)
    }
    ///Bit 1 - PLL4 noise canceling DAC enable in fractional mode.
    #[inline(always)]
    pub fn pll4dacen(&mut self) -> PLL4DACEN_W<'_, PLL4CFGR3rs> {
        PLL4DACEN_W::new(self, 1)
    }
    ///Bit 2 - PLL4 Modulation Spread-Spectrum Disable
    #[inline(always)]
    pub fn pll4modssdis(&mut self) -> PLL4MODSSDIS_W<'_, PLL4CFGR3rs> {
        PLL4MODSSDIS_W::new(self, 2)
    }
    ///Bit 3 - PLL4 Modulation Spread-Spectrum (and Fractional Divide) enable
    #[inline(always)]
    pub fn pll4moddsen(&mut self) -> PLL4MODDSEN_W<'_, PLL4CFGR3rs> {
        PLL4MODDSEN_W::new(self, 3)
    }
    ///Bit 4 - PLL4 Modulation Down Spread
    #[inline(always)]
    pub fn pll4modsprdw(&mut self) -> PLL4MODSPRDW_W<'_, PLL4CFGR3rs> {
        PLL4MODSPRDW_W::new(self, 4)
    }
    ///Bits 8:11 - PLL4 Modulation Division frequency adjustment
    #[inline(always)]
    pub fn pll4moddiv(&mut self) -> PLL4MODDIV_W<'_, PLL4CFGR3rs> {
        PLL4MODDIV_W::new(self, 8)
    }
    ///Bits 16:20 - PLL4 Modulation Spread depth adjustment
    #[inline(always)]
    pub fn pll4modspr(&mut self) -> PLL4MODSPR_W<'_, PLL4CFGR3rs> {
        PLL4MODSPR_W::new(self, 16)
    }
    ///Bits 24:26 - PLL4 VCO frequency divider level 2
    #[inline(always)]
    pub fn pll4pdiv2(&mut self) -> PLL4PDIV2_W<'_, PLL4CFGR3rs> {
        PLL4PDIV2_W::new(self, 24)
    }
    ///Bits 27:29 - PLL4 VCO frequency divider level 1
    #[inline(always)]
    pub fn pll4pdiv1(&mut self) -> PLL4PDIV1_W<'_, PLL4CFGR3rs> {
        PLL4PDIV1_W::new(self, 27)
    }
    ///Bit 30 - PLL4 post divider POSTDIV1, POSTDIV2, and PLL clock output enable
    #[inline(always)]
    pub fn pll4pdiven(&mut self) -> PLL4PDIVEN_W<'_, PLL4CFGR3rs> {
        PLL4PDIVEN_W::new(self, 30)
    }
}
/**RCC PLL4 configuration register 3

You can [`read`](crate::Reg::read) this register and get [`pll4cfgr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll4cfgr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:PLL4CFGR3)*/
pub struct PLL4CFGR3rs;
impl crate::RegisterSpec for PLL4CFGR3rs {
    type Ux = u32;
}
///`read()` method returns [`pll4cfgr3::R`](R) reader structure
impl crate::Readable for PLL4CFGR3rs {}
///`write(|w| ..)` method takes [`pll4cfgr3::W`](W) writer structure
impl crate::Writable for PLL4CFGR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PLL4CFGR3 to value 0x4900_0005
impl crate::Resettable for PLL4CFGR3rs {
    const RESET_VALUE: u32 = 0x4900_0005;
}
