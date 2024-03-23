#[doc = "Register `GLPMCFG` reader"]
pub type R = crate::R<GLPMCFGrs>;
#[doc = "Register `GLPMCFG` writer"]
pub type W = crate::W<GLPMCFGrs>;
#[doc = "Field `LPMEN` reader - LPM support enable"]
pub type LPMEN_R = crate::BitReader;
#[doc = "Field `LPMEN` writer - LPM support enable"]
pub type LPMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMACK` reader - LPM token acknowledge enable"]
pub type LPMACK_R = crate::BitReader;
#[doc = "Field `LPMACK` writer - LPM token acknowledge enable"]
pub type LPMACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BESL` reader - Best effort service latency"]
pub type BESL_R = crate::FieldReader;
#[doc = "Field `BESL` writer - Best effort service latency"]
pub type BESL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REMWAKE` reader - bRemoteWake value"]
pub type REMWAKE_R = crate::BitReader;
#[doc = "Field `REMWAKE` writer - bRemoteWake value"]
pub type REMWAKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1SSEN` reader - L1 Shallow Sleep enable"]
pub type L1SSEN_R = crate::BitReader;
#[doc = "Field `L1SSEN` writer - L1 Shallow Sleep enable"]
pub type L1SSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BESLTHRS` reader - BESL threshold"]
pub type BESLTHRS_R = crate::FieldReader;
#[doc = "Field `BESLTHRS` writer - BESL threshold"]
pub type BESLTHRS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `L1DSEN` reader - L1 deep sleep enable"]
pub type L1DSEN_R = crate::BitReader;
#[doc = "Field `L1DSEN` writer - L1 deep sleep enable"]
pub type L1DSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRSP` reader - LPM response"]
pub type LPMRSP_R = crate::FieldReader;
#[doc = "Field `LPMRSP` writer - LPM response"]
pub type LPMRSP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SLPSTS` reader - Port sleep status"]
pub type SLPSTS_R = crate::BitReader;
#[doc = "Field `SLPSTS` writer - Port sleep status"]
pub type SLPSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1RSMOK` reader - Sleep state resume OK"]
pub type L1RSMOK_R = crate::BitReader;
#[doc = "Field `L1RSMOK` writer - Sleep state resume OK"]
pub type L1RSMOK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMCHIDX` reader - LPM Channel Index"]
pub type LPMCHIDX_R = crate::FieldReader;
#[doc = "Field `LPMCHIDX` writer - LPM Channel Index"]
pub type LPMCHIDX_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PMRCNT` reader - LPM retry count"]
pub type PMRCNT_R = crate::FieldReader;
#[doc = "Field `PMRCNT` writer - LPM retry count"]
pub type PMRCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SNDLPM` reader - Send LPM transaction"]
pub type SNDLPM_R = crate::BitReader;
#[doc = "Field `SNDLPM` writer - Send LPM transaction"]
pub type SNDLPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPMRCNTSTS` reader - LPM retry count status"]
pub type LPMRCNTSTS_R = crate::FieldReader;
#[doc = "Field `LPMRCNTSTS` writer - LPM retry count status"]
pub type LPMRCNTSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ENBESL` reader - Enable best effort service latency"]
pub type ENBESL_R = crate::BitReader;
#[doc = "Field `ENBESL` writer - Enable best effort service latency"]
pub type ENBESL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    pub fn lpmen(&self) -> LPMEN_R {
        LPMEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    pub fn lpmack(&self) -> LPMACK_R {
        LPMACK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Best effort service latency"]
    #[inline(always)]
    pub fn besl(&self) -> BESL_R {
        BESL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - bRemoteWake value"]
    #[inline(always)]
    pub fn remwake(&self) -> REMWAKE_R {
        REMWAKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - L1 Shallow Sleep enable"]
    #[inline(always)]
    pub fn l1ssen(&self) -> L1SSEN_R {
        L1SSEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    pub fn beslthrs(&self) -> BESLTHRS_R {
        BESLTHRS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - L1 deep sleep enable"]
    #[inline(always)]
    pub fn l1dsen(&self) -> L1DSEN_R {
        L1DSEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - LPM response"]
    #[inline(always)]
    pub fn lpmrsp(&self) -> LPMRSP_R {
        LPMRSP_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Port sleep status"]
    #[inline(always)]
    pub fn slpsts(&self) -> SLPSTS_R {
        SLPSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Sleep state resume OK"]
    #[inline(always)]
    pub fn l1rsmok(&self) -> L1RSMOK_R {
        L1RSMOK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - LPM Channel Index"]
    #[inline(always)]
    pub fn lpmchidx(&self) -> LPMCHIDX_R {
        LPMCHIDX_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    pub fn pmrcnt(&self) -> PMRCNT_R {
        PMRCNT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    pub fn sndlpm(&self) -> SNDLPM_R {
        SNDLPM_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - LPM retry count status"]
    #[inline(always)]
    pub fn lpmrcntsts(&self) -> LPMRCNTSTS_R {
        LPMRCNTSTS_R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Enable best effort service latency"]
    #[inline(always)]
    pub fn enbesl(&self) -> ENBESL_R {
        ENBESL_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LPM support enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmen(&mut self) -> LPMEN_W<GLPMCFGrs> {
        LPMEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - LPM token acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpmack(&mut self) -> LPMACK_W<GLPMCFGrs> {
        LPMACK_W::new(self, 1)
    }
    #[doc = "Bits 2:5 - Best effort service latency"]
    #[inline(always)]
    #[must_use]
    pub fn besl(&mut self) -> BESL_W<GLPMCFGrs> {
        BESL_W::new(self, 2)
    }
    #[doc = "Bit 6 - bRemoteWake value"]
    #[inline(always)]
    #[must_use]
    pub fn remwake(&mut self) -> REMWAKE_W<GLPMCFGrs> {
        REMWAKE_W::new(self, 6)
    }
    #[doc = "Bit 7 - L1 Shallow Sleep enable"]
    #[inline(always)]
    #[must_use]
    pub fn l1ssen(&mut self) -> L1SSEN_W<GLPMCFGrs> {
        L1SSEN_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - BESL threshold"]
    #[inline(always)]
    #[must_use]
    pub fn beslthrs(&mut self) -> BESLTHRS_W<GLPMCFGrs> {
        BESLTHRS_W::new(self, 8)
    }
    #[doc = "Bit 12 - L1 deep sleep enable"]
    #[inline(always)]
    #[must_use]
    pub fn l1dsen(&mut self) -> L1DSEN_W<GLPMCFGrs> {
        L1DSEN_W::new(self, 12)
    }
    #[doc = "Bits 13:14 - LPM response"]
    #[inline(always)]
    #[must_use]
    pub fn lpmrsp(&mut self) -> LPMRSP_W<GLPMCFGrs> {
        LPMRSP_W::new(self, 13)
    }
    #[doc = "Bit 15 - Port sleep status"]
    #[inline(always)]
    #[must_use]
    pub fn slpsts(&mut self) -> SLPSTS_W<GLPMCFGrs> {
        SLPSTS_W::new(self, 15)
    }
    #[doc = "Bit 16 - Sleep state resume OK"]
    #[inline(always)]
    #[must_use]
    pub fn l1rsmok(&mut self) -> L1RSMOK_W<GLPMCFGrs> {
        L1RSMOK_W::new(self, 16)
    }
    #[doc = "Bits 17:20 - LPM Channel Index"]
    #[inline(always)]
    #[must_use]
    pub fn lpmchidx(&mut self) -> LPMCHIDX_W<GLPMCFGrs> {
        LPMCHIDX_W::new(self, 17)
    }
    #[doc = "Bits 21:23 - LPM retry count"]
    #[inline(always)]
    #[must_use]
    pub fn pmrcnt(&mut self) -> PMRCNT_W<GLPMCFGrs> {
        PMRCNT_W::new(self, 21)
    }
    #[doc = "Bit 24 - Send LPM transaction"]
    #[inline(always)]
    #[must_use]
    pub fn sndlpm(&mut self) -> SNDLPM_W<GLPMCFGrs> {
        SNDLPM_W::new(self, 24)
    }
    #[doc = "Bits 25:27 - LPM retry count status"]
    #[inline(always)]
    #[must_use]
    pub fn lpmrcntsts(&mut self) -> LPMRCNTSTS_W<GLPMCFGrs> {
        LPMRCNTSTS_W::new(self, 25)
    }
    #[doc = "Bit 28 - Enable best effort service latency"]
    #[inline(always)]
    #[must_use]
    pub fn enbesl(&mut self) -> ENBESL_W<GLPMCFGrs> {
        ENBESL_W::new(self, 28)
    }
}
#[doc = "OTG core LPM configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`glpmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`glpmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GLPMCFGrs;
impl crate::RegisterSpec for GLPMCFGrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`glpmcfg::R`](R) reader structure"]
impl crate::Readable for GLPMCFGrs {}
#[doc = "`write(|w| ..)` method takes [`glpmcfg::W`](W) writer structure"]
impl crate::Writable for GLPMCFGrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GLPMCFG to value 0"]
impl crate::Resettable for GLPMCFGrs {
    const RESET_VALUE: u32 = 0;
}
