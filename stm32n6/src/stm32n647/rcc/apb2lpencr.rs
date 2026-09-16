///Register `APB2LPENCR` writer
pub type W = crate::W<APB2LPENCRrs>;
///Field `TIM1LPENC` writer - TIM1 sleep enable
pub type TIM1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8LPENC` writer - TIM8 sleep enable
pub type TIM8LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1LPENC` writer - USART1 sleep enable
pub type USART1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6LPENC` writer - USART6 sleep enable
pub type USART6LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART9LPENC` writer - UART9 sleep enable
pub type UART9LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART10LPENC` writer - USART10 sleep enable
pub type USART10LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1LPENC` writer - SPI1 sleep enable
pub type SPI1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4LPENC` writer - SPI4 sleep enable
pub type SPI4LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM18LPENC` writer - TIM18 sleep enable
pub type TIM18LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15LPENC` writer - TIM15 sleep enable
pub type TIM15LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16LPENC` writer - TIM16 sleep enable
pub type TIM16LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17LPENC` writer - TIM17 sleep enable
pub type TIM17LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9LPENC` writer - TIM9 sleep enable
pub type TIM9LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5LPENC` writer - SPI5 sleep enable
pub type SPI5LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1LPENC` writer - SAI1 sleep enable
pub type SAI1LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2LPENC` writer - SAI2 sleep enable
pub type SAI2LPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB2LPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM1 sleep enable
    #[inline(always)]
    pub fn tim1lpenc(&mut self) -> TIM1LPENC_W<'_, APB2LPENCRrs> {
        TIM1LPENC_W::new(self, 0)
    }
    ///Bit 1 - TIM8 sleep enable
    #[inline(always)]
    pub fn tim8lpenc(&mut self) -> TIM8LPENC_W<'_, APB2LPENCRrs> {
        TIM8LPENC_W::new(self, 1)
    }
    ///Bit 4 - USART1 sleep enable
    #[inline(always)]
    pub fn usart1lpenc(&mut self) -> USART1LPENC_W<'_, APB2LPENCRrs> {
        USART1LPENC_W::new(self, 4)
    }
    ///Bit 5 - USART6 sleep enable
    #[inline(always)]
    pub fn usart6lpenc(&mut self) -> USART6LPENC_W<'_, APB2LPENCRrs> {
        USART6LPENC_W::new(self, 5)
    }
    ///Bit 6 - UART9 sleep enable
    #[inline(always)]
    pub fn uart9lpenc(&mut self) -> UART9LPENC_W<'_, APB2LPENCRrs> {
        UART9LPENC_W::new(self, 6)
    }
    ///Bit 7 - USART10 sleep enable
    #[inline(always)]
    pub fn usart10lpenc(&mut self) -> USART10LPENC_W<'_, APB2LPENCRrs> {
        USART10LPENC_W::new(self, 7)
    }
    ///Bit 12 - SPI1 sleep enable
    #[inline(always)]
    pub fn spi1lpenc(&mut self) -> SPI1LPENC_W<'_, APB2LPENCRrs> {
        SPI1LPENC_W::new(self, 12)
    }
    ///Bit 13 - SPI4 sleep enable
    #[inline(always)]
    pub fn spi4lpenc(&mut self) -> SPI4LPENC_W<'_, APB2LPENCRrs> {
        SPI4LPENC_W::new(self, 13)
    }
    ///Bit 15 - TIM18 sleep enable
    #[inline(always)]
    pub fn tim18lpenc(&mut self) -> TIM18LPENC_W<'_, APB2LPENCRrs> {
        TIM18LPENC_W::new(self, 15)
    }
    ///Bit 16 - TIM15 sleep enable
    #[inline(always)]
    pub fn tim15lpenc(&mut self) -> TIM15LPENC_W<'_, APB2LPENCRrs> {
        TIM15LPENC_W::new(self, 16)
    }
    ///Bit 17 - TIM16 sleep enable
    #[inline(always)]
    pub fn tim16lpenc(&mut self) -> TIM16LPENC_W<'_, APB2LPENCRrs> {
        TIM16LPENC_W::new(self, 17)
    }
    ///Bit 18 - TIM17 sleep enable
    #[inline(always)]
    pub fn tim17lpenc(&mut self) -> TIM17LPENC_W<'_, APB2LPENCRrs> {
        TIM17LPENC_W::new(self, 18)
    }
    ///Bit 19 - TIM9 sleep enable
    #[inline(always)]
    pub fn tim9lpenc(&mut self) -> TIM9LPENC_W<'_, APB2LPENCRrs> {
        TIM9LPENC_W::new(self, 19)
    }
    ///Bit 20 - SPI5 sleep enable
    #[inline(always)]
    pub fn spi5lpenc(&mut self) -> SPI5LPENC_W<'_, APB2LPENCRrs> {
        SPI5LPENC_W::new(self, 20)
    }
    ///Bit 21 - SAI1 sleep enable
    #[inline(always)]
    pub fn sai1lpenc(&mut self) -> SAI1LPENC_W<'_, APB2LPENCRrs> {
        SAI1LPENC_W::new(self, 21)
    }
    ///Bit 22 - SAI2 sleep enable
    #[inline(always)]
    pub fn sai2lpenc(&mut self) -> SAI2LPENC_W<'_, APB2LPENCRrs> {
        SAI2LPENC_W::new(self, 22)
    }
}
/**RCC APB2 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2LPENCR)*/
pub struct APB2LPENCRrs;
impl crate::RegisterSpec for APB2LPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb2lpencr::W`](W) writer structure
impl crate::Writable for APB2LPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2LPENCR to value 0
impl crate::Resettable for APB2LPENCRrs {}
