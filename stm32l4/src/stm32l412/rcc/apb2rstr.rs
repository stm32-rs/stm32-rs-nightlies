///Register `APB2RSTR` reader
pub type R = crate::R<APB2RSTRrs>;
///Register `APB2RSTR` writer
pub type W = crate::W<APB2RSTRrs>;
///Field `SYSCFGRST` reader - System configuration (SYSCFG) reset
pub type SYSCFGRST_R = crate::BitReader;
///Field `SYSCFGRST` writer - System configuration (SYSCFG) reset
pub type SYSCFGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM1RST` reader - TIM1 timer reset
pub type TIM1RST_R = crate::BitReader;
///Field `TIM1RST` writer - TIM1 timer reset
pub type TIM1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1RST` reader - SPI1 reset
pub type SPI1RST_R = crate::BitReader;
///Field `SPI1RST` writer - SPI1 reset
pub type SPI1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1RST` reader - USART1 reset
pub type USART1RST_R = crate::BitReader;
///Field `USART1RST` writer - USART1 reset
pub type USART1RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15RST` reader - TIM15 timer reset
pub type TIM15RST_R = crate::BitReader;
///Field `TIM15RST` writer - TIM15 timer reset
pub type TIM15RST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16RST` reader - TIM16 timer reset
pub type TIM16RST_R = crate::BitReader;
///Field `TIM16RST` writer - TIM16 timer reset
pub type TIM16RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - System configuration (SYSCFG) reset
    #[inline(always)]
    pub fn syscfgrst(&self) -> SYSCFGRST_R {
        SYSCFGRST_R::new((self.bits & 1) != 0)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&self) -> TIM1RST_R {
        TIM1RST_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rst(&self) -> SPI1RST_R {
        SPI1RST_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&self) -> USART1RST_R {
        USART1RST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    pub fn tim15rst(&self) -> TIM15RST_R {
        TIM15RST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    pub fn tim16rst(&self) -> TIM16RST_R {
        TIM16RST_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB2RSTR")
            .field("tim16rst", &self.tim16rst())
            .field("tim15rst", &self.tim15rst())
            .field("usart1rst", &self.usart1rst())
            .field("spi1rst", &self.spi1rst())
            .field("tim1rst", &self.tim1rst())
            .field("syscfgrst", &self.syscfgrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - System configuration (SYSCFG) reset
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SYSCFGRST_W<'_, APB2RSTRrs> {
        SYSCFGRST_W::new(self, 0)
    }
    ///Bit 11 - TIM1 timer reset
    #[inline(always)]
    pub fn tim1rst(&mut self) -> TIM1RST_W<'_, APB2RSTRrs> {
        TIM1RST_W::new(self, 11)
    }
    ///Bit 12 - SPI1 reset
    #[inline(always)]
    pub fn spi1rst(&mut self) -> SPI1RST_W<'_, APB2RSTRrs> {
        SPI1RST_W::new(self, 12)
    }
    ///Bit 14 - USART1 reset
    #[inline(always)]
    pub fn usart1rst(&mut self) -> USART1RST_W<'_, APB2RSTRrs> {
        USART1RST_W::new(self, 14)
    }
    ///Bit 16 - TIM15 timer reset
    #[inline(always)]
    pub fn tim15rst(&mut self) -> TIM15RST_W<'_, APB2RSTRrs> {
        TIM15RST_W::new(self, 16)
    }
    ///Bit 17 - TIM16 timer reset
    #[inline(always)]
    pub fn tim16rst(&mut self) -> TIM16RST_W<'_, APB2RSTRrs> {
        TIM16RST_W::new(self, 17)
    }
}
/**APB2 peripheral reset register

You can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L412.html#RCC:APB2RSTR)*/
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
