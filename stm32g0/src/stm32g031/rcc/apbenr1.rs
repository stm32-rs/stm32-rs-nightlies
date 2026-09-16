///Register `APBENR1` reader
pub type R = crate::R<APBENR1rs>;
///Register `APBENR1` writer
pub type W = crate::W<APBENR1rs>;
/**TIM2 timer clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2EN` reader - TIM2 timer clock enable
pub type TIM2EN_R = crate::BitReader<TIM2EN>;
impl TIM2EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM2EN {
        match self.bits {
            false => TIM2EN::Disabled,
            true => TIM2EN::Enabled,
        }
    }
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
///Field `TIM2EN` writer - TIM2 timer clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
///Field `TIM3EN` reader - TIM3 timer clock enable
pub use TIM2EN_R as TIM3EN_R;
///Field `RTCAPBEN` reader - RTC APB clock enable
pub use TIM2EN_R as RTCAPBEN_R;
///Field `WWDGEN` reader - WWDG clock enable
pub use TIM2EN_R as WWDGEN_R;
///Field `SPI2EN` reader - SPI2 clock enable
pub use TIM2EN_R as SPI2EN_R;
///Field `USART2EN` reader - USART2 clock enable
pub use TIM2EN_R as USART2EN_R;
///Field `LPUART1EN` reader - LPUART1 clock enable
pub use TIM2EN_R as LPUART1EN_R;
///Field `I2C1EN` reader - I2C1 clock enable
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C2EN` reader - I2C2 clock enable
pub use TIM2EN_R as I2C2EN_R;
///Field `DBGEN` reader - Debug support clock enable
pub use TIM2EN_R as DBGEN_R;
///Field `PWREN` reader - Power interface clock enable
pub use TIM2EN_R as PWREN_R;
///Field `LPTIM2EN` reader - LPTIM2 clock enable
pub use TIM2EN_R as LPTIM2EN_R;
///Field `LPTIM1EN` reader - LPTIM1 clock enable
pub use TIM2EN_R as LPTIM1EN_R;
///Field `TIM3EN` writer - TIM3 timer clock enable
pub use TIM2EN_W as TIM3EN_W;
///Field `RTCAPBEN` writer - RTC APB clock enable
pub use TIM2EN_W as RTCAPBEN_W;
///Field `WWDGEN` writer - WWDG clock enable
pub use TIM2EN_W as WWDGEN_W;
///Field `SPI2EN` writer - SPI2 clock enable
pub use TIM2EN_W as SPI2EN_W;
///Field `USART2EN` writer - USART2 clock enable
pub use TIM2EN_W as USART2EN_W;
///Field `LPUART1EN` writer - LPUART1 clock enable
pub use TIM2EN_W as LPUART1EN_W;
///Field `I2C1EN` writer - I2C1 clock enable
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C2EN` writer - I2C2 clock enable
pub use TIM2EN_W as I2C2EN_W;
///Field `DBGEN` writer - Debug support clock enable
pub use TIM2EN_W as DBGEN_W;
///Field `PWREN` writer - Power interface clock enable
pub use TIM2EN_W as PWREN_W;
///Field `LPTIM2EN` writer - LPTIM2 clock enable
pub use TIM2EN_W as LPTIM2EN_W;
///Field `LPTIM1EN` writer - LPTIM1 clock enable
pub use TIM2EN_W as LPTIM1EN_W;
impl R {
    ///Bit 0 - TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - WWDG clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - LPUART1 clock enable
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 27 - Debug support clock enable
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - LPTIM2 clock enable
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - LPTIM1 clock enable
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBENR1")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("rtcapben", &self.rtcapben())
            .field("wwdgen", &self.wwdgen())
            .field("spi2en", &self.spi2en())
            .field("usart2en", &self.usart2en())
            .field("lpuart1en", &self.lpuart1en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("dbgen", &self.dbgen())
            .field("pwren", &self.pwren())
            .field("lptim2en", &self.lptim2en())
            .field("lptim1en", &self.lptim1en())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 timer clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APBENR1rs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APBENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, APBENR1rs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 11 - WWDG clock enable
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APBENR1rs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APBENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APBENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 20 - LPUART1 clock enable
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<'_, APBENR1rs> {
        LPUART1EN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APBENR1rs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APBENR1rs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 27 - Debug support clock enable
    #[inline(always)]
    pub fn dbgen(&mut self) -> DBGEN_W<'_, APBENR1rs> {
        DBGEN_W::new(self, 27)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, APBENR1rs> {
        PWREN_W::new(self, 28)
    }
    ///Bit 30 - LPTIM2 clock enable
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, APBENR1rs> {
        LPTIM2EN_W::new(self, 30)
    }
    ///Bit 31 - LPTIM1 clock enable
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<'_, APBENR1rs> {
        LPTIM1EN_W::new(self, 31)
    }
}
/**APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apbenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G031.html#RCC:APBENR1)*/
pub struct APBENR1rs;
impl crate::RegisterSpec for APBENR1rs {
    type Ux = u32;
}
///`read()` method returns [`apbenr1::R`](R) reader structure
impl crate::Readable for APBENR1rs {}
///`write(|w| ..)` method takes [`apbenr1::W`](W) writer structure
impl crate::Writable for APBENR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APBENR1 to value 0
impl crate::Resettable for APBENR1rs {}
