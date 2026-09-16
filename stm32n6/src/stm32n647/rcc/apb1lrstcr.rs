///Register `APB1LRSTCR` writer
pub type W = crate::W<APB1LRSTCRrs>;
///Field `TIM2RSTC` writer - TIM2 reset
pub type TIM2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3RSTC` writer - TIM3 reset
pub type TIM3RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4RSTC` writer - TIM4 reset
pub type TIM4RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5RSTC` writer - TIM5 reset
pub type TIM5RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6RSTC` writer - TIM6 reset
pub type TIM6RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7RSTC` writer - TIM7 reset
pub type TIM7RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM12RSTC` writer - TIM12 reset
pub type TIM12RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13RSTC` writer - TIM13 reset
pub type TIM13RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14RSTC` writer - TIM14 reset
pub type TIM14RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1RSTC` writer - LPTIM1 reset
pub type LPTIM1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGRSTC` writer - WWDG reset
pub type WWDGRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10RSTC` writer - TIM10 reset
pub type TIM10RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11RSTC` writer - TIM11 reset
pub type TIM11RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2RSTC` writer - SPI2 reset
pub type SPI2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3RSTC` writer - SPI3 reset
pub type SPI3RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPDIFRX1RSTC` writer - SPDIFRX1 reset
pub type SPDIFRX1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2RSTC` writer - USART2 reset
pub type USART2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3RSTC` writer - USART3 reset
pub type USART3RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4RSTC` writer - UART4 reset
pub type UART4RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5RSTC` writer - UART5 reset
pub type UART5RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1RSTC` writer - I2C1 reset
pub type I2C1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2RSTC` writer - I2C2 reset
pub type I2C2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3RSTC` writer - I2C3 reset
pub type I2C3RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C1RSTC` writer - I3C1 reset
pub type I3C1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C2RSTC` writer - I3C2 reset
pub type I3C2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART7RSTC` writer - UART7 reset
pub type UART7RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART8RSTC` writer - UART8 reset
pub type UART8RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1LRSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM2 reset
    #[inline(always)]
    pub fn tim2rstc(&mut self) -> TIM2RSTC_W<'_, APB1LRSTCRrs> {
        TIM2RSTC_W::new(self, 0)
    }
    ///Bit 1 - TIM3 reset
    #[inline(always)]
    pub fn tim3rstc(&mut self) -> TIM3RSTC_W<'_, APB1LRSTCRrs> {
        TIM3RSTC_W::new(self, 1)
    }
    ///Bit 2 - TIM4 reset
    #[inline(always)]
    pub fn tim4rstc(&mut self) -> TIM4RSTC_W<'_, APB1LRSTCRrs> {
        TIM4RSTC_W::new(self, 2)
    }
    ///Bit 3 - TIM5 reset
    #[inline(always)]
    pub fn tim5rstc(&mut self) -> TIM5RSTC_W<'_, APB1LRSTCRrs> {
        TIM5RSTC_W::new(self, 3)
    }
    ///Bit 4 - TIM6 reset
    #[inline(always)]
    pub fn tim6rstc(&mut self) -> TIM6RSTC_W<'_, APB1LRSTCRrs> {
        TIM6RSTC_W::new(self, 4)
    }
    ///Bit 5 - TIM7 reset
    #[inline(always)]
    pub fn tim7rstc(&mut self) -> TIM7RSTC_W<'_, APB1LRSTCRrs> {
        TIM7RSTC_W::new(self, 5)
    }
    ///Bit 6 - TIM12 reset
    #[inline(always)]
    pub fn tim12rstc(&mut self) -> TIM12RSTC_W<'_, APB1LRSTCRrs> {
        TIM12RSTC_W::new(self, 6)
    }
    ///Bit 7 - TIM13 reset
    #[inline(always)]
    pub fn tim13rstc(&mut self) -> TIM13RSTC_W<'_, APB1LRSTCRrs> {
        TIM13RSTC_W::new(self, 7)
    }
    ///Bit 8 - TIM14 reset
    #[inline(always)]
    pub fn tim14rstc(&mut self) -> TIM14RSTC_W<'_, APB1LRSTCRrs> {
        TIM14RSTC_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 reset
    #[inline(always)]
    pub fn lptim1rstc(&mut self) -> LPTIM1RSTC_W<'_, APB1LRSTCRrs> {
        LPTIM1RSTC_W::new(self, 9)
    }
    ///Bit 11 - WWDG reset
    #[inline(always)]
    pub fn wwdgrstc(&mut self) -> WWDGRSTC_W<'_, APB1LRSTCRrs> {
        WWDGRSTC_W::new(self, 11)
    }
    ///Bit 12 - TIM10 reset
    #[inline(always)]
    pub fn tim10rstc(&mut self) -> TIM10RSTC_W<'_, APB1LRSTCRrs> {
        TIM10RSTC_W::new(self, 12)
    }
    ///Bit 13 - TIM11 reset
    #[inline(always)]
    pub fn tim11rstc(&mut self) -> TIM11RSTC_W<'_, APB1LRSTCRrs> {
        TIM11RSTC_W::new(self, 13)
    }
    ///Bit 14 - SPI2 reset
    #[inline(always)]
    pub fn spi2rstc(&mut self) -> SPI2RSTC_W<'_, APB1LRSTCRrs> {
        SPI2RSTC_W::new(self, 14)
    }
    ///Bit 15 - SPI3 reset
    #[inline(always)]
    pub fn spi3rstc(&mut self) -> SPI3RSTC_W<'_, APB1LRSTCRrs> {
        SPI3RSTC_W::new(self, 15)
    }
    ///Bit 16 - SPDIFRX1 reset
    #[inline(always)]
    pub fn spdifrx1rstc(&mut self) -> SPDIFRX1RSTC_W<'_, APB1LRSTCRrs> {
        SPDIFRX1RSTC_W::new(self, 16)
    }
    ///Bit 17 - USART2 reset
    #[inline(always)]
    pub fn usart2rstc(&mut self) -> USART2RSTC_W<'_, APB1LRSTCRrs> {
        USART2RSTC_W::new(self, 17)
    }
    ///Bit 18 - USART3 reset
    #[inline(always)]
    pub fn usart3rstc(&mut self) -> USART3RSTC_W<'_, APB1LRSTCRrs> {
        USART3RSTC_W::new(self, 18)
    }
    ///Bit 19 - UART4 reset
    #[inline(always)]
    pub fn uart4rstc(&mut self) -> UART4RSTC_W<'_, APB1LRSTCRrs> {
        UART4RSTC_W::new(self, 19)
    }
    ///Bit 20 - UART5 reset
    #[inline(always)]
    pub fn uart5rstc(&mut self) -> UART5RSTC_W<'_, APB1LRSTCRrs> {
        UART5RSTC_W::new(self, 20)
    }
    ///Bit 21 - I2C1 reset
    #[inline(always)]
    pub fn i2c1rstc(&mut self) -> I2C1RSTC_W<'_, APB1LRSTCRrs> {
        I2C1RSTC_W::new(self, 21)
    }
    ///Bit 22 - I2C2 reset
    #[inline(always)]
    pub fn i2c2rstc(&mut self) -> I2C2RSTC_W<'_, APB1LRSTCRrs> {
        I2C2RSTC_W::new(self, 22)
    }
    ///Bit 23 - I2C3 reset
    #[inline(always)]
    pub fn i2c3rstc(&mut self) -> I2C3RSTC_W<'_, APB1LRSTCRrs> {
        I2C3RSTC_W::new(self, 23)
    }
    ///Bit 24 - I3C1 reset
    #[inline(always)]
    pub fn i3c1rstc(&mut self) -> I3C1RSTC_W<'_, APB1LRSTCRrs> {
        I3C1RSTC_W::new(self, 24)
    }
    ///Bit 25 - I3C2 reset
    #[inline(always)]
    pub fn i3c2rstc(&mut self) -> I3C2RSTC_W<'_, APB1LRSTCRrs> {
        I3C2RSTC_W::new(self, 25)
    }
    ///Bit 30 - UART7 reset
    #[inline(always)]
    pub fn uart7rstc(&mut self) -> UART7RSTC_W<'_, APB1LRSTCRrs> {
        UART7RSTC_W::new(self, 30)
    }
    ///Bit 31 - UART8 reset
    #[inline(always)]
    pub fn uart8rstc(&mut self) -> UART8RSTC_W<'_, APB1LRSTCRrs> {
        UART8RSTC_W::new(self, 31)
    }
}
/**RCC APB1L reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LRSTCR)*/
pub struct APB1LRSTCRrs;
impl crate::RegisterSpec for APB1LRSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1lrstcr::W`](W) writer structure
impl crate::Writable for APB1LRSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LRSTCR to value 0
impl crate::Resettable for APB1LRSTCRrs {}
