///Register `AHB4RSTCLRR` reader
pub type R = crate::R<AHB4RSTCLRRrs>;
///Register `AHB4RSTCLRR` writer
pub type W = crate::W<AHB4RSTCLRRrs>;
///Field `GPIOARST` reader - GPIOARST
pub type GPIOARST_R = crate::BitReader;
///Field `GPIOARST` writer - GPIOARST
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOBRST` reader - GPIOBRST
pub type GPIOBRST_R = crate::BitReader;
///Field `GPIOBRST` writer - GPIOBRST
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOCRST` reader - GPIOCRST
pub type GPIOCRST_R = crate::BitReader;
///Field `GPIOCRST` writer - GPIOCRST
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIODRST` reader - GPIODRST
pub type GPIODRST_R = crate::BitReader;
///Field `GPIODRST` writer - GPIODRST
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOERST` reader - GPIOERST
pub type GPIOERST_R = crate::BitReader;
///Field `GPIOERST` writer - GPIOERST
pub type GPIOERST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOFRST` reader - GPIOFRST
pub type GPIOFRST_R = crate::BitReader;
///Field `GPIOFRST` writer - GPIOFRST
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOGRST` reader - GPIOGRST
pub type GPIOGRST_R = crate::BitReader;
///Field `GPIOGRST` writer - GPIOGRST
pub type GPIOGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOHRST` reader - GPIOHRST
pub type GPIOHRST_R = crate::BitReader;
///Field `GPIOHRST` writer - GPIOHRST
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOIRST` reader - GPIOIRST
pub type GPIOIRST_R = crate::BitReader;
///Field `GPIOIRST` writer - GPIOIRST
pub type GPIOIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOJRST` reader - GPIOJRST
pub type GPIOJRST_R = crate::BitReader;
///Field `GPIOJRST` writer - GPIOJRST
pub type GPIOJRST_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GPIOKRST` reader - GPIOKRST
pub type GPIOKRST_R = crate::BitReader;
///Field `GPIOKRST` writer - GPIOKRST
pub type GPIOKRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - GPIOARST
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - GPIOBRST
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - GPIOCRST
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - GPIODRST
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - GPIOERST
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - GPIOFRST
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - GPIOGRST
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - GPIOHRST
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - GPIOIRST
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - GPIOJRST
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - GPIOKRST
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB4RSTCLRR")
            .field("gpioarst", &self.gpioarst())
            .field("gpiobrst", &self.gpiobrst())
            .field("gpiocrst", &self.gpiocrst())
            .field("gpiodrst", &self.gpiodrst())
            .field("gpioerst", &self.gpioerst())
            .field("gpiofrst", &self.gpiofrst())
            .field("gpiogrst", &self.gpiogrst())
            .field("gpiohrst", &self.gpiohrst())
            .field("gpioirst", &self.gpioirst())
            .field("gpiojrst", &self.gpiojrst())
            .field("gpiokrst", &self.gpiokrst())
            .finish()
    }
}
impl W {
    ///Bit 0 - GPIOARST
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GPIOARST_W<'_, AHB4RSTCLRRrs> {
        GPIOARST_W::new(self, 0)
    }
    ///Bit 1 - GPIOBRST
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<'_, AHB4RSTCLRRrs> {
        GPIOBRST_W::new(self, 1)
    }
    ///Bit 2 - GPIOCRST
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<'_, AHB4RSTCLRRrs> {
        GPIOCRST_W::new(self, 2)
    }
    ///Bit 3 - GPIODRST
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<'_, AHB4RSTCLRRrs> {
        GPIODRST_W::new(self, 3)
    }
    ///Bit 4 - GPIOERST
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GPIOERST_W<'_, AHB4RSTCLRRrs> {
        GPIOERST_W::new(self, 4)
    }
    ///Bit 5 - GPIOFRST
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<'_, AHB4RSTCLRRrs> {
        GPIOFRST_W::new(self, 5)
    }
    ///Bit 6 - GPIOGRST
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<'_, AHB4RSTCLRRrs> {
        GPIOGRST_W::new(self, 6)
    }
    ///Bit 7 - GPIOHRST
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<'_, AHB4RSTCLRRrs> {
        GPIOHRST_W::new(self, 7)
    }
    ///Bit 8 - GPIOIRST
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<'_, AHB4RSTCLRRrs> {
        GPIOIRST_W::new(self, 8)
    }
    ///Bit 9 - GPIOJRST
    #[inline(always)]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<'_, AHB4RSTCLRRrs> {
        GPIOJRST_W::new(self, 9)
    }
    ///Bit 10 - GPIOKRST
    #[inline(always)]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W<'_, AHB4RSTCLRRrs> {
        GPIOKRST_W::new(self, 10)
    }
}
/**This register is used to release the reset of the corresponding peripheral.

You can [`read`](crate::Reg::read) this register and get [`ahb4rstclrr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstclrr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:AHB4RSTCLRR)*/
pub struct AHB4RSTCLRRrs;
impl crate::RegisterSpec for AHB4RSTCLRRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb4rstclrr::R`](R) reader structure
impl crate::Readable for AHB4RSTCLRRrs {}
///`write(|w| ..)` method takes [`ahb4rstclrr::W`](W) writer structure
impl crate::Writable for AHB4RSTCLRRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB4RSTCLRR to value 0
impl crate::Resettable for AHB4RSTCLRRrs {}
