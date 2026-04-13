///Register `APB1ENR` reader
pub type R = crate::R<APB1ENRrs>;
///Register `APB1ENR` writer
pub type W = crate::W<APB1ENRrs>;
///Field `TIM2EN` reader - Timer 2 clock enable
pub type TIM2EN_R = crate::BitReader;
///Field `TIM2EN` writer - Timer 2 clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3EN` reader - Timer 3 clock enable
pub type TIM3EN_R = crate::BitReader;
///Field `TIM3EN` writer - Timer 3 clock enable
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4EN` reader - Timer 4 clock enable
pub type TIM4EN_R = crate::BitReader;
///Field `TIM4EN` writer - Timer 4 clock enable
pub type TIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5EN` reader - Timer 5 clock enable
pub type TIM5EN_R = crate::BitReader;
///Field `TIM5EN` writer - Timer 5 clock enable
pub type TIM5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6EN` reader - Timer 6 clock enable
pub type TIM6EN_R = crate::BitReader;
///Field `TIM6EN` writer - Timer 6 clock enable
pub type TIM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7EN` reader - Timer 7 clock enable
pub type TIM7EN_R = crate::BitReader;
///Field `TIM7EN` writer - Timer 7 clock enable
pub type TIM7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCDEN` reader - LCD clock enable
pub type LCDEN_R = crate::BitReader;
///Field `LCDEN` writer - LCD clock enable
pub type LCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGEN` reader - Window watchdog clock enable
pub type WWDGEN_R = crate::BitReader;
///Field `WWDGEN` writer - Window watchdog clock enable
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2EN` reader - SPI 2 clock enable
pub type SPI2EN_R = crate::BitReader;
///Field `SPI2EN` writer - SPI 2 clock enable
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3EN` reader - SPI 3 clock enable
pub type SPI3EN_R = crate::BitReader;
///Field `SPI3EN` writer - SPI 3 clock enable
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2EN` reader - USART 2 clock enable
pub type USART2EN_R = crate::BitReader;
///Field `USART2EN` writer - USART 2 clock enable
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3EN` reader - USART 3 clock enable
pub type USART3EN_R = crate::BitReader;
///Field `USART3EN` writer - USART 3 clock enable
pub type USART3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART4EN` reader - UART 4 clock enable
pub type USART4EN_R = crate::BitReader;
///Field `USART4EN` writer - UART 4 clock enable
pub type USART4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART5EN` reader - UART 5 clock enable
pub type USART5EN_R = crate::BitReader;
///Field `USART5EN` writer - UART 5 clock enable
pub type USART5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1EN` reader - I2C 1 clock enable
pub type I2C1EN_R = crate::BitReader;
///Field `I2C1EN` writer - I2C 1 clock enable
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2EN` reader - I2C 2 clock enable
pub type I2C2EN_R = crate::BitReader;
///Field `I2C2EN` writer - I2C 2 clock enable
pub type I2C2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBEN` reader - USB clock enable
pub type USBEN_R = crate::BitReader;
///Field `USBEN` writer - USB clock enable
pub type USBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWREN` reader - Power interface clock enable
pub type PWREN_R = crate::BitReader;
///Field `PWREN` writer - Power interface clock enable
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DACEN` reader - DAC interface clock enable
pub type DACEN_R = crate::BitReader;
///Field `DACEN` writer - DAC interface clock enable
pub type DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMPEN` reader - COMP interface clock enable
pub type COMPEN_R = crate::BitReader;
///Field `COMPEN` writer - COMP interface clock enable
pub type COMPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Timer 2 clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timer 3 clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer 4 clock enable
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer 5 clock enable
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer 6 clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer 7 clock enable
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - LCD clock enable
    #[inline(always)]
    pub fn lcden(&self) -> LCDEN_R {
        LCDEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI 2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI 3 clock enable
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART 3 clock enable
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART 4 clock enable
    #[inline(always)]
    pub fn usart4en(&self) -> USART4EN_R {
        USART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART 5 clock enable
    #[inline(always)]
    pub fn usart5en(&self) -> USART5EN_R {
        USART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C 1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C 2 clock enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - USB clock enable
    #[inline(always)]
    pub fn usben(&self) -> USBEN_R {
        USBEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&self) -> PWREN_R {
        PWREN_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC interface clock enable
    #[inline(always)]
    pub fn dacen(&self) -> DACEN_R {
        DACEN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 31 - COMP interface clock enable
    #[inline(always)]
    pub fn compen(&self) -> COMPEN_R {
        COMPEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR")
            .field("compen", &self.compen())
            .field("dacen", &self.dacen())
            .field("pwren", &self.pwren())
            .field("usben", &self.usben())
            .field("i2c2en", &self.i2c2en())
            .field("i2c1en", &self.i2c1en())
            .field("usart5en", &self.usart5en())
            .field("usart4en", &self.usart4en())
            .field("usart3en", &self.usart3en())
            .field("usart2en", &self.usart2en())
            .field("spi3en", &self.spi3en())
            .field("spi2en", &self.spi2en())
            .field("wwdgen", &self.wwdgen())
            .field("lcden", &self.lcden())
            .field("tim7en", &self.tim7en())
            .field("tim6en", &self.tim6en())
            .field("tim5en", &self.tim5en())
            .field("tim4en", &self.tim4en())
            .field("tim3en", &self.tim3en())
            .field("tim2en", &self.tim2en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timer 2 clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1ENRrs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - Timer 3 clock enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APB1ENRrs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 2 - Timer 4 clock enable
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W<'_, APB1ENRrs> {
        TIM4EN_W::new(self, 2)
    }
    ///Bit 3 - Timer 5 clock enable
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W<'_, APB1ENRrs> {
        TIM5EN_W::new(self, 3)
    }
    ///Bit 4 - Timer 6 clock enable
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APB1ENRrs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - Timer 7 clock enable
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<'_, APB1ENRrs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 9 - LCD clock enable
    #[inline(always)]
    pub fn lcden(&mut self) -> LCDEN_W<'_, APB1ENRrs> {
        LCDEN_W::new(self, 9)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APB1ENRrs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - SPI 2 clock enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APB1ENRrs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 15 - SPI 3 clock enable
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APB1ENRrs> {
        SPI3EN_W::new(self, 15)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1ENRrs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART 3 clock enable
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W<'_, APB1ENRrs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 19 - UART 4 clock enable
    #[inline(always)]
    pub fn usart4en(&mut self) -> USART4EN_W<'_, APB1ENRrs> {
        USART4EN_W::new(self, 19)
    }
    ///Bit 20 - UART 5 clock enable
    #[inline(always)]
    pub fn usart5en(&mut self) -> USART5EN_W<'_, APB1ENRrs> {
        USART5EN_W::new(self, 20)
    }
    ///Bit 21 - I2C 1 clock enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1ENRrs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C 2 clock enable
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APB1ENRrs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 23 - USB clock enable
    #[inline(always)]
    pub fn usben(&mut self) -> USBEN_W<'_, APB1ENRrs> {
        USBEN_W::new(self, 23)
    }
    ///Bit 28 - Power interface clock enable
    #[inline(always)]
    pub fn pwren(&mut self) -> PWREN_W<'_, APB1ENRrs> {
        PWREN_W::new(self, 28)
    }
    ///Bit 29 - DAC interface clock enable
    #[inline(always)]
    pub fn dacen(&mut self) -> DACEN_W<'_, APB1ENRrs> {
        DACEN_W::new(self, 29)
    }
    ///Bit 31 - COMP interface clock enable
    #[inline(always)]
    pub fn compen(&mut self) -> COMPEN_W<'_, APB1ENRrs> {
        COMPEN_W::new(self, 31)
    }
}
/**APB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L151.html#RCC:APB1ENR)*/
pub struct APB1ENRrs;
impl crate::RegisterSpec for APB1ENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1enr::R`](R) reader structure
impl crate::Readable for APB1ENRrs {}
///`write(|w| ..)` method takes [`apb1enr::W`](W) writer structure
impl crate::Writable for APB1ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1ENR to value 0
impl crate::Resettable for APB1ENRrs {}
