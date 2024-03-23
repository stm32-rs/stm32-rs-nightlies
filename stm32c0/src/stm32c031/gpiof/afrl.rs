#[doc = "Register `AFRL` reader"]
pub type R = crate::R<AFRLrs>;
#[doc = "Register `AFRL` writer"]
pub type W = crate::W<AFRLrs>;
#[doc = "Field `AFSEL0` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL0_R = crate::FieldReader;
#[doc = "Field `AFSEL0` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL1` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL1_R = crate::FieldReader;
#[doc = "Field `AFSEL1` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL2` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL2_R = crate::FieldReader;
#[doc = "Field `AFSEL2` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL3` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL3_R = crate::FieldReader;
#[doc = "Field `AFSEL3` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL4` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL4_R = crate::FieldReader;
#[doc = "Field `AFSEL4` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL5` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL5_R = crate::FieldReader;
#[doc = "Field `AFSEL5` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL6` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL6_R = crate::FieldReader;
#[doc = "Field `AFSEL6` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFSEL7` reader - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL7_R = crate::FieldReader;
#[doc = "Field `AFSEL7` writer - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
pub type AFSEL7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afsel0(&self) -> AFSEL0_R {
        AFSEL0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afsel1(&self) -> AFSEL1_R {
        AFSEL1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afsel2(&self) -> AFSEL2_R {
        AFSEL2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afsel3(&self) -> AFSEL3_R {
        AFSEL3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afsel4(&self) -> AFSEL4_R {
        AFSEL4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afsel5(&self) -> AFSEL5_R {
        AFSEL5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afsel6(&self) -> AFSEL6_R {
        AFSEL6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    pub fn afsel7(&self) -> AFSEL7_R {
        AFSEL7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afsel0(&mut self) -> AFSEL0_W<AFRLrs> {
        AFSEL0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afsel1(&mut self) -> AFSEL1_W<AFRLrs> {
        AFSEL1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afsel2(&mut self) -> AFSEL2_W<AFRLrs> {
        AFSEL2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afsel3(&mut self) -> AFSEL3_W<AFRLrs> {
        AFSEL3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afsel4(&mut self) -> AFSEL4_W<AFRLrs> {
        AFSEL4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afsel5(&mut self) -> AFSEL5_W<AFRLrs> {
        AFSEL5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afsel6(&mut self) -> AFSEL6_W<AFRLrs> {
        AFSEL6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - Alternate function selection for port x pin y (y = 0 to 7) These bits are written by software to configure alternate function I/Os"]
    #[inline(always)]
    #[must_use]
    pub fn afsel7(&mut self) -> AFSEL7_W<AFRLrs> {
        AFSEL7_W::new(self, 28)
    }
}
#[doc = "GPIO alternate function low register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`afrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`afrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AFRLrs;
impl crate::RegisterSpec for AFRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`afrl::R`](R) reader structure"]
impl crate::Readable for AFRLrs {}
#[doc = "`write(|w| ..)` method takes [`afrl::W`](W) writer structure"]
impl crate::Writable for AFRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AFRL to value 0"]
impl crate::Resettable for AFRLrs {
    const RESET_VALUE: u32 = 0;
}
