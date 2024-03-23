#[doc = "Register `TIM7_CCR5` reader"]
pub type R = crate::R<TIM7_CCR5rs>;
#[doc = "Register `TIM7_CCR5` writer"]
pub type W = crate::W<TIM7_CCR5rs>;
#[doc = "Field `CCR5` reader - CCR5"]
pub type CCR5_R = crate::FieldReader<u16>;
#[doc = "Field `CCR5` writer - CCR5"]
pub type CCR5_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `GC5C1` reader - GC5C1"]
pub type GC5C1_R = crate::BitReader;
#[doc = "Field `GC5C1` writer - GC5C1"]
pub type GC5C1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC5C2` reader - GC5C2"]
pub type GC5C2_R = crate::BitReader;
#[doc = "Field `GC5C2` writer - GC5C2"]
pub type GC5C2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GC5C3` reader - GC5C3"]
pub type GC5C3_R = crate::BitReader;
#[doc = "Field `GC5C3` writer - GC5C3"]
pub type GC5C3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CCR5"]
    #[inline(always)]
    pub fn ccr5(&self) -> CCR5_R {
        CCR5_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 29 - GC5C1"]
    #[inline(always)]
    pub fn gc5c1(&self) -> GC5C1_R {
        GC5C1_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - GC5C2"]
    #[inline(always)]
    pub fn gc5c2(&self) -> GC5C2_R {
        GC5C2_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - GC5C3"]
    #[inline(always)]
    pub fn gc5c3(&self) -> GC5C3_R {
        GC5C3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CCR5"]
    #[inline(always)]
    #[must_use]
    pub fn ccr5(&mut self) -> CCR5_W<TIM7_CCR5rs> {
        CCR5_W::new(self, 0)
    }
    #[doc = "Bit 29 - GC5C1"]
    #[inline(always)]
    #[must_use]
    pub fn gc5c1(&mut self) -> GC5C1_W<TIM7_CCR5rs> {
        GC5C1_W::new(self, 29)
    }
    #[doc = "Bit 30 - GC5C2"]
    #[inline(always)]
    #[must_use]
    pub fn gc5c2(&mut self) -> GC5C2_W<TIM7_CCR5rs> {
        GC5C2_W::new(self, 30)
    }
    #[doc = "Bit 31 - GC5C3"]
    #[inline(always)]
    #[must_use]
    pub fn gc5c3(&mut self) -> GC5C3_W<TIM7_CCR5rs> {
        GC5C3_W::new(self, 31)
    }
}
#[doc = "TIM7 capture/compare register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim7_ccr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim7_ccr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIM7_CCR5rs;
impl crate::RegisterSpec for TIM7_CCR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim7_ccr5::R`](R) reader structure"]
impl crate::Readable for TIM7_CCR5rs {}
#[doc = "`write(|w| ..)` method takes [`tim7_ccr5::W`](W) writer structure"]
impl crate::Writable for TIM7_CCR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIM7_CCR5 to value 0"]
impl crate::Resettable for TIM7_CCR5rs {
    const RESET_VALUE: u32 = 0;
}
