#[doc = "Register `OTG_GLPMCFG` reader"]
pub type R = crate::R<OTG_GLPMCFGrs>;
#[doc = "Register `OTG_GLPMCFG` writer"]
pub type W = crate::W<OTG_GLPMCFGrs>;
#[doc = "Field `LPMEN` reader - LPMEN"]
pub type LPMEN_R = crate::BitReader;
#[doc = "Field `LPMEN` writer - LPMEN"]
pub type LPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMACK` reader - LPMACK"]
pub type LPMACK_R = crate::BitReader;
#[doc = "Field `LPMACK` writer - LPMACK"]
pub type LPMACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BESL` reader - BESL"]
pub type BESL_R = crate::FieldReader;
#[doc = "Field `BESL` writer - BESL"]
pub type BESL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REMWAKE` reader - REMWAKE"]
pub type REMWAKE_R = crate::BitReader;
#[doc = "Field `REMWAKE` writer - REMWAKE"]
pub type REMWAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1SSEN` reader - L1SSEN"]
pub type L1SSEN_R = crate::BitReader;
#[doc = "Field `L1SSEN` writer - L1SSEN"]
pub type L1SSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BESLTHRS` reader - BESLTHRS"]
pub type BESLTHRS_R = crate::FieldReader;
#[doc = "Field `BESLTHRS` writer - BESLTHRS"]
pub type BESLTHRS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `L1DSEN` reader - L1DSEN"]
pub type L1DSEN_R = crate::BitReader;
#[doc = "Field `L1DSEN` writer - L1DSEN"]
pub type L1DSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRSP` reader - LPMRSP"]
pub type LPMRSP_R = crate::FieldReader;
#[doc = "Field `SLPSTS` reader - SLPSTS"]
pub type SLPSTS_R = crate::BitReader;
#[doc = "Field `L1RSMOK` reader - L1RSMOK"]
pub type L1RSMOK_R = crate::BitReader;
#[doc = "Field `LPMCHIDX` reader - LPMCHIDX"]
pub type LPMCHIDX_R = crate::FieldReader;
#[doc = "Field `LPMCHIDX` writer - LPMCHIDX"]
pub type LPMCHIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPMRCNT` reader - LPMRCNT"]
pub type LPMRCNT_R = crate::FieldReader;
#[doc = "Field `LPMRCNT` writer - LPMRCNT"]
pub type LPMRCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SNDLPM` reader - SNDLPM"]
pub type SNDLPM_R = crate::BitReader;
#[doc = "Field `SNDLPM` writer - SNDLPM"]
pub type SNDLPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRCNTSTS` reader - LPMRCNTSTS"]
pub type LPMRCNTSTS_R = crate::FieldReader;
#[doc = "Field `ENBESL` reader - ENBESL"]
pub type ENBESL_R = crate::BitReader;
#[doc = "Field `ENBESL` writer - ENBESL"]
pub type ENBESL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPMEN"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPMACK"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - BESL"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - REMWAKE"]
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - L1SSEN"]
    #[inline(always)]
    pub fn l1ssen(&self) -> L1SSEN_R {
        L1SSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - BESLTHRS"]
    #[inline(always)]
    pub fn beslthrs(&self) -> BESLTHRS_R {
        BESLTHRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - L1DSEN"]
    #[inline(always)]
    pub fn l1dsen(&self) -> L1DSEN_R {
        L1DSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - LPMRSP"]
    #[inline(always)]
    pub fn lpmrsp(&self) -> LPMRSP_R {
        LPMRSP_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - SLPSTS"]
    #[inline(always)]
    pub fn slpsts(&self) -> SLPSTS_R {
        SLPSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L1RSMOK"]
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1RSMOK_R {
        L1RSMOK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - LPMCHIDX"]
    #[inline(always)]
    pub fn lpmchidx(&self) -> LPMCHIDX_R {
        LPMCHIDX_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:23 - LPMRCNT"]
    #[inline(always)]
    pub fn lpmrcnt(&self) -> LPMRCNT_R {
        LPMRCNT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - SNDLPM"]
    #[inline(always)]
    pub fn sndlpm(&self) -> SNDLPM_R {
        SNDLPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - LPMRCNTSTS"]
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LPMRCNTSTS_R {
        LPMRCNTSTS_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - ENBESL"]
    #[inline(always)]
    pub fn enbesl(&self) -> ENBESL_R {
        ENBESL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPMEN"]
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<OTG_GLPMCFGrs> {
        LPMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPMACK"]
    #[inline(always)]
    #[must_use]
    pub fn lpmack(&mut self) -> LPMACK_W<OTG_GLPMCFGrs> {
        LPMACK_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - BESL"]
    #[inline(always)]
    #[must_use]
    pub fn besl(&mut self) -> BESL_W<OTG_GLPMCFGrs> {
        BESL_W::new(self, 2)
    }
    #[doc = "Bit 6 - REMWAKE"]
    #[inline(always)]
    #[must_use]
    pub fn remwake(&mut self) -> REMWAKE_W<OTG_GLPMCFGrs> {
        REMWAKE_W::new(self, 6)
    }
    #[doc = "Bit 7 - L1SSEN"]
    #[inline(always)]
    #[must_use]
    pub fn l1ssen(&mut self) -> L1SSEN_W<OTG_GLPMCFGrs> {
        L1SSEN_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - BESLTHRS"]
    #[inline(always)]
    #[must_use]
    pub fn beslthrs(&mut self) -> BESLTHRS_W<OTG_GLPMCFGrs> {
        BESLTHRS_W::new(self, 8)
    }
    #[doc = "Bit 12 - L1DSEN"]
    #[inline(always)]
    #[must_use]
    pub fn l1dsen(&mut self) -> L1DSEN_W<OTG_GLPMCFGrs> {
        L1DSEN_W::new(self, 12)
    }
    #[doc = "Bits 17:20 - LPMCHIDX"]
    #[inline(always)]
    #[must_use]
    pub fn lpmchidx(&mut self) -> LPMCHIDX_W<OTG_GLPMCFGrs> {
        LPMCHIDX_W::new(self, 17)
    }
    #[doc = "Bits 21:23 - LPMRCNT"]
    #[inline(always)]
    #[must_use]
    pub fn lpmrcnt(&mut self) -> LPMRCNT_W<OTG_GLPMCFGrs> {
        LPMRCNT_W::new(self, 21)
    }
    #[doc = "Bit 24 - SNDLPM"]
    #[inline(always)]
    #[must_use]
    pub fn sndlpm(&mut self) -> SNDLPM_W<OTG_GLPMCFGrs> {
        SNDLPM_W::new(self, 24)
    }
    #[doc = "Bit 28 - ENBESL"]
    #[inline(always)]
    #[must_use]
    pub fn enbesl(&mut self) -> ENBESL_W<OTG_GLPMCFGrs> {
        ENBESL_W::new(self, 28)
    }
}
#[doc = "OTG core LPM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_glpmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_glpmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_GLPMCFGrs;
impl crate::RegisterSpec for OTG_GLPMCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_glpmcfg::R`](R) reader structure"]
impl crate::Readable for OTG_GLPMCFGrs {}
#[doc = "`write(|w| ..)` method takes [`otg_glpmcfg::W`](W) writer structure"]
impl crate::Writable for OTG_GLPMCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_GLPMCFG to value 0"]
impl crate::Resettable for OTG_GLPMCFGrs {
    const RESET_VALUE: u32 = 0;
}
