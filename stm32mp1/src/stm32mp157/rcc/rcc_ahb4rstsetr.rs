#[doc = "Register `RCC_AHB4RSTSETR` reader"]
pub type R = crate::R<RCC_AHB4RSTSETRrs>;
#[doc = "Register `RCC_AHB4RSTSETR` writer"]
pub type W = crate::W<RCC_AHB4RSTSETRrs>;
#[doc = "Field `GPIOARST` reader - GPIOARST"]
pub type GPIOARST_R = crate::BitReader;
#[doc = "Field `GPIOARST` writer - GPIOARST"]
pub type GPIOARST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - GPIOBRST"]
pub type GPIOBRST_R = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - GPIOBRST"]
pub type GPIOBRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - GPIOCRST"]
pub type GPIOCRST_R = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - GPIOCRST"]
pub type GPIOCRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - GPIODRST"]
pub type GPIODRST_R = crate::BitReader;
#[doc = "Field `GPIODRST` writer - GPIODRST"]
pub type GPIODRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - GPIOERST"]
pub type GPIOERST_R = crate::BitReader;
#[doc = "Field `GPIOERST` writer - GPIOERST"]
pub type GPIOERST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - GPIOFRST"]
pub type GPIOFRST_R = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - GPIOFRST"]
pub type GPIOFRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGRST` reader - GPIOGRST"]
pub type GPIOGRST_R = crate::BitReader;
#[doc = "Field `GPIOGRST` writer - GPIOGRST"]
pub type GPIOGRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHRST` reader - GPIOHRST"]
pub type GPIOHRST_R = crate::BitReader;
#[doc = "Field `GPIOHRST` writer - GPIOHRST"]
pub type GPIOHRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOIRST` reader - GPIOIRST"]
pub type GPIOIRST_R = crate::BitReader;
#[doc = "Field `GPIOIRST` writer - GPIOIRST"]
pub type GPIOIRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOJRST` reader - GPIOJRST"]
pub type GPIOJRST_R = crate::BitReader;
#[doc = "Field `GPIOJRST` writer - GPIOJRST"]
pub type GPIOJRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOKRST` reader - GPIOKRST"]
pub type GPIOKRST_R = crate::BitReader;
#[doc = "Field `GPIOKRST` writer - GPIOKRST"]
pub type GPIOKRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GPIOARST_R {
        GPIOARST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GPIOBRST_R {
        GPIOBRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GPIOCRST_R {
        GPIOCRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GPIODRST_R {
        GPIODRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GPIOERST_R {
        GPIOERST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GPIOFRST_R {
        GPIOFRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOGRST"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GPIOGRST_R {
        GPIOGRST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOHRST"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GPIOHRST_R {
        GPIOHRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOIRST"]
    #[inline(always)]
    pub fn gpioirst(&self) -> GPIOIRST_R {
        GPIOIRST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOJRST"]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GPIOJRST_R {
        GPIOJRST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOKRST"]
    #[inline(always)]
    pub fn gpiokrst(&self) -> GPIOKRST_R {
        GPIOKRST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOARST"]
    #[inline(always)]
    #[must_use]
    pub fn gpioarst(&mut self) -> GPIOARST_W<RCC_AHB4RSTSETRrs> {
        GPIOARST_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOBRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiobrst(&mut self) -> GPIOBRST_W<RCC_AHB4RSTSETRrs> {
        GPIOBRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOCRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiocrst(&mut self) -> GPIOCRST_W<RCC_AHB4RSTSETRrs> {
        GPIOCRST_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIODRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodrst(&mut self) -> GPIODRST_W<RCC_AHB4RSTSETRrs> {
        GPIODRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOERST"]
    #[inline(always)]
    #[must_use]
    pub fn gpioerst(&mut self) -> GPIOERST_W<RCC_AHB4RSTSETRrs> {
        GPIOERST_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOFRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiofrst(&mut self) -> GPIOFRST_W<RCC_AHB4RSTSETRrs> {
        GPIOFRST_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOGRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiogrst(&mut self) -> GPIOGRST_W<RCC_AHB4RSTSETRrs> {
        GPIOGRST_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOHRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohrst(&mut self) -> GPIOHRST_W<RCC_AHB4RSTSETRrs> {
        GPIOHRST_W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOIRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpioirst(&mut self) -> GPIOIRST_W<RCC_AHB4RSTSETRrs> {
        GPIOIRST_W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIOJRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiojrst(&mut self) -> GPIOJRST_W<RCC_AHB4RSTSETRrs> {
        GPIOJRST_W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIOKRST"]
    #[inline(always)]
    #[must_use]
    pub fn gpiokrst(&mut self) -> GPIOKRST_W<RCC_AHB4RSTSETRrs> {
        GPIOKRST_W::new(self, 10)
    }
}
#[doc = "This register is used to activate the reset of the corresponding peripheral\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_ahb4rstsetr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_ahb4rstsetr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_AHB4RSTSETRrs;
impl crate::RegisterSpec for RCC_AHB4RSTSETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_ahb4rstsetr::R`](R) reader structure"]
impl crate::Readable for RCC_AHB4RSTSETRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_ahb4rstsetr::W`](W) writer structure"]
impl crate::Writable for RCC_AHB4RSTSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_AHB4RSTSETR to value 0"]
impl crate::Resettable for RCC_AHB4RSTSETRrs {
    const RESET_VALUE: u32 = 0;
}
