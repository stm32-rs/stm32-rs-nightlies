#[doc = "Register `ETZPC_DECPROT4` reader"]
pub type R = crate::R<ETZPC_DECPROT4rs>;
#[doc = "Register `ETZPC_DECPROT4` writer"]
pub type W = crate::W<ETZPC_DECPROT4rs>;
#[doc = "Field `DECPROT0` reader - DECPROT0"]
pub type DECPROT0_R = crate::FieldReader;
#[doc = "Field `DECPROT0` writer - DECPROT0"]
pub type DECPROT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT1` reader - DECPROT1"]
pub type DECPROT1_R = crate::FieldReader;
#[doc = "Field `DECPROT1` writer - DECPROT1"]
pub type DECPROT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT2` reader - DECPROT2"]
pub type DECPROT2_R = crate::FieldReader;
#[doc = "Field `DECPROT2` writer - DECPROT2"]
pub type DECPROT2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT3` reader - DECPROT3"]
pub type DECPROT3_R = crate::FieldReader;
#[doc = "Field `DECPROT3` writer - DECPROT3"]
pub type DECPROT3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT4` reader - DECPROT4"]
pub type DECPROT4_R = crate::FieldReader;
#[doc = "Field `DECPROT4` writer - DECPROT4"]
pub type DECPROT4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT5` reader - DECPROT5"]
pub type DECPROT5_R = crate::FieldReader;
#[doc = "Field `DECPROT5` writer - DECPROT5"]
pub type DECPROT5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT6` reader - DECPROT6"]
pub type DECPROT6_R = crate::FieldReader;
#[doc = "Field `DECPROT6` writer - DECPROT6"]
pub type DECPROT6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT7` reader - DECPROT7"]
pub type DECPROT7_R = crate::FieldReader;
#[doc = "Field `DECPROT7` writer - DECPROT7"]
pub type DECPROT7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT8` reader - DECPROT8"]
pub type DECPROT8_R = crate::FieldReader;
#[doc = "Field `DECPROT8` writer - DECPROT8"]
pub type DECPROT8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT9` reader - DECPROT9"]
pub type DECPROT9_R = crate::FieldReader;
#[doc = "Field `DECPROT9` writer - DECPROT9"]
pub type DECPROT9_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT10` reader - DECPROT10"]
pub type DECPROT10_R = crate::FieldReader;
#[doc = "Field `DECPROT10` writer - DECPROT10"]
pub type DECPROT10_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT11` reader - DECPROT11"]
pub type DECPROT11_R = crate::FieldReader;
#[doc = "Field `DECPROT11` writer - DECPROT11"]
pub type DECPROT11_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT12` reader - DECPROT12"]
pub type DECPROT12_R = crate::FieldReader;
#[doc = "Field `DECPROT12` writer - DECPROT12"]
pub type DECPROT12_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT13` reader - DECPROT13"]
pub type DECPROT13_R = crate::FieldReader;
#[doc = "Field `DECPROT13` writer - DECPROT13"]
pub type DECPROT13_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT14` reader - DECPROT14"]
pub type DECPROT14_R = crate::FieldReader;
#[doc = "Field `DECPROT14` writer - DECPROT14"]
pub type DECPROT14_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DECPROT15` reader - DECPROT15"]
pub type DECPROT15_R = crate::FieldReader;
#[doc = "Field `DECPROT15` writer - DECPROT15"]
pub type DECPROT15_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - DECPROT0"]
    #[inline(always)]
    pub fn decprot0(&self) -> DECPROT0_R {
        DECPROT0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DECPROT1"]
    #[inline(always)]
    pub fn decprot1(&self) -> DECPROT1_R {
        DECPROT1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DECPROT2"]
    #[inline(always)]
    pub fn decprot2(&self) -> DECPROT2_R {
        DECPROT2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DECPROT3"]
    #[inline(always)]
    pub fn decprot3(&self) -> DECPROT3_R {
        DECPROT3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DECPROT4"]
    #[inline(always)]
    pub fn decprot4(&self) -> DECPROT4_R {
        DECPROT4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DECPROT5"]
    #[inline(always)]
    pub fn decprot5(&self) -> DECPROT5_R {
        DECPROT5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - DECPROT6"]
    #[inline(always)]
    pub fn decprot6(&self) -> DECPROT6_R {
        DECPROT6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DECPROT7"]
    #[inline(always)]
    pub fn decprot7(&self) -> DECPROT7_R {
        DECPROT7_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - DECPROT8"]
    #[inline(always)]
    pub fn decprot8(&self) -> DECPROT8_R {
        DECPROT8_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - DECPROT9"]
    #[inline(always)]
    pub fn decprot9(&self) -> DECPROT9_R {
        DECPROT9_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - DECPROT10"]
    #[inline(always)]
    pub fn decprot10(&self) -> DECPROT10_R {
        DECPROT10_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - DECPROT11"]
    #[inline(always)]
    pub fn decprot11(&self) -> DECPROT11_R {
        DECPROT11_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:25 - DECPROT12"]
    #[inline(always)]
    pub fn decprot12(&self) -> DECPROT12_R {
        DECPROT12_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - DECPROT13"]
    #[inline(always)]
    pub fn decprot13(&self) -> DECPROT13_R {
        DECPROT13_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - DECPROT14"]
    #[inline(always)]
    pub fn decprot14(&self) -> DECPROT14_R {
        DECPROT14_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bits 30:31 - DECPROT15"]
    #[inline(always)]
    pub fn decprot15(&self) -> DECPROT15_R {
        DECPROT15_R::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DECPROT0"]
    #[inline(always)]
    #[must_use]
    pub fn decprot0(&mut self) -> DECPROT0_W<ETZPC_DECPROT4rs> {
        DECPROT0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - DECPROT1"]
    #[inline(always)]
    #[must_use]
    pub fn decprot1(&mut self) -> DECPROT1_W<ETZPC_DECPROT4rs> {
        DECPROT1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - DECPROT2"]
    #[inline(always)]
    #[must_use]
    pub fn decprot2(&mut self) -> DECPROT2_W<ETZPC_DECPROT4rs> {
        DECPROT2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - DECPROT3"]
    #[inline(always)]
    #[must_use]
    pub fn decprot3(&mut self) -> DECPROT3_W<ETZPC_DECPROT4rs> {
        DECPROT3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - DECPROT4"]
    #[inline(always)]
    #[must_use]
    pub fn decprot4(&mut self) -> DECPROT4_W<ETZPC_DECPROT4rs> {
        DECPROT4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - DECPROT5"]
    #[inline(always)]
    #[must_use]
    pub fn decprot5(&mut self) -> DECPROT5_W<ETZPC_DECPROT4rs> {
        DECPROT5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - DECPROT6"]
    #[inline(always)]
    #[must_use]
    pub fn decprot6(&mut self) -> DECPROT6_W<ETZPC_DECPROT4rs> {
        DECPROT6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - DECPROT7"]
    #[inline(always)]
    #[must_use]
    pub fn decprot7(&mut self) -> DECPROT7_W<ETZPC_DECPROT4rs> {
        DECPROT7_W::new(self, 14)
    }
    #[doc = "Bits 16:17 - DECPROT8"]
    #[inline(always)]
    #[must_use]
    pub fn decprot8(&mut self) -> DECPROT8_W<ETZPC_DECPROT4rs> {
        DECPROT8_W::new(self, 16)
    }
    #[doc = "Bits 18:19 - DECPROT9"]
    #[inline(always)]
    #[must_use]
    pub fn decprot9(&mut self) -> DECPROT9_W<ETZPC_DECPROT4rs> {
        DECPROT9_W::new(self, 18)
    }
    #[doc = "Bits 20:21 - DECPROT10"]
    #[inline(always)]
    #[must_use]
    pub fn decprot10(&mut self) -> DECPROT10_W<ETZPC_DECPROT4rs> {
        DECPROT10_W::new(self, 20)
    }
    #[doc = "Bits 22:23 - DECPROT11"]
    #[inline(always)]
    #[must_use]
    pub fn decprot11(&mut self) -> DECPROT11_W<ETZPC_DECPROT4rs> {
        DECPROT11_W::new(self, 22)
    }
    #[doc = "Bits 24:25 - DECPROT12"]
    #[inline(always)]
    #[must_use]
    pub fn decprot12(&mut self) -> DECPROT12_W<ETZPC_DECPROT4rs> {
        DECPROT12_W::new(self, 24)
    }
    #[doc = "Bits 26:27 - DECPROT13"]
    #[inline(always)]
    #[must_use]
    pub fn decprot13(&mut self) -> DECPROT13_W<ETZPC_DECPROT4rs> {
        DECPROT13_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - DECPROT14"]
    #[inline(always)]
    #[must_use]
    pub fn decprot14(&mut self) -> DECPROT14_W<ETZPC_DECPROT4rs> {
        DECPROT14_W::new(self, 28)
    }
    #[doc = "Bits 30:31 - DECPROT15"]
    #[inline(always)]
    #[must_use]
    pub fn decprot15(&mut self) -> DECPROT15_W<ETZPC_DECPROT4rs> {
        DECPROT15_W::new(self, 30)
    }
}
#[doc = "Register reset values\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etzpc_decprot4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etzpc_decprot4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETZPC_DECPROT4rs;
impl crate::RegisterSpec for ETZPC_DECPROT4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etzpc_decprot4::R`](R) reader structure"]
impl crate::Readable for ETZPC_DECPROT4rs {}
#[doc = "`write(|w| ..)` method takes [`etzpc_decprot4::W`](W) writer structure"]
impl crate::Writable for ETZPC_DECPROT4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETZPC_DECPROT4 to value 0"]
impl crate::Resettable for ETZPC_DECPROT4rs {
    const RESET_VALUE: u32 = 0;
}
