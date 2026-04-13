///Register `APB2LPENSR` writer
pub type W = crate::W<APB2LPENSRrs>;
///Field `TIM1LPENS` writer - TIM1 sleep enable
pub type TIM1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8LPENS` writer - TIM8 sleep enable
pub type TIM8LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1LPENS` writer - USART1 sleep enable
pub type USART1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6LPENS` writer - USART6 sleep enable
pub type USART6LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART9LPENS` writer - UART9 sleep enable
pub type UART9LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART10LPENS` writer - USART10 sleep enable
pub type USART10LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1LPENS` writer - SPI1 sleep enable
pub type SPI1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4LPENS` writer - SPI4 sleep enable
pub type SPI4LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM18LPENS` writer - TIM18 sleep enable
pub type TIM18LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15LPENS` writer - TIM15 sleep enable
pub type TIM15LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16LPENS` writer - TIM16 sleep enable
pub type TIM16LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17LPENS` writer - TIM17 sleep enable
pub type TIM17LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9LPENS` writer - TIM9 sleep enable
pub type TIM9LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5LPENS` writer - SPI5 sleep enable
pub type SPI5LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1LPENS` writer - SAI1 sleep enable
pub type SAI1LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2LPENS` writer - SAI2 sleep enable
pub type SAI2LPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB2LPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM1 sleep enable
    #[inline(always)]
    pub fn tim1lpens(&mut self) -> TIM1LPENS_W<'_, APB2LPENSRrs> {
        TIM1LPENS_W::new(self, 0)
    }
    ///Bit 1 - TIM8 sleep enable
    #[inline(always)]
    pub fn tim8lpens(&mut self) -> TIM8LPENS_W<'_, APB2LPENSRrs> {
        TIM8LPENS_W::new(self, 1)
    }
    ///Bit 4 - USART1 sleep enable
    #[inline(always)]
    pub fn usart1lpens(&mut self) -> USART1LPENS_W<'_, APB2LPENSRrs> {
        USART1LPENS_W::new(self, 4)
    }
    ///Bit 5 - USART6 sleep enable
    #[inline(always)]
    pub fn usart6lpens(&mut self) -> USART6LPENS_W<'_, APB2LPENSRrs> {
        USART6LPENS_W::new(self, 5)
    }
    ///Bit 6 - UART9 sleep enable
    #[inline(always)]
    pub fn uart9lpens(&mut self) -> UART9LPENS_W<'_, APB2LPENSRrs> {
        UART9LPENS_W::new(self, 6)
    }
    ///Bit 7 - USART10 sleep enable
    #[inline(always)]
    pub fn usart10lpens(&mut self) -> USART10LPENS_W<'_, APB2LPENSRrs> {
        USART10LPENS_W::new(self, 7)
    }
    ///Bit 12 - SPI1 sleep enable
    #[inline(always)]
    pub fn spi1lpens(&mut self) -> SPI1LPENS_W<'_, APB2LPENSRrs> {
        SPI1LPENS_W::new(self, 12)
    }
    ///Bit 13 - SPI4 sleep enable
    #[inline(always)]
    pub fn spi4lpens(&mut self) -> SPI4LPENS_W<'_, APB2LPENSRrs> {
        SPI4LPENS_W::new(self, 13)
    }
    ///Bit 15 - TIM18 sleep enable
    #[inline(always)]
    pub fn tim18lpens(&mut self) -> TIM18LPENS_W<'_, APB2LPENSRrs> {
        TIM18LPENS_W::new(self, 15)
    }
    ///Bit 16 - TIM15 sleep enable
    #[inline(always)]
    pub fn tim15lpens(&mut self) -> TIM15LPENS_W<'_, APB2LPENSRrs> {
        TIM15LPENS_W::new(self, 16)
    }
    ///Bit 17 - TIM16 sleep enable
    #[inline(always)]
    pub fn tim16lpens(&mut self) -> TIM16LPENS_W<'_, APB2LPENSRrs> {
        TIM16LPENS_W::new(self, 17)
    }
    ///Bit 18 - TIM17 sleep enable
    #[inline(always)]
    pub fn tim17lpens(&mut self) -> TIM17LPENS_W<'_, APB2LPENSRrs> {
        TIM17LPENS_W::new(self, 18)
    }
    ///Bit 19 - TIM9 sleep enable
    #[inline(always)]
    pub fn tim9lpens(&mut self) -> TIM9LPENS_W<'_, APB2LPENSRrs> {
        TIM9LPENS_W::new(self, 19)
    }
    ///Bit 20 - SPI5 sleep enable
    #[inline(always)]
    pub fn spi5lpens(&mut self) -> SPI5LPENS_W<'_, APB2LPENSRrs> {
        SPI5LPENS_W::new(self, 20)
    }
    ///Bit 21 - SAI1 sleep enable
    #[inline(always)]
    pub fn sai1lpens(&mut self) -> SAI1LPENS_W<'_, APB2LPENSRrs> {
        SAI1LPENS_W::new(self, 21)
    }
    ///Bit 22 - SAI2 sleep enable
    #[inline(always)]
    pub fn sai2lpens(&mut self) -> SAI2LPENS_W<'_, APB2LPENSRrs> {
        SAI2LPENS_W::new(self, 22)
    }
}
/**RCC APB2 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB2LPENSR)*/
pub struct APB2LPENSRrs;
impl crate::RegisterSpec for APB2LPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb2lpensr::W`](W) writer structure
impl crate::Writable for APB2LPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2LPENSR to value 0
impl crate::Resettable for APB2LPENSRrs {}
