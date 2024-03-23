#[doc = "Register `SMPR1` reader"]
pub type R = crate::R<SMPR1rs>;
#[doc = "Register `SMPR1` writer"]
pub type W = crate::W<SMPR1rs>;
#[doc = "Field `SMP0` reader - Channel 0 sampling time selection"]
pub type SMP0_R = crate::FieldReader;
#[doc = "Field `SMP0` writer - Channel 0 sampling time selection"]
pub type SMP0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP1` reader - Channel 1 sampling time selection"]
pub type SMP1_R = crate::FieldReader;
#[doc = "Field `SMP1` writer - Channel 1 sampling time selection"]
pub type SMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP2` reader - Channel 2 sampling time selection"]
pub type SMP2_R = crate::FieldReader;
#[doc = "Field `SMP2` writer - Channel 2 sampling time selection"]
pub type SMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP3` reader - Channel 3 sampling time selection"]
pub type SMP3_R = crate::FieldReader;
#[doc = "Field `SMP3` writer - Channel 3 sampling time selection"]
pub type SMP3_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP4` reader - Channel 4 sampling time selection"]
pub type SMP4_R = crate::FieldReader;
#[doc = "Field `SMP4` writer - Channel 4 sampling time selection"]
pub type SMP4_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP5` reader - Channel 5 sampling time selection"]
pub type SMP5_R = crate::FieldReader;
#[doc = "Field `SMP5` writer - Channel 5 sampling time selection"]
pub type SMP5_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP6` reader - Channel 6 sampling time selection"]
pub type SMP6_R = crate::FieldReader;
#[doc = "Field `SMP6` writer - Channel 6 sampling time selection"]
pub type SMP6_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP7` reader - Channel 7 sampling time selection"]
pub type SMP7_R = crate::FieldReader;
#[doc = "Field `SMP7` writer - Channel 7 sampling time selection"]
pub type SMP7_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP8` reader - Channel 8 sampling time selection"]
pub type SMP8_R = crate::FieldReader;
#[doc = "Field `SMP8` writer - Channel 8 sampling time selection"]
pub type SMP8_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SMP9` reader - Channel 9 sampling time selection"]
pub type SMP9_R = crate::FieldReader;
#[doc = "Field `SMP9` writer - Channel 9 sampling time selection"]
pub type SMP9_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    pub fn smp0(&self) -> SMP0_R {
        SMP0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    pub fn smp1(&self) -> SMP1_R {
        SMP1_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    pub fn smp2(&self) -> SMP2_R {
        SMP2_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    pub fn smp3(&self) -> SMP3_R {
        SMP3_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    pub fn smp4(&self) -> SMP4_R {
        SMP4_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    pub fn smp5(&self) -> SMP5_R {
        SMP5_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    pub fn smp6(&self) -> SMP6_R {
        SMP6_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    pub fn smp7(&self) -> SMP7_R {
        SMP7_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    pub fn smp8(&self) -> SMP8_R {
        SMP8_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    pub fn smp9(&self) -> SMP9_R {
        SMP9_R::new(((self.bits >> 27) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Channel 0 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp0(&mut self) -> SMP0_W<SMPR1rs> {
        SMP0_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - Channel 1 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp1(&mut self) -> SMP1_W<SMPR1rs> {
        SMP1_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - Channel 2 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp2(&mut self) -> SMP2_W<SMPR1rs> {
        SMP2_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - Channel 3 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp3(&mut self) -> SMP3_W<SMPR1rs> {
        SMP3_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - Channel 4 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp4(&mut self) -> SMP4_W<SMPR1rs> {
        SMP4_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - Channel 5 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp5(&mut self) -> SMP5_W<SMPR1rs> {
        SMP5_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - Channel 6 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp6(&mut self) -> SMP6_W<SMPR1rs> {
        SMP6_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - Channel 7 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp7(&mut self) -> SMP7_W<SMPR1rs> {
        SMP7_W::new(self, 21)
    }
    #[doc = "Bits 24:26 - Channel 8 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp8(&mut self) -> SMP8_W<SMPR1rs> {
        SMP8_W::new(self, 24)
    }
    #[doc = "Bits 27:29 - Channel 9 sampling time selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp9(&mut self) -> SMP9_W<SMPR1rs> {
        SMP9_W::new(self, 27)
    }
}
#[doc = "ADC sampling time register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smpr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smpr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMPR1rs;
impl crate::RegisterSpec for SMPR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smpr1::R`](R) reader structure"]
impl crate::Readable for SMPR1rs {}
#[doc = "`write(|w| ..)` method takes [`smpr1::W`](W) writer structure"]
impl crate::Writable for SMPR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMPR1 to value 0"]
impl crate::Resettable for SMPR1rs {
    const RESET_VALUE: u32 = 0;
}
