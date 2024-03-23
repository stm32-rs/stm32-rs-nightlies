#[doc = "Register `EMR2` reader"]
pub type R = crate::R<EMR2rs>;
#[doc = "Register `EMR2` writer"]
pub type W = crate::W<EMR2rs>;
#[doc = "Field `EM32` reader - CPU wakeup with interrupt mask on event input"]
pub type EM32_R = crate::BitReader;
#[doc = "Field `EM32` writer - CPU wakeup with interrupt mask on event input"]
pub type EM32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM33` reader - CPU wakeup with interrupt mask on event input"]
pub type EM33_R = crate::BitReader;
#[doc = "Field `EM33` writer - CPU wakeup with interrupt mask on event input"]
pub type EM33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM34` reader - CPU wakeup with interrupt mask on event input"]
pub type EM34_R = crate::BitReader;
#[doc = "Field `EM34` writer - CPU wakeup with interrupt mask on event input"]
pub type EM34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM35` reader - CPU wakeup with interrupt mask on event input"]
pub type EM35_R = crate::BitReader;
#[doc = "Field `EM35` writer - CPU wakeup with interrupt mask on event input"]
pub type EM35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM36` reader - CPU wakeup with interrupt mask on event input"]
pub type EM36_R = crate::BitReader;
#[doc = "Field `EM36` writer - CPU wakeup with interrupt mask on event input"]
pub type EM36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM37` reader - CPU wakeup with interrupt mask on event input"]
pub type EM37_R = crate::BitReader;
#[doc = "Field `EM37` writer - CPU wakeup with interrupt mask on event input"]
pub type EM37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM38` reader - CPU wakeup with interrupt mask on event input"]
pub type EM38_R = crate::BitReader;
#[doc = "Field `EM38` writer - CPU wakeup with interrupt mask on event input"]
pub type EM38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM40` reader - CPU wakeup with interrupt mask on event input"]
pub type EM40_R = crate::BitReader;
#[doc = "Field `EM40` writer - CPU wakeup with interrupt mask on event input"]
pub type EM40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM41` reader - CPU wakeup with interrupt mask on event input"]
pub type EM41_R = crate::BitReader;
#[doc = "Field `EM41` writer - CPU wakeup with interrupt mask on event input"]
pub type EM41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM42` reader - CPU wakeup with interrupt mask on event input"]
pub type EM42_R = crate::BitReader;
#[doc = "Field `EM42` writer - CPU wakeup with interrupt mask on event input"]
pub type EM42_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em32(&self) -> EM32_R {
        EM32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em33(&self) -> EM33_R {
        EM33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em34(&self) -> EM34_R {
        EM34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em35(&self) -> EM35_R {
        EM35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em36(&self) -> EM36_R {
        EM36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em37(&self) -> EM37_R {
        EM37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em38(&self) -> EM38_R {
        EM38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em40(&self) -> EM40_R {
        EM40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em41(&self) -> EM41_R {
        EM41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    pub fn em42(&self) -> EM42_R {
        EM42_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em32(&mut self) -> EM32_W<EMR2rs> {
        EM32_W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em33(&mut self) -> EM33_W<EMR2rs> {
        EM33_W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em34(&mut self) -> EM34_W<EMR2rs> {
        EM34_W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em35(&mut self) -> EM35_W<EMR2rs> {
        EM35_W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em36(&mut self) -> EM36_W<EMR2rs> {
        EM36_W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em37(&mut self) -> EM37_W<EMR2rs> {
        EM37_W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em38(&mut self) -> EM38_W<EMR2rs> {
        EM38_W::new(self, 6)
    }
    #[doc = "Bit 8 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em40(&mut self) -> EM40_W<EMR2rs> {
        EM40_W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em41(&mut self) -> EM41_W<EMR2rs> {
        EM41_W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU wakeup with interrupt mask on event input"]
    #[inline(always)]
    #[must_use]
    pub fn em42(&mut self) -> EM42_W<EMR2rs> {
        EM42_W::new(self, 10)
    }
}
#[doc = "EXTI CPU wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`emr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`emr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EMR2rs;
impl crate::RegisterSpec for EMR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr2::R`](R) reader structure"]
impl crate::Readable for EMR2rs {}
#[doc = "`write(|w| ..)` method takes [`emr2::W`](W) writer structure"]
impl crate::Writable for EMR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EMR2 to value 0"]
impl crate::Resettable for EMR2rs {
    const RESET_VALUE: u32 = 0;
}
