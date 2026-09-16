///Register `APB1LENCR` writer
pub type W = crate::W<APB1LENCRrs>;
///Field `TIM2ENC` writer - TIM2 enable
pub type TIM2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3ENC` writer - TIM3 enable
pub type TIM3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4ENC` writer - TIM4 enable
pub type TIM4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5ENC` writer - TIM5 enable
pub type TIM5ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6ENC` writer - TIM6 enable
pub type TIM6ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7ENC` writer - TIM7 enable
pub type TIM7ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM12ENC` writer - TIM12 enable
pub type TIM12ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13ENC` writer - TIM13 enable
pub type TIM13ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14ENC` writer - TIM14 enable
pub type TIM14ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1ENC` writer - LPTIM1 enable
pub type LPTIM1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10ENC` writer - TIM10 enable
pub type TIM10ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11ENC` writer - TIM11 enable
pub type TIM11ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2ENC` writer - SPI2 enable
pub type SPI2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3ENC` writer - SPI3 enable
pub type SPI3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPDIFRX1ENC` writer - SPDIFRX1 enable
pub type SPDIFRX1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2ENC` writer - USART2 enable
pub type USART2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3ENC` writer - USART3 enable
pub type USART3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4ENC` writer - UART4 enable
pub type UART4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5ENC` writer - UART5 enable
pub type UART5ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1ENC` writer - I2C1 enable
pub type I2C1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2ENC` writer - I2C2 enable
pub type I2C2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3ENC` writer - I2C3 enable
pub type I2C3ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C1ENC` writer - I3C1 enable
pub type I3C1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C2ENC` writer - I3C2 enable
pub type I3C2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART7ENC` writer - UART7 enable
pub type UART7ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART8ENC` writer - UART8 enable
pub type UART8ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1LENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM2 enable
    #[inline(always)]
    pub fn tim2enc(&mut self) -> TIM2ENC_W<'_, APB1LENCRrs> {
        TIM2ENC_W::new(self, 0)
    }
    ///Bit 1 - TIM3 enable
    #[inline(always)]
    pub fn tim3enc(&mut self) -> TIM3ENC_W<'_, APB1LENCRrs> {
        TIM3ENC_W::new(self, 1)
    }
    ///Bit 2 - TIM4 enable
    #[inline(always)]
    pub fn tim4enc(&mut self) -> TIM4ENC_W<'_, APB1LENCRrs> {
        TIM4ENC_W::new(self, 2)
    }
    ///Bit 3 - TIM5 enable
    #[inline(always)]
    pub fn tim5enc(&mut self) -> TIM5ENC_W<'_, APB1LENCRrs> {
        TIM5ENC_W::new(self, 3)
    }
    ///Bit 4 - TIM6 enable
    #[inline(always)]
    pub fn tim6enc(&mut self) -> TIM6ENC_W<'_, APB1LENCRrs> {
        TIM6ENC_W::new(self, 4)
    }
    ///Bit 5 - TIM7 enable
    #[inline(always)]
    pub fn tim7enc(&mut self) -> TIM7ENC_W<'_, APB1LENCRrs> {
        TIM7ENC_W::new(self, 5)
    }
    ///Bit 6 - TIM12 enable
    #[inline(always)]
    pub fn tim12enc(&mut self) -> TIM12ENC_W<'_, APB1LENCRrs> {
        TIM12ENC_W::new(self, 6)
    }
    ///Bit 7 - TIM13 enable
    #[inline(always)]
    pub fn tim13enc(&mut self) -> TIM13ENC_W<'_, APB1LENCRrs> {
        TIM13ENC_W::new(self, 7)
    }
    ///Bit 8 - TIM14 enable
    #[inline(always)]
    pub fn tim14enc(&mut self) -> TIM14ENC_W<'_, APB1LENCRrs> {
        TIM14ENC_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 enable
    #[inline(always)]
    pub fn lptim1enc(&mut self) -> LPTIM1ENC_W<'_, APB1LENCRrs> {
        LPTIM1ENC_W::new(self, 9)
    }
    ///Bit 12 - TIM10 enable
    #[inline(always)]
    pub fn tim10enc(&mut self) -> TIM10ENC_W<'_, APB1LENCRrs> {
        TIM10ENC_W::new(self, 12)
    }
    ///Bit 13 - TIM11 enable
    #[inline(always)]
    pub fn tim11enc(&mut self) -> TIM11ENC_W<'_, APB1LENCRrs> {
        TIM11ENC_W::new(self, 13)
    }
    ///Bit 14 - SPI2 enable
    #[inline(always)]
    pub fn spi2enc(&mut self) -> SPI2ENC_W<'_, APB1LENCRrs> {
        SPI2ENC_W::new(self, 14)
    }
    ///Bit 15 - SPI3 enable
    #[inline(always)]
    pub fn spi3enc(&mut self) -> SPI3ENC_W<'_, APB1LENCRrs> {
        SPI3ENC_W::new(self, 15)
    }
    ///Bit 16 - SPDIFRX1 enable
    #[inline(always)]
    pub fn spdifrx1enc(&mut self) -> SPDIFRX1ENC_W<'_, APB1LENCRrs> {
        SPDIFRX1ENC_W::new(self, 16)
    }
    ///Bit 17 - USART2 enable
    #[inline(always)]
    pub fn usart2enc(&mut self) -> USART2ENC_W<'_, APB1LENCRrs> {
        USART2ENC_W::new(self, 17)
    }
    ///Bit 18 - USART3 enable
    #[inline(always)]
    pub fn usart3enc(&mut self) -> USART3ENC_W<'_, APB1LENCRrs> {
        USART3ENC_W::new(self, 18)
    }
    ///Bit 19 - UART4 enable
    #[inline(always)]
    pub fn uart4enc(&mut self) -> UART4ENC_W<'_, APB1LENCRrs> {
        UART4ENC_W::new(self, 19)
    }
    ///Bit 20 - UART5 enable
    #[inline(always)]
    pub fn uart5enc(&mut self) -> UART5ENC_W<'_, APB1LENCRrs> {
        UART5ENC_W::new(self, 20)
    }
    ///Bit 21 - I2C1 enable
    #[inline(always)]
    pub fn i2c1enc(&mut self) -> I2C1ENC_W<'_, APB1LENCRrs> {
        I2C1ENC_W::new(self, 21)
    }
    ///Bit 22 - I2C2 enable
    #[inline(always)]
    pub fn i2c2enc(&mut self) -> I2C2ENC_W<'_, APB1LENCRrs> {
        I2C2ENC_W::new(self, 22)
    }
    ///Bit 23 - I2C3 enable
    #[inline(always)]
    pub fn i2c3enc(&mut self) -> I2C3ENC_W<'_, APB1LENCRrs> {
        I2C3ENC_W::new(self, 23)
    }
    ///Bit 24 - I3C1 enable
    #[inline(always)]
    pub fn i3c1enc(&mut self) -> I3C1ENC_W<'_, APB1LENCRrs> {
        I3C1ENC_W::new(self, 24)
    }
    ///Bit 25 - I3C2 enable
    #[inline(always)]
    pub fn i3c2enc(&mut self) -> I3C2ENC_W<'_, APB1LENCRrs> {
        I3C2ENC_W::new(self, 25)
    }
    ///Bit 30 - UART7 enable
    #[inline(always)]
    pub fn uart7enc(&mut self) -> UART7ENC_W<'_, APB1LENCRrs> {
        UART7ENC_W::new(self, 30)
    }
    ///Bit 31 - UART8 enable
    #[inline(always)]
    pub fn uart8enc(&mut self) -> UART8ENC_W<'_, APB1LENCRrs> {
        UART8ENC_W::new(self, 31)
    }
}
/**RCC APB1L enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB1LENCR)*/
pub struct APB1LENCRrs;
impl crate::RegisterSpec for APB1LENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1lencr::W`](W) writer structure
impl crate::Writable for APB1LENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LENCR to value 0
impl crate::Resettable for APB1LENCRrs {}
