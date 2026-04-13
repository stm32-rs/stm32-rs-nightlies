///Register `APB2LPENR` reader
pub type R = crate::R<APB2LPENRrs>;
///Register `APB2LPENR` writer
pub type W = crate::W<APB2LPENRrs>;
///Field `TIM1LPEN` reader - TIM1 sleep enable
pub type TIM1LPEN_R = crate::BitReader;
///Field `TIM1LPEN` writer - TIM1 sleep enable
pub type TIM1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8LPEN` reader - TIM8 sleep enable
pub type TIM8LPEN_R = crate::BitReader;
///Field `TIM8LPEN` writer - TIM8 sleep enable
pub type TIM8LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1LPEN` reader - USART1 sleep enable
pub type USART1LPEN_R = crate::BitReader;
///Field `USART1LPEN` writer - USART1 sleep enable
pub type USART1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6LPEN` reader - USART6 sleep enable
pub type USART6LPEN_R = crate::BitReader;
///Field `USART6LPEN` writer - USART6 sleep enable
pub type USART6LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART9LPEN` reader - UART9 sleep enable
pub type UART9LPEN_R = crate::BitReader;
///Field `UART9LPEN` writer - UART9 sleep enable
pub type UART9LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART10LPEN` reader - USART10 sleep enable
pub type USART10LPEN_R = crate::BitReader;
///Field `USART10LPEN` writer - USART10 sleep enable
pub type USART10LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1LPEN` reader - SPI1 sleep enable
pub type SPI1LPEN_R = crate::BitReader;
///Field `SPI1LPEN` writer - SPI1 sleep enable
pub type SPI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4LPEN` reader - SPI4 sleep enable
pub type SPI4LPEN_R = crate::BitReader;
///Field `SPI4LPEN` writer - SPI4 sleep enable
pub type SPI4LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM18LPEN` reader - TIM18 sleep enable
pub type TIM18LPEN_R = crate::BitReader;
///Field `TIM18LPEN` writer - TIM18 sleep enable
pub type TIM18LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15LPEN` reader - TIM15 sleep enable
pub type TIM15LPEN_R = crate::BitReader;
///Field `TIM15LPEN` writer - TIM15 sleep enable
pub type TIM15LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16LPEN` reader - TIM16 sleep enable
pub type TIM16LPEN_R = crate::BitReader;
///Field `TIM16LPEN` writer - TIM16 sleep enable
pub type TIM16LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17LPEN` reader - TIM17 sleep enable
pub type TIM17LPEN_R = crate::BitReader;
///Field `TIM17LPEN` writer - TIM17 sleep enable
pub type TIM17LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9LPEN` reader - TIM9 sleep enable
pub type TIM9LPEN_R = crate::BitReader;
///Field `TIM9LPEN` writer - TIM9 sleep enable
pub type TIM9LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5LPEN` reader - SPI5 sleep enable
pub type SPI5LPEN_R = crate::BitReader;
///Field `SPI5LPEN` writer - SPI5 sleep enable
pub type SPI5LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1LPEN` reader - SAI1 sleep enable
pub type SAI1LPEN_R = crate::BitReader;
///Field `SAI1LPEN` writer - SAI1 sleep enable
pub type SAI1LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2LPEN` reader - SAI2 sleep enable
pub type SAI2LPEN_R = crate::BitReader;
///Field `SAI2LPEN` writer - SAI2 sleep enable
pub type SAI2LPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1 sleep enable
    #[inline(always)]
    pub fn tim1lpen(&self) -> TIM1LPEN_R {
        TIM1LPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 sleep enable
    #[inline(always)]
    pub fn tim8lpen(&self) -> TIM8LPEN_R {
        TIM8LPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USART1 sleep enable
    #[inline(always)]
    pub fn usart1lpen(&self) -> USART1LPEN_R {
        USART1LPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 sleep enable
    #[inline(always)]
    pub fn usart6lpen(&self) -> USART6LPEN_R {
        USART6LPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - UART9 sleep enable
    #[inline(always)]
    pub fn uart9lpen(&self) -> UART9LPEN_R {
        UART9LPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USART10 sleep enable
    #[inline(always)]
    pub fn usart10lpen(&self) -> USART10LPEN_R {
        USART10LPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - SPI1 sleep enable
    #[inline(always)]
    pub fn spi1lpen(&self) -> SPI1LPEN_R {
        SPI1LPEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI4 sleep enable
    #[inline(always)]
    pub fn spi4lpen(&self) -> SPI4LPEN_R {
        SPI4LPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - TIM18 sleep enable
    #[inline(always)]
    pub fn tim18lpen(&self) -> TIM18LPEN_R {
        TIM18LPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIM15 sleep enable
    #[inline(always)]
    pub fn tim15lpen(&self) -> TIM15LPEN_R {
        TIM15LPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 sleep enable
    #[inline(always)]
    pub fn tim16lpen(&self) -> TIM16LPEN_R {
        TIM16LPEN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 sleep enable
    #[inline(always)]
    pub fn tim17lpen(&self) -> TIM17LPEN_R {
        TIM17LPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIM9 sleep enable
    #[inline(always)]
    pub fn tim9lpen(&self) -> TIM9LPEN_R {
        TIM9LPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SPI5 sleep enable
    #[inline(always)]
    pub fn spi5lpen(&self) -> SPI5LPEN_R {
        SPI5LPEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SAI1 sleep enable
    #[inline(always)]
    pub fn sai1lpen(&self) -> SAI1LPEN_R {
        SAI1LPEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 sleep enable
    #[inline(always)]
    pub fn sai2lpen(&self) -> SAI2LPEN_R {
        SAI2LPEN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2LPENR")
            .field("tim1lpen", &self.tim1lpen())
            .field("tim8lpen", &self.tim8lpen())
            .field("usart1lpen", &self.usart1lpen())
            .field("usart6lpen", &self.usart6lpen())
            .field("uart9lpen", &self.uart9lpen())
            .field("usart10lpen", &self.usart10lpen())
            .field("spi1lpen", &self.spi1lpen())
            .field("spi4lpen", &self.spi4lpen())
            .field("tim18lpen", &self.tim18lpen())
            .field("tim15lpen", &self.tim15lpen())
            .field("tim16lpen", &self.tim16lpen())
            .field("tim17lpen", &self.tim17lpen())
            .field("tim9lpen", &self.tim9lpen())
            .field("spi5lpen", &self.spi5lpen())
            .field("sai1lpen", &self.sai1lpen())
            .field("sai2lpen", &self.sai2lpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 sleep enable
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> TIM1LPEN_W<'_, APB2LPENRrs> {
        TIM1LPEN_W::new(self, 0)
    }
    ///Bit 1 - TIM8 sleep enable
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> TIM8LPEN_W<'_, APB2LPENRrs> {
        TIM8LPEN_W::new(self, 1)
    }
    ///Bit 4 - USART1 sleep enable
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> USART1LPEN_W<'_, APB2LPENRrs> {
        USART1LPEN_W::new(self, 4)
    }
    ///Bit 5 - USART6 sleep enable
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> USART6LPEN_W<'_, APB2LPENRrs> {
        USART6LPEN_W::new(self, 5)
    }
    ///Bit 6 - UART9 sleep enable
    #[inline(always)]
    pub fn uart9lpen(&mut self) -> UART9LPEN_W<'_, APB2LPENRrs> {
        UART9LPEN_W::new(self, 6)
    }
    ///Bit 7 - USART10 sleep enable
    #[inline(always)]
    pub fn usart10lpen(&mut self) -> USART10LPEN_W<'_, APB2LPENRrs> {
        USART10LPEN_W::new(self, 7)
    }
    ///Bit 12 - SPI1 sleep enable
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> SPI1LPEN_W<'_, APB2LPENRrs> {
        SPI1LPEN_W::new(self, 12)
    }
    ///Bit 13 - SPI4 sleep enable
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> SPI4LPEN_W<'_, APB2LPENRrs> {
        SPI4LPEN_W::new(self, 13)
    }
    ///Bit 15 - TIM18 sleep enable
    #[inline(always)]
    pub fn tim18lpen(&mut self) -> TIM18LPEN_W<'_, APB2LPENRrs> {
        TIM18LPEN_W::new(self, 15)
    }
    ///Bit 16 - TIM15 sleep enable
    #[inline(always)]
    pub fn tim15lpen(&mut self) -> TIM15LPEN_W<'_, APB2LPENRrs> {
        TIM15LPEN_W::new(self, 16)
    }
    ///Bit 17 - TIM16 sleep enable
    #[inline(always)]
    pub fn tim16lpen(&mut self) -> TIM16LPEN_W<'_, APB2LPENRrs> {
        TIM16LPEN_W::new(self, 17)
    }
    ///Bit 18 - TIM17 sleep enable
    #[inline(always)]
    pub fn tim17lpen(&mut self) -> TIM17LPEN_W<'_, APB2LPENRrs> {
        TIM17LPEN_W::new(self, 18)
    }
    ///Bit 19 - TIM9 sleep enable
    #[inline(always)]
    pub fn tim9lpen(&mut self) -> TIM9LPEN_W<'_, APB2LPENRrs> {
        TIM9LPEN_W::new(self, 19)
    }
    ///Bit 20 - SPI5 sleep enable
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> SPI5LPEN_W<'_, APB2LPENRrs> {
        SPI5LPEN_W::new(self, 20)
    }
    ///Bit 21 - SAI1 sleep enable
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> SAI1LPEN_W<'_, APB2LPENRrs> {
        SAI1LPEN_W::new(self, 21)
    }
    ///Bit 22 - SAI2 sleep enable
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> SAI2LPEN_W<'_, APB2LPENRrs> {
        SAI2LPEN_W::new(self, 22)
    }
}
/**RCC APB2 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:APB2LPENR)*/
pub struct APB2LPENRrs;
impl crate::RegisterSpec for APB2LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2lpenr::R`](R) reader structure
impl crate::Readable for APB2LPENRrs {}
///`write(|w| ..)` method takes [`apb2lpenr::W`](W) writer structure
impl crate::Writable for APB2LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2LPENR to value 0
impl crate::Resettable for APB2LPENRrs {}
