#[doc = "Register `SR3` reader"]
pub type R = crate::R<SR3rs>;
#[doc = "Register `SR3` writer"]
pub type W = crate::W<SR3rs>;
#[doc = "Field `TZSCF` reader - TZSCF"]
pub type TZSCF_R = crate::BitReader;
#[doc = "Field `TZSCF` writer - TZSCF"]
pub type TZSCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZICF` reader - TZICF"]
pub type TZICF_R = crate::BitReader;
#[doc = "Field `TZICF` writer - TZICF"]
pub type TZICF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCWM1F` reader - MPCWM1F"]
pub type MPCWM1F_R = crate::BitReader;
#[doc = "Field `MPCWM1F` writer - MPCWM1F"]
pub type MPCWM1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCWM2F` reader - MPCWM2F"]
pub type MPCWM2F_R = crate::BitReader;
#[doc = "Field `MPCWM2F` writer - MPCWM2F"]
pub type MPCWM2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB1F` reader - MPCBB1F"]
pub type MPCBB1F_R = crate::BitReader;
#[doc = "Field `MPCBB1F` writer - MPCBB1F"]
pub type MPCBB1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB1_REGF` reader - MPCBB1_REGF"]
pub type MPCBB1_REGF_R = crate::BitReader;
#[doc = "Field `MPCBB1_REGF` writer - MPCBB1_REGF"]
pub type MPCBB1_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB2F` reader - MPCBB2F"]
pub type MPCBB2F_R = crate::BitReader;
#[doc = "Field `MPCBB2F` writer - MPCBB2F"]
pub type MPCBB2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB2_REGF` reader - MPCBB2_REGF"]
pub type MPCBB2_REGF_R = crate::BitReader;
#[doc = "Field `MPCBB2_REGF` writer - MPCBB2_REGF"]
pub type MPCBB2_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TZSCF"]
    #[inline(always)]
    pub fn tzscf(&self) -> TZSCF_R {
        TZSCF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TZICF"]
    #[inline(always)]
    pub fn tzicf(&self) -> TZICF_R {
        TZICF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MPCWM1F"]
    #[inline(always)]
    pub fn mpcwm1f(&self) -> MPCWM1F_R {
        MPCWM1F_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MPCWM2F"]
    #[inline(always)]
    pub fn mpcwm2f(&self) -> MPCWM2F_R {
        MPCWM2F_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MPCBB1F"]
    #[inline(always)]
    pub fn mpcbb1f(&self) -> MPCBB1F_R {
        MPCBB1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGF"]
    #[inline(always)]
    pub fn mpcbb1_regf(&self) -> MPCBB1_REGF_R {
        MPCBB1_REGF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MPCBB2F"]
    #[inline(always)]
    pub fn mpcbb2f(&self) -> MPCBB2F_R {
        MPCBB2F_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGF"]
    #[inline(always)]
    pub fn mpcbb2_regf(&self) -> MPCBB2_REGF_R {
        MPCBB2_REGF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCF"]
    #[inline(always)]
    #[must_use]
    pub fn tzscf(&mut self) -> TZSCF_W<SR3rs> {
        TZSCF_W::new(self, 0)
    }
    #[doc = "Bit 1 - TZICF"]
    #[inline(always)]
    #[must_use]
    pub fn tzicf(&mut self) -> TZICF_W<SR3rs> {
        TZICF_W::new(self, 1)
    }
    #[doc = "Bit 2 - MPCWM1F"]
    #[inline(always)]
    #[must_use]
    pub fn mpcwm1f(&mut self) -> MPCWM1F_W<SR3rs> {
        MPCWM1F_W::new(self, 2)
    }
    #[doc = "Bit 3 - MPCWM2F"]
    #[inline(always)]
    #[must_use]
    pub fn mpcwm2f(&mut self) -> MPCWM2F_W<SR3rs> {
        MPCWM2F_W::new(self, 3)
    }
    #[doc = "Bit 4 - MPCBB1F"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1f(&mut self) -> MPCBB1F_W<SR3rs> {
        MPCBB1F_W::new(self, 4)
    }
    #[doc = "Bit 5 - MPCBB1_REGF"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1_regf(&mut self) -> MPCBB1_REGF_W<SR3rs> {
        MPCBB1_REGF_W::new(self, 5)
    }
    #[doc = "Bit 6 - MPCBB2F"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2f(&mut self) -> MPCBB2F_W<SR3rs> {
        MPCBB2F_W::new(self, 6)
    }
    #[doc = "Bit 7 - MPCBB2_REGF"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2_regf(&mut self) -> MPCBB2_REGF_W<SR3rs> {
        MPCBB2_REGF_W::new(self, 7)
    }
}
#[doc = "TZIC interrupt status register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SR3rs;
impl crate::RegisterSpec for SR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr3::R`](R) reader structure"]
impl crate::Readable for SR3rs {}
#[doc = "`write(|w| ..)` method takes [`sr3::W`](W) writer structure"]
impl crate::Writable for SR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR3 to value 0"]
impl crate::Resettable for SR3rs {
    const RESET_VALUE: u32 = 0;
}
