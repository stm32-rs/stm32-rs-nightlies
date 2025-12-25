///Register `APBENR1` reader
pub type R = crate::R<APBENR1rs>;
///Register `APBENR1` writer
pub type W = crate::W<APBENR1rs>;
/**TIM3 timer clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM3EN {
    ///0: Peripheral disabled (typically saves power)
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<TIM3EN> for bool {
    #[inline(always)]
    fn from(variant: TIM3EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM3EN` reader - TIM3 timer clock enable
pub type TIM3EN_R = crate::BitReader<TIM3EN>;
impl TIM3EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM3EN {
        match self.bits {
            false => TIM3EN::Disabled,
            true => TIM3EN::Enabled,
        }
    }
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM3EN::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM3EN::Enabled
    }
}
///Field `TIM3EN` writer - TIM3 timer clock enable
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM3EN>;
impl<'a, REG> TIM3EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled (typically saves power)
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3EN::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM3EN::Enabled)
    }
}
///Field `TIM4EN` reader - TIM4 timer clock enable
pub use TIM3EN_R as TIM4EN_R;
///Field `TIM6EN` reader - TIM6 timer clock enable
pub use TIM3EN_R as TIM6EN_R;
///Field `TIM7EN` reader - TIM7 timer clock enable
pub use TIM3EN_R as TIM7EN_R;
///Field `USART5EN` reader - USART5EN
pub use TIM3EN_R as USART5EN_R;
///Field `USART6EN` reader - USART6EN
pub use TIM3EN_R as USART6EN_R;
///Field `RTCAPBEN` reader - RTC APB clock enable
pub use TIM3EN_R as RTCAPBEN_R;
///Field `WWDGEN` reader - WWDG clock enable
pub use TIM3EN_R as WWDGEN_R;
///Field `USBEN` reader - USBEN
pub use TIM3EN_R as USBEN_R;
///Field `SPI2EN` reader - SPI2 clock enable
pub use TIM3EN_R as SPI2EN_R;
///Field `SPI3EN` reader - SPI3 clock enable
pub use TIM3EN_R as SPI3EN_R;
///Field `USART2EN` reader - USART2 clock enable
pub use TIM3EN_R as USART2EN_R;
///Field `USART3EN` reader - USART3 clock enable
pub use TIM3EN_R as USART3EN_R;
///Field `USART4EN` reader - USART4 clock enable
pub use TIM3EN_R as USART4EN_R;
///Field `I2C1EN` reader - I2C1 clock enable
pub use TIM3EN_R as I2C1EN_R;
///Field `I2C2EN` reader - I2C2 clock enable
pub use TIM3EN_R as I2C2EN_R;
///Field `I2C3EN` reader - I2C3 clock enable
pub use TIM3EN_R as I2C3EN_R;
///Field `DBGEN` reader - Debug support clock enable
pub use TIM3EN_R as DBGEN_R;
///Field `PWREN` reader - Power interface clock enable
pub use TIM3EN_R as PWREN_R;
///Field `TIM4EN` writer - TIM4 timer clock enable
pub use TIM3EN_W as TIM4EN_W;
///Field `TIM6EN` writer - TIM6 timer clock enable
pub use TIM3EN_W as TIM6EN_W;
///Field `TIM7EN` writer - TIM7 timer clock enable
pub use TIM3EN_W as TIM7EN_W;
///Field `USART5EN` writer - USART5EN
pub use TIM3EN_W as USART5EN_W;
///Field `USART6EN` writer - USART6EN
pub use TIM3EN_W as USART6EN_W;
///Field `RTCAPBEN` writer - RTC APB clock enable
pub use TIM3EN_W as RTCAPBEN_W;
///Field `WWDGEN` writer - WWDG clock enable
pub use TIM3EN_W as WWDGEN_W;
///Field `USBEN` writer - USBEN
pub use TIM3EN_W as USBEN_W;
///Field `SPI2EN` writer - SPI2 clock enable
pub use TIM3EN_W as SPI2EN_W;
///Field `SPI3EN` writer - SPI3 clock enable
pub use TIM3EN_W as SPI3EN_W;
///Field `USART2EN` writer - USART2 clock enable
pub use TIM3EN_W as USART2EN_W;
///Field `USART3EN` writer - USART3 clock enable
pub use TIM3EN_W as USART3EN_W;
///Field `USART4EN` writer - USART4 clock enable
pub use TIM3EN_W as USART4EN_W;
///Field `I2C1EN` writer - I2C1 clock enable
pub use TIM3EN_W as I2C1EN_W;
///Field `I2C2EN` writer - I2C2 clock enable
pub use TIM3EN_W as I2C2EN_W;
///Field `I2C3EN` writer - I2C3 clock enable
pub use TIM3EN_W as I2C3EN_W;
///Field `DBGEN` writer - Debug support clock enable
pub use TIM3EN_W as DBGEN_W;
///Field `PWREN` writer - Power interface clock enable
pub use TIM3EN_W as PWREN_W;
impl R {
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 timer clock enable
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - TIM6 timer clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 timer clock enable
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - USART5EN
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - USART6EN
    #[inline(always)]
    pub fn usart6en(&self) -> USART6EN_R {
        USART6EN_R::new(((self.bits >> 9) & 1) != 0)
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
    ///Bit 13 - USBEN
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clock enable
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART4 clock enable
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
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
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APBENR1")
            .field("tim3en", &self.tim3en())
            .field("tim4en", &self.tim4en())
            .field("tim6en", &self.tim6en())
            .field("tim7en", &self.tim7en())
            .field("usart5en", &self.usart5en())
            .field("usart6en", &self.usart6en())
            .field("rtcapben", &self.rtcapben())
            .field("wwdgen", &self.wwdgen())
            .field("usben", &self.usben())
            .field("spi2en", &self.spi2en())
            .field("spi3en", &self.spi3en())
            .field("usart2en", &self.usart2en())
            .field("usart3en", &self.usart3en())
            .field("usart4en", &self.usart4en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("i2c3en", &self.i2c3en())
            .field("dbgen", &self.dbgen())
            .field("pwren", &self.pwren())
            .finish()
    }
}
impl W {
    ///Bit 1 - TIM3 timer clock enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APBENR1rs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 timer clock enable
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W<'_, APBENR1rs> {
        TIM4EN_W::new(self, 2)
    }
    ///Bit 4 - TIM6 timer clock enable
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APBENR1rs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 timer clock enable
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<'_, APBENR1rs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 8 - USART5EN
    #[inline(always)]
    pub fn usart5en(&mut self) -> USART5EN_W<'_, APBENR1rs> {
        USART5EN_W::new(self, 8)
    }
    ///Bit 9 - USART6EN
    #[inline(always)]
    pub fn usart6en(&mut self) -> USART6EN_W<'_, APBENR1rs> {
        USART6EN_W::new(self, 9)
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
    ///Bit 13 - USBEN
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W<'_, APBENR1rs> {
        USBEN_W::new(self, 13)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APBENR1rs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clock enable
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APBENR1rs> {
        SPI3EN_W::new(self, 15)
    }
    ///Bit 17 - USART2 clock enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APBENR1rs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W<'_, APBENR1rs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 19 - USART4 clock enable
    #[inline(always)]
    pub fn usart4en(&mut self) -> USART4EN_W<'_, APBENR1rs> {
        USART4EN_W::new(self, 19)
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
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APBENR1rs> {
        I2C3EN_W::new(self, 23)
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
}
/**APB peripheral clock enable register 1

You can [`read`](crate::Reg::read) this register and get [`apbenr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apbenr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B0.html#RCC:APBENR1)*/
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
