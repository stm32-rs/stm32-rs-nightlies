///Register `APB1ENR` reader
pub type R = crate::R<APB1ENRrs>;
///Register `APB1ENR` writer
pub type W = crate::W<APB1ENRrs>;
///Field `TIM2EN` reader - TIM2 clock enable
pub type TIM2EN_R = crate::BitReader;
///Field `TIM2EN` writer - TIM2 clock enable
pub type TIM2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3EN` reader - TIM3 clock enable
pub type TIM3EN_R = crate::BitReader;
///Field `TIM3EN` writer - TIM3 clock enable
pub type TIM3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4EN` reader - TIM4 clock enable
pub type TIM4EN_R = crate::BitReader;
///Field `TIM4EN` writer - TIM4 clock enable
pub type TIM4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5EN` reader - TIM5 clock enable
pub type TIM5EN_R = crate::BitReader;
///Field `TIM5EN` writer - TIM5 clock enable
pub type TIM5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6EN` reader - TIM6 clock enable
pub type TIM6EN_R = crate::BitReader;
///Field `TIM6EN` writer - TIM6 clock enable
pub type TIM6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7EN` reader - TIM7 clock enable
pub type TIM7EN_R = crate::BitReader;
///Field `TIM7EN` writer - TIM7 clock enable
pub type TIM7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM12EN` reader - TIM12 clock enable
pub type TIM12EN_R = crate::BitReader;
///Field `TIM12EN` writer - TIM12 clock enable
pub type TIM12EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13EN` reader - TIM13 clock enable
pub type TIM13EN_R = crate::BitReader;
///Field `TIM13EN` writer - TIM13 clock enable
pub type TIM13EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14EN` reader - TIM14 clock enable
pub type TIM14EN_R = crate::BitReader;
///Field `TIM14EN` writer - TIM14 clock enable
pub type TIM14EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGEN` reader - Window watchdog clock enable
pub type WWDGEN_R = crate::BitReader;
///Field `WWDGEN` writer - Window watchdog clock enable
pub type WWDGEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2EN` reader - SPI2 clock enable
pub type SPI2EN_R = crate::BitReader;
///Field `SPI2EN` writer - SPI2 clock enable
pub type SPI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3EN` reader - SPI3 clock enable
pub type SPI3EN_R = crate::BitReader;
///Field `SPI3EN` writer - SPI3 clock enable
pub type SPI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2EN` reader - USART 2 clock enable
pub type USART2EN_R = crate::BitReader;
///Field `USART2EN` writer - USART 2 clock enable
pub type USART2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3EN` reader - USART3 clock enable
pub type USART3EN_R = crate::BitReader;
///Field `USART3EN` writer - USART3 clock enable
pub type USART3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4EN` reader - UART4 clock enable
pub type UART4EN_R = crate::BitReader;
///Field `UART4EN` writer - UART4 clock enable
pub type UART4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5EN` reader - UART5 clock enable
pub type UART5EN_R = crate::BitReader;
///Field `UART5EN` writer - UART5 clock enable
pub type UART5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1EN` reader - I2C1 clock enable
pub type I2C1EN_R = crate::BitReader;
///Field `I2C1EN` writer - I2C1 clock enable
pub type I2C1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2EN` reader - I2C2 clock enable
pub type I2C2EN_R = crate::BitReader;
///Field `I2C2EN` writer - I2C2 clock enable
pub type I2C2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3EN` reader - I2C3 clock enable
pub type I2C3EN_R = crate::BitReader;
///Field `I2C3EN` writer - I2C3 clock enable
pub type I2C3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAN1EN` reader - CAN 1 clock enable
pub type CAN1EN_R = crate::BitReader;
///Field `CAN1EN` writer - CAN 1 clock enable
pub type CAN1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAN2EN` reader - CAN 2 clock enable
pub type CAN2EN_R = crate::BitReader;
///Field `CAN2EN` writer - CAN 2 clock enable
pub type CAN2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWREN` reader - Power interface clock enable
pub type PWREN_R = crate::BitReader;
///Field `PWREN` writer - Power interface clock enable
pub type PWREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DACEN` reader - DAC interface clock enable
pub type DACEN_R = crate::BitReader;
///Field `DACEN` writer - DAC interface clock enable
pub type DACEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART7ENR` reader - UART7 clock enable
pub type UART7ENR_R = crate::BitReader;
///Field `UART7ENR` writer - UART7 clock enable
pub type UART7ENR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART8ENR` reader - UART8 clock enable
pub type UART8ENR_R = crate::BitReader;
///Field `UART8ENR` writer - UART8 clock enable
pub type UART8ENR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TIM2 clock enable
    #[inline(always)]
    pub fn tim2en(&self) -> TIM2EN_R {
        TIM2EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TIM3 clock enable
    #[inline(always)]
    pub fn tim3en(&self) -> TIM3EN_R {
        TIM3EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - TIM4 clock enable
    #[inline(always)]
    pub fn tim4en(&self) -> TIM4EN_R {
        TIM4EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TIM5 clock enable
    #[inline(always)]
    pub fn tim5en(&self) -> TIM5EN_R {
        TIM5EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - TIM6 clock enable
    #[inline(always)]
    pub fn tim6en(&self) -> TIM6EN_R {
        TIM6EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - TIM7 clock enable
    #[inline(always)]
    pub fn tim7en(&self) -> TIM7EN_R {
        TIM7EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIM12 clock enable
    #[inline(always)]
    pub fn tim12en(&self) -> TIM12EN_R {
        TIM12EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - TIM13 clock enable
    #[inline(always)]
    pub fn tim13en(&self) -> TIM13EN_R {
        TIM13EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - TIM14 clock enable
    #[inline(always)]
    pub fn tim14en(&self) -> TIM14EN_R {
        TIM14EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&self) -> WWDGEN_R {
        WWDGEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&self) -> SPI2EN_R {
        SPI2EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - SPI3 clock enable
    #[inline(always)]
    pub fn spi3en(&self) -> SPI3EN_R {
        SPI3EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&self) -> USART2EN_R {
        USART2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&self) -> USART3EN_R {
        USART3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - UART4 clock enable
    #[inline(always)]
    pub fn uart4en(&self) -> UART4EN_R {
        UART4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - UART5 clock enable
    #[inline(always)]
    pub fn uart5en(&self) -> UART5EN_R {
        UART5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&self) -> I2C1EN_R {
        I2C1EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&self) -> I2C2EN_R {
        I2C2EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&self) -> I2C3EN_R {
        I2C3EN_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 25 - CAN 1 clock enable
    #[inline(always)]
    pub fn can1en(&self) -> CAN1EN_R {
        CAN1EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - CAN 2 clock enable
    #[inline(always)]
    pub fn can2en(&self) -> CAN2EN_R {
        CAN2EN_R::new(((self.bits >> 26) & 1) != 0)
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
    ///Bit 30 - UART7 clock enable
    #[inline(always)]
    pub fn uart7enr(&self) -> UART7ENR_R {
        UART7ENR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - UART8 clock enable
    #[inline(always)]
    pub fn uart8enr(&self) -> UART8ENR_R {
        UART8ENR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1ENR")
            .field("tim2en", &self.tim2en())
            .field("tim3en", &self.tim3en())
            .field("tim4en", &self.tim4en())
            .field("tim5en", &self.tim5en())
            .field("tim6en", &self.tim6en())
            .field("tim7en", &self.tim7en())
            .field("tim12en", &self.tim12en())
            .field("tim13en", &self.tim13en())
            .field("tim14en", &self.tim14en())
            .field("wwdgen", &self.wwdgen())
            .field("spi2en", &self.spi2en())
            .field("spi3en", &self.spi3en())
            .field("usart2en", &self.usart2en())
            .field("usart3en", &self.usart3en())
            .field("uart4en", &self.uart4en())
            .field("uart5en", &self.uart5en())
            .field("i2c1en", &self.i2c1en())
            .field("i2c2en", &self.i2c2en())
            .field("i2c3en", &self.i2c3en())
            .field("can1en", &self.can1en())
            .field("can2en", &self.can2en())
            .field("pwren", &self.pwren())
            .field("dacen", &self.dacen())
            .field("uart7enr", &self.uart7enr())
            .field("uart8enr", &self.uart8enr())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIM2 clock enable
    #[inline(always)]
    pub fn tim2en(&mut self) -> TIM2EN_W<'_, APB1ENRrs> {
        TIM2EN_W::new(self, 0)
    }
    ///Bit 1 - TIM3 clock enable
    #[inline(always)]
    pub fn tim3en(&mut self) -> TIM3EN_W<'_, APB1ENRrs> {
        TIM3EN_W::new(self, 1)
    }
    ///Bit 2 - TIM4 clock enable
    #[inline(always)]
    pub fn tim4en(&mut self) -> TIM4EN_W<'_, APB1ENRrs> {
        TIM4EN_W::new(self, 2)
    }
    ///Bit 3 - TIM5 clock enable
    #[inline(always)]
    pub fn tim5en(&mut self) -> TIM5EN_W<'_, APB1ENRrs> {
        TIM5EN_W::new(self, 3)
    }
    ///Bit 4 - TIM6 clock enable
    #[inline(always)]
    pub fn tim6en(&mut self) -> TIM6EN_W<'_, APB1ENRrs> {
        TIM6EN_W::new(self, 4)
    }
    ///Bit 5 - TIM7 clock enable
    #[inline(always)]
    pub fn tim7en(&mut self) -> TIM7EN_W<'_, APB1ENRrs> {
        TIM7EN_W::new(self, 5)
    }
    ///Bit 6 - TIM12 clock enable
    #[inline(always)]
    pub fn tim12en(&mut self) -> TIM12EN_W<'_, APB1ENRrs> {
        TIM12EN_W::new(self, 6)
    }
    ///Bit 7 - TIM13 clock enable
    #[inline(always)]
    pub fn tim13en(&mut self) -> TIM13EN_W<'_, APB1ENRrs> {
        TIM13EN_W::new(self, 7)
    }
    ///Bit 8 - TIM14 clock enable
    #[inline(always)]
    pub fn tim14en(&mut self) -> TIM14EN_W<'_, APB1ENRrs> {
        TIM14EN_W::new(self, 8)
    }
    ///Bit 11 - Window watchdog clock enable
    #[inline(always)]
    pub fn wwdgen(&mut self) -> WWDGEN_W<'_, APB1ENRrs> {
        WWDGEN_W::new(self, 11)
    }
    ///Bit 14 - SPI2 clock enable
    #[inline(always)]
    pub fn spi2en(&mut self) -> SPI2EN_W<'_, APB1ENRrs> {
        SPI2EN_W::new(self, 14)
    }
    ///Bit 15 - SPI3 clock enable
    #[inline(always)]
    pub fn spi3en(&mut self) -> SPI3EN_W<'_, APB1ENRrs> {
        SPI3EN_W::new(self, 15)
    }
    ///Bit 17 - USART 2 clock enable
    #[inline(always)]
    pub fn usart2en(&mut self) -> USART2EN_W<'_, APB1ENRrs> {
        USART2EN_W::new(self, 17)
    }
    ///Bit 18 - USART3 clock enable
    #[inline(always)]
    pub fn usart3en(&mut self) -> USART3EN_W<'_, APB1ENRrs> {
        USART3EN_W::new(self, 18)
    }
    ///Bit 19 - UART4 clock enable
    #[inline(always)]
    pub fn uart4en(&mut self) -> UART4EN_W<'_, APB1ENRrs> {
        UART4EN_W::new(self, 19)
    }
    ///Bit 20 - UART5 clock enable
    #[inline(always)]
    pub fn uart5en(&mut self) -> UART5EN_W<'_, APB1ENRrs> {
        UART5EN_W::new(self, 20)
    }
    ///Bit 21 - I2C1 clock enable
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2C1EN_W<'_, APB1ENRrs> {
        I2C1EN_W::new(self, 21)
    }
    ///Bit 22 - I2C2 clock enable
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2C2EN_W<'_, APB1ENRrs> {
        I2C2EN_W::new(self, 22)
    }
    ///Bit 23 - I2C3 clock enable
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2C3EN_W<'_, APB1ENRrs> {
        I2C3EN_W::new(self, 23)
    }
    ///Bit 25 - CAN 1 clock enable
    #[inline(always)]
    pub fn can1en(&mut self) -> CAN1EN_W<'_, APB1ENRrs> {
        CAN1EN_W::new(self, 25)
    }
    ///Bit 26 - CAN 2 clock enable
    #[inline(always)]
    pub fn can2en(&mut self) -> CAN2EN_W<'_, APB1ENRrs> {
        CAN2EN_W::new(self, 26)
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
    ///Bit 30 - UART7 clock enable
    #[inline(always)]
    pub fn uart7enr(&mut self) -> UART7ENR_W<'_, APB1ENRrs> {
        UART7ENR_W::new(self, 30)
    }
    ///Bit 31 - UART8 clock enable
    #[inline(always)]
    pub fn uart8enr(&mut self) -> UART8ENR_W<'_, APB1ENRrs> {
        UART8ENR_W::new(self, 31)
    }
}
/**APB1 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`apb1enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#RCC:APB1ENR)*/
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
