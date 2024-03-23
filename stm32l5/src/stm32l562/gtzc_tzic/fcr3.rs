#[doc = "Register `FCR3` reader"]
pub type R = crate::R<FCR3rs>;
#[doc = "Register `FCR3` writer"]
pub type W = crate::W<FCR3rs>;
#[doc = "Field `TZSCFC` reader - TZSCFC"]
pub type TZSCFC_R = crate::BitReader;
#[doc = "Field `TZSCFC` writer - TZSCFC"]
pub type TZSCFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TZICFC` reader - TZICFC"]
pub type TZICFC_R = crate::BitReader;
#[doc = "Field `TZICFC` writer - TZICFC"]
pub type TZICFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCWM1FC` reader - MPCWM1FC"]
pub type MPCWM1FC_R = crate::BitReader;
#[doc = "Field `MPCWM1FC` writer - MPCWM1FC"]
pub type MPCWM1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCWM2FC` reader - MPCWM2FC"]
pub type MPCWM2FC_R = crate::BitReader;
#[doc = "Field `MPCWM2FC` writer - MPCWM2FC"]
pub type MPCWM2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB1FC` reader - MPCBB1FC"]
pub type MPCBB1FC_R = crate::BitReader;
#[doc = "Field `MPCBB1FC` writer - MPCBB1FC"]
pub type MPCBB1FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB1_REGFC` reader - MPCBB1_REGFC"]
pub type MPCBB1_REGFC_R = crate::BitReader;
#[doc = "Field `MPCBB1_REGFC` writer - MPCBB1_REGFC"]
pub type MPCBB1_REGFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB2FC` reader - MPCBB2FC"]
pub type MPCBB2FC_R = crate::BitReader;
#[doc = "Field `MPCBB2FC` writer - MPCBB2FC"]
pub type MPCBB2FC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCBB2_REGFC` reader - MPCBB2_REGFC"]
pub type MPCBB2_REGFC_R = crate::BitReader;
#[doc = "Field `MPCBB2_REGFC` writer - MPCBB2_REGFC"]
pub type MPCBB2_REGFC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TZSCFC"]
    #[inline(always)]
    pub fn tzscfc(&self) -> TZSCFC_R {
        TZSCFC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TZICFC"]
    #[inline(always)]
    pub fn tzicfc(&self) -> TZICFC_R {
        TZICFC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MPCWM1FC"]
    #[inline(always)]
    pub fn mpcwm1fc(&self) -> MPCWM1FC_R {
        MPCWM1FC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MPCWM2FC"]
    #[inline(always)]
    pub fn mpcwm2fc(&self) -> MPCWM2FC_R {
        MPCWM2FC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MPCBB1FC"]
    #[inline(always)]
    pub fn mpcbb1fc(&self) -> MPCBB1FC_R {
        MPCBB1FC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MPCBB1_REGFC"]
    #[inline(always)]
    pub fn mpcbb1_regfc(&self) -> MPCBB1_REGFC_R {
        MPCBB1_REGFC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MPCBB2FC"]
    #[inline(always)]
    pub fn mpcbb2fc(&self) -> MPCBB2FC_R {
        MPCBB2FC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MPCBB2_REGFC"]
    #[inline(always)]
    pub fn mpcbb2_regfc(&self) -> MPCBB2_REGFC_R {
        MPCBB2_REGFC_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TZSCFC"]
    #[inline(always)]
    #[must_use]
    pub fn tzscfc(&mut self) -> TZSCFC_W<FCR3rs> {
        TZSCFC_W::new(self, 0)
    }
    #[doc = "Bit 1 - TZICFC"]
    #[inline(always)]
    #[must_use]
    pub fn tzicfc(&mut self) -> TZICFC_W<FCR3rs> {
        TZICFC_W::new(self, 1)
    }
    #[doc = "Bit 2 - MPCWM1FC"]
    #[inline(always)]
    #[must_use]
    pub fn mpcwm1fc(&mut self) -> MPCWM1FC_W<FCR3rs> {
        MPCWM1FC_W::new(self, 2)
    }
    #[doc = "Bit 3 - MPCWM2FC"]
    #[inline(always)]
    #[must_use]
    pub fn mpcwm2fc(&mut self) -> MPCWM2FC_W<FCR3rs> {
        MPCWM2FC_W::new(self, 3)
    }
    #[doc = "Bit 4 - MPCBB1FC"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1fc(&mut self) -> MPCBB1FC_W<FCR3rs> {
        MPCBB1FC_W::new(self, 4)
    }
    #[doc = "Bit 5 - MPCBB1_REGFC"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb1_regfc(&mut self) -> MPCBB1_REGFC_W<FCR3rs> {
        MPCBB1_REGFC_W::new(self, 5)
    }
    #[doc = "Bit 6 - MPCBB2FC"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2fc(&mut self) -> MPCBB2FC_W<FCR3rs> {
        MPCBB2FC_W::new(self, 6)
    }
    #[doc = "Bit 7 - MPCBB2_REGFC"]
    #[inline(always)]
    #[must_use]
    pub fn mpcbb2_regfc(&mut self) -> MPCBB2_REGFC_W<FCR3rs> {
        MPCBB2_REGFC_W::new(self, 7)
    }
}
#[doc = "TZIC interrupt clear register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR3rs;
impl crate::RegisterSpec for FCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr3::R`](R) reader structure"]
impl crate::Readable for FCR3rs {}
#[doc = "`write(|w| ..)` method takes [`fcr3::W`](W) writer structure"]
impl crate::Writable for FCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR3 to value 0"]
impl crate::Resettable for FCR3rs {
    const RESET_VALUE: u32 = 0;
}
