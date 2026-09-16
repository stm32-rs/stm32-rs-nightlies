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
///Field `ADCRST` reader - ADC interface reset (common to all ADCs)
pub type ADCRST_R = crate::BitReader;
///Field `ADCRST` writer - ADC interface reset (common to all ADCs)
pub type ADCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIORST` reader - SDIO reset
pub type SDIORST_R = crate::BitReader;
///Field `SDIORST` writer - SDIO reset
pub type SDIORST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1RST` reader - SPI 1 reset
pub type SPI1RST_R = crate::BitReader;
///Field `SPI1RST` writer - SPI 1 reset
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYSCFGRST` reader - System configuration controller reset
pub type SYSCFGRST_R = crate::BitReader;
///Field `SYSCFGRST` writer - System configuration controller reset
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9RST` reader - TIM9 reset
pub type TIM9RST_R = crate::BitReader;
///Field `TIM9RST` writer - TIM9 reset
pub type TIM9RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10RST` reader - TIM10 reset
pub type TIM10RST_R = crate::BitReader;
///Field `TIM10RST` writer - TIM10 reset
pub type TIM10RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11RST` reader - TIM11 reset
pub type TIM11RST_R = crate::BitReader;
///Field `TIM11RST` writer - TIM11 reset
pub type TIM11RST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 8 - ADC interface reset (common to all ADCs)
    #[inline(always)]
    pub fn adcrst(&self) -> ADCRST_R {
        ADCRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - SDIO reset
    #[inline(always)]
    pub fn sdiorst(&self) -> SDIORST_R {
        SDIORST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - System configuration controller reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM9 reset
    #[inline(always)]
    pub fn tim9rst(&self) -> TIM9RST_R {
        TIM9RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM10 reset
    #[inline(always)]
    pub fn tim10rst(&self) -> TIM10RST_R {
        TIM10RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - TIM11 reset
    #[inline(always)]
    pub fn tim11rst(&self) -> TIM11RST_R {
        TIM11RST_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("tim1rst", &self.tim1rst())
            .field("tim8rst", &self.tim8rst())
            .field("usart1rst", &self.usart1rst())
            .field("usart6rst", &self.usart6rst())
            .field("adcrst", &self.adcrst())
            .field("sdiorst", &self.sdiorst())
            .field("spi1rst", &self.spi1rst())
            .field("syscfgrst", &self.syscfgrst())
            .field("tim9rst", &self.tim9rst())
            .field("tim10rst", &self.tim10rst())
            .field("tim11rst", &self.tim11rst())
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
    ///Bit 8 - ADC interface reset (common to all ADCs)
    #[inline(always)]
    pub fn adcrst(&mut self) -> ADCRST_W<'_, APB2RSTRrs> {
        ADCRST_W::new(self, 8)
    }
    ///Bit 11 - SDIO reset
    #[inline(always)]
    pub fn sdiorst(&mut self) -> SDIORST_W<'_, APB2RSTRrs> {
        SDIORST_W::new(self, 11)
    }
    ///Bit 12 - SPI 1 reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 14 - System configuration controller reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB2RSTRrs> {
        SYSCFGRST_W::new(self, 14)
    }
    ///Bit 16 - TIM9 reset
    #[inline(always)]
    pub fn tim9rst(&mut self) -> TIM9RST_W<'_, APB2RSTRrs> {
        TIM9RST_W::new(self, 16)
    }
    ///Bit 17 - TIM10 reset
    #[inline(always)]
    pub fn tim10rst(&mut self) -> TIM10RST_W<'_, APB2RSTRrs> {
        TIM10RST_W::new(self, 17)
    }
    ///Bit 18 - TIM11 reset
    #[inline(always)]
    pub fn tim11rst(&mut self) -> TIM11RST_W<'_, APB2RSTRrs> {
        TIM11RST_W::new(self, 18)
    }
}
/**APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#RCC:APB2RSTR)*/
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
