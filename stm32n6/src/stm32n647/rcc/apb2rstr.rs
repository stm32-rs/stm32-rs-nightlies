///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
///Field `TIM1RST` reader - TIM1 reset
pub type TIM1RST_R = crate::BitReader;
///Field `TIM1RST` writer - TIM1 reset
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8RST` reader - TIM8 reset
pub type TIM8RST_R = crate::BitReader;
///Field `TIM8RST` writer - TIM8 reset
pub type TIM8RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1RST` reader - USART1 reset
pub type USART1RST_R = crate::BitReader;
///Field `USART1RST` writer - USART1 reset
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6RST` reader - USART6 reset
pub type USART6RST_R = crate::BitReader;
///Field `USART6RST` writer - USART6 reset
pub type USART6RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART9RST` reader - UART9 reset
pub type UART9RST_R = crate::BitReader;
///Field `UART9RST` writer - UART9 reset
pub type UART9RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART10RST` reader - USART10 reset
pub type USART10RST_R = crate::BitReader;
///Field `USART10RST` writer - USART10 reset
pub type USART10RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1RST` reader - SPI1 reset
pub type SPI1RST_R = crate::BitReader;
///Field `SPI1RST` writer - SPI1 reset
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4RST` reader - SPI4 reset
pub type SPI4RST_R = crate::BitReader;
///Field `SPI4RST` writer - SPI4 reset
pub type SPI4RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM18RST` reader - TIM18 reset
pub type TIM18RST_R = crate::BitReader;
///Field `TIM18RST` writer - TIM18 reset
pub type TIM18RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15RST` reader - TIM15 reset
pub type TIM15RST_R = crate::BitReader;
///Field `TIM15RST` writer - TIM15 reset
pub type TIM15RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16RST` reader - TIM16 reset
pub type TIM16RST_R = crate::BitReader;
///Field `TIM16RST` writer - TIM16 reset
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17RST` reader - TIM17 reset
pub type TIM17RST_R = crate::BitReader;
///Field `TIM17RST` writer - TIM17 reset
pub type TIM17RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9RST` reader - TIM9 reset
pub type TIM9RST_R = crate::BitReader;
///Field `TIM9RST` writer - TIM9 reset
pub type TIM9RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5RST` reader - SPI5 reset
pub type SPI5RST_R = crate::BitReader;
///Field `SPI5RST` writer - SPI5 reset
pub type SPI5RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1RST` reader - SAI1 reset
pub type SAI1RST_R = crate::BitReader;
///Field `SAI1RST` writer - SAI1 reset
pub type SAI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2RST` reader - SAI2 reset
pub type SAI2RST_R = crate::BitReader;
///Field `SAI2RST` writer - SAI2 reset
pub type SAI2RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM8 reset
    #[inline(always)]
    pub fn tim8rst(&self) -> TIM8RST_R {
        TIM8RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    pub fn usart6rst(&self) -> USART6RST_R {
        USART6RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - UART9 reset
    #[inline(always)]
    pub fn uart9rst(&self) -> UART9RST_R {
        UART9RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - USART10 reset
    #[inline(always)]
    pub fn usart10rst(&self) -> USART10RST_R {
        USART10RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SPI4 reset
    #[inline(always)]
    pub fn spi4rst(&self) -> SPI4RST_R {
        SPI4RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - TIM18 reset
    #[inline(always)]
    pub fn tim18rst(&self) -> TIM18RST_R {
        TIM18RST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - TIM15 reset
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 reset
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM17 reset
    #[inline(always)]
    pub fn tim17rst(&self) -> TIM17RST_R {
        TIM17RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - TIM9 reset
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SPI5 reset
    #[inline(always)]
    pub fn spi5rst(&self) -> SPI5RST_R {
        SPI5RST_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SAI1 reset
    #[inline(always)]
    pub fn sai1rst(&self) -> SAI1RST_R {
        SAI1RST_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SAI2 reset
    #[inline(always)]
    pub fn sai2rst(&self) -> SAI2RST_R {
        SAI2RST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("tim1rst", &self.tim1rst())
            .field("tim8rst", &self.tim8rst())
            .field("usart1rst", &self.usart1rst())
            .field("usart6rst", &self.usart6rst())
            .field("uart9rst", &self.uart9rst())
            .field("usart10rst", &self.usart10rst())
            .field("spi1rst", &self.spi1rst())
            .field("spi4rst", &self.spi4rst())
            .field("tim18rst", &self.tim18rst())
            .field("tim15rst", &self.tim15rst())
            .field("tim16rst", &self.tim16rst())
            .field("tim17rst", &self.tim17rst())
            .field("tim9rst", &self.tim9rst())
            .field("spi5rst", &self.spi5rst())
            .field("sai1rst", &self.sai1rst())
            .field("sai2rst", &self.sai2rst())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM1 reset
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, APB2RSTRrs> {
        TIM1RST_W::new(self, 0)
    }
    ///Bit 1 - TIM8 reset
    #[inline(always)]
    pub fn tim8rst(&mut self) -> TIM8RST_W<'_, APB2RSTRrs> {
        TIM8RST_W::new(self, 1)
    }
    ///Bit 4 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB2RSTRrs> {
        USART1RST_W::new(self, 4)
    }
    ///Bit 5 - USART6 reset
    #[inline(always)]
    pub fn usart6rst(&mut self) -> USART6RST_W<'_, APB2RSTRrs> {
        USART6RST_W::new(self, 5)
    }
    ///Bit 6 - UART9 reset
    #[inline(always)]
    pub fn uart9rst(&mut self) -> UART9RST_W<'_, APB2RSTRrs> {
        UART9RST_W::new(self, 6)
    }
    ///Bit 7 - USART10 reset
    #[inline(always)]
    pub fn usart10rst(&mut self) -> USART10RST_W<'_, APB2RSTRrs> {
        USART10RST_W::new(self, 7)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 13 - SPI4 reset
    #[inline(always)]
    pub fn spi4rst(&mut self) -> SPI4RST_W<'_, APB2RSTRrs> {
        SPI4RST_W::new(self, 13)
    }
    ///Bit 15 - TIM18 reset
    #[inline(always)]
    pub fn tim18rst(&mut self) -> TIM18RST_W<'_, APB2RSTRrs> {
        TIM18RST_W::new(self, 15)
    }
    ///Bit 16 - TIM15 reset
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<'_, APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    ///Bit 17 - TIM16 reset
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
    ///Bit 18 - TIM17 reset
    #[inline(always)]
    pub fn tim17rst(&mut self) -> TIM17RST_W<'_, APB2RSTRrs> {
        TIM17RST_W::new(self, 18)
    }
    ///Bit 19 - TIM9 reset
    #[inline(always)]
    pub fn tim9rst(&mut self) -> TIM9RST_W<'_, APB2RSTRrs> {
        TIM9RST_W::new(self, 19)
    }
    ///Bit 20 - SPI5 reset
    #[inline(always)]
    pub fn spi5rst(&mut self) -> SPI5RST_W<'_, APB2RSTRrs> {
        SPI5RST_W::new(self, 20)
    }
    ///Bit 21 - SAI1 reset
    #[inline(always)]
    pub fn sai1rst(&mut self) -> SAI1RST_W<'_, APB2RSTRrs> {
        SAI1RST_W::new(self, 21)
    }
    ///Bit 22 - SAI2 reset
    #[inline(always)]
    pub fn sai2rst(&mut self) -> SAI2RST_W<'_, APB2RSTRrs> {
        SAI2RST_W::new(self, 22)
    }
}
/**RCC APB2 reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:APB2RSTR)*/
pub struct APB2RSTRrs;
impl crate::RegisterSpec for APB2RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`apb2rstr::R`](R) reader structure
impl crate::Readable for APB2RSTRrs {}
///`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure
impl crate::Writable for APB2RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2RSTR to value 0
impl crate::Resettable for APB2RSTRrs {}
