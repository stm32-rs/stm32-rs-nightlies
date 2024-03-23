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
#[doc = "Field `ITAMP3E` reader - Internal tamper 3 enable: LSE monitoring"]
pub type ITAMP3E_R = crate::BitReader;
#[doc = "Field `ITAMP3E` writer - Internal tamper 3 enable: LSE monitoring"]
pub type ITAMP3E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP4E` reader - Internal tamper 4 enable: HSE monitoring"]
pub type ITAMP4E_R = crate::BitReader;
#[doc = "Field `ITAMP4E` writer - Internal tamper 4 enable: HSE monitoring"]
pub type ITAMP4E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP5E` reader - Internal tamper 5 enable: RTC calendar overflow"]
pub type ITAMP5E_R = crate::BitReader;
#[doc = "Field `ITAMP5E` writer - Internal tamper 5 enable: RTC calendar overflow"]
pub type ITAMP5E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITAMP6E` reader - Internal tamper 6 enable: ST manufacturer readout"]
pub type ITAMP6E_R = crate::BitReader;
#[doc = "Field `ITAMP6E` writer - Internal tamper 6 enable: ST manufacturer readout"]
pub type ITAMP6E_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 18 - Internal tamper 3 enable: LSE monitoring"]
    #[inline(always)]
    pub fn itamp3e(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable: HSE monitoring"]
    #[inline(always)]
    pub fn itamp4e(&self) -> ITAMP4E_R {
        ITAMP4E_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable: RTC calendar overflow"]
    #[inline(always)]
    pub fn itamp5e(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable: ST manufacturer readout"]
    #[inline(always)]
    pub fn itamp6e(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 1) != 0)
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
    #[doc = "Bit 18 - Internal tamper 3 enable: LSE monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn itamp3e(&mut self) -> ITAMP3E_W<CR1rs> {
        ITAMP3E_W::new(self, 18)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable: HSE monitoring"]
    #[inline(always)]
    #[must_use]
    pub fn itamp4e(&mut self) -> ITAMP4E_W<CR1rs> {
        ITAMP4E_W::new(self, 19)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable: RTC calendar overflow"]
    #[inline(always)]
    #[must_use]
    pub fn itamp5e(&mut self) -> ITAMP5E_W<CR1rs> {
        ITAMP5E_W::new(self, 20)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable: ST manufacturer readout"]
    #[inline(always)]
    #[must_use]
    pub fn itamp6e(&mut self) -> ITAMP6E_W<CR1rs> {
        ITAMP6E_W::new(self, 21)
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
#[doc = "`reset()` method sets CR1 to value 0xffff_0000"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
