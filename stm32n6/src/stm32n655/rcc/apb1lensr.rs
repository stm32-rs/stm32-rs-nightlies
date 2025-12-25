///Register `APB1LENSR` writer
pub type W = crate::W<APB1LENSRrs>;
///Field `TIM2ENS` writer - TIM2 enable
pub type TIM2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM3ENS` writer - TIM3 enable
pub type TIM3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM4ENS` writer - TIM4 enable
pub type TIM4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM5ENS` writer - TIM5 enable
pub type TIM5ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM6ENS` writer - TIM6 enable
pub type TIM6ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM7ENS` writer - TIM7 enable
pub type TIM7ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM12ENS` writer - TIM12 enable
pub type TIM12ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM13ENS` writer - TIM13 enable
pub type TIM13ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM14ENS` writer - TIM14 enable
pub type TIM14ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPTIM1ENS` writer - LPTIM1 enable
pub type LPTIM1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WWDGENS` writer - WWDG enable
pub type WWDGENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM10ENS` writer - TIM10 enable
pub type TIM10ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM11ENS` writer - TIM11 enable
pub type TIM11ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI2ENS` writer - SPI2 enable
pub type SPI2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI3ENS` writer - SPI3 enable
pub type SPI3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPDIFRX1ENS` writer - SPDIFRX1 enable
pub type SPDIFRX1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART2ENS` writer - USART2 enable
pub type USART2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART3ENS` writer - USART3 enable
pub type USART3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART4ENS` writer - UART4 enable
pub type UART4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART5ENS` writer - UART5 enable
pub type UART5ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C1ENS` writer - I2C1 enable
pub type I2C1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C2ENS` writer - I2C2 enable
pub type I2C2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I2C3ENS` writer - I2C3 enable
pub type I2C3ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C1ENS` writer - I3C1 enable
pub type I3C1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `I3C2ENS` writer - I3C2 enable
pub type I3C2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART7ENS` writer - UART7 enable
pub type UART7ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART8ENS` writer - UART8 enable
pub type UART8ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB1LENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM2 enable
    #[inline(always)]
    pub fn tim2ens(&mut self) -> TIM2ENS_W<'_, APB1LENSRrs> {
        TIM2ENS_W::new(self, 0)
    }
    ///Bit 1 - TIM3 enable
    #[inline(always)]
    pub fn tim3ens(&mut self) -> TIM3ENS_W<'_, APB1LENSRrs> {
        TIM3ENS_W::new(self, 1)
    }
    ///Bit 2 - TIM4 enable
    #[inline(always)]
    pub fn tim4ens(&mut self) -> TIM4ENS_W<'_, APB1LENSRrs> {
        TIM4ENS_W::new(self, 2)
    }
    ///Bit 3 - TIM5 enable
    #[inline(always)]
    pub fn tim5ens(&mut self) -> TIM5ENS_W<'_, APB1LENSRrs> {
        TIM5ENS_W::new(self, 3)
    }
    ///Bit 4 - TIM6 enable
    #[inline(always)]
    pub fn tim6ens(&mut self) -> TIM6ENS_W<'_, APB1LENSRrs> {
        TIM6ENS_W::new(self, 4)
    }
    ///Bit 5 - TIM7 enable
    #[inline(always)]
    pub fn tim7ens(&mut self) -> TIM7ENS_W<'_, APB1LENSRrs> {
        TIM7ENS_W::new(self, 5)
    }
    ///Bit 6 - TIM12 enable
    #[inline(always)]
    pub fn tim12ens(&mut self) -> TIM12ENS_W<'_, APB1LENSRrs> {
        TIM12ENS_W::new(self, 6)
    }
    ///Bit 7 - TIM13 enable
    #[inline(always)]
    pub fn tim13ens(&mut self) -> TIM13ENS_W<'_, APB1LENSRrs> {
        TIM13ENS_W::new(self, 7)
    }
    ///Bit 8 - TIM14 enable
    #[inline(always)]
    pub fn tim14ens(&mut self) -> TIM14ENS_W<'_, APB1LENSRrs> {
        TIM14ENS_W::new(self, 8)
    }
    ///Bit 9 - LPTIM1 enable
    #[inline(always)]
    pub fn lptim1ens(&mut self) -> LPTIM1ENS_W<'_, APB1LENSRrs> {
        LPTIM1ENS_W::new(self, 9)
    }
    ///Bit 11 - WWDG enable
    #[inline(always)]
    pub fn wwdgens(&mut self) -> WWDGENS_W<'_, APB1LENSRrs> {
        WWDGENS_W::new(self, 11)
    }
    ///Bit 12 - TIM10 enable
    #[inline(always)]
    pub fn tim10ens(&mut self) -> TIM10ENS_W<'_, APB1LENSRrs> {
        TIM10ENS_W::new(self, 12)
    }
    ///Bit 13 - TIM11 enable
    #[inline(always)]
    pub fn tim11ens(&mut self) -> TIM11ENS_W<'_, APB1LENSRrs> {
        TIM11ENS_W::new(self, 13)
    }
    ///Bit 14 - SPI2 enable
    #[inline(always)]
    pub fn spi2ens(&mut self) -> SPI2ENS_W<'_, APB1LENSRrs> {
        SPI2ENS_W::new(self, 14)
    }
    ///Bit 15 - SPI3 enable
    #[inline(always)]
    pub fn spi3ens(&mut self) -> SPI3ENS_W<'_, APB1LENSRrs> {
        SPI3ENS_W::new(self, 15)
    }
    ///Bit 16 - SPDIFRX1 enable
    #[inline(always)]
    pub fn spdifrx1ens(&mut self) -> SPDIFRX1ENS_W<'_, APB1LENSRrs> {
        SPDIFRX1ENS_W::new(self, 16)
    }
    ///Bit 17 - USART2 enable
    #[inline(always)]
    pub fn usart2ens(&mut self) -> USART2ENS_W<'_, APB1LENSRrs> {
        USART2ENS_W::new(self, 17)
    }
    ///Bit 18 - USART3 enable
    #[inline(always)]
    pub fn usart3ens(&mut self) -> USART3ENS_W<'_, APB1LENSRrs> {
        USART3ENS_W::new(self, 18)
    }
    ///Bit 19 - UART4 enable
    #[inline(always)]
    pub fn uart4ens(&mut self) -> UART4ENS_W<'_, APB1LENSRrs> {
        UART4ENS_W::new(self, 19)
    }
    ///Bit 20 - UART5 enable
    #[inline(always)]
    pub fn uart5ens(&mut self) -> UART5ENS_W<'_, APB1LENSRrs> {
        UART5ENS_W::new(self, 20)
    }
    ///Bit 21 - I2C1 enable
    #[inline(always)]
    pub fn i2c1ens(&mut self) -> I2C1ENS_W<'_, APB1LENSRrs> {
        I2C1ENS_W::new(self, 21)
    }
    ///Bit 22 - I2C2 enable
    #[inline(always)]
    pub fn i2c2ens(&mut self) -> I2C2ENS_W<'_, APB1LENSRrs> {
        I2C2ENS_W::new(self, 22)
    }
    ///Bit 23 - I2C3 enable
    #[inline(always)]
    pub fn i2c3ens(&mut self) -> I2C3ENS_W<'_, APB1LENSRrs> {
        I2C3ENS_W::new(self, 23)
    }
    ///Bit 24 - I3C1 enable
    #[inline(always)]
    pub fn i3c1ens(&mut self) -> I3C1ENS_W<'_, APB1LENSRrs> {
        I3C1ENS_W::new(self, 24)
    }
    ///Bit 25 - I3C2 enable
    #[inline(always)]
    pub fn i3c2ens(&mut self) -> I3C2ENS_W<'_, APB1LENSRrs> {
        I3C2ENS_W::new(self, 25)
    }
    ///Bit 30 - UART7 enable
    #[inline(always)]
    pub fn uart7ens(&mut self) -> UART7ENS_W<'_, APB1LENSRrs> {
        UART7ENS_W::new(self, 30)
    }
    ///Bit 31 - UART8 enable
    #[inline(always)]
    pub fn uart8ens(&mut self) -> UART8ENS_W<'_, APB1LENSRrs> {
        UART8ENS_W::new(self, 31)
    }
}
/**RCC APB1L enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB1LENSR)*/
pub struct APB1LENSRrs;
impl crate::RegisterSpec for APB1LENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb1lensr::W`](W) writer structure
impl crate::Writable for APB1LENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1LENSR to value 0
impl crate::Resettable for APB1LENSRrs {}
