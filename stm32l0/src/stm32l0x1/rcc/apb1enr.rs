///Register `APB1ENR` reader
pub type R = crate::R<APB1ENRrs>;
///Register `APB1ENR` writer
pub type W = crate::W<APB1ENRrs>;
/**Timer2 clock enable bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIM2EN {
    ///0: Clock disabled
    Disabled = 0,
    ///1: Clock enabled
    Enabled = 1,
}
impl From<TIM2EN> for bool {
    #[inline(always)]
    fn from(variant: TIM2EN) -> Self {
        variant as u8 != 0
    }
}
///Field `TIM2EN` reader - Timer2 clock enable bit
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
    ///Clock disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIM2EN::Disabled
    }
    ///Clock enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIM2EN::Enabled
    }
}
///Field `TIM2EN` writer - Timer2 clock enable bit
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG, TIM2EN>;
impl<'a, REG> TIM2EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Disabled)
    }
    ///Clock enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIM2EN::Enabled)
    }
}
///Field `TIM3EN` reader - Timer 3 clock enbale bit
pub use TIM2EN_R as TIM3EN_R;
///Field `TIM6EN` reader - Timer 6 clock enable bit
pub use TIM2EN_R as TIM6EN_R;
///Field `TIM7EN` reader - Timer 7 clock enable bit
pub use TIM2EN_R as TIM7EN_R;
///Field `WWDGEN` reader - Window watchdog clock enable bit
pub use TIM2EN_R as WWDGEN_R;
///Field `SPI2EN` reader - SPI2 clock enable bit
pub use TIM2EN_R as SPI2EN_R;
///Field `USART2EN` reader - UART2 clock enable bit
pub use TIM2EN_R as USART2EN_R;
///Field `LPUART1EN` reader - LPUART1 clock enable bit
pub use TIM2EN_R as LPUART1EN_R;
///Field `USART4EN` reader - USART4 clock enable bit
pub use TIM2EN_R as USART4EN_R;
///Field `USART5EN` reader - USART5 clock enable bit
pub use TIM2EN_R as USART5EN_R;
///Field `I2C1EN` reader - I2C1 clock enable bit
pub use TIM2EN_R as I2C1EN_R;
///Field `I2C2EN` reader - I2C2 clock enable bit
pub use TIM2EN_R as I2C2EN_R;
///Field `PWREN` reader - Power interface clock enable bit
pub use TIM2EN_R as PWREN_R;
///Field `I2C3EN` reader - I2C3 clock enable bit
pub use TIM2EN_R as I2C3EN_R;
///Field `LPTIM1EN` reader - Low power timer clock enable bit
pub use TIM2EN_R as LPTIM1EN_R;
///Field `TIM3EN` writer - Timer 3 clock enbale bit
pub use TIM2EN_W as TIM3EN_W;
///Field `TIM6EN` writer - Timer 6 clock enable bit
pub use TIM2EN_W as TIM6EN_W;
///Field `TIM7EN` writer - Timer 7 clock enable bit
pub use TIM2EN_W as TIM7EN_W;
///Field `WWDGEN` writer - Window watchdog clock enable bit
pub use TIM2EN_W as WWDGEN_W;
///Field `SPI2EN` writer - SPI2 clock enable bit
pub use TIM2EN_W as SPI2EN_W;
///Field `USART2EN` writer - UART2 clock enable bit
pub use TIM2EN_W as USART2EN_W;
///Field `LPUART1EN` writer - LPUART1 clock enable bit
pub use TIM2EN_W as LPUART1EN_W;
///Field `USART4EN` writer - USART4 clock enable bit
pub use TIM2EN_W as USART4EN_W;
///Field `USART5EN` writer - USART5 clock enable bit
pub use TIM2EN_W as USART5EN_W;
///Field `I2C1EN` writer - I2C1 clock enable bit
pub use TIM2EN_W as I2C1EN_W;
///Field `I2C2EN` writer - I2C2 clock enable bit
pub use TIM2EN_W as I2C2EN_W;
///Field `PWREN` writer - Power interface clock enable bit
pub use TIM2EN_W as PWREN_W;
///Field `I2C3EN` writer - I2C3 clock enable bit
pub use TIM2EN_W as I2C3EN_W;
///Field `LPTIM1EN` writer - Low power timer clock enable bit
pub use TIM2EN_W as LPTIM1EN_W;
impl R {
    ///Bit 0 - Timer2 clock enable bit
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer 3 clock enbale bit
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Timer 6 clock enable bit
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer 7 clock enable bit
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable bit
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable bit
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - UART2 clock enable bit
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - LPUART1 clock enable bit
    #[inline(always)]
    pub fn lpuart1en(&self) -> LPUART1EN_R {
        LPUART1EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - USART4 clock enable bit
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - USART5 clock enable bit
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable bit
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable bit
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable bit
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 30 - I2C3 clock enable bit
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Low power timer clock enable bit
    #[inline(always)]
    pub fn lptim1en(&self) -> LPTIM1EN_R {
        LPTIM1EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR")
            .field("tim2en", &self.tim2en())
            .field("lptim1en", &self.lptim1en())
            .field("pwren", &self.pwren())
            .field("i2c2en", &self.i2c2en())
            .field("i2c1en", &self.i2c1en())
            .field("lpuart1en", &self.lpuart1en())
            .field("usart2en", &self.usart2en())
            .field("spi2en", &self.spi2en())
            .field("wwdgen", &self.wwdgen())
            .field("tim6en", &self.tim6en())
            .field("tim3en", &self.tim3en())
            .field("tim7en", &self.tim7en())
            .field("usart4en", &self.usart4en())
            .field("usart5en", &self.usart5en())
            .field("i2c3en", &self.i2c3en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer2 clock enable bit
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1ENRrs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - Timer 3 clock enbale bit
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APB1ENRrs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 4 - Timer 6 clock enable bit
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APB1ENRrs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - Timer 7 clock enable bit
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<'_, APB1ENRrs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 11 - Window watchdog clock enable bit
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APB1ENRrs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable bit
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APB1ENRrs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 17 - UART2 clock enable bit
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1ENRrs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - LPUART1 clock enable bit
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> LPUART1EN_W<'_, APB1ENRrs> {
        LPUART1EN_W::new(self, 18)
    }
    ///Bit 19 - USART4 clock enable bit
    #[inline(always)]
    pub fn usart4en(&mut self) -> USART4EN_W<'_, APB1ENRrs> {
        USART4EN_W::new(self, 19)
    }
    ///Bit 20 - USART5 clock enable bit
    #[inline(always)]
    pub fn usart5en(&mut self) -> USART5EN_W<'_, APB1ENRrs> {
        USART5EN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable bit
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1ENRrs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable bit
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APB1ENRrs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 28 - Power interface clock enable bit
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, APB1ENRrs> {
        PWREN_W::new(self, 28)
    }
    ///Bit 30 - I2C3 clock enable bit
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APB1ENRrs> {
        I2C3EN_W::new(self, 30)
    }
    ///Bit 31 - Low power timer clock enable bit
    #[inline(always)]
    pub fn lptim1en(&mut self) -> LPTIM1EN_W<'_, APB1ENRrs> {
        LPTIM1EN_W::new(self, 31)
    }
}
/**APB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#RCC:APB1ENR)*/
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
