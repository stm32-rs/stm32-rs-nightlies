///Register `APB4LPENR` reader
pub type R = crate::R<APB4LPENRrs>;
///Register `APB4LPENR` writer
pub type W = crate::W<APB4LPENRrs>;
/**SYSCFG peripheral clock enable during CSleep mode Set and reset by software.

Value on reset: 1*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYSCFGLPEN {
    ///0: The selected clock is disabled during csleep mode
    Disabled = 0,
    ///1: The selected clock is enabled during csleep mode
    Enabled = 1,
}
impl From<SYSCFGLPEN> for bool {
    #[inline(always)]
    fn from(variant: SYSCFGLPEN) -> Self {
        variant as u8 != 0
    }
}
///Field `SYSCFGLPEN` reader - SYSCFG peripheral clock enable during CSleep mode Set and reset by software.
pub type SYSCFGLPEN_R = crate::BitReader<SYSCFGLPEN>;
impl SYSCFGLPEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYSCFGLPEN {
        match self.bits {
            false => SYSCFGLPEN::Disabled,
            true => SYSCFGLPEN::Enabled,
        }
    }
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSCFGLPEN::Disabled
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SYSCFGLPEN::Enabled
    }
}
///Field `SYSCFGLPEN` writer - SYSCFG peripheral clock enable during CSleep mode Set and reset by software.
pub type SYSCFGLPEN_W<'a, REG> = crate::BitWriter<'a, REG, SYSCFGLPEN>;
impl<'a, REG> SYSCFGLPEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled during csleep mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGLPEN::Disabled)
    }
    ///The selected clock is enabled during csleep mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSCFGLPEN::Enabled)
    }
}
///Field `LPUART1LPEN` reader - LPUART1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to lpuart_ker_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_R as LPUART1LPEN_R;
///Field `SPI6LPEN` reader - SPI6 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_R as SPI6LPEN_R;
///Field `I2C4LPEN` reader - I2C4 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the I2C4 are the kernel clock selected by I2C4SEL and provided to i2C_ker_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_R as I2C4LPEN_R;
///Field `LPTIM2LPEN` reader - LPTIM2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM2SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_R as LPTIM2LPEN_R;
///Field `LPTIM3LPEN` reader - LPTIM3 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM345SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_R as LPTIM3LPEN_R;
///Field `DAC2LPEN` reader - DAC2 (containing one converter) peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_R as DAC2LPEN_R;
///Field `COMP12LPEN` reader - COMP1 and 2 peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_R as COMP12LPEN_R;
///Field `VREFLPEN` reader - VREF peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_R as VREFLPEN_R;
///Field `RTCAPBLPEN` reader - RTC APB clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_R as RTCAPBLPEN_R;
///Field `DTSLPEN` reader - temperature sensor peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_R as DTSLPEN_R;
///Field `DFSDM2LPEN` reader - DFSDM2 peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_R as DFSDM2LPEN_R;
///Field `LPUART1LPEN` writer - LPUART1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to lpuart_ker_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_W as LPUART1LPEN_W;
///Field `SPI6LPEN` writer - SPI6 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_W as SPI6LPEN_W;
///Field `I2C4LPEN` writer - I2C4 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the I2C4 are the kernel clock selected by I2C4SEL and provided to i2C_ker_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_W as I2C4LPEN_W;
///Field `LPTIM2LPEN` writer - LPTIM2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM2SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_W as LPTIM2LPEN_W;
///Field `LPTIM3LPEN` writer - LPTIM3 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM345SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock.
pub use SYSCFGLPEN_W as LPTIM3LPEN_W;
///Field `DAC2LPEN` writer - DAC2 (containing one converter) peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_W as DAC2LPEN_W;
///Field `COMP12LPEN` writer - COMP1 and 2 peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_W as COMP12LPEN_W;
///Field `VREFLPEN` writer - VREF peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_W as VREFLPEN_W;
///Field `RTCAPBLPEN` writer - RTC APB clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_W as RTCAPBLPEN_W;
///Field `DTSLPEN` writer - temperature sensor peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_W as DTSLPEN_W;
///Field `DFSDM2LPEN` writer - DFSDM2 peripheral clock enable during CSleep mode Set and reset by software.
pub use SYSCFGLPEN_W as DFSDM2LPEN_W;
impl R {
    ///Bit 1 - SYSCFG peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - LPUART1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to lpuart_ker_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI6 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - I2C4 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the I2C4 are the kernel clock selected by I2C4SEL and provided to i2C_ker_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM2SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM345SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 13 - DAC2 (containing one converter) peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dac2lpen(&self) -> DAC2LPEN_R {
        DAC2LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - COMP1 and 2 peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn comp12lpen(&self) -> COMP12LPEN_R {
        COMP12LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VREF peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC APB clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 26 - temperature sensor peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - DFSDM2 peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dfsdm2lpen(&self) -> DFSDM2LPEN_R {
        DFSDM2LPEN_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB4LPENR")
            .field("syscfglpen", &self.syscfglpen())
            .field("lpuart1lpen", &self.lpuart1lpen())
            .field("spi6lpen", &self.spi6lpen())
            .field("i2c4lpen", &self.i2c4lpen())
            .field("lptim2lpen", &self.lptim2lpen())
            .field("lptim3lpen", &self.lptim3lpen())
            .field("dac2lpen", &self.dac2lpen())
            .field("comp12lpen", &self.comp12lpen())
            .field("vreflpen", &self.vreflpen())
            .field("rtcapblpen", &self.rtcapblpen())
            .field("dtslpen", &self.dtslpen())
            .field("dfsdm2lpen", &self.dfsdm2lpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<'_, APB4LPENRrs> {
        SYSCFGLPEN_W::new(self, 1)
    }
    ///Bit 3 - LPUART1 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPUART1 are the kernel clock selected by LPUART1SEL and provided to lpuart_ker_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<'_, APB4LPENRrs> {
        LPUART1LPEN_W::new(self, 3)
    }
    ///Bit 5 - SPI6 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the SPI6 are the kernel clock selected by SPI6SEL and provided to com_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<'_, APB4LPENRrs> {
        SPI6LPEN_W::new(self, 5)
    }
    ///Bit 7 - I2C4 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the I2C4 are the kernel clock selected by I2C4SEL and provided to i2C_ker_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<'_, APB4LPENRrs> {
        I2C4LPEN_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM2 are the kernel clock selected by LPTIM2SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<'_, APB4LPENRrs> {
        LPTIM2LPEN_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 peripheral clocks enable during CSleep mode Set and reset by software. The peripheral clocks of the LPTIM3 are the kernel clock selected by LPTIM345SEL and provided to lptim_ker_ck input, and the rcc_pclk4 bus interface clock.
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<'_, APB4LPENRrs> {
        LPTIM3LPEN_W::new(self, 10)
    }
    ///Bit 13 - DAC2 (containing one converter) peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dac2lpen(&mut self) -> DAC2LPEN_W<'_, APB4LPENRrs> {
        DAC2LPEN_W::new(self, 13)
    }
    ///Bit 14 - COMP1 and 2 peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn comp12lpen(&mut self) -> COMP12LPEN_W<'_, APB4LPENRrs> {
        COMP12LPEN_W::new(self, 14)
    }
    ///Bit 15 - VREF peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<'_, APB4LPENRrs> {
        VREFLPEN_W::new(self, 15)
    }
    ///Bit 16 - RTC APB clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<'_, APB4LPENRrs> {
        RTCAPBLPEN_W::new(self, 16)
    }
    ///Bit 26 - temperature sensor peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<'_, APB4LPENRrs> {
        DTSLPEN_W::new(self, 26)
    }
    ///Bit 27 - DFSDM2 peripheral clock enable during CSleep mode Set and reset by software.
    #[inline(always)]
    pub fn dfsdm2lpen(&mut self) -> DFSDM2LPEN_W<'_, APB4LPENRrs> {
        DFSDM2LPEN_W::new(self, 27)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`apb4lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7B0.html#RCC:APB4LPENR)*/
pub struct APB4LPENRrs;
impl crate::RegisterSpec for APB4LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb4lpenr::R`](R) reader structure
impl crate::Readable for APB4LPENRrs {}
///`write(|w| ..)` method takes [`apb4lpenr::W`](W) writer structure
impl crate::Writable for APB4LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB4LPENR to value 0x0c01_e6aa
impl crate::Resettable for APB4LPENRrs {
    const RESET_VALUE: u32 = 0x0c01_e6aa;
}
