///Register `CFGR1` reader
pub type R = crate::R<CFGR1rs>;
///Register `CFGR1` writer
pub type W = crate::W<CFGR1rs>;
///Field `BOOSTEN` reader - I/O analog switch voltage booster enable Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table121 for setting.
pub type BOOSTEN_R = crate::BitReader;
///Field `BOOSTEN` writer - I/O analog switch voltage booster enable Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table121 for setting.
pub type BOOSTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANASWVDD` reader - GPIO analog switch control voltage selection Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table121 for setting.
pub type ANASWVDD_R = crate::BitReader;
///Field `ANASWVDD` writer - GPIO analog switch control voltage selection Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table121 for setting.
pub type ANASWVDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA6_FMP` reader - Fast-mode Plus drive capability activation on PA6 This bit can be read and written only with secure access if PA6 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA6 when PA6 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC6.
pub type PA6_FMP_R = crate::BitReader;
///Field `PA6_FMP` writer - Fast-mode Plus drive capability activation on PA6 This bit can be read and written only with secure access if PA6 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA6 when PA6 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC6.
pub type PA6_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA7_FMP` reader - Fast-mode Plus drive capability activation on PA7 This bit can be read and written only with secure access if PA7 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA7 when PA7 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC7.
pub type PA7_FMP_R = crate::BitReader;
///Field `PA7_FMP` writer - Fast-mode Plus drive capability activation on PA7 This bit can be read and written only with secure access if PA7 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA7 when PA7 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC7.
pub type PA7_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PA15_FMP` reader - Fast-mode Plus drive capability activation on PA15 This bit can be read and written only with secure access if PA15 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA15 when PA15 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC15.
pub type PA15_FMP_R = crate::BitReader;
///Field `PA15_FMP` writer - Fast-mode Plus drive capability activation on PA15 This bit can be read and written only with secure access if PA15 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA15 when PA15 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC15.
pub type PA15_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PB3_FMP` reader - Fast-mode Plus drive capability activation on PB3 This bit can be read and written only with secure access if PB3 is secure in GPIOB. This bit enables the Fast-mode Plus drive mode for PB3 when PB3 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOB SEC3.
pub type PB3_FMP_R = crate::BitReader;
///Field `PB3_FMP` writer - Fast-mode Plus drive capability activation on PB3 This bit can be read and written only with secure access if PB3 is secure in GPIOB. This bit enables the Fast-mode Plus drive mode for PB3 when PB3 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOB SEC3.
pub type PB3_FMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - I/O analog switch voltage booster enable Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table121 for setting.
    #[inline(always)]
    pub fn boosten(&self) -> BOOSTEN_R {
        BOOSTEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIO analog switch control voltage selection Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table121 for setting.
    #[inline(always)]
    pub fn anaswvdd(&self) -> ANASWVDD_R {
        ANASWVDD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 16 - Fast-mode Plus drive capability activation on PA6 This bit can be read and written only with secure access if PA6 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA6 when PA6 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC6.
    #[inline(always)]
    pub fn pa6_fmp(&self) -> PA6_FMP_R {
        PA6_FMP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Fast-mode Plus drive capability activation on PA7 This bit can be read and written only with secure access if PA7 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA7 when PA7 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC7.
    #[inline(always)]
    pub fn pa7_fmp(&self) -> PA7_FMP_R {
        PA7_FMP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Fast-mode Plus drive capability activation on PA15 This bit can be read and written only with secure access if PA15 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA15 when PA15 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC15.
    #[inline(always)]
    pub fn pa15_fmp(&self) -> PA15_FMP_R {
        PA15_FMP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Fast-mode Plus drive capability activation on PB3 This bit can be read and written only with secure access if PB3 is secure in GPIOB. This bit enables the Fast-mode Plus drive mode for PB3 when PB3 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOB SEC3.
    #[inline(always)]
    pub fn pb3_fmp(&self) -> PB3_FMP_R {
        PB3_FMP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFGR1")
            .field("boosten", &self.boosten())
            .field("anaswvdd", &self.anaswvdd())
            .field("pa6_fmp", &self.pa6_fmp())
            .field("pa7_fmp", &self.pa7_fmp())
            .field("pa15_fmp", &self.pa15_fmp())
            .field("pb3_fmp", &self.pb3_fmp())
            .finish()
    }
}
impl W {
    ///Bit 8 - I/O analog switch voltage booster enable Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table121 for setting.
    #[inline(always)]
    pub fn boosten(&mut self) -> BOOSTEN_W<'_, CFGR1rs> {
        BOOSTEN_W::new(self, 8)
    }
    ///Bit 9 - GPIO analog switch control voltage selection Access can be protected by GTZC_TZSC ADC4SEC. Note: Refer to Table121 for setting.
    #[inline(always)]
    pub fn anaswvdd(&mut self) -> ANASWVDD_W<'_, CFGR1rs> {
        ANASWVDD_W::new(self, 9)
    }
    ///Bit 16 - Fast-mode Plus drive capability activation on PA6 This bit can be read and written only with secure access if PA6 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA6 when PA6 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC6.
    #[inline(always)]
    pub fn pa6_fmp(&mut self) -> PA6_FMP_W<'_, CFGR1rs> {
        PA6_FMP_W::new(self, 16)
    }
    ///Bit 17 - Fast-mode Plus drive capability activation on PA7 This bit can be read and written only with secure access if PA7 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA7 when PA7 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC7.
    #[inline(always)]
    pub fn pa7_fmp(&mut self) -> PA7_FMP_W<'_, CFGR1rs> {
        PA7_FMP_W::new(self, 17)
    }
    ///Bit 18 - Fast-mode Plus drive capability activation on PA15 This bit can be read and written only with secure access if PA15 is secure in GPIOA. This bit enables the Fast-mode Plus drive mode for PA15 when PA15 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOA SEC15.
    #[inline(always)]
    pub fn pa15_fmp(&mut self) -> PA15_FMP_W<'_, CFGR1rs> {
        PA15_FMP_W::new(self, 18)
    }
    ///Bit 19 - Fast-mode Plus drive capability activation on PB3 This bit can be read and written only with secure access if PB3 is secure in GPIOB. This bit enables the Fast-mode Plus drive mode for PB3 when PB3 is not used by I2C peripheral. This can be used to dive a LED for instance. Access can be protected by GPIOB SEC3.
    #[inline(always)]
    pub fn pb3_fmp(&mut self) -> PB3_FMP_W<'_, CFGR1rs> {
        PB3_FMP_W::new(self, 19)
    }
}
/**SYSCFG configuration register 1

You can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WBA54.html#SYSCFG:CFGR1)*/
pub struct CFGR1rs;
impl crate::RegisterSpec for CFGR1rs {
    type Ux = u32;
}
///`read()` method returns [`cfgr1::R`](R) reader structure
impl crate::Readable for CFGR1rs {}
///`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure
impl crate::Writable for CFGR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CFGR1 to value 0
impl crate::Resettable for CFGR1rs {}
