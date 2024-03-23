#[doc = "Register `IMR2` reader"]
pub type R = crate::R<IMR2rs>;
#[doc = "Register `IMR2` writer"]
pub type W = crate::W<IMR2rs>;
#[doc = "Field `IM32` reader - CPU wakeup with interrupt mask on event input"]
pub type IM32_R = crate::BitReader;
#[doc = "Field `IM32` writer - CPU wakeup with interrupt mask on event input"]
pub type IM32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM33` reader - CPU wakeup with interrupt mask on event input"]
pub type IM33_R = crate::BitReader;
#[doc = "Field `IM33` writer - CPU wakeup with interrupt mask on event input"]
pub type IM33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM34` reader - CPU wakeup with interrupt mask on event input"]
pub type IM34_R = crate::BitReader;
#[doc = "Field `IM34` writer - CPU wakeup with interrupt mask on event input"]
pub type IM34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM35` reader - CPU wakeup with interrupt mask on event input"]
pub type IM35_R = crate::BitReader;
#[doc = "Field `IM35` writer - CPU wakeup with interrupt mask on event input"]
pub type IM35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM36` reader - CPU wakeup with interrupt mask on event input"]
pub type IM36_R = crate::BitReader;
#[doc = "Field `IM36` writer - CPU wakeup with interrupt mask on event input"]
pub type IM36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM37` reader - CPU wakeup with interrupt mask on event input"]
pub type IM37_R = crate::BitReader;
#[doc = "Field `IM37` writer - CPU wakeup with interrupt mask on event input"]
pub type IM37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM38` reader - CPU wakeup with interrupt mask on event input"]
pub type IM38_R = crate::BitReader;
#[doc = "Field `IM38` writer - CPU wakeup with interrupt mask on event input"]
pub type IM38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM40` reader - CPU wakeup with interrupt mask on event input"]
pub type IM40_R = crate::BitReader;
#[doc = "Field `IM40` writer - CPU wakeup with interrupt mask on event input"]
pub type IM40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM41` reader - CPU wakeup with interrupt mask on event input"]
pub type IM41_R = crate::BitReader;
#[doc = "Field `IM41` writer - CPU wakeup with interrupt mask on event input"]
pub type IM41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM42` reader - CPU wakeup with interrupt mask on event input"]
pub type IM42_R = crate::BitReader;
#[doc = "Field `IM42` writer - CPU wakeup with interrupt mask on event input"]
pub type IM42_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im32(&self) -> IM32_R {
        IM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im33(&self) -> IM33_R {
        IM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im34(&self) -> IM34_R {
        IM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im35(&self) -> IM35_R {
        IM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im36(&self) -> IM36_R {
        IM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im37(&self) -> IM37_R {
        IM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im38(&self) -> IM38_R {
        IM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im40(&self) -> IM40_R {
        IM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im41(&self) -> IM41_R {
        IM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn im42(&self) -> IM42_R {
        IM42_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im32(&mut self) -> IM32_W<IMR2rs> {
        IM32_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im33(&mut self) -> IM33_W<IMR2rs> {
        IM33_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im34(&mut self) -> IM34_W<IMR2rs> {
        IM34_W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im35(&mut self) -> IM35_W<IMR2rs> {
        IM35_W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im36(&mut self) -> IM36_W<IMR2rs> {
        IM36_W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im37(&mut self) -> IM37_W<IMR2rs> {
        IM37_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im38(&mut self) -> IM38_W<IMR2rs> {
        IM38_W::new(self, 6)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im40(&mut self) -> IM40_W<IMR2rs> {
        IM40_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im41(&mut self) -> IM41_W<IMR2rs> {
        IM41_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn im42(&mut self) -> IM42_W<IMR2rs> {
        IM42_W::new(self, 10)
    }
}
#[doc = "EXTI CPUm wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR2rs;
impl crate::RegisterSpec for IMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr2::R`](R) reader structure"]
impl crate::Readable for IMR2rs {}
#[doc = "`write(|w| ..)` method takes [`imr2::W`](W) writer structure"]
impl crate::Writable for IMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR2 to value 0x0787"]
impl crate::Resettable for IMR2rs {
    const RESET_VALUE: u32 = 0x0787;
}
