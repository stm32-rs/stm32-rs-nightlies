#[doc = "Register `GPIOZ_AFRH` reader"]
pub type R = crate::R<GPIOZ_AFRHrs>;
#[doc = "Register `GPIOZ_AFRH` writer"]
pub type W = crate::W<GPIOZ_AFRHrs>;
#[doc = "Field `AFR8` reader - AFR8"]
pub type AFR8_R = crate::FieldReader;
#[doc = "Field `AFR8` writer - AFR8"]
pub type AFR8_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR9` reader - AFR9"]
pub type AFR9_R = crate::FieldReader;
#[doc = "Field `AFR9` writer - AFR9"]
pub type AFR9_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR10` reader - AFR10"]
pub type AFR10_R = crate::FieldReader;
#[doc = "Field `AFR10` writer - AFR10"]
pub type AFR10_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR11` reader - AFR11"]
pub type AFR11_R = crate::FieldReader;
#[doc = "Field `AFR11` writer - AFR11"]
pub type AFR11_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR12` reader - AFR12"]
pub type AFR12_R = crate::FieldReader;
#[doc = "Field `AFR12` writer - AFR12"]
pub type AFR12_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR13` reader - AFR13"]
pub type AFR13_R = crate::FieldReader;
#[doc = "Field `AFR13` writer - AFR13"]
pub type AFR13_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR14` reader - AFR14"]
pub type AFR14_R = crate::FieldReader;
#[doc = "Field `AFR14` writer - AFR14"]
pub type AFR14_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `AFR15` reader - AFR15"]
pub type AFR15_R = crate::FieldReader;
#[doc = "Field `AFR15` writer - AFR15"]
pub type AFR15_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - AFR8"]
    #[inline(always)]
    pub fn afr8(&self) -> AFR8_R {
        AFR8_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - AFR9"]
    #[inline(always)]
    pub fn afr9(&self) -> AFR9_R {
        AFR9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - AFR10"]
    #[inline(always)]
    pub fn afr10(&self) -> AFR10_R {
        AFR10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - AFR11"]
    #[inline(always)]
    pub fn afr11(&self) -> AFR11_R {
        AFR11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - AFR12"]
    #[inline(always)]
    pub fn afr12(&self) -> AFR12_R {
        AFR12_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - AFR13"]
    #[inline(always)]
    pub fn afr13(&self) -> AFR13_R {
        AFR13_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - AFR14"]
    #[inline(always)]
    pub fn afr14(&self) -> AFR14_R {
        AFR14_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - AFR15"]
    #[inline(always)]
    pub fn afr15(&self) -> AFR15_R {
        AFR15_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - AFR8"]
    #[inline(always)]
    #[must_use]
    pub fn afr8(&mut self) -> AFR8_W<GPIOZ_AFRHrs> {
        AFR8_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - AFR9"]
    #[inline(always)]
    #[must_use]
    pub fn afr9(&mut self) -> AFR9_W<GPIOZ_AFRHrs> {
        AFR9_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - AFR10"]
    #[inline(always)]
    #[must_use]
    pub fn afr10(&mut self) -> AFR10_W<GPIOZ_AFRHrs> {
        AFR10_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - AFR11"]
    #[inline(always)]
    #[must_use]
    pub fn afr11(&mut self) -> AFR11_W<GPIOZ_AFRHrs> {
        AFR11_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - AFR12"]
    #[inline(always)]
    #[must_use]
    pub fn afr12(&mut self) -> AFR12_W<GPIOZ_AFRHrs> {
        AFR12_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - AFR13"]
    #[inline(always)]
    #[must_use]
    pub fn afr13(&mut self) -> AFR13_W<GPIOZ_AFRHrs> {
        AFR13_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - AFR14"]
    #[inline(always)]
    #[must_use]
    pub fn afr14(&mut self) -> AFR14_W<GPIOZ_AFRHrs> {
        AFR14_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - AFR15"]
    #[inline(always)]
    #[must_use]
    pub fn afr15(&mut self) -> AFR15_W<GPIOZ_AFRHrs> {
        AFR15_W::new(self, 28)
    }
}
#[doc = "GPIO alternate function high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpioz_afrh::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpioz_afrh::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIOZ_AFRHrs;
impl crate::RegisterSpec for GPIOZ_AFRHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpioz_afrh::R`](R) reader structure"]
impl crate::Readable for GPIOZ_AFRHrs {}
#[doc = "`write(|w| ..)` method takes [`gpioz_afrh::W`](W) writer structure"]
impl crate::Writable for GPIOZ_AFRHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIOZ_AFRH to value 0"]
impl crate::Resettable for GPIOZ_AFRHrs {
    const RESET_VALUE: u32 = 0;
}
