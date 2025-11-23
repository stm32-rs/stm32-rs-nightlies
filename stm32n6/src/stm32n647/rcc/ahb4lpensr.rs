///Register `AHB4LPENSR` writer
pub type W = crate::W<AHB4LPENSRrs>;
///Field `GPIOALPENS` writer - GPIOA sleep enable
pub type GPIOALPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBLPENS` writer - GPIOB sleep enable
pub type GPIOBLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCLPENS` writer - GPIOC sleep enable
pub type GPIOCLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODLPENS` writer - GPIOD sleep enable
pub type GPIODLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOELPENS` writer - GPIOE sleep enable
pub type GPIOELPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFLPENS` writer - GPIOF sleep enable
pub type GPIOFLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGLPENS` writer - GPIOG sleep enable
pub type GPIOGLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHLPENS` writer - GPIOH sleep enable
pub type GPIOHLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIONLPENS` writer - GPION sleep enable
pub type GPIONLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOOLPENS` writer - GPIOO sleep enable
pub type GPIOOLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPLPENS` writer - GPIOP sleep enable
pub type GPIOPLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOQLPENS` writer - GPIOQ sleep enable
pub type GPIOQLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRLPENS` writer - PWR sleep enable
pub type PWRLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCLPENS` writer - CRC sleep enable
pub type CRCLPENS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB4LPENSRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - GPIOA sleep enable
    #[inline(always)]
    pub fn gpioalpens(&mut self) -> GPIOALPENS_W<'_, AHB4LPENSRrs> {
        GPIOALPENS_W::new(self, 0)
    }
    ///Bit 1 - GPIOB sleep enable
    #[inline(always)]
    pub fn gpioblpens(&mut self) -> GPIOBLPENS_W<'_, AHB4LPENSRrs> {
        GPIOBLPENS_W::new(self, 1)
    }
    ///Bit 2 - GPIOC sleep enable
    #[inline(always)]
    pub fn gpioclpens(&mut self) -> GPIOCLPENS_W<'_, AHB4LPENSRrs> {
        GPIOCLPENS_W::new(self, 2)
    }
    ///Bit 3 - GPIOD sleep enable
    #[inline(always)]
    pub fn gpiodlpens(&mut self) -> GPIODLPENS_W<'_, AHB4LPENSRrs> {
        GPIODLPENS_W::new(self, 3)
    }
    ///Bit 4 - GPIOE sleep enable
    #[inline(always)]
    pub fn gpioelpens(&mut self) -> GPIOELPENS_W<'_, AHB4LPENSRrs> {
        GPIOELPENS_W::new(self, 4)
    }
    ///Bit 5 - GPIOF sleep enable
    #[inline(always)]
    pub fn gpioflpens(&mut self) -> GPIOFLPENS_W<'_, AHB4LPENSRrs> {
        GPIOFLPENS_W::new(self, 5)
    }
    ///Bit 6 - GPIOG sleep enable
    #[inline(always)]
    pub fn gpioglpens(&mut self) -> GPIOGLPENS_W<'_, AHB4LPENSRrs> {
        GPIOGLPENS_W::new(self, 6)
    }
    ///Bit 7 - GPIOH sleep enable
    #[inline(always)]
    pub fn gpiohlpens(&mut self) -> GPIOHLPENS_W<'_, AHB4LPENSRrs> {
        GPIOHLPENS_W::new(self, 7)
    }
    ///Bit 13 - GPION sleep enable
    #[inline(always)]
    pub fn gpionlpens(&mut self) -> GPIONLPENS_W<'_, AHB4LPENSRrs> {
        GPIONLPENS_W::new(self, 13)
    }
    ///Bit 14 - GPIOO sleep enable
    #[inline(always)]
    pub fn gpioolpens(&mut self) -> GPIOOLPENS_W<'_, AHB4LPENSRrs> {
        GPIOOLPENS_W::new(self, 14)
    }
    ///Bit 15 - GPIOP sleep enable
    #[inline(always)]
    pub fn gpioplpens(&mut self) -> GPIOPLPENS_W<'_, AHB4LPENSRrs> {
        GPIOPLPENS_W::new(self, 15)
    }
    ///Bit 16 - GPIOQ sleep enable
    #[inline(always)]
    pub fn gpioqlpens(&mut self) -> GPIOQLPENS_W<'_, AHB4LPENSRrs> {
        GPIOQLPENS_W::new(self, 16)
    }
    ///Bit 18 - PWR sleep enable
    #[inline(always)]
    pub fn pwrlpens(&mut self) -> PWRLPENS_W<'_, AHB4LPENSRrs> {
        PWRLPENS_W::new(self, 18)
    }
    ///Bit 19 - CRC sleep enable
    #[inline(always)]
    pub fn crclpens(&mut self) -> CRCLPENS_W<'_, AHB4LPENSRrs> {
        CRCLPENS_W::new(self, 19)
    }
}
/**RCC AHB4 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpensr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4LPENSR)*/
pub struct AHB4LPENSRrs;
impl crate::RegisterSpec for AHB4LPENSRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb4lpensr::W`](W) writer structure
impl crate::Writable for AHB4LPENSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4LPENSR to value 0
impl crate::Resettable for AHB4LPENSRrs {}
