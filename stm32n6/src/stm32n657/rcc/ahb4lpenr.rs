///Register `AHB4LPENR` reader
pub type R = crate::R<AHB4LPENRrs>;
///Register `AHB4LPENR` writer
pub type W = crate::W<AHB4LPENRrs>;
///Field `GPIOALPEN` reader - GPIOA sleep enable
pub type GPIOALPEN_R = crate::BitReader;
///Field `GPIOALPEN` writer - GPIOA sleep enable
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBLPEN` reader - GPIOB sleep enable
pub type GPIOBLPEN_R = crate::BitReader;
///Field `GPIOBLPEN` writer - GPIOB sleep enable
pub type GPIOBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCLPEN` reader - GPIOC sleep enable
pub type GPIOCLPEN_R = crate::BitReader;
///Field `GPIOCLPEN` writer - GPIOC sleep enable
pub type GPIOCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODLPEN` reader - GPIOD sleep enable
pub type GPIODLPEN_R = crate::BitReader;
///Field `GPIODLPEN` writer - GPIOD sleep enable
pub type GPIODLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOELPEN` reader - GPIOE sleep enable
pub type GPIOELPEN_R = crate::BitReader;
///Field `GPIOELPEN` writer - GPIOE sleep enable
pub type GPIOELPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFLPEN` reader - GPIOF sleep enable
pub type GPIOFLPEN_R = crate::BitReader;
///Field `GPIOFLPEN` writer - GPIOF sleep enable
pub type GPIOFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGLPEN` reader - GPIOG sleep enable
pub type GPIOGLPEN_R = crate::BitReader;
///Field `GPIOGLPEN` writer - GPIOG sleep enable
pub type GPIOGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHLPEN` reader - GPIOH sleep enable
pub type GPIOHLPEN_R = crate::BitReader;
///Field `GPIOHLPEN` writer - GPIOH sleep enable
pub type GPIOHLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIONLPEN` reader - GPION sleep enable
pub type GPIONLPEN_R = crate::BitReader;
///Field `GPIONLPEN` writer - GPION sleep enable
pub type GPIONLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOOLPEN` reader - GPIOO sleep enable
pub type GPIOOLPEN_R = crate::BitReader;
///Field `GPIOOLPEN` writer - GPIOO sleep enable
pub type GPIOOLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPLPEN` reader - GPIOP sleep enable
pub type GPIOPLPEN_R = crate::BitReader;
///Field `GPIOPLPEN` writer - GPIOP sleep enable
pub type GPIOPLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOQLPEN` reader - GPIOQ sleep enable
pub type GPIOQLPEN_R = crate::BitReader;
///Field `GPIOQLPEN` writer - GPIOQ sleep enable
pub type GPIOQLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRLPEN` reader - PWR sleep enable
pub type PWRLPEN_R = crate::BitReader;
///Field `PWRLPEN` writer - PWR sleep enable
pub type PWRLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCLPEN` reader - CRC sleep enable
pub type CRCLPEN_R = crate::BitReader;
///Field `CRCLPEN` writer - CRC sleep enable
pub type CRCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIOA sleep enable
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB sleep enable
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC sleep enable
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD sleep enable
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE sleep enable
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF sleep enable
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG sleep enable
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH sleep enable
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - GPION sleep enable
    #[inline(always)]
    pub fn gpionlpen(&self) -> GPIONLPEN_R {
        GPIONLPEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GPIOO sleep enable
    #[inline(always)]
    pub fn gpioolpen(&self) -> GPIOOLPEN_R {
        GPIOOLPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GPIOP sleep enable
    #[inline(always)]
    pub fn gpioplpen(&self) -> GPIOPLPEN_R {
        GPIOPLPEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - GPIOQ sleep enable
    #[inline(always)]
    pub fn gpioqlpen(&self) -> GPIOQLPEN_R {
        GPIOQLPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - PWR sleep enable
    #[inline(always)]
    pub fn pwrlpen(&self) -> PWRLPEN_R {
        PWRLPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CRC sleep enable
    #[inline(always)]
    pub fn crclpen(&self) -> CRCLPEN_R {
        CRCLPEN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4LPENR")
            .field("gpioalpen", &self.gpioalpen())
            .field("gpioblpen", &self.gpioblpen())
            .field("gpioclpen", &self.gpioclpen())
            .field("gpiodlpen", &self.gpiodlpen())
            .field("gpioelpen", &self.gpioelpen())
            .field("gpioflpen", &self.gpioflpen())
            .field("gpioglpen", &self.gpioglpen())
            .field("gpiohlpen", &self.gpiohlpen())
            .field("gpionlpen", &self.gpionlpen())
            .field("gpioolpen", &self.gpioolpen())
            .field("gpioplpen", &self.gpioplpen())
            .field("gpioqlpen", &self.gpioqlpen())
            .field("pwrlpen", &self.pwrlpen())
            .field("crclpen", &self.crclpen())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOA sleep enable
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<'_, AHB4LPENRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    ///Bit 1 - GPIOB sleep enable
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<'_, AHB4LPENRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    ///Bit 2 - GPIOC sleep enable
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<'_, AHB4LPENRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    ///Bit 3 - GPIOD sleep enable
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<'_, AHB4LPENRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    ///Bit 4 - GPIOE sleep enable
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<'_, AHB4LPENRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    ///Bit 5 - GPIOF sleep enable
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<'_, AHB4LPENRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    ///Bit 6 - GPIOG sleep enable
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<'_, AHB4LPENRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    ///Bit 7 - GPIOH sleep enable
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<'_, AHB4LPENRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    ///Bit 13 - GPION sleep enable
    #[inline(always)]
    pub fn gpionlpen(&mut self) -> GPIONLPEN_W<'_, AHB4LPENRrs> {
        GPIONLPEN_W::new(self, 13)
    }
    ///Bit 14 - GPIOO sleep enable
    #[inline(always)]
    pub fn gpioolpen(&mut self) -> GPIOOLPEN_W<'_, AHB4LPENRrs> {
        GPIOOLPEN_W::new(self, 14)
    }
    ///Bit 15 - GPIOP sleep enable
    #[inline(always)]
    pub fn gpioplpen(&mut self) -> GPIOPLPEN_W<'_, AHB4LPENRrs> {
        GPIOPLPEN_W::new(self, 15)
    }
    ///Bit 16 - GPIOQ sleep enable
    #[inline(always)]
    pub fn gpioqlpen(&mut self) -> GPIOQLPEN_W<'_, AHB4LPENRrs> {
        GPIOQLPEN_W::new(self, 16)
    }
    ///Bit 18 - PWR sleep enable
    #[inline(always)]
    pub fn pwrlpen(&mut self) -> PWRLPEN_W<'_, AHB4LPENRrs> {
        PWRLPEN_W::new(self, 18)
    }
    ///Bit 19 - CRC sleep enable
    #[inline(always)]
    pub fn crclpen(&mut self) -> CRCLPEN_W<'_, AHB4LPENRrs> {
        CRCLPEN_W::new(self, 19)
    }
}
/**RCC AHB4 Sleep enable register

You can [`read`](crate::Reg::read) this register and get [`ahb4lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RCC:AHB4LPENR)*/
pub struct AHB4LPENRrs;
impl crate::RegisterSpec for AHB4LPENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4lpenr::R`](R) reader structure
impl crate::Readable for AHB4LPENRrs {}
///`write(|w| ..)` method takes [`ahb4lpenr::W`](W) writer structure
impl crate::Writable for AHB4LPENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4LPENR to value 0x0004_0000
impl crate::Resettable for AHB4LPENRrs {
    const RESET_VALUE: u32 = 0x0004_0000;
}
