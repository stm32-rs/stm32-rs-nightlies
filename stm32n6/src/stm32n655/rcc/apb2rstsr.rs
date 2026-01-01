///Register `APB2RSTSR` writer
pub type W = crate::W<APB2RSTSRrs>;
///Field `TIM1RSTS` writer - TIM1 reset
pub type TIM1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8RSTS` writer - TIM8 reset
pub type TIM8RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1RSTS` writer - USART1 reset
pub type USART1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6RSTS` writer - USART6 reset
pub type USART6RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART9RSTS` writer - UART9 reset
pub type UART9RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART10RSTS` writer - USART10 reset
pub type USART10RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1RSTS` writer - SPI1 reset
pub type SPI1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4RSTS` writer - SPI4 reset
pub type SPI4RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM18RSTS` writer - TIM18 reset
pub type TIM18RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15RSTS` writer - TIM15 reset
pub type TIM15RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16RSTS` writer - TIM16 reset
pub type TIM16RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17RSTS` writer - TIM17 reset
pub type TIM17RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9RSTS` writer - TIM9 reset
pub type TIM9RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5RSTS` writer - SPI5 reset
pub type SPI5RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1RSTS` writer - SAI1 reset
pub type SAI1RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2RSTS` writer - SAI2 reset
pub type SAI2RSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB2RSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    pub fn tim1rsts(&mut self) -> TIM1RSTS_W<'_, APB2RSTSRrs> {
        TIM1RSTS_W::new(self, 0)
    }
    ///Bit 1 - TIM8 reset
    #[inline(always)]
    pub fn tim8rsts(&mut self) -> TIM8RSTS_W<'_, APB2RSTSRrs> {
        TIM8RSTS_W::new(self, 1)
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    pub fn usart1rsts(&mut self) -> USART1RSTS_W<'_, APB2RSTSRrs> {
        USART1RSTS_W::new(self, 4)
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    pub fn usart6rsts(&mut self) -> USART6RSTS_W<'_, APB2RSTSRrs> {
        USART6RSTS_W::new(self, 5)
    }
    ///Bit 6 - UART9 reset
    #[inline(always)]
    pub fn uart9rsts(&mut self) -> UART9RSTS_W<'_, APB2RSTSRrs> {
        UART9RSTS_W::new(self, 6)
    }
    ///Bit 7 - USART10 reset
    #[inline(always)]
    pub fn usart10rsts(&mut self) -> USART10RSTS_W<'_, APB2RSTSRrs> {
        USART10RSTS_W::new(self, 7)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rsts(&mut self) -> SPI1RSTS_W<'_, APB2RSTSRrs> {
        SPI1RSTS_W::new(self, 12)
    }
    ///Bit 13 - SPI4 reset
    #[inline(always)]
    pub fn spi4rsts(&mut self) -> SPI4RSTS_W<'_, APB2RSTSRrs> {
        SPI4RSTS_W::new(self, 13)
    }
    ///Bit 15 - TIM18 reset
    #[inline(always)]
    pub fn tim18rsts(&mut self) -> TIM18RSTS_W<'_, APB2RSTSRrs> {
        TIM18RSTS_W::new(self, 15)
    }
    ///Bit 16 - TIM15 reset
    #[inline(always)]
    pub fn tim15rsts(&mut self) -> TIM15RSTS_W<'_, APB2RSTSRrs> {
        TIM15RSTS_W::new(self, 16)
    }
    ///Bit 17 - TIM16 reset
    #[inline(always)]
    pub fn tim16rsts(&mut self) -> TIM16RSTS_W<'_, APB2RSTSRrs> {
        TIM16RSTS_W::new(self, 17)
    }
    ///Bit 18 - TIM17 reset
    #[inline(always)]
    pub fn tim17rsts(&mut self) -> TIM17RSTS_W<'_, APB2RSTSRrs> {
        TIM17RSTS_W::new(self, 18)
    }
    ///Bit 19 - TIM9 reset
    #[inline(always)]
    pub fn tim9rsts(&mut self) -> TIM9RSTS_W<'_, APB2RSTSRrs> {
        TIM9RSTS_W::new(self, 19)
    }
    ///Bit 20 - SPI5 reset
    #[inline(always)]
    pub fn spi5rsts(&mut self) -> SPI5RSTS_W<'_, APB2RSTSRrs> {
        SPI5RSTS_W::new(self, 20)
    }
    ///Bit 21 - SAI1 reset
    #[inline(always)]
    pub fn sai1rsts(&mut self) -> SAI1RSTS_W<'_, APB2RSTSRrs> {
        SAI1RSTS_W::new(self, 21)
    }
    ///Bit 22 - SAI2 reset
    #[inline(always)]
    pub fn sai2rsts(&mut self) -> SAI2RSTS_W<'_, APB2RSTSRrs> {
        SAI2RSTS_W::new(self, 22)
    }
}
/**RCC APB2 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB2RSTSR)*/
pub struct APB2RSTSRrs;
impl crate::RegisterSpec for APB2RSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb2rstsr::W`](W) writer structure
impl crate::Writable for APB2RSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2RSTSR to value 0
impl crate::Resettable for APB2RSTSRrs {}
