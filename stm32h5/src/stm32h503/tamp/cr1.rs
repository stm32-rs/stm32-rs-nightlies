#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Field `TAMP1E` reader - Tamper detection on TAMP_IN1 enable"]
pub type TAMP1E_R = crate::BitReader;
#[doc = "Field `TAMP1E` writer - Tamper detection on TAMP_IN1 enable"]
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2E` reader - Tamper detection on TAMP_IN2 enable"]
pub type TAMP2E_R = crate::BitReader;
#[doc = "Field `TAMP2E` writer - Tamper detection on TAMP_IN2 enable"]
pub type TAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP1E` reader - Internal tamper 1 enable"]
pub type ITAMP1E_R = crate::BitReader;
#[doc = "Field `ITAMP1E` writer - Internal tamper 1 enable"]
pub type ITAMP1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP2E` reader - Internal tamper 2 enable"]
pub type ITAMP2E_R = crate::BitReader;
#[doc = "Field `ITAMP2E` writer - Internal tamper 2 enable"]
pub type ITAMP2E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP3E` reader - Internal tamper 3 enable"]
pub type ITAMP3E_R = crate::BitReader;
#[doc = "Field `ITAMP3E` writer - Internal tamper 3 enable"]
pub type ITAMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP4E` reader - Internal tamper 4 enable"]
pub type ITAMP4E_R = crate::BitReader;
#[doc = "Field `ITAMP4E` writer - Internal tamper 4 enable"]
pub type ITAMP4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP5E` reader - Internal tamper 5 enable"]
pub type ITAMP5E_R = crate::BitReader;
#[doc = "Field `ITAMP5E` writer - Internal tamper 5 enable"]
pub type ITAMP5E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP6E` reader - Internal tamper 6 enable"]
pub type ITAMP6E_R = crate::BitReader;
#[doc = "Field `ITAMP6E` writer - Internal tamper 6 enable"]
pub type ITAMP6E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP7E` reader - Internal tamper 7 enable"]
pub type ITAMP7E_R = crate::BitReader;
#[doc = "Field `ITAMP7E` writer - Internal tamper 7 enable"]
pub type ITAMP7E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP8E` reader - Internal tamper 8 enable"]
pub type ITAMP8E_R = crate::BitReader;
#[doc = "Field `ITAMP8E` writer - Internal tamper 8 enable"]
pub type ITAMP8E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP9E` reader - Internal tamper 9 enable"]
pub type ITAMP9E_R = crate::BitReader;
#[doc = "Field `ITAMP9E` writer - Internal tamper 9 enable"]
pub type ITAMP9E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP11E` reader - Internal tamper 11 enable"]
pub type ITAMP11E_R = crate::BitReader;
#[doc = "Field `ITAMP11E` writer - Internal tamper 11 enable"]
pub type ITAMP11E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP12E` reader - Internal tamper 12 enable"]
pub type ITAMP12E_R = crate::BitReader;
#[doc = "Field `ITAMP12E` writer - Internal tamper 12 enable"]
pub type ITAMP12E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP13E` reader - Internal tamper 13 enable"]
pub type ITAMP13E_R = crate::BitReader;
#[doc = "Field `ITAMP13E` writer - Internal tamper 13 enable"]
pub type ITAMP13E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP15E` reader - Internal tamper 15 enable"]
pub type ITAMP15E_R = crate::BitReader;
#[doc = "Field `ITAMP15E` writer - Internal tamper 15 enable"]
pub type ITAMP15E_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tamper detection on TAMP_IN1 enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper detection on TAMP_IN2 enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - Internal tamper 1 enable"]
    #[inline(always)]
    pub fn itamp1e(&self) -> ITAMP1E_R {
        ITAMP1E_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal tamper 2 enable"]
    #[inline(always)]
    pub fn itamp2e(&self) -> ITAMP2E_R {
        ITAMP2E_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 enable"]
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable"]
    #[inline(always)]
    pub fn itamp4e(&self) -> ITAMP4E_R {
        ITAMP4E_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable"]
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable"]
    #[inline(always)]
    pub fn itamp6e(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Internal tamper 7 enable"]
    #[inline(always)]
    pub fn itamp7e(&self) -> ITAMP7E_R {
        ITAMP7E_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Internal tamper 8 enable"]
    #[inline(always)]
    pub fn itamp8e(&self) -> ITAMP8E_R {
        ITAMP8E_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Internal tamper 9 enable"]
    #[inline(always)]
    pub fn itamp9e(&self) -> ITAMP9E_R {
        ITAMP9E_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Internal tamper 11 enable"]
    #[inline(always)]
    pub fn itamp11e(&self) -> ITAMP11E_R {
        ITAMP11E_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Internal tamper 12 enable"]
    #[inline(always)]
    pub fn itamp12e(&self) -> ITAMP12E_R {
        ITAMP12E_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Internal tamper 13 enable"]
    #[inline(always)]
    pub fn itamp13e(&self) -> ITAMP13E_R {
        ITAMP13E_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Internal tamper 15 enable"]
    #[inline(always)]
    pub fn itamp15e(&self) -> ITAMP15E_R {
        ITAMP15E_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper detection on TAMP_IN1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp1e(&mut self) -> TAMP1E_W<CR1rs> {
        TAMP1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper detection on TAMP_IN2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn tamp2e(&mut self) -> TAMP2E_W<CR1rs> {
        TAMP2E_W::new(self, 1)
    }
    #[doc = "Bit 16 - Internal tamper 1 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp1e(&mut self) -> ITAMP1E_W<CR1rs> {
        ITAMP1E_W::new(self, 16)
    }
    #[doc = "Bit 17 - Internal tamper 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp2e(&mut self) -> ITAMP2E_W<CR1rs> {
        ITAMP2E_W::new(self, 17)
    }
    #[doc = "Bit 18 - Internal tamper 3 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3e(&mut self) -> ITAMP3E_W<CR1rs> {
        ITAMP3E_W::new(self, 18)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4e(&mut self) -> ITAMP4E_W<CR1rs> {
        ITAMP4E_W::new(self, 19)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5e(&mut self) -> ITAMP5E_W<CR1rs> {
        ITAMP5E_W::new(self, 20)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6e(&mut self) -> ITAMP6E_W<CR1rs> {
        ITAMP6E_W::new(self, 21)
    }
    #[doc = "Bit 22 - Internal tamper 7 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp7e(&mut self) -> ITAMP7E_W<CR1rs> {
        ITAMP7E_W::new(self, 22)
    }
    #[doc = "Bit 23 - Internal tamper 8 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp8e(&mut self) -> ITAMP8E_W<CR1rs> {
        ITAMP8E_W::new(self, 23)
    }
    #[doc = "Bit 24 - Internal tamper 9 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp9e(&mut self) -> ITAMP9E_W<CR1rs> {
        ITAMP9E_W::new(self, 24)
    }
    #[doc = "Bit 26 - Internal tamper 11 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp11e(&mut self) -> ITAMP11E_W<CR1rs> {
        ITAMP11E_W::new(self, 26)
    }
    #[doc = "Bit 27 - Internal tamper 12 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp12e(&mut self) -> ITAMP12E_W<CR1rs> {
        ITAMP12E_W::new(self, 27)
    }
    #[doc = "Bit 28 - Internal tamper 13 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp13e(&mut self) -> ITAMP13E_W<CR1rs> {
        ITAMP13E_W::new(self, 28)
    }
    #[doc = "Bit 30 - Internal tamper 15 enable"]
    #[inline(always)]
    #[must_use]
    pub fn itamp15e(&mut self) -> ITAMP15E_W<CR1rs> {
        ITAMP15E_W::new(self, 30)
    }
}
#[doc = "TAMP control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
