#[doc = "Register `GPIOB_AFRL` reader"]
pub type R = crate::R<GPIOB_AFRLrs>;
#[doc = "Register `GPIOB_AFRL` writer"]
pub type W = crate::W<GPIOB_AFRLrs>;
#[doc = "Field `AFR0` reader - AFR0"]
pub type AFR0_R = crate::FieldReader;
#[doc = "Field `AFR0` writer - AFR0"]
pub type AFR0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR1` reader - AFR1"]
pub type AFR1_R = crate::FieldReader;
#[doc = "Field `AFR1` writer - AFR1"]
pub type AFR1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR2` reader - AFR2"]
pub type AFR2_R = crate::FieldReader;
#[doc = "Field `AFR2` writer - AFR2"]
pub type AFR2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR3` reader - AFR3"]
pub type AFR3_R = crate::FieldReader;
#[doc = "Field `AFR3` writer - AFR3"]
pub type AFR3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR4` reader - AFR4"]
pub type AFR4_R = crate::FieldReader;
#[doc = "Field `AFR4` writer - AFR4"]
pub type AFR4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR5` reader - AFR5"]
pub type AFR5_R = crate::FieldReader;
#[doc = "Field `AFR5` writer - AFR5"]
pub type AFR5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR6` reader - AFR6"]
pub type AFR6_R = crate::FieldReader;
#[doc = "Field `AFR6` writer - AFR6"]
pub type AFR6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR7` reader - AFR7"]
pub type AFR7_R = crate::FieldReader;
#[doc = "Field `AFR7` writer - AFR7"]
pub type AFR7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AFR0"]
    #[inline(always)]
    pub fn afr0(&self) -> AFR0_R {
        AFR0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AFR1"]
    #[inline(always)]
    pub fn afr1(&self) -> AFR1_R {
        AFR1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AFR2"]
    #[inline(always)]
    pub fn afr2(&self) -> AFR2_R {
        AFR2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AFR3"]
    #[inline(always)]
    pub fn afr3(&self) -> AFR3_R {
        AFR3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AFR4"]
    #[inline(always)]
    pub fn afr4(&self) -> AFR4_R {
        AFR4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AFR5"]
    #[inline(always)]
    pub fn afr5(&self) -> AFR5_R {
        AFR5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AFR6"]
    #[inline(always)]
    pub fn afr6(&self) -> AFR6_R {
        AFR6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AFR7"]
    #[inline(always)]
    pub fn afr7(&self) -> AFR7_R {
        AFR7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AFR0"]
    #[inline(always)]
    #[must_use]
    pub fn afr0(&mut self) -> AFR0_W<GPIOB_AFRLrs> {
        AFR0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - AFR1"]
    #[inline(always)]
    #[must_use]
    pub fn afr1(&mut self) -> AFR1_W<GPIOB_AFRLrs> {
        AFR1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - AFR2"]
    #[inline(always)]
    #[must_use]
    pub fn afr2(&mut self) -> AFR2_W<GPIOB_AFRLrs> {
        AFR2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - AFR3"]
    #[inline(always)]
    #[must_use]
    pub fn afr3(&mut self) -> AFR3_W<GPIOB_AFRLrs> {
        AFR3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - AFR4"]
    #[inline(always)]
    #[must_use]
    pub fn afr4(&mut self) -> AFR4_W<GPIOB_AFRLrs> {
        AFR4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - AFR5"]
    #[inline(always)]
    #[must_use]
    pub fn afr5(&mut self) -> AFR5_W<GPIOB_AFRLrs> {
        AFR5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - AFR6"]
    #[inline(always)]
    #[must_use]
    pub fn afr6(&mut self) -> AFR6_W<GPIOB_AFRLrs> {
        AFR6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - AFR7"]
    #[inline(always)]
    #[must_use]
    pub fn afr7(&mut self) -> AFR7_W<GPIOB_AFRLrs> {
        AFR7_W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpiob_afrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpiob_afrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOB_AFRLrs;
impl crate::RegisterSpec for GPIOB_AFRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpiob_afrl::R`](R) reader structure"]
impl crate::Readable for GPIOB_AFRLrs {}
#[doc = "`write(|w| ..)` method takes [`gpiob_afrl::W`](W) writer structure"]
impl crate::Writable for GPIOB_AFRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOB_AFRL to value 0"]
impl crate::Resettable for GPIOB_AFRLrs {
    const RESET_VALUE: u32 = 0;
}
