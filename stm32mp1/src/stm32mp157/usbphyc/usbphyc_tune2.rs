#[doc = "Register `USBPHYC_TUNE2` reader"]
pub type R = crate::R<USBPHYC_TUNE2rs>;
#[doc = "Register `USBPHYC_TUNE2` writer"]
pub type W = crate::W<USBPHYC_TUNE2rs>;
#[doc = "Field `INCURREN` reader - INCURREN"]
pub type INCURREN_R = crate::BitReader;
#[doc = "Field `INCURREN` writer - INCURREN"]
pub type INCURREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INCURRINT` reader - INCURRINT"]
pub type INCURRINT_R = crate::BitReader;
#[doc = "Field `INCURRINT` writer - INCURRINT"]
pub type INCURRINT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LFSCAPEN` reader - LFSCAPEN"]
pub type LFSCAPEN_R = crate::BitReader;
#[doc = "Field `LFSCAPEN` writer - LFSCAPEN"]
pub type LFSCAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDRVSLEW` reader - HSDRVSLEW"]
pub type HSDRVSLEW_R = crate::BitReader;
#[doc = "Field `HSDRVSLEW` writer - HSDRVSLEW"]
pub type HSDRVSLEW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDRVDCCUR` reader - HSDRVDCCUR"]
pub type HSDRVDCCUR_R = crate::BitReader;
#[doc = "Field `HSDRVDCCUR` writer - HSDRVDCCUR"]
pub type HSDRVDCCUR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDRVDCLEV` reader - HSDRVDCLEV"]
pub type HSDRVDCLEV_R = crate::BitReader;
#[doc = "Field `HSDRVDCLEV` writer - HSDRVDCLEV"]
pub type HSDRVDCLEV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDRVCURINCR` reader - HSDRVCURINCR"]
pub type HSDRVCURINCR_R = crate::BitReader;
#[doc = "Field `HSDRVCURINCR` writer - HSDRVCURINCR"]
pub type HSDRVCURINCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSDRVRFADJ` reader - FSDRVRFADJ"]
pub type FSDRVRFADJ_R = crate::BitReader;
#[doc = "Field `FSDRVRFADJ` writer - FSDRVRFADJ"]
pub type FSDRVRFADJ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDRVRFRED` reader - HSDRVRFRED"]
pub type HSDRVRFRED_R = crate::BitReader;
#[doc = "Field `HSDRVRFRED` writer - HSDRVRFRED"]
pub type HSDRVRFRED_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSDRVCHKITRM` reader - HSDRVCHKITRM"]
pub type HSDRVCHKITRM_R = crate::FieldReader;
#[doc = "Field `HSDRVCHKITRM` writer - HSDRVCHKITRM"]
pub type HSDRVCHKITRM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HSDRVCHKZTRM` reader - HSDRVCHKZTRM"]
pub type HSDRVCHKZTRM_R = crate::FieldReader;
#[doc = "Field `HSDRVCHKZTRM` writer - HSDRVCHKZTRM"]
pub type HSDRVCHKZTRM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OTPCOMP` reader - OTPCOMP"]
pub type OTPCOMP_R = crate::FieldReader;
#[doc = "Field `OTPCOMP` writer - OTPCOMP"]
pub type OTPCOMP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SQLCHCTL` reader - SQLCHCTL"]
pub type SQLCHCTL_R = crate::FieldReader;
#[doc = "Field `SQLCHCTL` writer - SQLCHCTL"]
pub type SQLCHCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HDRXGNEQEN` reader - HDRXGNEQEN"]
pub type HDRXGNEQEN_R = crate::BitReader;
#[doc = "Field `HDRXGNEQEN` writer - HDRXGNEQEN"]
pub type HDRXGNEQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSRXOFF` reader - HSRXOFF"]
pub type HSRXOFF_R = crate::FieldReader;
#[doc = "Field `HSRXOFF` writer - HSRXOFF"]
pub type HSRXOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSFALLPREEM` reader - HSFALLPREEM"]
pub type HSFALLPREEM_R = crate::BitReader;
#[doc = "Field `HSFALLPREEM` writer - HSFALLPREEM"]
pub type HSFALLPREEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHTCCTCTLPROT` reader - SHTCCTCTLPROT"]
pub type SHTCCTCTLPROT_R = crate::BitReader;
#[doc = "Field `SHTCCTCTLPROT` writer - SHTCCTCTLPROT"]
pub type SHTCCTCTLPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STAGSEL` reader - STAGSEL"]
pub type STAGSEL_R = crate::BitReader;
#[doc = "Field `STAGSEL` writer - STAGSEL"]
pub type STAGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - INCURREN"]
    #[inline(always)]
    pub fn incurren(&self) -> INCURREN_R {
        INCURREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INCURRINT"]
    #[inline(always)]
    pub fn incurrint(&self) -> INCURRINT_R {
        INCURRINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LFSCAPEN"]
    #[inline(always)]
    pub fn lfscapen(&self) -> LFSCAPEN_R {
        LFSCAPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSDRVSLEW"]
    #[inline(always)]
    pub fn hsdrvslew(&self) -> HSDRVSLEW_R {
        HSDRVSLEW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - HSDRVDCCUR"]
    #[inline(always)]
    pub fn hsdrvdccur(&self) -> HSDRVDCCUR_R {
        HSDRVDCCUR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HSDRVDCLEV"]
    #[inline(always)]
    pub fn hsdrvdclev(&self) -> HSDRVDCLEV_R {
        HSDRVDCLEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - HSDRVCURINCR"]
    #[inline(always)]
    pub fn hsdrvcurincr(&self) -> HSDRVCURINCR_R {
        HSDRVCURINCR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FSDRVRFADJ"]
    #[inline(always)]
    pub fn fsdrvrfadj(&self) -> FSDRVRFADJ_R {
        FSDRVRFADJ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - HSDRVRFRED"]
    #[inline(always)]
    pub fn hsdrvrfred(&self) -> HSDRVRFRED_R {
        HSDRVRFRED_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - HSDRVCHKITRM"]
    #[inline(always)]
    pub fn hsdrvchkitrm(&self) -> HSDRVCHKITRM_R {
        HSDRVCHKITRM_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:14 - HSDRVCHKZTRM"]
    #[inline(always)]
    pub fn hsdrvchkztrm(&self) -> HSDRVCHKZTRM_R {
        HSDRVCHKZTRM_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bits 15:19 - OTPCOMP"]
    #[inline(always)]
    pub fn otpcomp(&self) -> OTPCOMP_R {
        OTPCOMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:21 - SQLCHCTL"]
    #[inline(always)]
    pub fn sqlchctl(&self) -> SQLCHCTL_R {
        SQLCHCTL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - HDRXGNEQEN"]
    #[inline(always)]
    pub fn hdrxgneqen(&self) -> HDRXGNEQEN_R {
        HDRXGNEQEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - HSRXOFF"]
    #[inline(always)]
    pub fn hsrxoff(&self) -> HSRXOFF_R {
        HSRXOFF_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - HSFALLPREEM"]
    #[inline(always)]
    pub fn hsfallpreem(&self) -> HSFALLPREEM_R {
        HSFALLPREEM_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - SHTCCTCTLPROT"]
    #[inline(always)]
    pub fn shtcctctlprot(&self) -> SHTCCTCTLPROT_R {
        SHTCCTCTLPROT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - STAGSEL"]
    #[inline(always)]
    pub fn stagsel(&self) -> STAGSEL_R {
        STAGSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - INCURREN"]
    #[inline(always)]
    #[must_use]
    pub fn incurren(&mut self) -> INCURREN_W<USBPHYC_TUNE2rs> {
        INCURREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - INCURRINT"]
    #[inline(always)]
    #[must_use]
    pub fn incurrint(&mut self) -> INCURRINT_W<USBPHYC_TUNE2rs> {
        INCURRINT_W::new(self, 1)
    }
    #[doc = "Bit 2 - LFSCAPEN"]
    #[inline(always)]
    #[must_use]
    pub fn lfscapen(&mut self) -> LFSCAPEN_W<USBPHYC_TUNE2rs> {
        LFSCAPEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - HSDRVSLEW"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvslew(&mut self) -> HSDRVSLEW_W<USBPHYC_TUNE2rs> {
        HSDRVSLEW_W::new(self, 3)
    }
    #[doc = "Bit 4 - HSDRVDCCUR"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvdccur(&mut self) -> HSDRVDCCUR_W<USBPHYC_TUNE2rs> {
        HSDRVDCCUR_W::new(self, 4)
    }
    #[doc = "Bit 5 - HSDRVDCLEV"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvdclev(&mut self) -> HSDRVDCLEV_W<USBPHYC_TUNE2rs> {
        HSDRVDCLEV_W::new(self, 5)
    }
    #[doc = "Bit 6 - HSDRVCURINCR"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvcurincr(&mut self) -> HSDRVCURINCR_W<USBPHYC_TUNE2rs> {
        HSDRVCURINCR_W::new(self, 6)
    }
    #[doc = "Bit 7 - FSDRVRFADJ"]
    #[inline(always)]
    #[must_use]
    pub fn fsdrvrfadj(&mut self) -> FSDRVRFADJ_W<USBPHYC_TUNE2rs> {
        FSDRVRFADJ_W::new(self, 7)
    }
    #[doc = "Bit 8 - HSDRVRFRED"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvrfred(&mut self) -> HSDRVRFRED_W<USBPHYC_TUNE2rs> {
        HSDRVRFRED_W::new(self, 8)
    }
    #[doc = "Bits 9:12 - HSDRVCHKITRM"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvchkitrm(&mut self) -> HSDRVCHKITRM_W<USBPHYC_TUNE2rs> {
        HSDRVCHKITRM_W::new(self, 9)
    }
    #[doc = "Bits 13:14 - HSDRVCHKZTRM"]
    #[inline(always)]
    #[must_use]
    pub fn hsdrvchkztrm(&mut self) -> HSDRVCHKZTRM_W<USBPHYC_TUNE2rs> {
        HSDRVCHKZTRM_W::new(self, 13)
    }
    #[doc = "Bits 15:19 - OTPCOMP"]
    #[inline(always)]
    #[must_use]
    pub fn otpcomp(&mut self) -> OTPCOMP_W<USBPHYC_TUNE2rs> {
        OTPCOMP_W::new(self, 15)
    }
    #[doc = "Bits 20:21 - SQLCHCTL"]
    #[inline(always)]
    #[must_use]
    pub fn sqlchctl(&mut self) -> SQLCHCTL_W<USBPHYC_TUNE2rs> {
        SQLCHCTL_W::new(self, 20)
    }
    #[doc = "Bit 22 - HDRXGNEQEN"]
    #[inline(always)]
    #[must_use]
    pub fn hdrxgneqen(&mut self) -> HDRXGNEQEN_W<USBPHYC_TUNE2rs> {
        HDRXGNEQEN_W::new(self, 22)
    }
    #[doc = "Bits 23:24 - HSRXOFF"]
    #[inline(always)]
    #[must_use]
    pub fn hsrxoff(&mut self) -> HSRXOFF_W<USBPHYC_TUNE2rs> {
        HSRXOFF_W::new(self, 23)
    }
    #[doc = "Bit 25 - HSFALLPREEM"]
    #[inline(always)]
    #[must_use]
    pub fn hsfallpreem(&mut self) -> HSFALLPREEM_W<USBPHYC_TUNE2rs> {
        HSFALLPREEM_W::new(self, 25)
    }
    #[doc = "Bit 26 - SHTCCTCTLPROT"]
    #[inline(always)]
    #[must_use]
    pub fn shtcctctlprot(&mut self) -> SHTCCTCTLPROT_W<USBPHYC_TUNE2rs> {
        SHTCCTCTLPROT_W::new(self, 26)
    }
    #[doc = "Bit 27 - STAGSEL"]
    #[inline(always)]
    #[must_use]
    pub fn stagsel(&mut self) -> STAGSEL_W<USBPHYC_TUNE2rs> {
        STAGSEL_W::new(self, 27)
    }
}
#[doc = "This register is used to control the tune interface of the HS PHY, port #x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usbphyc_tune2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usbphyc_tune2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USBPHYC_TUNE2rs;
impl crate::RegisterSpec for USBPHYC_TUNE2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbphyc_tune2::R`](R) reader structure"]
impl crate::Readable for USBPHYC_TUNE2rs {}
#[doc = "`write(|w| ..)` method takes [`usbphyc_tune2::W`](W) writer structure"]
impl crate::Writable for USBPHYC_TUNE2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets USBPHYC_TUNE2 to value 0x0407_0004"]
impl crate::Resettable for USBPHYC_TUNE2rs {
    const RESET_VALUE: u32 = 0x0407_0004;
}
