#[doc = "Register `CFGR3` reader"]
pub type R = crate::R<CFGR3rs>;
#[doc = "Register `CFGR3` writer"]
pub type W = crate::W<CFGR3rs>;
#[doc = "Field `TRIM1_NG_CCRPD` reader - SW trim value for RPD resistors on the CC1 line"]
pub type TRIM1_NG_CCRPD_R = crate::FieldReader;
#[doc = "Field `TRIM1_NG_CCRPD` writer - SW trim value for RPD resistors on the CC1 line"]
pub type TRIM1_NG_CCRPD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM1_NG_CC1A5` reader - SW trim value for RP1A5 resistors on the CC1 line"]
pub type TRIM1_NG_CC1A5_R = crate::FieldReader;
#[doc = "Field `TRIM1_NG_CC1A5` writer - SW trim value for RP1A5 resistors on the CC1 line"]
pub type TRIM1_NG_CC1A5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIM1_NG_CC3A0` reader - SW trim value for RP3A0 resistors on the CC1 line"]
pub type TRIM1_NG_CC3A0_R = crate::FieldReader;
#[doc = "Field `TRIM1_NG_CC3A0` writer - SW trim value for RP3A0 resistors on the CC1 line"]
pub type TRIM1_NG_CC3A0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM2_NG_CCRPD` reader - SW trim value for RPD resistors on the CC2 line"]
pub type TRIM2_NG_CCRPD_R = crate::FieldReader;
#[doc = "Field `TRIM2_NG_CCRPD` writer - SW trim value for RPD resistors on the CC2 line"]
pub type TRIM2_NG_CCRPD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TRIM2_NG_CC1A5` reader - SW trim value for RP1A5 resistors on the CC2 line"]
pub type TRIM2_NG_CC1A5_R = crate::FieldReader;
#[doc = "Field `TRIM2_NG_CC1A5` writer - SW trim value for RP1A5 resistors on the CC2 line"]
pub type TRIM2_NG_CC1A5_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRIM2_NG_CC3A0` reader - SW trim value for RP3A0 resistors on the CC2 line"]
pub type TRIM2_NG_CC3A0_R = crate::FieldReader;
#[doc = "Field `TRIM2_NG_CC3A0` writer - SW trim value for RP3A0 resistors on the CC2 line"]
pub type TRIM2_NG_CC3A0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - SW trim value for RPD resistors on the CC1 line"]
    #[inline(always)]
    pub fn trim1_ng_ccrpd(&self) -> TRIM1_NG_CCRPD_R {
        TRIM1_NG_CCRPD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:8 - SW trim value for RP1A5 resistors on the CC1 line"]
    #[inline(always)]
    pub fn trim1_ng_cc1a5(&self) -> TRIM1_NG_CC1A5_R {
        TRIM1_NG_CC1A5_R::new(((self.bits >> 4) & 0x1f) as u8)
    }
    #[doc = "Bits 9:12 - SW trim value for RP3A0 resistors on the CC1 line"]
    #[inline(always)]
    pub fn trim1_ng_cc3a0(&self) -> TRIM1_NG_CC3A0_R {
        TRIM1_NG_CC3A0_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - SW trim value for RPD resistors on the CC2 line"]
    #[inline(always)]
    pub fn trim2_ng_ccrpd(&self) -> TRIM2_NG_CCRPD_R {
        TRIM2_NG_CCRPD_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:24 - SW trim value for RP1A5 resistors on the CC2 line"]
    #[inline(always)]
    pub fn trim2_ng_cc1a5(&self) -> TRIM2_NG_CC1A5_R {
        TRIM2_NG_CC1A5_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 25:28 - SW trim value for RP3A0 resistors on the CC2 line"]
    #[inline(always)]
    pub fn trim2_ng_cc3a0(&self) -> TRIM2_NG_CC3A0_R {
        TRIM2_NG_CC3A0_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - SW trim value for RPD resistors on the CC1 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_ccrpd(&mut self) -> TRIM1_NG_CCRPD_W<CFGR3rs> {
        TRIM1_NG_CCRPD_W::new(self, 0)
    }
    #[doc = "Bits 4:8 - SW trim value for RP1A5 resistors on the CC1 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_cc1a5(&mut self) -> TRIM1_NG_CC1A5_W<CFGR3rs> {
        TRIM1_NG_CC1A5_W::new(self, 4)
    }
    #[doc = "Bits 9:12 - SW trim value for RP3A0 resistors on the CC1 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim1_ng_cc3a0(&mut self) -> TRIM1_NG_CC3A0_W<CFGR3rs> {
        TRIM1_NG_CC3A0_W::new(self, 9)
    }
    #[doc = "Bits 16:19 - SW trim value for RPD resistors on the CC2 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_ccrpd(&mut self) -> TRIM2_NG_CCRPD_W<CFGR3rs> {
        TRIM2_NG_CCRPD_W::new(self, 16)
    }
    #[doc = "Bits 20:24 - SW trim value for RP1A5 resistors on the CC2 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_cc1a5(&mut self) -> TRIM2_NG_CC1A5_W<CFGR3rs> {
        TRIM2_NG_CC1A5_W::new(self, 20)
    }
    #[doc = "Bits 25:28 - SW trim value for RP3A0 resistors on the CC2 line"]
    #[inline(always)]
    #[must_use]
    pub fn trim2_ng_cc3a0(&mut self) -> TRIM2_NG_CC3A0_W<CFGR3rs> {
        TRIM2_NG_CC3A0_W::new(self, 25)
    }
}
#[doc = "UCPD configuration register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGR3rs;
impl crate::RegisterSpec for CFGR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr3::R`](R) reader structure"]
impl crate::Readable for CFGR3rs {}
#[doc = "`write(|w| ..)` method takes [`cfgr3::W`](W) writer structure"]
impl crate::Writable for CFGR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR3 to value 0"]
impl crate::Resettable for CFGR3rs {
    const RESET_VALUE: u32 = 0;
}
