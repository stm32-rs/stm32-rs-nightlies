#[doc = "Register `IMR1` reader"]
pub type R = crate::R<IMR1rs>;
#[doc = "Register `IMR1` writer"]
pub type W = crate::W<IMR1rs>;
#[doc = "Field `IM` reader - CPU wakeup with interrupt mask"]
pub type IM_R = crate::FieldReader<u16>;
#[doc = "Field `IM` writer - CPU wakeup with interrupt mask"]
pub type IM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `IM19` reader - IM19"]
pub type IM19_R = crate::BitReader;
#[doc = "Field `IM19` writer - IM19"]
pub type IM19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM23` reader - IM23"]
pub type IM23_R = crate::BitReader;
#[doc = "Field `IM23` writer - IM23"]
pub type IM23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM25` reader - IM25"]
pub type IM25_R = crate::BitReader;
#[doc = "Field `IM25` writer - IM25"]
pub type IM25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IM31` reader - IM31"]
pub type IM31_R = crate::BitReader;
#[doc = "Field `IM31` writer - IM31"]
pub type IM31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - CPU wakeup with interrupt mask"]
    #[inline(always)]
    pub fn im(&self) -> IM_R {
        IM_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 19 - IM19"]
    #[inline(always)]
    pub fn im19(&self) -> IM19_R {
        IM19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 23 - IM23"]
    #[inline(always)]
    pub fn im23(&self) -> IM23_R {
        IM23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - IM25"]
    #[inline(always)]
    pub fn im25(&self) -> IM25_R {
        IM25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 31 - IM31"]
    #[inline(always)]
    pub fn im31(&self) -> IM31_R {
        IM31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - CPU wakeup with interrupt mask"]
    #[inline(always)]
    #[must_use]
    pub fn im(&mut self) -> IM_W<IMR1rs> {
        IM_W::new(self, 0)
    }
    #[doc = "Bit 19 - IM19"]
    #[inline(always)]
    #[must_use]
    pub fn im19(&mut self) -> IM19_W<IMR1rs> {
        IM19_W::new(self, 19)
    }
    #[doc = "Bit 23 - IM23"]
    #[inline(always)]
    #[must_use]
    pub fn im23(&mut self) -> IM23_W<IMR1rs> {
        IM23_W::new(self, 23)
    }
    #[doc = "Bit 25 - IM25"]
    #[inline(always)]
    #[must_use]
    pub fn im25(&mut self) -> IM25_W<IMR1rs> {
        IM25_W::new(self, 25)
    }
    #[doc = "Bit 31 - IM31"]
    #[inline(always)]
    #[must_use]
    pub fn im31(&mut self) -> IM31_W<IMR1rs> {
        IM31_W::new(self, 31)
    }
}
#[doc = "EXTI CPU wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`imr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`imr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IMR1rs;
impl crate::RegisterSpec for IMR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr1::R`](R) reader structure"]
impl crate::Readable for IMR1rs {}
#[doc = "`write(|w| ..)` method takes [`imr1::W`](W) writer structure"]
impl crate::Writable for IMR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IMR1 to value 0xfff8_0000"]
impl crate::Resettable for IMR1rs {
    const RESET_VALUE: u32 = 0xfff8_0000;
}
