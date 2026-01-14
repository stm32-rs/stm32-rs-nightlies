///Register `AHB4LPENCR` writer
pub type W = crate::W<AHB4LPENCRrs>;
///Field `GPIOALPENC` writer - GPIOA sleep enable
pub type GPIOALPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBLPENC` writer - GPIOB sleep enable
pub type GPIOBLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCLPENC` writer - GPIOC sleep enable
pub type GPIOCLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODLPENC` writer - GPIOD sleep enable
pub type GPIODLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOELPENC` writer - GPIOE sleep enable
pub type GPIOELPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFLPENC` writer - GPIOF sleep enable
pub type GPIOFLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGLPENC` writer - GPIOG sleep enable
pub type GPIOGLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHLPENC` writer - GPIOH sleep enable
pub type GPIOHLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIONLPENC` writer - GPION sleep enable
pub type GPIONLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOOLPENC` writer - GPIOO sleep enable
pub type GPIOOLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPLPENC` writer - GPIOP sleep enable
pub type GPIOPLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOQLPENC` writer - GPIOQ sleep enable
pub type GPIOQLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRLPENC` writer - PWR sleep enable
pub type PWRLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCLPENC` writer - CRC sleep enable
pub type CRCLPENC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<AHB4LPENCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - GPIOA sleep enable
    #[inline(always)]
    pub fn gpioalpenc(&mut self) -> GPIOALPENC_W<'_, AHB4LPENCRrs> {
        GPIOALPENC_W::new(self, 0)
    }
    ///Bit 1 - GPIOB sleep enable
    #[inline(always)]
    pub fn gpioblpenc(&mut self) -> GPIOBLPENC_W<'_, AHB4LPENCRrs> {
        GPIOBLPENC_W::new(self, 1)
    }
    ///Bit 2 - GPIOC sleep enable
    #[inline(always)]
    pub fn gpioclpenc(&mut self) -> GPIOCLPENC_W<'_, AHB4LPENCRrs> {
        GPIOCLPENC_W::new(self, 2)
    }
    ///Bit 3 - GPIOD sleep enable
    #[inline(always)]
    pub fn gpiodlpenc(&mut self) -> GPIODLPENC_W<'_, AHB4LPENCRrs> {
        GPIODLPENC_W::new(self, 3)
    }
    ///Bit 4 - GPIOE sleep enable
    #[inline(always)]
    pub fn gpioelpenc(&mut self) -> GPIOELPENC_W<'_, AHB4LPENCRrs> {
        GPIOELPENC_W::new(self, 4)
    }
    ///Bit 5 - GPIOF sleep enable
    #[inline(always)]
    pub fn gpioflpenc(&mut self) -> GPIOFLPENC_W<'_, AHB4LPENCRrs> {
        GPIOFLPENC_W::new(self, 5)
    }
    ///Bit 6 - GPIOG sleep enable
    #[inline(always)]
    pub fn gpioglpenc(&mut self) -> GPIOGLPENC_W<'_, AHB4LPENCRrs> {
        GPIOGLPENC_W::new(self, 6)
    }
    ///Bit 7 - GPIOH sleep enable
    #[inline(always)]
    pub fn gpiohlpenc(&mut self) -> GPIOHLPENC_W<'_, AHB4LPENCRrs> {
        GPIOHLPENC_W::new(self, 7)
    }
    ///Bit 13 - GPION sleep enable
    #[inline(always)]
    pub fn gpionlpenc(&mut self) -> GPIONLPENC_W<'_, AHB4LPENCRrs> {
        GPIONLPENC_W::new(self, 13)
    }
    ///Bit 14 - GPIOO sleep enable
    #[inline(always)]
    pub fn gpioolpenc(&mut self) -> GPIOOLPENC_W<'_, AHB4LPENCRrs> {
        GPIOOLPENC_W::new(self, 14)
    }
    ///Bit 15 - GPIOP sleep enable
    #[inline(always)]
    pub fn gpioplpenc(&mut self) -> GPIOPLPENC_W<'_, AHB4LPENCRrs> {
        GPIOPLPENC_W::new(self, 15)
    }
    ///Bit 16 - GPIOQ sleep enable
    #[inline(always)]
    pub fn gpioqlpenc(&mut self) -> GPIOQLPENC_W<'_, AHB4LPENCRrs> {
        GPIOQLPENC_W::new(self, 16)
    }
    ///Bit 18 - PWR sleep enable
    #[inline(always)]
    pub fn pwrlpenc(&mut self) -> PWRLPENC_W<'_, AHB4LPENCRrs> {
        PWRLPENC_W::new(self, 18)
    }
    ///Bit 19 - CRC sleep enable
    #[inline(always)]
    pub fn crclpenc(&mut self) -> CRCLPENC_W<'_, AHB4LPENCRrs> {
        CRCLPENC_W::new(self, 19)
    }
}
/**RCC AHB4 Sleep enable register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpencr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#RCC:AHB4LPENCR)*/
pub struct AHB4LPENCRrs;
impl crate::RegisterSpec for AHB4LPENCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ahb4lpencr::W`](W) writer structure
impl crate::Writable for AHB4LPENCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4LPENCR to value 0
impl crate::Resettable for AHB4LPENCRrs {}
