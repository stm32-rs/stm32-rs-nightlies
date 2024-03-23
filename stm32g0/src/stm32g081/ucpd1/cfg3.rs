#[doc = "Register `CFG3` reader"]
pub type R = crate::R<CFG3rs>;
#[doc = "Register `CFG3` writer"]
pub type W = crate::W<CFG3rs>;
#[doc = "Field `TRIM1_NG_CCRPD` reader - TRIM1_NG_CCRPD"]
pub type TRIM1_NG_CCRPD_R = crate::FieldReader;
#[doc = "Field `TRIM1_NG_CCRPD` writer - TRIM1_NG_CCRPD"]
pub type TRIM1_NG_CCRPD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM1_NG_CC1A5` reader - TRIM1_NG_CC1A5"]
pub type TRIM1_NG_CC1A5_R = crate::FieldReader;
#[doc = "Field `TRIM1_NG_CC1A5` writer - TRIM1_NG_CC1A5"]
pub type TRIM1_NG_CC1A5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIM1_NG_CC3A0` reader - TRIM1_NG_CC3A0"]
pub type TRIM1_NG_CC3A0_R = crate::FieldReader;
#[doc = "Field `TRIM1_NG_CC3A0` writer - TRIM1_NG_CC3A0"]
pub type TRIM1_NG_CC3A0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM2_NG_CCRPD` reader - TRIM2_NG_CCRPD"]
pub type TRIM2_NG_CCRPD_R = crate::FieldReader;
#[doc = "Field `TRIM2_NG_CCRPD` writer - TRIM2_NG_CCRPD"]
pub type TRIM2_NG_CCRPD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM2_NG_CC1A5` reader - TRIM2_NG_CC1A5"]
pub type TRIM2_NG_CC1A5_R = crate::FieldReader;
#[doc = "Field `TRIM2_NG_CC1A5` writer - TRIM2_NG_CC1A5"]
pub type TRIM2_NG_CC1A5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIM2_NG_CC3A0` reader - TRIM2_NG_CC3A0"]
pub type TRIM2_NG_CC3A0_R = crate::FieldReader;
#[doc = "Field `TRIM2_NG_CC3A0` writer - TRIM2_NG_CC3A0"]
pub type TRIM2_NG_CC3A0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TRIM1_NG_CCRPD"]
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&self) -> TRIM1_NG_CCRPD_R {
        TRIM1_NG_CCRPD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - TRIM1_NG_CC1A5"]
    #[inline(always)]
    pub fn trim1_ng_cc1a5(&self) -> TRIM1_NG_CC1A5_R {
        TRIM1_NG_CC1A5_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:12 - TRIM1_NG_CC3A0"]
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&self) -> TRIM1_NG_CC3A0_R {
        TRIM1_NG_CC3A0_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - TRIM2_NG_CCRPD"]
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&self) -> TRIM2_NG_CCRPD_R {
        TRIM2_NG_CCRPD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - TRIM2_NG_CC1A5"]
    #[inline(always)]
    pub fn trim2_ng_cc1a5(&self) -> TRIM2_NG_CC1A5_R {
        TRIM2_NG_CC1A5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:28 - TRIM2_NG_CC3A0"]
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&self) -> TRIM2_NG_CC3A0_R {
        TRIM2_NG_CC3A0_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TRIM1_NG_CCRPD"]
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_ccrpd(&mut self) -> TRIM1_NG_CCRPD_W<CFG3rs> {
        TRIM1_NG_CCRPD_W::new(self, 0)
    }
    #[doc = "Bits 4:8 - TRIM1_NG_CC1A5"]
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_cc1a5(&mut self) -> TRIM1_NG_CC1A5_W<CFG3rs> {
        TRIM1_NG_CC1A5_W::new(self, 4)
    }
    #[doc = "Bits 9:12 - TRIM1_NG_CC3A0"]
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_cc3a0(&mut self) -> TRIM1_NG_CC3A0_W<CFG3rs> {
        TRIM1_NG_CC3A0_W::new(self, 9)
    }
    #[doc = "Bits 16:19 - TRIM2_NG_CCRPD"]
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_ccrpd(&mut self) -> TRIM2_NG_CCRPD_W<CFG3rs> {
        TRIM2_NG_CCRPD_W::new(self, 16)
    }
    #[doc = "Bits 20:24 - TRIM2_NG_CC1A5"]
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_cc1a5(&mut self) -> TRIM2_NG_CC1A5_W<CFG3rs> {
        TRIM2_NG_CC1A5_W::new(self, 20)
    }
    #[doc = "Bits 25:28 - TRIM2_NG_CC3A0"]
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_cc3a0(&mut self) -> TRIM2_NG_CC3A0_W<CFG3rs> {
        TRIM2_NG_CC3A0_W::new(self, 25)
    }
}
#[doc = "UCPD configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG3rs;
impl crate::RegisterSpec for CFG3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg3::R`](R) reader structure"]
impl crate::Readable for CFG3rs {}
#[doc = "`write(|w| ..)` method takes [`cfg3::W`](W) writer structure"]
impl crate::Writable for CFG3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG3 to value 0"]
impl crate::Resettable for CFG3rs {
    const RESET_VALUE: u32 = 0;
}
