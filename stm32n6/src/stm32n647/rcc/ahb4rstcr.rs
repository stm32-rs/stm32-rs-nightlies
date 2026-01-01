///Register `AHB4RSTCR` writer
pub type W = crate::W<AHB4RSTCRrs>;
///Field `GPIOARSTC` writer - GPIOA reset
pub type GPIOARSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBRSTC` writer - GPIOB reset
pub type GPIOBRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCRSTC` writer - GPIOC reset
pub type GPIOCRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODRSTC` writer - GPIOD reset
pub type GPIODRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOERSTC` writer - GPIOE reset
pub type GPIOERSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFRSTC` writer - GPIOF reset
pub type GPIOFRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGRSTC` writer - GPIOG reset
pub type GPIOGRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHRSTC` writer - GPIOH reset
pub type GPIOHRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIONRSTC` writer - GPION reset
pub type GPIONRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOORSTC` writer - GPIOO reset
pub type GPIOORSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPRSTC` writer - GPIOP reset
pub type GPIOPRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOQRSTC` writer - GPIOQ reset
pub type GPIOQRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRRSTC` writer - PWR reset
pub type PWRRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRSTC` writer - CRC reset
pub type CRCRSTC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB4RSTCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - GPIOA reset
    #[inline(always)]
    pub fn gpioarstc(&mut self) -> GPIOARSTC_W<'_, AHB4RSTCRrs> {
        GPIOARSTC_W::new(self, 0)
    }
    ///Bit 1 - GPIOB reset
    #[inline(always)]
    pub fn gpiobrstc(&mut self) -> GPIOBRSTC_W<'_, AHB4RSTCRrs> {
        GPIOBRSTC_W::new(self, 1)
    }
    ///Bit 2 - GPIOC reset
    #[inline(always)]
    pub fn gpiocrstc(&mut self) -> GPIOCRSTC_W<'_, AHB4RSTCRrs> {
        GPIOCRSTC_W::new(self, 2)
    }
    ///Bit 3 - GPIOD reset
    #[inline(always)]
    pub fn gpiodrstc(&mut self) -> GPIODRSTC_W<'_, AHB4RSTCRrs> {
        GPIODRSTC_W::new(self, 3)
    }
    ///Bit 4 - GPIOE reset
    #[inline(always)]
    pub fn gpioerstc(&mut self) -> GPIOERSTC_W<'_, AHB4RSTCRrs> {
        GPIOERSTC_W::new(self, 4)
    }
    ///Bit 5 - GPIOF reset
    #[inline(always)]
    pub fn gpiofrstc(&mut self) -> GPIOFRSTC_W<'_, AHB4RSTCRrs> {
        GPIOFRSTC_W::new(self, 5)
    }
    ///Bit 6 - GPIOG reset
    #[inline(always)]
    pub fn gpiogrstc(&mut self) -> GPIOGRSTC_W<'_, AHB4RSTCRrs> {
        GPIOGRSTC_W::new(self, 6)
    }
    ///Bit 7 - GPIOH reset
    #[inline(always)]
    pub fn gpiohrstc(&mut self) -> GPIOHRSTC_W<'_, AHB4RSTCRrs> {
        GPIOHRSTC_W::new(self, 7)
    }
    ///Bit 13 - GPION reset
    #[inline(always)]
    pub fn gpionrstc(&mut self) -> GPIONRSTC_W<'_, AHB4RSTCRrs> {
        GPIONRSTC_W::new(self, 13)
    }
    ///Bit 14 - GPIOO reset
    #[inline(always)]
    pub fn gpioorstc(&mut self) -> GPIOORSTC_W<'_, AHB4RSTCRrs> {
        GPIOORSTC_W::new(self, 14)
    }
    ///Bit 15 - GPIOP reset
    #[inline(always)]
    pub fn gpioprstc(&mut self) -> GPIOPRSTC_W<'_, AHB4RSTCRrs> {
        GPIOPRSTC_W::new(self, 15)
    }
    ///Bit 16 - GPIOQ reset
    #[inline(always)]
    pub fn gpioqrstc(&mut self) -> GPIOQRSTC_W<'_, AHB4RSTCRrs> {
        GPIOQRSTC_W::new(self, 16)
    }
    ///Bit 18 - PWR reset
    #[inline(always)]
    pub fn pwrrstc(&mut self) -> PWRRSTC_W<'_, AHB4RSTCRrs> {
        PWRRSTC_W::new(self, 18)
    }
    ///Bit 19 - CRC reset
    #[inline(always)]
    pub fn crcrstc(&mut self) -> CRCRSTC_W<'_, AHB4RSTCRrs> {
        CRCRSTC_W::new(self, 19)
    }
}
/**RCC AHB4 reset register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4RSTCR)*/
pub struct AHB4RSTCRrs;
impl crate::RegisterSpec for AHB4RSTCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb4rstcr::W`](W) writer structure
impl crate::Writable for AHB4RSTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4RSTCR to value 0
impl crate::Resettable for AHB4RSTCRrs {}
