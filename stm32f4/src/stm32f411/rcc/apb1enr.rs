///Register `APB1ENR` reader
pub type R = crate::R<APB1ENRrs>;
///Register `APB1ENR` writer
pub type W = crate::W<APB1ENRrs>;
/**TIM2 clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2EN` reader - TIM2 clock enable
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
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
///Field `TIM2EN` writer - TIM2 clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
///Field `TIM3EN` reader - TIM3 clock enable
pub use TIM2EN_R as TIM3EN_R;
///Field `TIM4EN` reader - TIM4 clock enable
pub use TIM2EN_R as TIM4EN_R;
///Field `TIM5EN` reader - TIM5 clock enable
pub use TIM2EN_R as TIM5EN_R;
///Field `WWDGEN` reader - Window watchdog clock enable
pub use TIM2EN_R as WWDGEN_R;
///Field `SPI2EN` reader - SPI2 clock enable
pub use TIM2EN_R as SPI2EN_R;
///Field `SPI3EN` reader - SPI3 clock enable
pub use TIM2EN_R as SPI3EN_R;
///Field `USART2EN` reader - USART 2 clock enable
pub use TIM2EN_R as USART2EN_R;
///Field `I2C1EN` reader - I2C1 clock enable
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C2EN` reader - I2C2 clock enable
pub use TIM2EN_R as I2C2EN_R;
///Field `I2C3EN` reader - I2C3 clock enable
pub use TIM2EN_R as I2C3EN_R;
///Field `PWREN` reader - Power interface clock enable
pub use TIM2EN_R as PWREN_R;
///Field `TIM3EN` writer - TIM3 clock enable
pub use TIM2EN_W as TIM3EN_W;
///Field `TIM4EN` writer - TIM4 clock enable
pub use TIM2EN_W as TIM4EN_W;
///Field `TIM5EN` writer - TIM5 clock enable
pub use TIM2EN_W as TIM5EN_W;
///Field `WWDGEN` writer - Window watchdog clock enable
pub use TIM2EN_W as WWDGEN_W;
///Field `SPI2EN` writer - SPI2 clock enable
pub use TIM2EN_W as SPI2EN_W;
///Field `SPI3EN` writer - SPI3 clock enable
pub use TIM2EN_W as SPI3EN_W;
///Field `USART2EN` writer - USART 2 clock enable
pub use TIM2EN_W as USART2EN_W;
///Field `I2C1EN` writer - I2C1 clock enable
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C2EN` writer - I2C2 clock enable
pub use TIM2EN_W as I2C2EN_W;
///Field `I2C3EN` writer - I2C3 clock enable
pub use TIM2EN_W as I2C3EN_W;
///Field `PWREN` writer - Power interface clock enable
pub use TIM2EN_W as PWREN_W;
impl R {
    ///Bit 0 - TIM2 clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 clock enable
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 clock enable
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
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
    ///Bit 15 - SPI3 clock enable
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
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
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR")
            .field("tim2en", &self.tim2en())
            .field("pwren", &self.pwren())
            .field("i2c3en", &self.i2c3en())
            .field("i2c2en", &self.i2c2en())
            .field("i2c1en", &self.i2c1en())
            .field("usart2en", &self.usart2en())
            .field("spi3en", &self.spi3en())
            .field("spi2en", &self.spi2en())
            .field("wwdgen", &self.wwdgen())
            .field("tim5en", &self.tim5en())
            .field("tim4en", &self.tim4en())
            .field("tim3en", &self.tim3en())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1ENRrs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 clock enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APB1ENRrs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 clock enable
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W<'_, APB1ENRrs> {
        TIM4EN_W::new(self, 2)
    }
    ///Bit 3 - TIM5 clock enable
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W<'_, APB1ENRrs> {
        TIM5EN_W::new(self, 3)
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
    ///Bit 15 - SPI3 clock enable
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APB1ENRrs> {
        SPI3EN_W::new(self, 15)
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
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APB1ENRrs> {
        I2C3EN_W::new(self, 23)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, APB1ENRrs> {
        PWREN_W::new(self, 28)
    }
}
/**APB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F411.html#RCC:APB1ENR)*/
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
