///Register `APB1ENR` reader
pub type R = crate::R<APB1ENRrs>;
///Register `APB1ENR` writer
pub type W = crate::W<APB1ENRrs>;
/**TIM5 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM5EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<TIM5EN> for bool {
    #[inline(always)]
    fn from(variant: TIM5EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM5EN` reader - TIM5 clock enable
pub type TIM5EN_R = crate::BitReader<TIM5EN>;
impl TIM5EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM5EN {
        match self.bits {
            false => TIM5EN::Disabled,
            true => TIM5EN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM5EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM5EN::Enabled
    }
}
///Field `TIM5EN` writer - TIM5 clock enable
pub type TIM5EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM5EN>;
impl<'a, REG> TIM5EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM5EN::Enabled)
    }
}
///Field `TIM6EN` reader - TIM6 clock enable
pub use TIM5EN_R as TIM6EN_R;
///Field `LPTIM1EN` reader - LPTIM1 clock enable
pub use TIM5EN_R as LPTIM1EN_R;
///Field `RTCAPBEN` reader - RTC APB clock enable
pub use TIM5EN_R as RTCAPBEN_R;
///Field `WWDGEN` reader - Window watchdog clock enable
pub use TIM5EN_R as WWDGEN_R;
///Field `SPI2EN` reader - SPI2 clock enable
pub use TIM5EN_R as SPI2EN_R;
///Field `USART2EN` reader - USART 2 clock enable
pub use TIM5EN_R as USART2EN_R;
///Field `I2C1EN` reader - I2C1 clock enable
pub use TIM5EN_R as I2C1EN_R;
///Field `I2C2EN` reader - I2C2 clock enable
pub use TIM5EN_R as I2C2EN_R;
///Field `FMPI2C1EN` reader - FMPI2C1 clock enable
pub use TIM5EN_R as FMPI2C1EN_R;
///Field `PWREN` reader - Power interface clock enable
pub use TIM5EN_R as PWREN_R;
///Field `DACEN` reader - DAC interface clock enable
pub use TIM5EN_R as DACEN_R;
///Field `TIM6EN` writer - TIM6 clock enable
pub use TIM5EN_W as TIM6EN_W;
///Field `LPTIM1EN` writer - LPTIM1 clock enable
pub use TIM5EN_W as LPTIM1EN_W;
///Field `RTCAPBEN` writer - RTC APB clock enable
pub use TIM5EN_W as RTCAPBEN_W;
///Field `WWDGEN` writer - Window watchdog clock enable
pub use TIM5EN_W as WWDGEN_W;
///Field `SPI2EN` writer - SPI2 clock enable
pub use TIM5EN_W as SPI2EN_W;
///Field `USART2EN` writer - USART 2 clock enable
pub use TIM5EN_W as USART2EN_W;
///Field `I2C1EN` writer - I2C1 clock enable
pub use TIM5EN_W as I2C1EN_W;
///Field `I2C2EN` writer - I2C2 clock enable
pub use TIM5EN_W as I2C2EN_W;
///Field `FMPI2C1EN` writer - FMPI2C1 clock enable
pub use TIM5EN_W as FMPI2C1EN_W;
///Field `PWREN` writer - Power interface clock enable
pub use TIM5EN_W as PWREN_W;
///Field `DACEN` writer - DAC interface clock enable
pub use TIM5EN_W as DACEN_W;
impl R {
    ///Bit 3 - TIM5 clock enable
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 9 - LPTIM1 clock enable
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&self) -> RTCAPBEN_R {
        RTCAPBEN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
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
    ///Bit 24 - FMPI2C1 clock enable
    #[inline(always)]
    pub fn fmpi2c1en(&self) -> FMPI2C1EN_R {
        FMPI2C1EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface clock enable
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR")
            .field("tim5en", &self.tim5en())
            .field("tim6en", &self.tim6en())
            .field("lptim1en", &self.lptim1en())
            .field("rtcapben", &self.rtcapben())
            .field("wwdgen", &self.wwdgen())
            .field("spi2en", &self.spi2en())
            .field("usart2en", &self.usart2en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("fmpi2c1en", &self.fmpi2c1en())
            .field("pwren", &self.pwren())
            .field("dacen", &self.dacen())
            .finish()
    }
}
impl W {
    ///Bit 3 - TIM5 clock enable
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W<'_, APB1ENRrs> {
        TIM5EN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 clock enable
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APB1ENRrs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 9 - LPTIM1 clock enable
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<'_, APB1ENRrs> {
        LPTIM1EN_W::new(self, 9)
    }
    ///Bit 10 - RTC APB clock enable
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RTCAPBEN_W<'_, APB1ENRrs> {
        RTCAPBEN_W::new(self, 10)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APB1ENRrs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APB1ENRrs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1ENRrs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1ENRrs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APB1ENRrs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 24 - FMPI2C1 clock enable
    #[inline(always)]
    pub fn fmpi2c1en(&mut self) -> FMPI2C1EN_W<'_, APB1ENRrs> {
        FMPI2C1EN_W::new(self, 24)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, APB1ENRrs> {
        PWREN_W::new(self, 28)
    }
    ///Bit 29 - DAC interface clock enable
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W<'_, APB1ENRrs> {
        DACEN_W::new(self, 29)
    }
}
/**APB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F410.html#RCC:APB1ENR)*/
pub struct APB1ENRrs;
impl crate::RegisterSpec for APB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr::R`](R) reader structure
impl crate::Readable for APB1ENRrs {}
///`write(|w| ..)` method takes [`apb1enr::W`](W) writer structure
impl crate::Writable for APB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1ENR to value 0
impl crate::Resettable for APB1ENRrs {}
