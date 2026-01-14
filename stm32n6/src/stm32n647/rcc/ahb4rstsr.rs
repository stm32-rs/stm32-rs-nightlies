///Register `AHB4RSTSR` writer
pub type W = crate::W<AHB4RSTSRrs>;
///Field `GPIOARSTS` writer - GPIOA reset
pub type GPIOARSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBRSTS` writer - GPIOB reset
pub type GPIOBRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCRSTS` writer - GPIOC reset
pub type GPIOCRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODRSTS` writer - GPIOD reset
pub type GPIODRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOERSTS` writer - GPIOE reset
pub type GPIOERSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFRSTS` writer - GPIOF reset
pub type GPIOFRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGRSTS` writer - GPIOG reset
pub type GPIOGRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHRSTS` writer - GPIOH reset
pub type GPIOHRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIONRSTS` writer - GPION reset
pub type GPIONRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOORSTS` writer - GPIOO reset
pub type GPIOORSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPRSTS` writer - GPIOP reset
pub type GPIOPRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOQRSTS` writer - GPIOQ reset
pub type GPIOQRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRRSTS` writer - PWR reset
pub type PWRRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRSTS` writer - CRC reset
pub type CRCRSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB4RSTSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - GPIOA reset
    #[inline(always)]
    pub fn gpioarsts(&mut self) -> GPIOARSTS_W<'_, AHB4RSTSRrs> {
        GPIOARSTS_W::new(self, 0)
    }
    ///Bit 1 - GPIOB reset
    #[inline(always)]
    pub fn gpiobrsts(&mut self) -> GPIOBRSTS_W<'_, AHB4RSTSRrs> {
        GPIOBRSTS_W::new(self, 1)
    }
    ///Bit 2 - GPIOC reset
    #[inline(always)]
    pub fn gpiocrsts(&mut self) -> GPIOCRSTS_W<'_, AHB4RSTSRrs> {
        GPIOCRSTS_W::new(self, 2)
    }
    ///Bit 3 - GPIOD reset
    #[inline(always)]
    pub fn gpiodrsts(&mut self) -> GPIODRSTS_W<'_, AHB4RSTSRrs> {
        GPIODRSTS_W::new(self, 3)
    }
    ///Bit 4 - GPIOE reset
    #[inline(always)]
    pub fn gpioersts(&mut self) -> GPIOERSTS_W<'_, AHB4RSTSRrs> {
        GPIOERSTS_W::new(self, 4)
    }
    ///Bit 5 - GPIOF reset
    #[inline(always)]
    pub fn gpiofrsts(&mut self) -> GPIOFRSTS_W<'_, AHB4RSTSRrs> {
        GPIOFRSTS_W::new(self, 5)
    }
    ///Bit 6 - GPIOG reset
    #[inline(always)]
    pub fn gpiogrsts(&mut self) -> GPIOGRSTS_W<'_, AHB4RSTSRrs> {
        GPIOGRSTS_W::new(self, 6)
    }
    ///Bit 7 - GPIOH reset
    #[inline(always)]
    pub fn gpiohrsts(&mut self) -> GPIOHRSTS_W<'_, AHB4RSTSRrs> {
        GPIOHRSTS_W::new(self, 7)
    }
    ///Bit 13 - GPION reset
    #[inline(always)]
    pub fn gpionrsts(&mut self) -> GPIONRSTS_W<'_, AHB4RSTSRrs> {
        GPIONRSTS_W::new(self, 13)
    }
    ///Bit 14 - GPIOO reset
    #[inline(always)]
    pub fn gpioorsts(&mut self) -> GPIOORSTS_W<'_, AHB4RSTSRrs> {
        GPIOORSTS_W::new(self, 14)
    }
    ///Bit 15 - GPIOP reset
    #[inline(always)]
    pub fn gpioprsts(&mut self) -> GPIOPRSTS_W<'_, AHB4RSTSRrs> {
        GPIOPRSTS_W::new(self, 15)
    }
    ///Bit 16 - GPIOQ reset
    #[inline(always)]
    pub fn gpioqrsts(&mut self) -> GPIOQRSTS_W<'_, AHB4RSTSRrs> {
        GPIOQRSTS_W::new(self, 16)
    }
    ///Bit 18 - PWR reset
    #[inline(always)]
    pub fn pwrrsts(&mut self) -> PWRRSTS_W<'_, AHB4RSTSRrs> {
        PWRRSTS_W::new(self, 18)
    }
    ///Bit 19 - CRC reset
    #[inline(always)]
    pub fn crcrsts(&mut self) -> CRCRSTS_W<'_, AHB4RSTSRrs> {
        CRCRSTS_W::new(self, 19)
    }
}
/**RCC AHB4 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstsr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4RSTSR)*/
pub struct AHB4RSTSRrs;
impl crate::RegisterSpec for AHB4RSTSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb4rstsr::W`](W) writer structure
impl crate::Writable for AHB4RSTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4RSTSR to value 0
impl crate::Resettable for AHB4RSTSRrs {}
