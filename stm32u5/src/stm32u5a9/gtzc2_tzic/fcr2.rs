#[doc = "Register `FCR2` writer"]
pub type W = crate::W<FCR2rs>;
#[doc = "Field `CSYSCFGF` writer - clear the illegal access flag for SYSCFG"]
pub type CSYSCFGF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRTCF` writer - clear the illegal access flag for RTC"]
pub type CRTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTAMPF` writer - clear the illegal access flag for TAMP"]
pub type CTAMPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPWRF` writer - clear the illegal access flag for PWR"]
pub type CPWRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCCF` writer - clear the illegal access flag for RCC"]
pub type CRCCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLPDMA1F` writer - clear the illegal access flag for LPDMA"]
pub type CLPDMA1F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CEXTIF` writer - clear the illegal access flag for EXTI"]
pub type CEXTIF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTZSC2F` writer - clear the illegal access flag for GTZC2 TZSC registers"]
pub type CTZSC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTZIC2F` writer - clear the illegal access flag for GTZC2 TZIC registers"]
pub type CTZIC2F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRAM4F` writer - clear the illegal access flag for SRAM4"]
pub type CSRAM4F_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMPCBB4_REGF` writer - clear the illegal access flag for MPCBB4 registers"]
pub type CMPCBB4_REGF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - clear the illegal access flag for SYSCFG"]
    #[inline(always)]
    #[must_use]
    pub fn csyscfgf(&mut self) -> CSYSCFGF_W<FCR2rs> {
        CSYSCFGF_W::new(self, 0)
    }
    #[doc = "Bit 1 - clear the illegal access flag for RTC"]
    #[inline(always)]
    #[must_use]
    pub fn crtcf(&mut self) -> CRTCF_W<FCR2rs> {
        CRTCF_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear the illegal access flag for TAMP"]
    #[inline(always)]
    #[must_use]
    pub fn ctampf(&mut self) -> CTAMPF_W<FCR2rs> {
        CTAMPF_W::new(self, 2)
    }
    #[doc = "Bit 3 - clear the illegal access flag for PWR"]
    #[inline(always)]
    #[must_use]
    pub fn cpwrf(&mut self) -> CPWRF_W<FCR2rs> {
        CPWRF_W::new(self, 3)
    }
    #[doc = "Bit 4 - clear the illegal access flag for RCC"]
    #[inline(always)]
    #[must_use]
    pub fn crccf(&mut self) -> CRCCF_W<FCR2rs> {
        CRCCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - clear the illegal access flag for LPDMA"]
    #[inline(always)]
    #[must_use]
    pub fn clpdma1f(&mut self) -> CLPDMA1F_W<FCR2rs> {
        CLPDMA1F_W::new(self, 5)
    }
    #[doc = "Bit 6 - clear the illegal access flag for EXTI"]
    #[inline(always)]
    #[must_use]
    pub fn cextif(&mut self) -> CEXTIF_W<FCR2rs> {
        CEXTIF_W::new(self, 6)
    }
    #[doc = "Bit 14 - clear the illegal access flag for GTZC2 TZSC registers"]
    #[inline(always)]
    #[must_use]
    pub fn ctzsc2f(&mut self) -> CTZSC2F_W<FCR2rs> {
        CTZSC2F_W::new(self, 14)
    }
    #[doc = "Bit 15 - clear the illegal access flag for GTZC2 TZIC registers"]
    #[inline(always)]
    #[must_use]
    pub fn ctzic2f(&mut self) -> CTZIC2F_W<FCR2rs> {
        CTZIC2F_W::new(self, 15)
    }
    #[doc = "Bit 24 - clear the illegal access flag for SRAM4"]
    #[inline(always)]
    #[must_use]
    pub fn csram4f(&mut self) -> CSRAM4F_W<FCR2rs> {
        CSRAM4F_W::new(self, 24)
    }
    #[doc = "Bit 25 - clear the illegal access flag for MPCBB4 registers"]
    #[inline(always)]
    #[must_use]
    pub fn cmpcbb4_regf(&mut self) -> CMPCBB4_REGF_W<FCR2rs> {
        CMPCBB4_REGF_W::new(self, 25)
    }
}
#[doc = "TZIC flag clear register 2\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr2::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCR2rs;
impl crate::RegisterSpec for FCR2rs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr2::W`](W) writer structure"]
impl crate::Writable for FCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FCR2 to value 0"]
impl crate::Resettable for FCR2rs {
    const RESET_VALUE: u32 = 0;
}
