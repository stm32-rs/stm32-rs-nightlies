#[doc = "Register `SRDAMR` reader"]
pub type R = crate::R<SRDAMRrs>;
#[doc = "Register `SRDAMR` writer"]
pub type W = crate::W<SRDAMRrs>;
#[doc = "SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BDMA2AMEN {
    #[doc = "0: Clock disabled in autonomous mode"]
    Disabled = 0,
    #[doc = "1: Clock enabled in autonomous mode"]
    Enabled = 1,
}
impl From<BDMA2AMEN> for bool {
    #[inline(always)]
    fn from(variant: BDMA2AMEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BDMA2AMEN` reader - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type BDMA2AMEN_R = crate::BitReader<BDMA2AMEN>;
impl BDMA2AMEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BDMA2AMEN {
        match self.bits {
            false => BDMA2AMEN::Disabled,
            true => BDMA2AMEN::Enabled,
        }
    }
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BDMA2AMEN::Disabled
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BDMA2AMEN::Enabled
    }
}
#[doc = "Field `BDMA2AMEN` writer - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub type BDMA2AMEN_W<'a, REG> = crate::BitWriter<'a, REG, BDMA2AMEN>;
impl<'a, REG> BDMA2AMEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled in autonomous mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BDMA2AMEN::Disabled)
    }
    #[doc = "Clock enabled in autonomous mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BDMA2AMEN::Enabled)
    }
}
#[doc = "Field `GPIOAMEN` reader - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as GPIOAMEN_R;
#[doc = "Field `LPUART1AMEN` reader - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as LPUART1AMEN_R;
#[doc = "Field `SPI6AMEN` reader - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as SPI6AMEN_R;
#[doc = "Field `I2C4AMEN` reader - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as I2C4AMEN_R;
#[doc = "Field `LPTIM2AMEN` reader - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
pub use BDMA2AMEN_R as LPTIM2AMEN_R;
#[doc = "Field `LPTIM3AMEN` reader - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as LPTIM3AMEN_R;
#[doc = "Field `DAC2AMEN` reader - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as DAC2AMEN_R;
#[doc = "Field `COMP12AMEN` reader - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as COMP12AMEN_R;
#[doc = "Field `VREFAMEN` reader - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as VREFAMEN_R;
#[doc = "Field `RTCAMEN` reader - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as RTCAMEN_R;
#[doc = "Field `DTSAMEN` reader - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as DTSAMEN_R;
#[doc = "Field `DFSDM2AMEN` reader - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as DFSDM2AMEN_R;
#[doc = "Field `BKPRAMAMEN` reader - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as BKPRAMAMEN_R;
#[doc = "Field `SRDSRAMAMEN` reader - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_R as SRDSRAMAMEN_R;
#[doc = "Field `GPIOAMEN` writer - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as GPIOAMEN_W;
#[doc = "Field `LPUART1AMEN` writer - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as LPUART1AMEN_W;
#[doc = "Field `SPI6AMEN` writer - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as SPI6AMEN_W;
#[doc = "Field `I2C4AMEN` writer - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as I2C4AMEN_W;
#[doc = "Field `LPTIM2AMEN` writer - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
pub use BDMA2AMEN_W as LPTIM2AMEN_W;
#[doc = "Field `LPTIM3AMEN` writer - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as LPTIM3AMEN_W;
#[doc = "Field `DAC2AMEN` writer - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as DAC2AMEN_W;
#[doc = "Field `COMP12AMEN` writer - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as COMP12AMEN_W;
#[doc = "Field `VREFAMEN` writer - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as VREFAMEN_W;
#[doc = "Field `RTCAMEN` writer - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as RTCAMEN_W;
#[doc = "Field `DTSAMEN` writer - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as DTSAMEN_W;
#[doc = "Field `DFSDM2AMEN` writer - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as DFSDM2AMEN_W;
#[doc = "Field `BKPRAMAMEN` writer - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as BKPRAMAMEN_W;
#[doc = "Field `SRDSRAMAMEN` writer - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
pub use BDMA2AMEN_W as SRDSRAMAMEN_W;
impl R {
    #[doc = "Bit 0 - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn bdma2amen(&self) -> BDMA2AMEN_R {
        BDMA2AMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn gpioamen(&self) -> GPIOAMEN_R {
        GPIOAMEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn lpuart1amen(&self) -> LPUART1AMEN_R {
        LPUART1AMEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn spi6amen(&self) -> SPI6AMEN_R {
        SPI6AMEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn i2c4amen(&self) -> I2C4AMEN_R {
        I2C4AMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
    #[inline(always)]
    pub fn lptim2amen(&self) -> LPTIM2AMEN_R {
        LPTIM2AMEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn lptim3amen(&self) -> LPTIM3AMEN_R {
        LPTIM3AMEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dac2amen(&self) -> DAC2AMEN_R {
        DAC2AMEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn comp12amen(&self) -> COMP12AMEN_R {
        COMP12AMEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn vrefamen(&self) -> VREFAMEN_R {
        VREFAMEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn rtcamen(&self) -> RTCAMEN_R {
        RTCAMEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 26 - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dtsamen(&self) -> DTSAMEN_R {
        DTSAMEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn dfsdm2amen(&self) -> DFSDM2AMEN_R {
        DFSDM2AMEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn bkpramamen(&self) -> BKPRAMAMEN_R {
        BKPRAMAMEN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    pub fn srdsramamen(&self) -> SRDSRAMAMEN_R {
        SRDSRAMAMEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SmartRun domain DMA and DMAMUX Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn bdma2amen(&mut self) -> BDMA2AMEN_W<SRDAMRrs> {
        BDMA2AMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn gpioamen(&mut self) -> GPIOAMEN_W<SRDAMRrs> {
        GPIOAMEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1amen(&mut self) -> LPUART1AMEN_W<SRDAMRrs> {
        LPUART1AMEN_W::new(self, 3)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn spi6amen(&mut self) -> SPI6AMEN_W<SRDAMRrs> {
        SPI6AMEN_W::new(self, 5)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn i2c4amen(&mut self) -> I2C4AMEN_W<SRDAMRrs> {
        I2C4AMEN_W::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable Set and reset by software. Refer to for additional information"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2amen(&mut self) -> LPTIM2AMEN_W<SRDAMRrs> {
        LPTIM2AMEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn lptim3amen(&mut self) -> LPTIM3AMEN_W<SRDAMRrs> {
        LPTIM3AMEN_W::new(self, 10)
    }
    #[doc = "Bit 13 - DAC2 (containing one converter) Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn dac2amen(&mut self) -> DAC2AMEN_W<SRDAMRrs> {
        DAC2AMEN_W::new(self, 13)
    }
    #[doc = "Bit 14 - COMP1 and 2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn comp12amen(&mut self) -> COMP12AMEN_W<SRDAMRrs> {
        COMP12AMEN_W::new(self, 14)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn vrefamen(&mut self) -> VREFAMEN_W<SRDAMRrs> {
        VREFAMEN_W::new(self, 15)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn rtcamen(&mut self) -> RTCAMEN_W<SRDAMRrs> {
        RTCAMEN_W::new(self, 16)
    }
    #[doc = "Bit 26 - Digital temperature sensor Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn dtsamen(&mut self) -> DTSAMEN_W<SRDAMRrs> {
        DTSAMEN_W::new(self, 26)
    }
    #[doc = "Bit 27 - DFSDM2 Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn dfsdm2amen(&mut self) -> DFSDM2AMEN_W<SRDAMRrs> {
        DFSDM2AMEN_W::new(self, 27)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn bkpramamen(&mut self) -> BKPRAMAMEN_W<SRDAMRrs> {
        BKPRAMAMEN_W::new(self, 28)
    }
    #[doc = "Bit 29 - SmartRun domain SRAM Autonomous mode enable Set and reset by software. Refer to for additional information."]
    #[inline(always)]
    #[must_use]
    pub fn srdsramamen(&mut self) -> SRDSRAMAMEN_W<SRDAMRrs> {
        SRDSRAMAMEN_W::new(self, 29)
    }
}
#[doc = "RCC SmartRun domain Autonomous mode register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srdamr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srdamr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRDAMRrs;
impl crate::RegisterSpec for SRDAMRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srdamr::R`](R) reader structure"]
impl crate::Readable for SRDAMRrs {}
#[doc = "`write(|w| ..)` method takes [`srdamr::W`](W) writer structure"]
impl crate::Writable for SRDAMRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SRDAMR to value 0"]
impl crate::Resettable for SRDAMRrs {
    const RESET_VALUE: u32 = 0;
}
