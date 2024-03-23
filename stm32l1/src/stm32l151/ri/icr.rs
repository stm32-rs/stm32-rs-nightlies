#[doc = "Register `ICR` reader"]
pub type R = crate::R<ICRrs>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `IC1IOS` reader - Input capture 1 select bits"]
pub type IC1IOS_R = crate::FieldReader;
#[doc = "Field `IC1IOS` writer - Input capture 1 select bits"]
pub type IC1IOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IC2IOS` reader - Input capture 2 select bits"]
pub type IC2IOS_R = crate::FieldReader;
#[doc = "Field `IC2IOS` writer - Input capture 2 select bits"]
pub type IC2IOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IC3IOS` reader - Input capture 3 select bits"]
pub type IC3IOS_R = crate::FieldReader;
#[doc = "Field `IC3IOS` writer - Input capture 3 select bits"]
pub type IC3IOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IC4IOS` reader - Input capture 4 select bits"]
pub type IC4IOS_R = crate::FieldReader;
#[doc = "Field `IC4IOS` writer - Input capture 4 select bits"]
pub type IC4IOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TIM` reader - Timer select bits"]
pub type TIM_R = crate::FieldReader;
#[doc = "Field `TIM` writer - Timer select bits"]
pub type TIM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `IC1` reader - IC1"]
pub type IC1_R = crate::BitReader;
#[doc = "Field `IC1` writer - IC1"]
pub type IC1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC2` reader - IC2"]
pub type IC2_R = crate::BitReader;
#[doc = "Field `IC2` writer - IC2"]
pub type IC2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC3` reader - IC3"]
pub type IC3_R = crate::BitReader;
#[doc = "Field `IC3` writer - IC3"]
pub type IC3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IC4` reader - IC4"]
pub type IC4_R = crate::BitReader;
#[doc = "Field `IC4` writer - IC4"]
pub type IC4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Input capture 1 select bits"]
    #[inline(always)]
    pub fn ic1ios(&self) -> IC1IOS_R {
        IC1IOS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 2 select bits"]
    #[inline(always)]
    pub fn ic2ios(&self) -> IC2IOS_R {
        IC2IOS_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Input capture 3 select bits"]
    #[inline(always)]
    pub fn ic3ios(&self) -> IC3IOS_R {
        IC3IOS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 4 select bits"]
    #[inline(always)]
    pub fn ic4ios(&self) -> IC4IOS_R {
        IC4IOS_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Timer select bits"]
    #[inline(always)]
    pub fn tim(&self) -> TIM_R {
        TIM_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - IC1"]
    #[inline(always)]
    pub fn ic1(&self) -> IC1_R {
        IC1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - IC2"]
    #[inline(always)]
    pub fn ic2(&self) -> IC2_R {
        IC2_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - IC3"]
    #[inline(always)]
    pub fn ic3(&self) -> IC3_R {
        IC3_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - IC4"]
    #[inline(always)]
    pub fn ic4(&self) -> IC4_R {
        IC4_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Input capture 1 select bits"]
    #[inline(always)]
    #[must_use]
    pub fn ic1ios(&mut self) -> IC1IOS_W<ICRrs> {
        IC1IOS_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Input capture 2 select bits"]
    #[inline(always)]
    #[must_use]
    pub fn ic2ios(&mut self) -> IC2IOS_W<ICRrs> {
        IC2IOS_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Input capture 3 select bits"]
    #[inline(always)]
    #[must_use]
    pub fn ic3ios(&mut self) -> IC3IOS_W<ICRrs> {
        IC3IOS_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Input capture 4 select bits"]
    #[inline(always)]
    #[must_use]
    pub fn ic4ios(&mut self) -> IC4IOS_W<ICRrs> {
        IC4IOS_W::new(self, 12)
    }
    #[doc = "Bits 16:17 - Timer select bits"]
    #[inline(always)]
    #[must_use]
    pub fn tim(&mut self) -> TIM_W<ICRrs> {
        TIM_W::new(self, 16)
    }
    #[doc = "Bit 18 - IC1"]
    #[inline(always)]
    #[must_use]
    pub fn ic1(&mut self) -> IC1_W<ICRrs> {
        IC1_W::new(self, 18)
    }
    #[doc = "Bit 19 - IC2"]
    #[inline(always)]
    #[must_use]
    pub fn ic2(&mut self) -> IC2_W<ICRrs> {
        IC2_W::new(self, 19)
    }
    #[doc = "Bit 20 - IC3"]
    #[inline(always)]
    #[must_use]
    pub fn ic3(&mut self) -> IC3_W<ICRrs> {
        IC3_W::new(self, 20)
    }
    #[doc = "Bit 21 - IC4"]
    #[inline(always)]
    #[must_use]
    pub fn ic4(&mut self) -> IC4_W<ICRrs> {
        IC4_W::new(self, 21)
    }
}
#[doc = "RI input capture register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for ICRrs {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
