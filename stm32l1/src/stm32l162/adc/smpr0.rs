#[doc = "Register `SMPR0` reader"]
pub type R = crate::R<SMPR0rs>;
#[doc = "Register `SMPR0` writer"]
pub type W = crate::W<SMPR0rs>;
#[doc = "Field `SMP30` reader - Channel 30 sampling time selection"]
pub type SMP30_R = crate::FieldReader;
#[doc = "Field `SMP30` writer - Channel 30 sampling time selection"]
pub type SMP30_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP31` reader - Channel 31 sampling time selection"]
pub type SMP31_R = crate::FieldReader;
#[doc = "Field `SMP31` writer - Channel 31 sampling time selection"]
pub type SMP31_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Channel 30 sampling time selection"]
    #[inline(always)]
    pub fn smp30(&self) -> SMP30_R {
        SMP30_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 31 sampling time selection"]
    #[inline(always)]
    pub fn smp31(&self) -> SMP31_R {
        SMP31_R::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 30 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp30(&mut self) -> SMP30_W<SMPR0rs> {
        SMP30_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 31 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp31(&mut self) -> SMP31_W<SMPR0rs> {
        SMP31_W::new(self, 3)
    }
}
#[doc = "sample time register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR0rs;
impl crate::RegisterSpec for SMPR0rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr0::R`](R) reader structure"]
impl crate::Readable for SMPR0rs {}
#[doc = "`write(|w| ..)` method takes [`smpr0::W`](W) writer structure"]
impl crate::Writable for SMPR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR0 to value 0"]
impl crate::Resettable for SMPR0rs {
    const RESET_VALUE: u32 = 0;
}
