///Register `C1_APB4LPENR` reader
pub type R = crate::R<C1_APB4LPENRrs>;
///Register `C1_APB4LPENR` writer
pub type W = crate::W<C1_APB4LPENRrs>;
/**SYSCFG peripheral clock enable during CSleep mode

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
///Field `SYSCFGLPEN` reader - SYSCFG peripheral clock enable during CSleep mode
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
///Field `SYSCFGLPEN` writer - SYSCFG peripheral clock enable during CSleep mode
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
///Field `LPUART1LPEN` reader - LPUART1 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPUART1LPEN_R;
///Field `SPI6LPEN` reader - SPI6 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as SPI6LPEN_R;
///Field `I2C4LPEN` reader - I2C4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as I2C4LPEN_R;
///Field `LPTIM2LPEN` reader - LPTIM2 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPTIM2LPEN_R;
///Field `LPTIM3LPEN` reader - LPTIM3 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPTIM3LPEN_R;
///Field `LPTIM4LPEN` reader - LPTIM4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPTIM4LPEN_R;
///Field `LPTIM5LPEN` reader - LPTIM5 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as LPTIM5LPEN_R;
///Field `COMP12LPEN` reader - COMP1/2 peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_R as COMP12LPEN_R;
///Field `VREFLPEN` reader - VREF peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_R as VREFLPEN_R;
///Field `RTCAPBLPEN` reader - RTC APB Clock Enable During CSleep Mode
pub use SYSCFGLPEN_R as RTCAPBLPEN_R;
///Field `SAI4LPEN` reader - SAI4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_R as SAI4LPEN_R;
///Field `DTSLPEN` reader - Digital temperature sensor peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_R as DTSLPEN_R;
///Field `LPUART1LPEN` writer - LPUART1 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPUART1LPEN_W;
///Field `SPI6LPEN` writer - SPI6 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as SPI6LPEN_W;
///Field `I2C4LPEN` writer - I2C4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as I2C4LPEN_W;
///Field `LPTIM2LPEN` writer - LPTIM2 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPTIM2LPEN_W;
///Field `LPTIM3LPEN` writer - LPTIM3 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPTIM3LPEN_W;
///Field `LPTIM4LPEN` writer - LPTIM4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPTIM4LPEN_W;
///Field `LPTIM5LPEN` writer - LPTIM5 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as LPTIM5LPEN_W;
///Field `COMP12LPEN` writer - COMP1/2 peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_W as COMP12LPEN_W;
///Field `VREFLPEN` writer - VREF peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_W as VREFLPEN_W;
///Field `RTCAPBLPEN` writer - RTC APB Clock Enable During CSleep Mode
pub use SYSCFGLPEN_W as RTCAPBLPEN_W;
///Field `SAI4LPEN` writer - SAI4 Peripheral Clocks Enable During CSleep Mode
pub use SYSCFGLPEN_W as SAI4LPEN_W;
///Field `DTSLPEN` writer - Digital temperature sensor peripheral clock enable during CSleep mode
pub use SYSCFGLPEN_W as DTSLPEN_W;
impl R {
    ///Bit 1 - SYSCFG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn syscfglpen(&self) -> SYSCFGLPEN_R {
        SYSCFGLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> LPUART1LPEN_R {
        LPUART1LPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi6lpen(&self) -> SPI6LPEN_R {
        SPI6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2C4LPEN_R {
        I2C4LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim2lpen(&self) -> LPTIM2LPEN_R {
        LPTIM2LPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim3lpen(&self) -> LPTIM3LPEN_R {
        LPTIM3LPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim4lpen(&self) -> LPTIM4LPEN_R {
        LPTIM4LPEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim5lpen(&self) -> LPTIM5LPEN_R {
        LPTIM5LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - COMP1/2 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn comp12lpen(&self) -> COMP12LPEN_R {
        COMP12LPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - VREF peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn vreflpen(&self) -> VREFLPEN_R {
        VREFLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - RTC APB Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RTCAPBLPEN_R {
        RTCAPBLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn sai4lpen(&self) -> SAI4LPEN_R {
        SAI4LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 26 - Digital temperature sensor peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn dtslpen(&self) -> DTSLPEN_R {
        DTSLPEN_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("C1_APB4LPENR")
            .field("syscfglpen", &self.syscfglpen())
            .field("lpuart1lpen", &self.lpuart1lpen())
            .field("spi6lpen", &self.spi6lpen())
            .field("i2c4lpen", &self.i2c4lpen())
            .field("lptim2lpen", &self.lptim2lpen())
            .field("lptim3lpen", &self.lptim3lpen())
            .field("lptim4lpen", &self.lptim4lpen())
            .field("lptim5lpen", &self.lptim5lpen())
            .field("comp12lpen", &self.comp12lpen())
            .field("vreflpen", &self.vreflpen())
            .field("rtcapblpen", &self.rtcapblpen())
            .field("sai4lpen", &self.sai4lpen())
            .field("dtslpen", &self.dtslpen())
            .finish()
    }
}
impl W {
    ///Bit 1 - SYSCFG peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SYSCFGLPEN_W<'_, C1_APB4LPENRrs> {
        SYSCFGLPEN_W::new(self, 1)
    }
    ///Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lpuart1lpen(&mut self) -> LPUART1LPEN_W<'_, C1_APB4LPENRrs> {
        LPUART1LPEN_W::new(self, 3)
    }
    ///Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> SPI6LPEN_W<'_, C1_APB4LPENRrs> {
        SPI6LPEN_W::new(self, 5)
    }
    ///Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2C4LPEN_W<'_, C1_APB4LPENRrs> {
        I2C4LPEN_W::new(self, 7)
    }
    ///Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> LPTIM2LPEN_W<'_, C1_APB4LPENRrs> {
        LPTIM2LPEN_W::new(self, 9)
    }
    ///Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> LPTIM3LPEN_W<'_, C1_APB4LPENRrs> {
        LPTIM3LPEN_W::new(self, 10)
    }
    ///Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim4lpen(&mut self) -> LPTIM4LPEN_W<'_, C1_APB4LPENRrs> {
        LPTIM4LPEN_W::new(self, 11)
    }
    ///Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn lptim5lpen(&mut self) -> LPTIM5LPEN_W<'_, C1_APB4LPENRrs> {
        LPTIM5LPEN_W::new(self, 12)
    }
    ///Bit 14 - COMP1/2 peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn comp12lpen(&mut self) -> COMP12LPEN_W<'_, C1_APB4LPENRrs> {
        COMP12LPEN_W::new(self, 14)
    }
    ///Bit 15 - VREF peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn vreflpen(&mut self) -> VREFLPEN_W<'_, C1_APB4LPENRrs> {
        VREFLPEN_W::new(self, 15)
    }
    ///Bit 16 - RTC APB Clock Enable During CSleep Mode
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RTCAPBLPEN_W<'_, C1_APB4LPENRrs> {
        RTCAPBLPEN_W::new(self, 16)
    }
    ///Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode
    #[inline(always)]
    pub fn sai4lpen(&mut self) -> SAI4LPEN_W<'_, C1_APB4LPENRrs> {
        SAI4LPEN_W::new(self, 21)
    }
    ///Bit 26 - Digital temperature sensor peripheral clock enable during CSleep mode
    #[inline(always)]
    pub fn dtslpen(&mut self) -> DTSLPEN_W<'_, C1_APB4LPENRrs> {
        DTSLPEN_W::new(self, 26)
    }
}
/**RCC APB4 Sleep Clock Register

You can [`read`](crate::Reg::read) this register and get [`c1_apb4lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb4lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#RCC:C1_APB4LPENR)*/
pub struct C1_APB4LPENRrs;
impl crate::RegisterSpec for C1_APB4LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`c1_apb4lpenr::R`](R) reader structure
impl crate::Readable for C1_APB4LPENRrs {}
///`write(|w| ..)` method takes [`c1_apb4lpenr::W`](W) writer structure
impl crate::Writable for C1_APB4LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C1_APB4LPENR to value 0x0421_deaa
impl crate::Resettable for C1_APB4LPENRrs {
    const RESET_VALUE: u32 = 0x0421_deaa;
}
