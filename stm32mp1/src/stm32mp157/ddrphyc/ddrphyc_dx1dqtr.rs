#[doc = "Register `DDRPHYC_DX1DQTR` reader"]
pub type R = crate::R<DDRPHYC_DX1DQTRrs>;
#[doc = "Register `DDRPHYC_DX1DQTR` writer"]
pub type W = crate::W<DDRPHYC_DX1DQTRrs>;
#[doc = "Field `DQDLY0` reader - DQDLY0"]
pub type DQDLY0_R = crate::FieldReader;
#[doc = "Field `DQDLY0` writer - DQDLY0"]
pub type DQDLY0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQDLY1` reader - DQDLY1"]
pub type DQDLY1_R = crate::FieldReader;
#[doc = "Field `DQDLY1` writer - DQDLY1"]
pub type DQDLY1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQDLY2` reader - DQDLY2"]
pub type DQDLY2_R = crate::FieldReader;
#[doc = "Field `DQDLY2` writer - DQDLY2"]
pub type DQDLY2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQDLY3` reader - DQDLY3"]
pub type DQDLY3_R = crate::FieldReader;
#[doc = "Field `DQDLY3` writer - DQDLY3"]
pub type DQDLY3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQDLY4` reader - DQDLY4"]
pub type DQDLY4_R = crate::FieldReader;
#[doc = "Field `DQDLY4` writer - DQDLY4"]
pub type DQDLY4_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQDLY5` reader - DQDLY5"]
pub type DQDLY5_R = crate::FieldReader;
#[doc = "Field `DQDLY5` writer - DQDLY5"]
pub type DQDLY5_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQDLY6` reader - DQDLY6"]
pub type DQDLY6_R = crate::FieldReader;
#[doc = "Field `DQDLY6` writer - DQDLY6"]
pub type DQDLY6_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DQDLY7` reader - DQDLY7"]
pub type DQDLY7_R = crate::FieldReader;
#[doc = "Field `DQDLY7` writer - DQDLY7"]
pub type DQDLY7_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - DQDLY0"]
    #[inline(always)]
    pub fn dqdly0(&self) -> DQDLY0_R {
        DQDLY0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - DQDLY1"]
    #[inline(always)]
    pub fn dqdly1(&self) -> DQDLY1_R {
        DQDLY1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DQDLY2"]
    #[inline(always)]
    pub fn dqdly2(&self) -> DQDLY2_R {
        DQDLY2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DQDLY3"]
    #[inline(always)]
    pub fn dqdly3(&self) -> DQDLY3_R {
        DQDLY3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - DQDLY4"]
    #[inline(always)]
    pub fn dqdly4(&self) -> DQDLY4_R {
        DQDLY4_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - DQDLY5"]
    #[inline(always)]
    pub fn dqdly5(&self) -> DQDLY5_R {
        DQDLY5_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - DQDLY6"]
    #[inline(always)]
    pub fn dqdly6(&self) -> DQDLY6_R {
        DQDLY6_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - DQDLY7"]
    #[inline(always)]
    pub fn dqdly7(&self) -> DQDLY7_R {
        DQDLY7_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - DQDLY0"]
    #[inline(always)]
    #[must_use]
    pub fn dqdly0(&mut self) -> DQDLY0_W<DDRPHYC_DX1DQTRrs> {
        DQDLY0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - DQDLY1"]
    #[inline(always)]
    #[must_use]
    pub fn dqdly1(&mut self) -> DQDLY1_W<DDRPHYC_DX1DQTRrs> {
        DQDLY1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - DQDLY2"]
    #[inline(always)]
    #[must_use]
    pub fn dqdly2(&mut self) -> DQDLY2_W<DDRPHYC_DX1DQTRrs> {
        DQDLY2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - DQDLY3"]
    #[inline(always)]
    #[must_use]
    pub fn dqdly3(&mut self) -> DQDLY3_W<DDRPHYC_DX1DQTRrs> {
        DQDLY3_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - DQDLY4"]
    #[inline(always)]
    #[must_use]
    pub fn dqdly4(&mut self) -> DQDLY4_W<DDRPHYC_DX1DQTRrs> {
        DQDLY4_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - DQDLY5"]
    #[inline(always)]
    #[must_use]
    pub fn dqdly5(&mut self) -> DQDLY5_W<DDRPHYC_DX1DQTRrs> {
        DQDLY5_W::new(self, 20)
    }
    #[doc = "Bits 24:27 - DQDLY6"]
    #[inline(always)]
    #[must_use]
    pub fn dqdly6(&mut self) -> DQDLY6_W<DDRPHYC_DX1DQTRrs> {
        DQDLY6_W::new(self, 24)
    }
    #[doc = "Bits 28:31 - DQDLY7"]
    #[inline(always)]
    #[must_use]
    pub fn dqdly7(&mut self) -> DQDLY7_W<DDRPHYC_DX1DQTRrs> {
        DQDLY7_W::new(self, 28)
    }
}
#[doc = "DDRPHYC byte lane 1 DQT register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dx1dqtr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dx1dqtr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DX1DQTRrs;
impl crate::RegisterSpec for DDRPHYC_DX1DQTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dx1dqtr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DX1DQTRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dx1dqtr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DX1DQTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DX1DQTR to value 0xffff_ffff"]
impl crate::Resettable for DDRPHYC_DX1DQTRrs {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
