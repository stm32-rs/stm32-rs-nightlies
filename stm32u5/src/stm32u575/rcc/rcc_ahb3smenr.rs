///Register `RCC_AHB3SMENR` reader
pub type R = crate::R<RCC_AHB3SMENRrs>;
///Register `RCC_AHB3SMENR` writer
pub type W = crate::W<RCC_AHB3SMENRrs>;
///Field `LPGPIO1SMEN` reader - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software.
pub type LPGPIO1SMEN_R = crate::BitReader;
///Field `LPGPIO1SMEN` writer - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software.
pub type LPGPIO1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRSMEN` reader - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type PWRSMEN_R = crate::BitReader;
///Field `PWRSMEN` writer - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type PWRSMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADC4SMEN` reader - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADC4SMEN_R = crate::BitReader;
///Field `ADC4SMEN` writer - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADC4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DAC1SMEN` reader - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type DAC1SMEN_R = crate::BitReader;
///Field `DAC1SMEN` writer - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type DAC1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPDMA1SMEN` reader - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPDMA1SMEN_R = crate::BitReader;
///Field `LPDMA1SMEN` writer - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type LPDMA1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADF1SMEN` reader - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADF1SMEN_R = crate::BitReader;
///Field `ADF1SMEN` writer - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
pub type ADF1SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GTZC2SMEN` reader - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type GTZC2SMEN_R = crate::BitReader;
///Field `GTZC2SMEN` writer - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type GTZC2SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAM4SMEN` reader - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type SRAM4SMEN_R = crate::BitReader;
///Field `SRAM4SMEN` writer - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
pub type SRAM4SMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn lpgpio1smen(&self) -> LPGPIO1SMEN_R {
        LPGPIO1SMEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn pwrsmen(&self) -> PWRSMEN_R {
        PWRSMEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adc4smen(&self) -> ADC4SMEN_R {
        ADC4SMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn dac1smen(&self) -> DAC1SMEN_R {
        DAC1SMEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn lpdma1smen(&self) -> LPDMA1SMEN_R {
        LPDMA1SMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    pub fn adf1smen(&self) -> ADF1SMEN_R {
        ADF1SMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn gtzc2smen(&self) -> GTZC2SMEN_R {
        GTZC2SMEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 31 - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    pub fn sram4smen(&self) -> SRAM4SMEN_R {
        SRAM4SMEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_AHB3SMENR")
            .field("lpgpio1smen", &self.lpgpio1smen())
            .field("pwrsmen", &self.pwrsmen())
            .field("adc4smen", &self.adc4smen())
            .field("dac1smen", &self.dac1smen())
            .field("lpdma1smen", &self.lpdma1smen())
            .field("adf1smen", &self.adf1smen())
            .field("gtzc2smen", &self.gtzc2smen())
            .field("sram4smen", &self.sram4smen())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPGPIO1 enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn lpgpio1smen(&mut self) -> LPGPIO1SMEN_W<RCC_AHB3SMENRrs> {
        LPGPIO1SMEN_W::new(self, 0)
    }
    ///Bit 2 - PWR clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn pwrsmen(&mut self) -> PWRSMEN_W<RCC_AHB3SMENRrs> {
        PWRSMEN_W::new(self, 2)
    }
    ///Bit 5 - ADC4 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn adc4smen(&mut self) -> ADC4SMEN_W<RCC_AHB3SMENRrs> {
        ADC4SMEN_W::new(self, 5)
    }
    ///Bit 6 - DAC1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn dac1smen(&mut self) -> DAC1SMEN_W<RCC_AHB3SMENRrs> {
        DAC1SMEN_W::new(self, 6)
    }
    ///Bit 9 - LPDMA1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn lpdma1smen(&mut self) -> LPDMA1SMEN_W<RCC_AHB3SMENRrs> {
        LPDMA1SMEN_W::new(self, 9)
    }
    ///Bit 10 - ADF1 clock enable during Sleep and Stop modes This bit is set and cleared by software. Note: This bit must be set to allow the peripheral to wake up from Stop modes.
    #[inline(always)]
    #[must_use]
    pub fn adf1smen(&mut self) -> ADF1SMEN_W<RCC_AHB3SMENRrs> {
        ADF1SMEN_W::new(self, 10)
    }
    ///Bit 12 - GTZC2 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn gtzc2smen(&mut self) -> GTZC2SMEN_W<RCC_AHB3SMENRrs> {
        GTZC2SMEN_W::new(self, 12)
    }
    ///Bit 31 - SRAM4 clock enable during Sleep and Stop modes This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn sram4smen(&mut self) -> SRAM4SMEN_W<RCC_AHB3SMENRrs> {
        SRAM4SMEN_W::new(self, 31)
    }
}
/**RCC AHB3 peripheral clock enable in Sleep and Stop modes register

You can [`read`](crate::Reg::read) this register and get [`rcc_ahb3smenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_ahb3smenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#RCC:RCC_AHB3SMENR)*/
pub struct RCC_AHB3SMENRrs;
impl crate::RegisterSpec for RCC_AHB3SMENRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_ahb3smenr::R`](R) reader structure
impl crate::Readable for RCC_AHB3SMENRrs {}
///`write(|w| ..)` method takes [`rcc_ahb3smenr::W`](W) writer structure
impl crate::Writable for RCC_AHB3SMENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_AHB3SMENR to value 0xffff_ffff
impl crate::Resettable for RCC_AHB3SMENRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
