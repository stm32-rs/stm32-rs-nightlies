///Register `AHB4ENSR` writer
pub type W = crate::W<AHB4ENSRrs>;
///Field `GPIOAENS` writer - GPIOA enable
pub type GPIOAENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBENS` writer - GPIOB enable
pub type GPIOBENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCENS` writer - GPIOC enable
pub type GPIOCENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODENS` writer - GPIOD enable
pub type GPIODENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOEENS` writer - GPIOE enable
pub type GPIOEENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFENS` writer - GPIOF enable
pub type GPIOFENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGENS` writer - GPIOG enable
pub type GPIOGENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHENS` writer - GPIOH enable
pub type GPIOHENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIONENS` writer - GPION enable
pub type GPIONENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOOENS` writer - GPIOO enable
pub type GPIOOENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPENS` writer - GPIOP enable
pub type GPIOPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOQENS` writer - GPIOQ enable
pub type GPIOQENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRENS` writer - PWR enable
pub type PWRENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCENS` writer - CRC enable
pub type CRCENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB4ENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - GPIOA enable
    #[inline(always)]
    pub fn gpioaens(&mut self) -> GPIOAENS_W<'_, AHB4ENSRrs> {
        GPIOAENS_W::new(self, 0)
    }
    ///Bit 1 - GPIOB enable
    #[inline(always)]
    pub fn gpiobens(&mut self) -> GPIOBENS_W<'_, AHB4ENSRrs> {
        GPIOBENS_W::new(self, 1)
    }
    ///Bit 2 - GPIOC enable
    #[inline(always)]
    pub fn gpiocens(&mut self) -> GPIOCENS_W<'_, AHB4ENSRrs> {
        GPIOCENS_W::new(self, 2)
    }
    ///Bit 3 - GPIOD enable
    #[inline(always)]
    pub fn gpiodens(&mut self) -> GPIODENS_W<'_, AHB4ENSRrs> {
        GPIODENS_W::new(self, 3)
    }
    ///Bit 4 - GPIOE enable
    #[inline(always)]
    pub fn gpioeens(&mut self) -> GPIOEENS_W<'_, AHB4ENSRrs> {
        GPIOEENS_W::new(self, 4)
    }
    ///Bit 5 - GPIOF enable
    #[inline(always)]
    pub fn gpiofens(&mut self) -> GPIOFENS_W<'_, AHB4ENSRrs> {
        GPIOFENS_W::new(self, 5)
    }
    ///Bit 6 - GPIOG enable
    #[inline(always)]
    pub fn gpiogens(&mut self) -> GPIOGENS_W<'_, AHB4ENSRrs> {
        GPIOGENS_W::new(self, 6)
    }
    ///Bit 7 - GPIOH enable
    #[inline(always)]
    pub fn gpiohens(&mut self) -> GPIOHENS_W<'_, AHB4ENSRrs> {
        GPIOHENS_W::new(self, 7)
    }
    ///Bit 13 - GPION enable
    #[inline(always)]
    pub fn gpionens(&mut self) -> GPIONENS_W<'_, AHB4ENSRrs> {
        GPIONENS_W::new(self, 13)
    }
    ///Bit 14 - GPIOO enable
    #[inline(always)]
    pub fn gpiooens(&mut self) -> GPIOOENS_W<'_, AHB4ENSRrs> {
        GPIOOENS_W::new(self, 14)
    }
    ///Bit 15 - GPIOP enable
    #[inline(always)]
    pub fn gpiopens(&mut self) -> GPIOPENS_W<'_, AHB4ENSRrs> {
        GPIOPENS_W::new(self, 15)
    }
    ///Bit 16 - GPIOQ enable
    #[inline(always)]
    pub fn gpioqens(&mut self) -> GPIOQENS_W<'_, AHB4ENSRrs> {
        GPIOQENS_W::new(self, 16)
    }
    ///Bit 18 - PWR enable
    #[inline(always)]
    pub fn pwrens(&mut self) -> PWRENS_W<'_, AHB4ENSRrs> {
        PWRENS_W::new(self, 18)
    }
    ///Bit 19 - CRC enable
    #[inline(always)]
    pub fn crcens(&mut self) -> CRCENS_W<'_, AHB4ENSRrs> {
        CRCENS_W::new(self, 19)
    }
}
/**RCC AHB4 enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4ensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4ENSR)*/
pub struct AHB4ENSRrs;
impl crate::RegisterSpec for AHB4ENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb4ensr::W`](W) writer structure
impl crate::Writable for AHB4ENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4ENSR to value 0
impl crate::Resettable for AHB4ENSRrs {}
