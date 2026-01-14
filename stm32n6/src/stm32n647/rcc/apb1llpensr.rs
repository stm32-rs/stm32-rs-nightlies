///Register `APB1LLPENSR` writer
pub type W = crate::W<APB1LLPENSRrs>;
///Field `TIM2LPENS` writer - TIM2 sleep enable
pub type TIM2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3LPENS` writer - TIM3 sleep enable
pub type TIM3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4LPENS` writer - TIM4 sleep enable
pub type TIM4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5LPENS` writer - TIM5 sleep enable
pub type TIM5LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6LPENS` writer - TIM6 sleep enable
pub type TIM6LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7LPENS` writer - TIM7 sleep enable
pub type TIM7LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM12LPENS` writer - TIM12 sleep enable
pub type TIM12LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13LPENS` writer - TIM13 sleep enable
pub type TIM13LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14LPENS` writer - TIM14 sleep enable
pub type TIM14LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1LPENS` writer - LPTIM1 sleep enable
pub type LPTIM1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGLPENS` writer - WWDG sleep enable
pub type WWDGLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10LPENS` writer - TIM10 sleep enable
pub type TIM10LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11LPENS` writer - TIM11 sleep enable
pub type TIM11LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2LPENS` writer - SPI2 sleep enable
pub type SPI2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3LPENS` writer - SPI3 sleep enable
pub type SPI3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPDIFRX1LPENS` writer - SPDIFRX1 sleep enable
pub type SPDIFRX1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2LPENS` writer - USART2 sleep enable
pub type USART2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3LPENS` writer - USART3 sleep enable
pub type USART3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4LPENS` writer - UART4 sleep enable
pub type UART4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5LPENS` writer - UART5 sleep enable
pub type UART5LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1LPENS` writer - I2C1 sleep enable
pub type I2C1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2LPENS` writer - I2C2 sleep enable
pub type I2C2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3LPENS` writer - I2C3 sleep enable
pub type I2C3LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C1LPENS` writer - I3C1 sleep enable
pub type I3C1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C2LPENS` writer - I3C2 sleep enable
pub type I3C2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART7LPENS` writer - UART7 sleep enable
pub type UART7LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART8LPENS` writer - UART8 sleep enable
pub type UART8LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1LLPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM2 sleep enable
    #[inline(always)]
    pub fn tim2lpens(&mut self) -> TIM2LPENS_W<'_, APB1LLPENSRrs> {
        TIM2LPENS_W::new(self, 0)
    }
    ///Bit 1 - TIM3 sleep enable
    #[inline(always)]
    pub fn tim3lpens(&mut self) -> TIM3LPENS_W<'_, APB1LLPENSRrs> {
        TIM3LPENS_W::new(self, 1)
    }
    ///Bit 2 - TIM4 sleep enable
    #[inline(always)]
    pub fn tim4lpens(&mut self) -> TIM4LPENS_W<'_, APB1LLPENSRrs> {
        TIM4LPENS_W::new(self, 2)
    }
    ///Bit 3 - TIM5 sleep enable
    #[inline(always)]
    pub fn tim5lpens(&mut self) -> TIM5LPENS_W<'_, APB1LLPENSRrs> {
        TIM5LPENS_W::new(self, 3)
    }
    ///Bit 4 - TIM6 sleep enable
    #[inline(always)]
    pub fn tim6lpens(&mut self) -> TIM6LPENS_W<'_, APB1LLPENSRrs> {
        TIM6LPENS_W::new(self, 4)
    }
    ///Bit 5 - TIM7 sleep enable
    #[inline(always)]
    pub fn tim7lpens(&mut self) -> TIM7LPENS_W<'_, APB1LLPENSRrs> {
        TIM7LPENS_W::new(self, 5)
    }
    ///Bit 6 - TIM12 sleep enable
    #[inline(always)]
    pub fn tim12lpens(&mut self) -> TIM12LPENS_W<'_, APB1LLPENSRrs> {
        TIM12LPENS_W::new(self, 6)
    }
    ///Bit 7 - TIM13 sleep enable
    #[inline(always)]
    pub fn tim13lpens(&mut self) -> TIM13LPENS_W<'_, APB1LLPENSRrs> {
        TIM13LPENS_W::new(self, 7)
    }
    ///Bit 8 - TIM14 sleep enable
    #[inline(always)]
    pub fn tim14lpens(&mut self) -> TIM14LPENS_W<'_, APB1LLPENSRrs> {
        TIM14LPENS_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 sleep enable
    #[inline(always)]
    pub fn lptim1lpens(&mut self) -> LPTIM1LPENS_W<'_, APB1LLPENSRrs> {
        LPTIM1LPENS_W::new(self, 9)
    }
    ///Bit 11 - WWDG sleep enable
    #[inline(always)]
    pub fn wwdglpens(&mut self) -> WWDGLPENS_W<'_, APB1LLPENSRrs> {
        WWDGLPENS_W::new(self, 11)
    }
    ///Bit 12 - TIM10 sleep enable
    #[inline(always)]
    pub fn tim10lpens(&mut self) -> TIM10LPENS_W<'_, APB1LLPENSRrs> {
        TIM10LPENS_W::new(self, 12)
    }
    ///Bit 13 - TIM11 sleep enable
    #[inline(always)]
    pub fn tim11lpens(&mut self) -> TIM11LPENS_W<'_, APB1LLPENSRrs> {
        TIM11LPENS_W::new(self, 13)
    }
    ///Bit 14 - SPI2 sleep enable
    #[inline(always)]
    pub fn spi2lpens(&mut self) -> SPI2LPENS_W<'_, APB1LLPENSRrs> {
        SPI2LPENS_W::new(self, 14)
    }
    ///Bit 15 - SPI3 sleep enable
    #[inline(always)]
    pub fn spi3lpens(&mut self) -> SPI3LPENS_W<'_, APB1LLPENSRrs> {
        SPI3LPENS_W::new(self, 15)
    }
    ///Bit 16 - SPDIFRX1 sleep enable
    #[inline(always)]
    pub fn spdifrx1lpens(&mut self) -> SPDIFRX1LPENS_W<'_, APB1LLPENSRrs> {
        SPDIFRX1LPENS_W::new(self, 16)
    }
    ///Bit 17 - USART2 sleep enable
    #[inline(always)]
    pub fn usart2lpens(&mut self) -> USART2LPENS_W<'_, APB1LLPENSRrs> {
        USART2LPENS_W::new(self, 17)
    }
    ///Bit 18 - USART3 sleep enable
    #[inline(always)]
    pub fn usart3lpens(&mut self) -> USART3LPENS_W<'_, APB1LLPENSRrs> {
        USART3LPENS_W::new(self, 18)
    }
    ///Bit 19 - UART4 sleep enable
    #[inline(always)]
    pub fn uart4lpens(&mut self) -> UART4LPENS_W<'_, APB1LLPENSRrs> {
        UART4LPENS_W::new(self, 19)
    }
    ///Bit 20 - UART5 sleep enable
    #[inline(always)]
    pub fn uart5lpens(&mut self) -> UART5LPENS_W<'_, APB1LLPENSRrs> {
        UART5LPENS_W::new(self, 20)
    }
    ///Bit 21 - I2C1 sleep enable
    #[inline(always)]
    pub fn i2c1lpens(&mut self) -> I2C1LPENS_W<'_, APB1LLPENSRrs> {
        I2C1LPENS_W::new(self, 21)
    }
    ///Bit 22 - I2C2 sleep enable
    #[inline(always)]
    pub fn i2c2lpens(&mut self) -> I2C2LPENS_W<'_, APB1LLPENSRrs> {
        I2C2LPENS_W::new(self, 22)
    }
    ///Bit 23 - I2C3 sleep enable
    #[inline(always)]
    pub fn i2c3lpens(&mut self) -> I2C3LPENS_W<'_, APB1LLPENSRrs> {
        I2C3LPENS_W::new(self, 23)
    }
    ///Bit 24 - I3C1 sleep enable
    #[inline(always)]
    pub fn i3c1lpens(&mut self) -> I3C1LPENS_W<'_, APB1LLPENSRrs> {
        I3C1LPENS_W::new(self, 24)
    }
    ///Bit 25 - I3C2 sleep enable
    #[inline(always)]
    pub fn i3c2lpens(&mut self) -> I3C2LPENS_W<'_, APB1LLPENSRrs> {
        I3C2LPENS_W::new(self, 25)
    }
    ///Bit 30 - UART7 sleep enable
    #[inline(always)]
    pub fn uart7lpens(&mut self) -> UART7LPENS_W<'_, APB1LLPENSRrs> {
        UART7LPENS_W::new(self, 30)
    }
    ///Bit 31 - UART8 sleep enable
    #[inline(always)]
    pub fn uart8lpens(&mut self) -> UART8LPENS_W<'_, APB1LLPENSRrs> {
        UART8LPENS_W::new(self, 31)
    }
}
/**RCC APB1L Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LLPENSR)*/
pub struct APB1LLPENSRrs;
impl crate::RegisterSpec for APB1LLPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1llpensr::W`](W) writer structure
impl crate::Writable for APB1LLPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LLPENSR to value 0
impl crate::Resettable for APB1LLPENSRrs {}
