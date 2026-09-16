///Register `APB2RSTCR` writer
pub type W = crate::W<APB2RSTCRrs>;
///Field `TIM1RSTC` writer - TIM1 reset
pub type TIM1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8RSTC` writer - TIM8 reset
pub type TIM8RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1RSTC` writer - USART1 reset
pub type USART1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6RSTC` writer - USART6 reset
pub type USART6RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART9RSTC` writer - UART9 reset
pub type UART9RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART10RSTC` writer - USART10 reset
pub type USART10RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1RSTC` writer - SPI1 reset
pub type SPI1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4RSTC` writer - SPI4 reset
pub type SPI4RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM18RSTC` writer - TIM18 reset
pub type TIM18RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15RSTC` writer - TIM15 reset
pub type TIM15RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16RSTC` writer - TIM16 reset
pub type TIM16RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17RSTC` writer - TIM17 reset
pub type TIM17RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9RSTC` writer - TIM9 reset
pub type TIM9RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5RSTC` writer - SPI5 reset
pub type SPI5RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1RSTC` writer - SAI1 reset
pub type SAI1RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2RSTC` writer - SAI2 reset
pub type SAI2RSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB2RSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    pub fn tim1rstc(&mut self) -> TIM1RSTC_W<'_, APB2RSTCRrs> {
        TIM1RSTC_W::new(self, 0)
    }
    ///Bit 1 - TIM8 reset
    #[inline(always)]
    pub fn tim8rstc(&mut self) -> TIM8RSTC_W<'_, APB2RSTCRrs> {
        TIM8RSTC_W::new(self, 1)
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    pub fn usart1rstc(&mut self) -> USART1RSTC_W<'_, APB2RSTCRrs> {
        USART1RSTC_W::new(self, 4)
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    pub fn usart6rstc(&mut self) -> USART6RSTC_W<'_, APB2RSTCRrs> {
        USART6RSTC_W::new(self, 5)
    }
    ///Bit 6 - UART9 reset
    #[inline(always)]
    pub fn uart9rstc(&mut self) -> UART9RSTC_W<'_, APB2RSTCRrs> {
        UART9RSTC_W::new(self, 6)
    }
    ///Bit 7 - USART10 reset
    #[inline(always)]
    pub fn usart10rstc(&mut self) -> USART10RSTC_W<'_, APB2RSTCRrs> {
        USART10RSTC_W::new(self, 7)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rstc(&mut self) -> SPI1RSTC_W<'_, APB2RSTCRrs> {
        SPI1RSTC_W::new(self, 12)
    }
    ///Bit 13 - SPI4 reset
    #[inline(always)]
    pub fn spi4rstc(&mut self) -> SPI4RSTC_W<'_, APB2RSTCRrs> {
        SPI4RSTC_W::new(self, 13)
    }
    ///Bit 15 - TIM18 reset
    #[inline(always)]
    pub fn tim18rstc(&mut self) -> TIM18RSTC_W<'_, APB2RSTCRrs> {
        TIM18RSTC_W::new(self, 15)
    }
    ///Bit 16 - TIM15 reset
    #[inline(always)]
    pub fn tim15rstc(&mut self) -> TIM15RSTC_W<'_, APB2RSTCRrs> {
        TIM15RSTC_W::new(self, 16)
    }
    ///Bit 17 - TIM16 reset
    #[inline(always)]
    pub fn tim16rstc(&mut self) -> TIM16RSTC_W<'_, APB2RSTCRrs> {
        TIM16RSTC_W::new(self, 17)
    }
    ///Bit 18 - TIM17 reset
    #[inline(always)]
    pub fn tim17rstc(&mut self) -> TIM17RSTC_W<'_, APB2RSTCRrs> {
        TIM17RSTC_W::new(self, 18)
    }
    ///Bit 19 - TIM9 reset
    #[inline(always)]
    pub fn tim9rstc(&mut self) -> TIM9RSTC_W<'_, APB2RSTCRrs> {
        TIM9RSTC_W::new(self, 19)
    }
    ///Bit 20 - SPI5 reset
    #[inline(always)]
    pub fn spi5rstc(&mut self) -> SPI5RSTC_W<'_, APB2RSTCRrs> {
        SPI5RSTC_W::new(self, 20)
    }
    ///Bit 21 - SAI1 reset
    #[inline(always)]
    pub fn sai1rstc(&mut self) -> SAI1RSTC_W<'_, APB2RSTCRrs> {
        SAI1RSTC_W::new(self, 21)
    }
    ///Bit 22 - SAI2 reset
    #[inline(always)]
    pub fn sai2rstc(&mut self) -> SAI2RSTC_W<'_, APB2RSTCRrs> {
        SAI2RSTC_W::new(self, 22)
    }
}
/**RCC APB2 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2RSTCR)*/
pub struct APB2RSTCRrs;
impl crate::RegisterSpec for APB2RSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb2rstcr::W`](W) writer structure
impl crate::Writable for APB2RSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2RSTCR to value 0
impl crate::Resettable for APB2RSTCRrs {}
