///Register `APB2ENCR` writer
pub type W = crate::W<APB2ENCRrs>;
///Field `TIM1ENC` writer - TIM1 enable
pub type TIM1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8ENC` writer - TIM8 enable
pub type TIM8ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1ENC` writer - USART1 enable
pub type USART1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6ENC` writer - USART6 enable
pub type USART6ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART9ENC` writer - UART9 enable
pub type UART9ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART10ENC` writer - USART10 enable
pub type USART10ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1ENC` writer - SPI1 enable
pub type SPI1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4ENC` writer - SPI4 enable
pub type SPI4ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM18ENC` writer - TIM18 enable
pub type TIM18ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15ENC` writer - TIM15 enable
pub type TIM15ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16ENC` writer - TIM16 enable
pub type TIM16ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17ENC` writer - TIM17 enable
pub type TIM17ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9ENC` writer - TIM9 enable
pub type TIM9ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5ENC` writer - SPI5 enable
pub type SPI5ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1ENC` writer - SAI1 enable
pub type SAI1ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2ENC` writer - SAI2 enable
pub type SAI2ENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB2ENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM1 enable
    #[inline(always)]
    pub fn tim1enc(&mut self) -> TIM1ENC_W<'_, APB2ENCRrs> {
        TIM1ENC_W::new(self, 0)
    }
    ///Bit 1 - TIM8 enable
    #[inline(always)]
    pub fn tim8enc(&mut self) -> TIM8ENC_W<'_, APB2ENCRrs> {
        TIM8ENC_W::new(self, 1)
    }
    ///Bit 4 - USART1 enable
    #[inline(always)]
    pub fn usart1enc(&mut self) -> USART1ENC_W<'_, APB2ENCRrs> {
        USART1ENC_W::new(self, 4)
    }
    ///Bit 5 - USART6 enable
    #[inline(always)]
    pub fn usart6enc(&mut self) -> USART6ENC_W<'_, APB2ENCRrs> {
        USART6ENC_W::new(self, 5)
    }
    ///Bit 6 - UART9 enable
    #[inline(always)]
    pub fn uart9enc(&mut self) -> UART9ENC_W<'_, APB2ENCRrs> {
        UART9ENC_W::new(self, 6)
    }
    ///Bit 7 - USART10 enable
    #[inline(always)]
    pub fn usart10enc(&mut self) -> USART10ENC_W<'_, APB2ENCRrs> {
        USART10ENC_W::new(self, 7)
    }
    ///Bit 12 - SPI1 enable
    #[inline(always)]
    pub fn spi1enc(&mut self) -> SPI1ENC_W<'_, APB2ENCRrs> {
        SPI1ENC_W::new(self, 12)
    }
    ///Bit 13 - SPI4 enable
    #[inline(always)]
    pub fn spi4enc(&mut self) -> SPI4ENC_W<'_, APB2ENCRrs> {
        SPI4ENC_W::new(self, 13)
    }
    ///Bit 15 - TIM18 enable
    #[inline(always)]
    pub fn tim18enc(&mut self) -> TIM18ENC_W<'_, APB2ENCRrs> {
        TIM18ENC_W::new(self, 15)
    }
    ///Bit 16 - TIM15 enable
    #[inline(always)]
    pub fn tim15enc(&mut self) -> TIM15ENC_W<'_, APB2ENCRrs> {
        TIM15ENC_W::new(self, 16)
    }
    ///Bit 17 - TIM16 enable
    #[inline(always)]
    pub fn tim16enc(&mut self) -> TIM16ENC_W<'_, APB2ENCRrs> {
        TIM16ENC_W::new(self, 17)
    }
    ///Bit 18 - TIM17 enable
    #[inline(always)]
    pub fn tim17enc(&mut self) -> TIM17ENC_W<'_, APB2ENCRrs> {
        TIM17ENC_W::new(self, 18)
    }
    ///Bit 19 - TIM9 enable
    #[inline(always)]
    pub fn tim9enc(&mut self) -> TIM9ENC_W<'_, APB2ENCRrs> {
        TIM9ENC_W::new(self, 19)
    }
    ///Bit 20 - SPI5 enable
    #[inline(always)]
    pub fn spi5enc(&mut self) -> SPI5ENC_W<'_, APB2ENCRrs> {
        SPI5ENC_W::new(self, 20)
    }
    ///Bit 21 - SAI1 enable
    #[inline(always)]
    pub fn sai1enc(&mut self) -> SAI1ENC_W<'_, APB2ENCRrs> {
        SAI1ENC_W::new(self, 21)
    }
    ///Bit 22 - SAI2 enable
    #[inline(always)]
    pub fn sai2enc(&mut self) -> SAI2ENC_W<'_, APB2ENCRrs> {
        SAI2ENC_W::new(self, 22)
    }
}
/**RCC APB2 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2encr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB2ENCR)*/
pub struct APB2ENCRrs;
impl crate::RegisterSpec for APB2ENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb2encr::W`](W) writer structure
impl crate::Writable for APB2ENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2ENCR to value 0
impl crate::Resettable for APB2ENCRrs {}
