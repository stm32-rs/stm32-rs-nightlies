///Register `AHB4RSTR` reader
pub type R = crate::R<AHB4RSTRrs>;
///Register `AHB4RSTR` writer
pub type W = crate::W<AHB4RSTRrs>;
///Field `GPIOARST` reader - GPIOA reset
pub type GPIOARST_R = crate::BitReader;
///Field `GPIOARST` writer - GPIOA reset
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBRST` reader - GPIOB reset
pub type GPIOBRST_R = crate::BitReader;
///Field `GPIOBRST` writer - GPIOB reset
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCRST` reader - GPIOC reset
pub type GPIOCRST_R = crate::BitReader;
///Field `GPIOCRST` writer - GPIOC reset
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODRST` reader - GPIOD reset
pub type GPIODRST_R = crate::BitReader;
///Field `GPIODRST` writer - GPIOD reset
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOERST` reader - GPIOE reset
pub type GPIOERST_R = crate::BitReader;
///Field `GPIOERST` writer - GPIOE reset
pub type GPIOERST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFRST` reader - GPIOF reset
pub type GPIOFRST_R = crate::BitReader;
///Field `GPIOFRST` writer - GPIOF reset
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGRST` reader - GPIOG reset
pub type GPIOGRST_R = crate::BitReader;
///Field `GPIOGRST` writer - GPIOG reset
pub type GPIOGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHRST` reader - GPIOH reset
pub type GPIOHRST_R = crate::BitReader;
///Field `GPIOHRST` writer - GPIOH reset
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIONRST` reader - GPION reset
pub type GPIONRST_R = crate::BitReader;
///Field `GPIONRST` writer - GPION reset
pub type GPIONRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOORST` reader - GPIOO reset
pub type GPIOORST_R = crate::BitReader;
///Field `GPIOORST` writer - GPIOO reset
pub type GPIOORST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOPRST` reader - GPIOP reset
pub type GPIOPRST_R = crate::BitReader;
///Field `GPIOPRST` writer - GPIOP reset
pub type GPIOPRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOQRST` reader - GPIOQ reset
pub type GPIOQRST_R = crate::BitReader;
///Field `GPIOQRST` writer - GPIOQ reset
pub type GPIOQRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PWRRST` reader - PWR reset
pub type PWRRST_R = crate::BitReader;
///Field `PWRRST` writer - PWR reset
pub type PWRRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCRST` reader - CRC reset
pub type CRCRST_R = crate::BitReader;
///Field `CRCRST` writer - CRC reset
pub type CRCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIOA reset
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOB reset
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOC reset
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIOD reset
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOE reset
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOF reset
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOG reset
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOH reset
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 13 - GPION reset
    #[inline(always)]
    pub fn gpionrst(&self) -> GPIONRST_R {
        GPIONRST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - GPIOO reset
    #[inline(always)]
    pub fn gpioorst(&self) -> GPIOORST_R {
        GPIOORST_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - GPIOP reset
    #[inline(always)]
    pub fn gpioprst(&self) -> GPIOPRST_R {
        GPIOPRST_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - GPIOQ reset
    #[inline(always)]
    pub fn gpioqrst(&self) -> GPIOQRST_R {
        GPIOQRST_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 18 - PWR reset
    #[inline(always)]
    pub fn pwrrst(&self) -> PWRRST_R {
        PWRRST_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - CRC reset
    #[inline(always)]
    pub fn crcrst(&self) -> CRCRST_R {
        CRCRST_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4RSTR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("gpionrst", &self.gpionrst())
            .field("gpioorst", &self.gpioorst())
            .field("gpioprst", &self.gpioprst())
            .field("gpioqrst", &self.gpioqrst())
            .field("pwrrst", &self.pwrrst())
            .field("crcrst", &self.crcrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOA reset
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHB4RSTRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - GPIOB reset
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHB4RSTRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - GPIOC reset
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, AHB4RSTRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - GPIOD reset
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, AHB4RSTRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - GPIOE reset
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, AHB4RSTRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - GPIOF reset
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, AHB4RSTRrs> {
        GPIOFRST_W::new(self, 5)
    }
    ///Bit 6 - GPIOG reset
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<'_, AHB4RSTRrs> {
        GPIOGRST_W::new(self, 6)
    }
    ///Bit 7 - GPIOH reset
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<'_, AHB4RSTRrs> {
        GPIOHRST_W::new(self, 7)
    }
    ///Bit 13 - GPION reset
    #[inline(always)]
    pub fn gpionrst(&mut self) -> GPIONRST_W<'_, AHB4RSTRrs> {
        GPIONRST_W::new(self, 13)
    }
    ///Bit 14 - GPIOO reset
    #[inline(always)]
    pub fn gpioorst(&mut self) -> GPIOORST_W<'_, AHB4RSTRrs> {
        GPIOORST_W::new(self, 14)
    }
    ///Bit 15 - GPIOP reset
    #[inline(always)]
    pub fn gpioprst(&mut self) -> GPIOPRST_W<'_, AHB4RSTRrs> {
        GPIOPRST_W::new(self, 15)
    }
    ///Bit 16 - GPIOQ reset
    #[inline(always)]
    pub fn gpioqrst(&mut self) -> GPIOQRST_W<'_, AHB4RSTRrs> {
        GPIOQRST_W::new(self, 16)
    }
    ///Bit 18 - PWR reset
    #[inline(always)]
    pub fn pwrrst(&mut self) -> PWRRST_W<'_, AHB4RSTRrs> {
        PWRRST_W::new(self, 18)
    }
    ///Bit 19 - CRC reset
    #[inline(always)]
    pub fn crcrst(&mut self) -> CRCRST_W<'_, AHB4RSTRrs> {
        CRCRST_W::new(self, 19)
    }
}
/**RCC AHB4 reset register

You can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#RCC:AHB4RSTR)*/
pub struct AHB4RSTRrs;
impl crate::RegisterSpec for AHB4RSTRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4rstr::R`](R) reader structure
impl crate::Readable for AHB4RSTRrs {}
///`write(|w| ..)` method takes [`ahb4rstr::W`](W) writer structure
impl crate::Writable for AHB4RSTRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4RSTR to value 0
impl crate::Resettable for AHB4RSTRrs {}
