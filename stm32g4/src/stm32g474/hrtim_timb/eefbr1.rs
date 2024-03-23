#[doc = "Register `EEFBR1` reader"]
pub type R = crate::R<EEFBR1rs>;
#[doc = "Register `EEFBR1` writer"]
pub type W = crate::W<EEFBR1rs>;
#[doc = "Field `EE1LTCH` reader - External Event 1 latch"]
pub type EE1LTCH_R = crate::BitReader;
#[doc = "Field `EE1LTCH` writer - External Event 1 latch"]
pub type EE1LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE1FLTR` reader - External Event 1 filter"]
pub type EE1FLTR_R = crate::FieldReader;
#[doc = "Field `EE1FLTR` writer - External Event 1 filter"]
pub type EE1FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE2LTCH` reader - External Event 2 latch"]
pub type EE2LTCH_R = crate::BitReader;
#[doc = "Field `EE2LTCH` writer - External Event 2 latch"]
pub type EE2LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE2FLTR` reader - External Event 2 filter"]
pub type EE2FLTR_R = crate::FieldReader;
#[doc = "Field `EE2FLTR` writer - External Event 2 filter"]
pub type EE2FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE3LTCH` reader - External Event 3 latch"]
pub type EE3LTCH_R = crate::BitReader;
#[doc = "Field `EE3LTCH` writer - External Event 3 latch"]
pub type EE3LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE3FLTR` reader - External Event 3 filter"]
pub type EE3FLTR_R = crate::FieldReader;
#[doc = "Field `EE3FLTR` writer - External Event 3 filter"]
pub type EE3FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE4LTCH` reader - External Event 4 latch"]
pub type EE4LTCH_R = crate::BitReader;
#[doc = "Field `EE4LTCH` writer - External Event 4 latch"]
pub type EE4LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE4FLTR` reader - External Event 4 filter"]
pub type EE4FLTR_R = crate::FieldReader;
#[doc = "Field `EE4FLTR` writer - External Event 4 filter"]
pub type EE4FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `EE5LTCH` reader - External Event 5 latch"]
pub type EE5LTCH_R = crate::BitReader;
#[doc = "Field `EE5LTCH` writer - External Event 5 latch"]
pub type EE5LTCH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EE5FLTR` reader - External Event 5 filter"]
pub type EE5FLTR_R = crate::FieldReader;
#[doc = "Field `EE5FLTR` writer - External Event 5 filter"]
pub type EE5FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    pub fn ee1ltch(&self) -> EE1LTCH_R {
        EE1LTCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    pub fn ee1fltr(&self) -> EE1FLTR_R {
        EE1FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    pub fn ee2ltch(&self) -> EE2LTCH_R {
        EE2LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    pub fn ee2fltr(&self) -> EE2FLTR_R {
        EE2FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    pub fn ee3ltch(&self) -> EE3LTCH_R {
        EE3LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    pub fn ee3fltr(&self) -> EE3FLTR_R {
        EE3FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    pub fn ee4ltch(&self) -> EE4LTCH_R {
        EE4LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    pub fn ee4fltr(&self) -> EE4FLTR_R {
        EE4FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    pub fn ee5ltch(&self) -> EE5LTCH_R {
        EE5LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    pub fn ee5fltr(&self) -> EE5FLTR_R {
        EE5FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee1ltch(&mut self) -> EE1LTCH_W<EEFBR1rs> {
        EE1LTCH_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee1fltr(&mut self) -> EE1FLTR_W<EEFBR1rs> {
        EE1FLTR_W::new(self, 1)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee2ltch(&mut self) -> EE2LTCH_W<EEFBR1rs> {
        EE2LTCH_W::new(self, 6)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee2fltr(&mut self) -> EE2FLTR_W<EEFBR1rs> {
        EE2FLTR_W::new(self, 7)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee3ltch(&mut self) -> EE3LTCH_W<EEFBR1rs> {
        EE3LTCH_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee3fltr(&mut self) -> EE3FLTR_W<EEFBR1rs> {
        EE3FLTR_W::new(self, 13)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee4ltch(&mut self) -> EE4LTCH_W<EEFBR1rs> {
        EE4LTCH_W::new(self, 18)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee4fltr(&mut self) -> EE4FLTR_W<EEFBR1rs> {
        EE4FLTR_W::new(self, 19)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee5ltch(&mut self) -> EE5LTCH_W<EEFBR1rs> {
        EE5LTCH_W::new(self, 24)
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee5fltr(&mut self) -> EE5FLTR_W<EEFBR1rs> {
        EE5FLTR_W::new(self, 25)
    }
}
#[doc = "Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefbr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefbr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEFBR1rs;
impl crate::RegisterSpec for EEFBR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefbr1::R`](R) reader structure"]
impl crate::Readable for EEFBR1rs {}
#[doc = "`write(|w| ..)` method takes [`eefbr1::W`](W) writer structure"]
impl crate::Writable for EEFBR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEFBR1 to value 0"]
impl crate::Resettable for EEFBR1rs {
    const RESET_VALUE: u32 = 0;
}
