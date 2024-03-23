#[doc = "Register `RCC_MP_AHB4LPENCLRR` reader"]
pub type R = crate::R<RCC_MP_AHB4LPENCLRRrs>;
#[doc = "Register `RCC_MP_AHB4LPENCLRR` writer"]
pub type W = crate::W<RCC_MP_AHB4LPENCLRRrs>;
#[doc = "Field `GPIOALPEN` reader - GPIOALPEN"]
pub type GPIOALPEN_R = crate::BitReader;
#[doc = "Field `GPIOALPEN` writer - GPIOALPEN"]
pub type GPIOALPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBLPEN` reader - GPIOBLPEN"]
pub type GPIOBLPEN_R = crate::BitReader;
#[doc = "Field `GPIOBLPEN` writer - GPIOBLPEN"]
pub type GPIOBLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCLPEN` reader - GPIOCLPEN"]
pub type GPIOCLPEN_R = crate::BitReader;
#[doc = "Field `GPIOCLPEN` writer - GPIOCLPEN"]
pub type GPIOCLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODLPEN` reader - GPIODLPEN"]
pub type GPIODLPEN_R = crate::BitReader;
#[doc = "Field `GPIODLPEN` writer - GPIODLPEN"]
pub type GPIODLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOELPEN` reader - GPIOELPEN"]
pub type GPIOELPEN_R = crate::BitReader;
#[doc = "Field `GPIOELPEN` writer - GPIOELPEN"]
pub type GPIOELPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFLPEN` reader - GPIOFLPEN"]
pub type GPIOFLPEN_R = crate::BitReader;
#[doc = "Field `GPIOFLPEN` writer - GPIOFLPEN"]
pub type GPIOFLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGLPEN` reader - GPIOGLPEN"]
pub type GPIOGLPEN_R = crate::BitReader;
#[doc = "Field `GPIOGLPEN` writer - GPIOGLPEN"]
pub type GPIOGLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHLPEN` reader - GPIOHLPEN"]
pub type GPIOHLPEN_R = crate::BitReader;
#[doc = "Field `GPIOHLPEN` writer - GPIOHLPEN"]
pub type GPIOHLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOILPEN` reader - GPIOILPEN"]
pub type GPIOILPEN_R = crate::BitReader;
#[doc = "Field `GPIOILPEN` writer - GPIOILPEN"]
pub type GPIOILPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOJLPEN` reader - GPIOJLPEN"]
pub type GPIOJLPEN_R = crate::BitReader;
#[doc = "Field `GPIOJLPEN` writer - GPIOJLPEN"]
pub type GPIOJLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOKLPEN` reader - GPIOKLPEN"]
pub type GPIOKLPEN_R = crate::BitReader;
#[doc = "Field `GPIOKLPEN` writer - GPIOKLPEN"]
pub type GPIOKLPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIOALPEN"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GPIOALPEN_R {
        GPIOALPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIOBLPEN"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GPIOBLPEN_R {
        GPIOBLPEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIOCLPEN"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GPIOCLPEN_R {
        GPIOCLPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIODLPEN"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GPIODLPEN_R {
        GPIODLPEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIOELPEN"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GPIOELPEN_R {
        GPIOELPEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIOFLPEN"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GPIOFLPEN_R {
        GPIOFLPEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIOGLPEN"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GPIOGLPEN_R {
        GPIOGLPEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIOHLPEN"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GPIOHLPEN_R {
        GPIOHLPEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIOILPEN"]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GPIOILPEN_R {
        GPIOILPEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIOJLPEN"]
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GPIOJLPEN_R {
        GPIOJLPEN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIOKLPEN"]
    #[inline(always)]
    pub fn gpioklpen(&self) -> GPIOKLPEN_R {
        GPIOKLPEN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIOALPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioalpen(&mut self) -> GPIOALPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOALPEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - GPIOBLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioblpen(&mut self) -> GPIOBLPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOBLPEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - GPIOCLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioclpen(&mut self) -> GPIOCLPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOCLPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - GPIODLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiodlpen(&mut self) -> GPIODLPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIODLPEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - GPIOELPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioelpen(&mut self) -> GPIOELPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOELPEN_W::new(self, 4)
    }
    #[doc = "Bit 5 - GPIOFLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioflpen(&mut self) -> GPIOFLPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOFLPEN_W::new(self, 5)
    }
    #[doc = "Bit 6 - GPIOGLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioglpen(&mut self) -> GPIOGLPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOGLPEN_W::new(self, 6)
    }
    #[doc = "Bit 7 - GPIOHLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiohlpen(&mut self) -> GPIOHLPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOHLPEN_W::new(self, 7)
    }
    #[doc = "Bit 8 - GPIOILPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioilpen(&mut self) -> GPIOILPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOILPEN_W::new(self, 8)
    }
    #[doc = "Bit 9 - GPIOJLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpiojlpen(&mut self) -> GPIOJLPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOJLPEN_W::new(self, 9)
    }
    #[doc = "Bit 10 - GPIOKLPEN"]
    #[inline(always)]
    #[must_use]
    pub fn gpioklpen(&mut self) -> GPIOKLPEN_W<RCC_MP_AHB4LPENCLRRrs> {
        GPIOKLPEN_W::new(self, 10)
    }
}
#[doc = "This register is used by the MPU\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcc_mp_ahb4lpenclrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcc_mp_ahb4lpenclrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCC_MP_AHB4LPENCLRRrs;
impl crate::RegisterSpec for RCC_MP_AHB4LPENCLRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcc_mp_ahb4lpenclrr::R`](R) reader structure"]
impl crate::Readable for RCC_MP_AHB4LPENCLRRrs {}
#[doc = "`write(|w| ..)` method takes [`rcc_mp_ahb4lpenclrr::W`](W) writer structure"]
impl crate::Writable for RCC_MP_AHB4LPENCLRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCC_MP_AHB4LPENCLRR to value 0x07ff"]
impl crate::Resettable for RCC_MP_AHB4LPENCLRRrs {
    const RESET_VALUE: u32 = 0x07ff;
}
