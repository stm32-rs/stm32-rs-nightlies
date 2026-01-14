///Register `APB1ENR` reader
pub type R = crate::R<APB1ENRrs>;
///Register `APB1ENR` writer
pub type W = crate::W<APB1ENRrs>;
/**Timer 2 clock enable

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
///Field `TIM2EN` reader - Timer 2 clock enable
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
///Field `TIM2EN` writer - Timer 2 clock enable
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
///Field `TIM3EN` reader - Timer 3 clock enable
pub use TIM2EN_R as TIM3EN_R;
///Field `TIM4EN` reader - Timer 4 clock enable
pub use TIM2EN_R as TIM4EN_R;
///Field `TIM6EN` reader - Timer 6 clock enable
pub use TIM2EN_R as TIM6EN_R;
///Field `WWDGEN` reader - Window watchdog clock enable
pub use TIM2EN_R as WWDGEN_R;
///Field `SPI2EN` reader - SPI 2 clock enable
pub use TIM2EN_R as SPI2EN_R;
///Field `SPI3EN` reader - SPI 3 clock enable
pub use TIM2EN_R as SPI3EN_R;
///Field `USART2EN` reader - USART 2 clock enable
pub use TIM2EN_R as USART2EN_R;
///Field `USART3EN` reader - USART3 clock enable
pub use TIM2EN_R as USART3EN_R;
///Field `UART4EN` reader - UART4 clock enable
pub use TIM2EN_R as UART4EN_R;
///Field `UART5EN` reader - UART5 clock enable
pub use TIM2EN_R as UART5EN_R;
///Field `I2C1EN` reader - I2C 1 clock enable
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C2EN` reader - I2C 2 clock enable
pub use TIM2EN_R as I2C2EN_R;
///Field `USBEN` reader - USB clock enable
pub use TIM2EN_R as USBEN_R;
///Field `CANEN` reader - CAN clock enable
pub use TIM2EN_R as CANEN_R;
///Field `PWREN` reader - Power interface clock enable
pub use TIM2EN_R as PWREN_R;
///Field `DAC1EN` reader - DAC interface clock enable
pub use TIM2EN_R as DAC1EN_R;
///Field `I2C3EN` reader - I2C3 clock enable
pub use TIM2EN_R as I2C3EN_R;
///Field `TIM3EN` writer - Timer 3 clock enable
pub use TIM2EN_W as TIM3EN_W;
///Field `TIM4EN` writer - Timer 4 clock enable
pub use TIM2EN_W as TIM4EN_W;
///Field `TIM6EN` writer - Timer 6 clock enable
pub use TIM2EN_W as TIM6EN_W;
///Field `WWDGEN` writer - Window watchdog clock enable
pub use TIM2EN_W as WWDGEN_W;
///Field `SPI2EN` writer - SPI 2 clock enable
pub use TIM2EN_W as SPI2EN_W;
///Field `SPI3EN` writer - SPI 3 clock enable
pub use TIM2EN_W as SPI3EN_W;
///Field `USART2EN` writer - USART 2 clock enable
pub use TIM2EN_W as USART2EN_W;
///Field `USART3EN` writer - USART3 clock enable
pub use TIM2EN_W as USART3EN_W;
///Field `UART4EN` writer - UART4 clock enable
pub use TIM2EN_W as UART4EN_W;
///Field `UART5EN` writer - UART5 clock enable
pub use TIM2EN_W as UART5EN_W;
///Field `I2C1EN` writer - I2C 1 clock enable
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C2EN` writer - I2C 2 clock enable
pub use TIM2EN_W as I2C2EN_W;
///Field `USBEN` writer - USB clock enable
pub use TIM2EN_W as USBEN_W;
///Field `CANEN` writer - CAN clock enable
pub use TIM2EN_W as CANEN_W;
///Field `PWREN` writer - Power interface clock enable
pub use TIM2EN_W as PWREN_W;
///Field `DAC1EN` writer - DAC interface clock enable
pub use TIM2EN_W as DAC1EN_W;
///Field `I2C3EN` writer - I2C3 clock enable
pub use TIM2EN_W as I2C3EN_W;
impl R {
    ///Bit 0 - Timer 2 clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer 3 clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer 4 clock enable
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 4 - Timer 6 clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI 2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI 3 clock enable
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clock enable
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clock enable
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C 1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C 2 clock enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - USB clock enable
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CAN clock enable
    #[inline(always)]
    pub fn canen(&self) -> CANEN_R {
        CANEN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface clock enable
    #[inline(always)]
    pub fn dac1en(&self) -> DAC1EN_R {
        DAC1EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("tim4en", &self.tim4en())
            .field("tim6en", &self.tim6en())
            .field("wwdgen", &self.wwdgen())
            .field("spi2en", &self.spi2en())
            .field("spi3en", &self.spi3en())
            .field("usart2en", &self.usart2en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("usben", &self.usben())
            .field("canen", &self.canen())
            .field("pwren", &self.pwren())
            .field("dac1en", &self.dac1en())
            .field("usart3en", &self.usart3en())
            .field("uart4en", &self.uart4en())
            .field("uart5en", &self.uart5en())
            .field("i2c3en", &self.i2c3en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer 2 clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1ENRrs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - Timer 3 clock enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APB1ENRrs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 2 - Timer 4 clock enable
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W<'_, APB1ENRrs> {
        TIM4EN_W::new(self, 2)
    }
    ///Bit 4 - Timer 6 clock enable
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APB1ENRrs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APB1ENRrs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - SPI 2 clock enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APB1ENRrs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 15 - SPI 3 clock enable
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APB1ENRrs> {
        SPI3EN_W::new(self, 15)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1ENRrs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W<'_, APB1ENRrs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 19 - UART4 clock enable
    #[inline(always)]
    pub fn uart4en(&mut self) -> UART4EN_W<'_, APB1ENRrs> {
        UART4EN_W::new(self, 19)
    }
    ///Bit 20 - UART5 clock enable
    #[inline(always)]
    pub fn uart5en(&mut self) -> UART5EN_W<'_, APB1ENRrs> {
        UART5EN_W::new(self, 20)
    }
    ///Bit 21 - I2C 1 clock enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1ENRrs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C 2 clock enable
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APB1ENRrs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 23 - USB clock enable
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W<'_, APB1ENRrs> {
        USBEN_W::new(self, 23)
    }
    ///Bit 25 - CAN clock enable
    #[inline(always)]
    pub fn canen(&mut self) -> CANEN_W<'_, APB1ENRrs> {
        CANEN_W::new(self, 25)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, APB1ENRrs> {
        PWREN_W::new(self, 28)
    }
    ///Bit 29 - DAC interface clock enable
    #[inline(always)]
    pub fn dac1en(&mut self) -> DAC1EN_W<'_, APB1ENRrs> {
        DAC1EN_W::new(self, 29)
    }
    ///Bit 30 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APB1ENRrs> {
        I2C3EN_W::new(self, 30)
    }
}
/**APB1 peripheral clock enable register (RCC_APB1ENR)

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#RCC:APB1ENR)*/
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
