///Register `APB1LRSTSR` writer
pub type W = crate::W<APB1LRSTSRrs>;
///Field `TIM2RSTS` writer - TIM2 reset
pub type TIM2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3RSTS` writer - TIM3 reset
pub type TIM3RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4RSTS` writer - TIM4 reset
pub type TIM4RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5RSTS` writer - TIM5 reset
pub type TIM5RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6RSTS` writer - TIM6 reset
pub type TIM6RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7RSTS` writer - TIM7 reset
pub type TIM7RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM12RSTS` writer - TIM12 reset
pub type TIM12RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13RSTS` writer - TIM13 reset
pub type TIM13RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14RSTS` writer - TIM14 reset
pub type TIM14RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1RSTS` writer - LPTIM1 reset
pub type LPTIM1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGRSTS` writer - WWDG reset
pub type WWDGRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10RSTS` writer - TIM10 reset
pub type TIM10RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11RSTS` writer - TIM11 reset
pub type TIM11RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2RSTS` writer - SPI2 reset
pub type SPI2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3RSTS` writer - SPI3 reset
pub type SPI3RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPDIFRX1RSTS` writer - SPDIFRX1 reset
pub type SPDIFRX1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2RSTS` writer - USART2 reset
pub type USART2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3RSTS` writer - USART3 reset
pub type USART3RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4RSTS` writer - UART4 reset
pub type UART4RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5RSTS` writer - UART5 reset
pub type UART5RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1RSTS` writer - I2C1 reset
pub type I2C1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2RSTS` writer - I2C2 reset
pub type I2C2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3RSTS` writer - I2C3 reset
pub type I2C3RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C1RSTS` writer - I3C1 reset
pub type I3C1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C2RSTS` writer - I3C2 reset
pub type I3C2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART7RSTS` writer - UART7 reset
pub type UART7RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART8RSTS` writer - UART8 reset
pub type UART8RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1LRSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM2 reset
    #[inline(always)]
    pub fn tim2rsts(&mut self) -> TIM2RSTS_W<'_, APB1LRSTSRrs> {
        TIM2RSTS_W::new(self, 0)
    }
    ///Bit 1 - TIM3 reset
    #[inline(always)]
    pub fn tim3rsts(&mut self) -> TIM3RSTS_W<'_, APB1LRSTSRrs> {
        TIM3RSTS_W::new(self, 1)
    }
    ///Bit 2 - TIM4 reset
    #[inline(always)]
    pub fn tim4rsts(&mut self) -> TIM4RSTS_W<'_, APB1LRSTSRrs> {
        TIM4RSTS_W::new(self, 2)
    }
    ///Bit 3 - TIM5 reset
    #[inline(always)]
    pub fn tim5rsts(&mut self) -> TIM5RSTS_W<'_, APB1LRSTSRrs> {
        TIM5RSTS_W::new(self, 3)
    }
    ///Bit 4 - TIM6 reset
    #[inline(always)]
    pub fn tim6rsts(&mut self) -> TIM6RSTS_W<'_, APB1LRSTSRrs> {
        TIM6RSTS_W::new(self, 4)
    }
    ///Bit 5 - TIM7 reset
    #[inline(always)]
    pub fn tim7rsts(&mut self) -> TIM7RSTS_W<'_, APB1LRSTSRrs> {
        TIM7RSTS_W::new(self, 5)
    }
    ///Bit 6 - TIM12 reset
    #[inline(always)]
    pub fn tim12rsts(&mut self) -> TIM12RSTS_W<'_, APB1LRSTSRrs> {
        TIM12RSTS_W::new(self, 6)
    }
    ///Bit 7 - TIM13 reset
    #[inline(always)]
    pub fn tim13rsts(&mut self) -> TIM13RSTS_W<'_, APB1LRSTSRrs> {
        TIM13RSTS_W::new(self, 7)
    }
    ///Bit 8 - TIM14 reset
    #[inline(always)]
    pub fn tim14rsts(&mut self) -> TIM14RSTS_W<'_, APB1LRSTSRrs> {
        TIM14RSTS_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 reset
    #[inline(always)]
    pub fn lptim1rsts(&mut self) -> LPTIM1RSTS_W<'_, APB1LRSTSRrs> {
        LPTIM1RSTS_W::new(self, 9)
    }
    ///Bit 11 - WWDG reset
    #[inline(always)]
    pub fn wwdgrsts(&mut self) -> WWDGRSTS_W<'_, APB1LRSTSRrs> {
        WWDGRSTS_W::new(self, 11)
    }
    ///Bit 12 - TIM10 reset
    #[inline(always)]
    pub fn tim10rsts(&mut self) -> TIM10RSTS_W<'_, APB1LRSTSRrs> {
        TIM10RSTS_W::new(self, 12)
    }
    ///Bit 13 - TIM11 reset
    #[inline(always)]
    pub fn tim11rsts(&mut self) -> TIM11RSTS_W<'_, APB1LRSTSRrs> {
        TIM11RSTS_W::new(self, 13)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rsts(&mut self) -> SPI2RSTS_W<'_, APB1LRSTSRrs> {
        SPI2RSTS_W::new(self, 14)
    }
    ///Bit 15 - SPI3 reset
    #[inline(always)]
    pub fn spi3rsts(&mut self) -> SPI3RSTS_W<'_, APB1LRSTSRrs> {
        SPI3RSTS_W::new(self, 15)
    }
    ///Bit 16 - SPDIFRX1 reset
    #[inline(always)]
    pub fn spdifrx1rsts(&mut self) -> SPDIFRX1RSTS_W<'_, APB1LRSTSRrs> {
        SPDIFRX1RSTS_W::new(self, 16)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rsts(&mut self) -> USART2RSTS_W<'_, APB1LRSTSRrs> {
        USART2RSTS_W::new(self, 17)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rsts(&mut self) -> USART3RSTS_W<'_, APB1LRSTSRrs> {
        USART3RSTS_W::new(self, 18)
    }
    ///Bit 19 - UART4 reset
    #[inline(always)]
    pub fn uart4rsts(&mut self) -> UART4RSTS_W<'_, APB1LRSTSRrs> {
        UART4RSTS_W::new(self, 19)
    }
    ///Bit 20 - UART5 reset
    #[inline(always)]
    pub fn uart5rsts(&mut self) -> UART5RSTS_W<'_, APB1LRSTSRrs> {
        UART5RSTS_W::new(self, 20)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rsts(&mut self) -> I2C1RSTS_W<'_, APB1LRSTSRrs> {
        I2C1RSTS_W::new(self, 21)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rsts(&mut self) -> I2C2RSTS_W<'_, APB1LRSTSRrs> {
        I2C2RSTS_W::new(self, 22)
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rsts(&mut self) -> I2C3RSTS_W<'_, APB1LRSTSRrs> {
        I2C3RSTS_W::new(self, 23)
    }
    ///Bit 24 - I3C1 reset
    #[inline(always)]
    pub fn i3c1rsts(&mut self) -> I3C1RSTS_W<'_, APB1LRSTSRrs> {
        I3C1RSTS_W::new(self, 24)
    }
    ///Bit 25 - I3C2 reset
    #[inline(always)]
    pub fn i3c2rsts(&mut self) -> I3C2RSTS_W<'_, APB1LRSTSRrs> {
        I3C2RSTS_W::new(self, 25)
    }
    ///Bit 30 - UART7 reset
    #[inline(always)]
    pub fn uart7rsts(&mut self) -> UART7RSTS_W<'_, APB1LRSTSRrs> {
        UART7RSTS_W::new(self, 30)
    }
    ///Bit 31 - UART8 reset
    #[inline(always)]
    pub fn uart8rsts(&mut self) -> UART8RSTS_W<'_, APB1LRSTSRrs> {
        UART8RSTS_W::new(self, 31)
    }
}
/**RCC APB1L reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LRSTSR)*/
pub struct APB1LRSTSRrs;
impl crate::RegisterSpec for APB1LRSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1lrstsr::W`](W) writer structure
impl crate::Writable for APB1LRSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LRSTSR to value 0
impl crate::Resettable for APB1LRSTSRrs {}
