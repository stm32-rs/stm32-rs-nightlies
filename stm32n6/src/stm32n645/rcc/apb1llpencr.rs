///Register `APB1LLPENCR` writer
pub type W = crate::W<APB1LLPENCRrs>;
///Field `TIM2LPENC` writer - TIM2 sleep enable
pub type TIM2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3LPENC` writer - TIM3 sleep enable
pub type TIM3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4LPENC` writer - TIM4 sleep enable
pub type TIM4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5LPENC` writer - TIM5 sleep enable
pub type TIM5LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6LPENC` writer - TIM6 sleep enable
pub type TIM6LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7LPENC` writer - TIM7 sleep enable
pub type TIM7LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM12LPENC` writer - TIM12 sleep enable
pub type TIM12LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13LPENC` writer - TIM13 sleep enable
pub type TIM13LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14LPENC` writer - TIM14 sleep enable
pub type TIM14LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1LPENC` writer - LPTIM1 sleep enable
pub type LPTIM1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGLPENC` writer - WWDG sleep enable
pub type WWDGLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10LPENC` writer - TIM10 sleep enable
pub type TIM10LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11LPENC` writer - TIM11 sleep enable
pub type TIM11LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2LPENC` writer - SPI2 sleep enable
pub type SPI2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3LPENC` writer - SPI3 sleep enable
pub type SPI3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPDIFRX1LPENC` writer - SPDIFRX1 sleep enable
pub type SPDIFRX1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2LPENC` writer - USART2 sleep enable
pub type USART2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3LPENC` writer - USART3 sleep enable
pub type USART3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4LPENC` writer - UART4 sleep enable
pub type UART4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5LPENC` writer - UART5 sleep enable
pub type UART5LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1LPENC` writer - I2C1 sleep enable
pub type I2C1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2LPENC` writer - I2C2 sleep enable
pub type I2C2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3LPENC` writer - I2C3 sleep enable
pub type I2C3LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C1LPENC` writer - I3C1 sleep enable
pub type I3C1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C2LPENC` writer - I3C2 sleep enable
pub type I3C2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART7LPENC` writer - UART7 sleep enable
pub type UART7LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART8LPENC` writer - UART8 sleep enable
pub type UART8LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1LLPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM2 sleep enable
    #[inline(always)]
    pub fn tim2lpenc(&mut self) -> TIM2LPENC_W<'_, APB1LLPENCRrs> {
        TIM2LPENC_W::new(self, 0)
    }
    ///Bit 1 - TIM3 sleep enable
    #[inline(always)]
    pub fn tim3lpenc(&mut self) -> TIM3LPENC_W<'_, APB1LLPENCRrs> {
        TIM3LPENC_W::new(self, 1)
    }
    ///Bit 2 - TIM4 sleep enable
    #[inline(always)]
    pub fn tim4lpenc(&mut self) -> TIM4LPENC_W<'_, APB1LLPENCRrs> {
        TIM4LPENC_W::new(self, 2)
    }
    ///Bit 3 - TIM5 sleep enable
    #[inline(always)]
    pub fn tim5lpenc(&mut self) -> TIM5LPENC_W<'_, APB1LLPENCRrs> {
        TIM5LPENC_W::new(self, 3)
    }
    ///Bit 4 - TIM6 sleep enable
    #[inline(always)]
    pub fn tim6lpenc(&mut self) -> TIM6LPENC_W<'_, APB1LLPENCRrs> {
        TIM6LPENC_W::new(self, 4)
    }
    ///Bit 5 - TIM7 sleep enable
    #[inline(always)]
    pub fn tim7lpenc(&mut self) -> TIM7LPENC_W<'_, APB1LLPENCRrs> {
        TIM7LPENC_W::new(self, 5)
    }
    ///Bit 6 - TIM12 sleep enable
    #[inline(always)]
    pub fn tim12lpenc(&mut self) -> TIM12LPENC_W<'_, APB1LLPENCRrs> {
        TIM12LPENC_W::new(self, 6)
    }
    ///Bit 7 - TIM13 sleep enable
    #[inline(always)]
    pub fn tim13lpenc(&mut self) -> TIM13LPENC_W<'_, APB1LLPENCRrs> {
        TIM13LPENC_W::new(self, 7)
    }
    ///Bit 8 - TIM14 sleep enable
    #[inline(always)]
    pub fn tim14lpenc(&mut self) -> TIM14LPENC_W<'_, APB1LLPENCRrs> {
        TIM14LPENC_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 sleep enable
    #[inline(always)]
    pub fn lptim1lpenc(&mut self) -> LPTIM1LPENC_W<'_, APB1LLPENCRrs> {
        LPTIM1LPENC_W::new(self, 9)
    }
    ///Bit 11 - WWDG sleep enable
    #[inline(always)]
    pub fn wwdglpenc(&mut self) -> WWDGLPENC_W<'_, APB1LLPENCRrs> {
        WWDGLPENC_W::new(self, 11)
    }
    ///Bit 12 - TIM10 sleep enable
    #[inline(always)]
    pub fn tim10lpenc(&mut self) -> TIM10LPENC_W<'_, APB1LLPENCRrs> {
        TIM10LPENC_W::new(self, 12)
    }
    ///Bit 13 - TIM11 sleep enable
    #[inline(always)]
    pub fn tim11lpenc(&mut self) -> TIM11LPENC_W<'_, APB1LLPENCRrs> {
        TIM11LPENC_W::new(self, 13)
    }
    ///Bit 14 - SPI2 sleep enable
    #[inline(always)]
    pub fn spi2lpenc(&mut self) -> SPI2LPENC_W<'_, APB1LLPENCRrs> {
        SPI2LPENC_W::new(self, 14)
    }
    ///Bit 15 - SPI3 sleep enable
    #[inline(always)]
    pub fn spi3lpenc(&mut self) -> SPI3LPENC_W<'_, APB1LLPENCRrs> {
        SPI3LPENC_W::new(self, 15)
    }
    ///Bit 16 - SPDIFRX1 sleep enable
    #[inline(always)]
    pub fn spdifrx1lpenc(&mut self) -> SPDIFRX1LPENC_W<'_, APB1LLPENCRrs> {
        SPDIFRX1LPENC_W::new(self, 16)
    }
    ///Bit 17 - USART2 sleep enable
    #[inline(always)]
    pub fn usart2lpenc(&mut self) -> USART2LPENC_W<'_, APB1LLPENCRrs> {
        USART2LPENC_W::new(self, 17)
    }
    ///Bit 18 - USART3 sleep enable
    #[inline(always)]
    pub fn usart3lpenc(&mut self) -> USART3LPENC_W<'_, APB1LLPENCRrs> {
        USART3LPENC_W::new(self, 18)
    }
    ///Bit 19 - UART4 sleep enable
    #[inline(always)]
    pub fn uart4lpenc(&mut self) -> UART4LPENC_W<'_, APB1LLPENCRrs> {
        UART4LPENC_W::new(self, 19)
    }
    ///Bit 20 - UART5 sleep enable
    #[inline(always)]
    pub fn uart5lpenc(&mut self) -> UART5LPENC_W<'_, APB1LLPENCRrs> {
        UART5LPENC_W::new(self, 20)
    }
    ///Bit 21 - I2C1 sleep enable
    #[inline(always)]
    pub fn i2c1lpenc(&mut self) -> I2C1LPENC_W<'_, APB1LLPENCRrs> {
        I2C1LPENC_W::new(self, 21)
    }
    ///Bit 22 - I2C2 sleep enable
    #[inline(always)]
    pub fn i2c2lpenc(&mut self) -> I2C2LPENC_W<'_, APB1LLPENCRrs> {
        I2C2LPENC_W::new(self, 22)
    }
    ///Bit 23 - I2C3 sleep enable
    #[inline(always)]
    pub fn i2c3lpenc(&mut self) -> I2C3LPENC_W<'_, APB1LLPENCRrs> {
        I2C3LPENC_W::new(self, 23)
    }
    ///Bit 24 - I3C1 sleep enable
    #[inline(always)]
    pub fn i3c1lpenc(&mut self) -> I3C1LPENC_W<'_, APB1LLPENCRrs> {
        I3C1LPENC_W::new(self, 24)
    }
    ///Bit 25 - I3C2 sleep enable
    #[inline(always)]
    pub fn i3c2lpenc(&mut self) -> I3C2LPENC_W<'_, APB1LLPENCRrs> {
        I3C2LPENC_W::new(self, 25)
    }
    ///Bit 30 - UART7 sleep enable
    #[inline(always)]
    pub fn uart7lpenc(&mut self) -> UART7LPENC_W<'_, APB1LLPENCRrs> {
        UART7LPENC_W::new(self, 30)
    }
    ///Bit 31 - UART8 sleep enable
    #[inline(always)]
    pub fn uart8lpenc(&mut self) -> UART8LPENC_W<'_, APB1LLPENCRrs> {
        UART8LPENC_W::new(self, 31)
    }
}
/**RCC APB1L Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB1LLPENCR)*/
pub struct APB1LLPENCRrs;
impl crate::RegisterSpec for APB1LLPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1llpencr::W`](W) writer structure
impl crate::Writable for APB1LLPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LLPENCR to value 0
impl crate::Resettable for APB1LLPENCRrs {}
