///Register `APB2ENSR` writer
pub type W = crate::W<APB2ENSRrs>;
///Field `TIM1ENS` writer - TIM1 enable
pub type TIM1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM8ENS` writer - TIM8 enable
pub type TIM8ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART1ENS` writer - USART1 enable
pub type USART1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART6ENS` writer - USART6 enable
pub type USART6ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART9ENS` writer - UART9 enable
pub type UART9ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USART10ENS` writer - USART10 enable
pub type USART10ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI1ENS` writer - SPI1 enable
pub type SPI1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI4ENS` writer - SPI4 enable
pub type SPI4ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM18ENS` writer - TIM18 enable
pub type TIM18ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM15ENS` writer - TIM15 enable
pub type TIM15ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM16ENS` writer - TIM16 enable
pub type TIM16ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM17ENS` writer - TIM17 enable
pub type TIM17ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIM9ENS` writer - TIM9 enable
pub type TIM9ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SPI5ENS` writer - SPI5 enable
pub type SPI5ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI1ENS` writer - SAI1 enable
pub type SAI1ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAI2ENS` writer - SAI2 enable
pub type SAI2ENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<APB2ENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - TIM1 enable
    #[inline(always)]
    pub fn tim1ens(&mut self) -> TIM1ENS_W<'_, APB2ENSRrs> {
        TIM1ENS_W::new(self, 0)
    }
    ///Bit 1 - TIM8 enable
    #[inline(always)]
    pub fn tim8ens(&mut self) -> TIM8ENS_W<'_, APB2ENSRrs> {
        TIM8ENS_W::new(self, 1)
    }
    ///Bit 4 - USART1 enable
    #[inline(always)]
    pub fn usart1ens(&mut self) -> USART1ENS_W<'_, APB2ENSRrs> {
        USART1ENS_W::new(self, 4)
    }
    ///Bit 5 - USART6 enable
    #[inline(always)]
    pub fn usart6ens(&mut self) -> USART6ENS_W<'_, APB2ENSRrs> {
        USART6ENS_W::new(self, 5)
    }
    ///Bit 6 - UART9 enable
    #[inline(always)]
    pub fn uart9ens(&mut self) -> UART9ENS_W<'_, APB2ENSRrs> {
        UART9ENS_W::new(self, 6)
    }
    ///Bit 7 - USART10 enable
    #[inline(always)]
    pub fn usart10ens(&mut self) -> USART10ENS_W<'_, APB2ENSRrs> {
        USART10ENS_W::new(self, 7)
    }
    ///Bit 12 - SPI1 enable
    #[inline(always)]
    pub fn spi1ens(&mut self) -> SPI1ENS_W<'_, APB2ENSRrs> {
        SPI1ENS_W::new(self, 12)
    }
    ///Bit 13 - SPI4 enable
    #[inline(always)]
    pub fn spi4ens(&mut self) -> SPI4ENS_W<'_, APB2ENSRrs> {
        SPI4ENS_W::new(self, 13)
    }
    ///Bit 15 - TIM18 enable
    #[inline(always)]
    pub fn tim18ens(&mut self) -> TIM18ENS_W<'_, APB2ENSRrs> {
        TIM18ENS_W::new(self, 15)
    }
    ///Bit 16 - TIM15 enable
    #[inline(always)]
    pub fn tim15ens(&mut self) -> TIM15ENS_W<'_, APB2ENSRrs> {
        TIM15ENS_W::new(self, 16)
    }
    ///Bit 17 - TIM16 enable
    #[inline(always)]
    pub fn tim16ens(&mut self) -> TIM16ENS_W<'_, APB2ENSRrs> {
        TIM16ENS_W::new(self, 17)
    }
    ///Bit 18 - TIM17 enable
    #[inline(always)]
    pub fn tim17ens(&mut self) -> TIM17ENS_W<'_, APB2ENSRrs> {
        TIM17ENS_W::new(self, 18)
    }
    ///Bit 19 - TIM9 enable
    #[inline(always)]
    pub fn tim9ens(&mut self) -> TIM9ENS_W<'_, APB2ENSRrs> {
        TIM9ENS_W::new(self, 19)
    }
    ///Bit 20 - SPI5 enable
    #[inline(always)]
    pub fn spi5ens(&mut self) -> SPI5ENS_W<'_, APB2ENSRrs> {
        SPI5ENS_W::new(self, 20)
    }
    ///Bit 21 - SAI1 enable
    #[inline(always)]
    pub fn sai1ens(&mut self) -> SAI1ENS_W<'_, APB2ENSRrs> {
        SAI1ENS_W::new(self, 21)
    }
    ///Bit 22 - SAI2 enable
    #[inline(always)]
    pub fn sai2ens(&mut self) -> SAI2ENS_W<'_, APB2ENSRrs> {
        SAI2ENS_W::new(self, 22)
    }
}
/**RCC APB2 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2ensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#RCC:APB2ENSR)*/
pub struct APB2ENSRrs;
impl crate::RegisterSpec for APB2ENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`apb2ensr::W`](W) writer structure
impl crate::Writable for APB2ENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB2ENSR to value 0
impl crate::Resettable for APB2ENSRrs {}
