///Register `TUNE1` reader
pub type R = crate::R<TUNE1rs>;
///Register `TUNE1` writer
pub type W = crate::W<TUNE1rs>;
///Field `INCURREN` reader - INCURREN
pub type INCURREN_R = crate::BitReader;
///Field `INCURREN` writer - INCURREN
pub type INCURREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INCURRINT` reader - INCURRINT
pub type INCURRINT_R = crate::BitReader;
///Field `INCURRINT` writer - INCURRINT
pub type INCURRINT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LFSCAPEN` reader - LFSCAPEN
pub type LFSCAPEN_R = crate::BitReader;
///Field `LFSCAPEN` writer - LFSCAPEN
pub type LFSCAPEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVSLEW` reader - HSDRVSLEW
pub type HSDRVSLEW_R = crate::BitReader;
///Field `HSDRVSLEW` writer - HSDRVSLEW
pub type HSDRVSLEW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVDCCUR` reader - HSDRVDCCUR
pub type HSDRVDCCUR_R = crate::BitReader;
///Field `HSDRVDCCUR` writer - HSDRVDCCUR
pub type HSDRVDCCUR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVDCLEV` reader - HSDRVDCLEV
pub type HSDRVDCLEV_R = crate::BitReader;
///Field `HSDRVDCLEV` writer - HSDRVDCLEV
pub type HSDRVDCLEV_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVCURINCR` reader - HSDRVCURINCR
pub type HSDRVCURINCR_R = crate::BitReader;
///Field `HSDRVCURINCR` writer - HSDRVCURINCR
pub type HSDRVCURINCR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FSDRVRFADJ` reader - FSDRVRFADJ
pub type FSDRVRFADJ_R = crate::BitReader;
///Field `FSDRVRFADJ` writer - FSDRVRFADJ
pub type FSDRVRFADJ_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVRFRED` reader - HSDRVRFRED
pub type HSDRVRFRED_R = crate::BitReader;
///Field `HSDRVRFRED` writer - HSDRVRFRED
pub type HSDRVRFRED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSDRVCHKITRM` reader - HSDRVCHKITRM
pub type HSDRVCHKITRM_R = crate::FieldReader;
///Field `HSDRVCHKITRM` writer - HSDRVCHKITRM
pub type HSDRVCHKITRM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `HSDRVCHKZTRM` reader - HSDRVCHKZTRM
pub type HSDRVCHKZTRM_R = crate::FieldReader;
///Field `HSDRVCHKZTRM` writer - HSDRVCHKZTRM
pub type HSDRVCHKZTRM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `OTPCOMP` reader - OTPCOMP
pub type OTPCOMP_R = crate::FieldReader;
///Field `OTPCOMP` writer - OTPCOMP
pub type OTPCOMP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SQLCHCTL` reader - SQLCHCTL
pub type SQLCHCTL_R = crate::FieldReader;
///Field `SQLCHCTL` writer - SQLCHCTL
pub type SQLCHCTL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HDRXGNEQEN` reader - HDRXGNEQEN
pub type HDRXGNEQEN_R = crate::BitReader;
///Field `HDRXGNEQEN` writer - HDRXGNEQEN
pub type HDRXGNEQEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSRXOFF` reader - HSRXOFF
pub type HSRXOFF_R = crate::FieldReader;
///Field `HSRXOFF` writer - HSRXOFF
pub type HSRXOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `HSFALLPREEM` reader - HSFALLPREEM
pub type HSFALLPREEM_R = crate::BitReader;
///Field `HSFALLPREEM` writer - HSFALLPREEM
pub type HSFALLPREEM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SHTCCTCTLPROT` reader - SHTCCTCTLPROT
pub type SHTCCTCTLPROT_R = crate::BitReader;
///Field `SHTCCTCTLPROT` writer - SHTCCTCTLPROT
pub type SHTCCTCTLPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STAGSEL` reader - STAGSEL
pub type STAGSEL_R = crate::BitReader;
///Field `STAGSEL` writer - STAGSEL
pub type STAGSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - INCURREN
    #[inline(always)]
    pub fn incurren(&self) -> INCURREN_R {
        INCURREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - INCURRINT
    #[inline(always)]
    pub fn incurrint(&self) -> INCURRINT_R {
        INCURRINT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - LFSCAPEN
    #[inline(always)]
    pub fn lfscapen(&self) -> LFSCAPEN_R {
        LFSCAPEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - HSDRVSLEW
    #[inline(always)]
    pub fn hsdrvslew(&self) -> HSDRVSLEW_R {
        HSDRVSLEW_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - HSDRVDCCUR
    #[inline(always)]
    pub fn hsdrvdccur(&self) -> HSDRVDCCUR_R {
        HSDRVDCCUR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - HSDRVDCLEV
    #[inline(always)]
    pub fn hsdrvdclev(&self) -> HSDRVDCLEV_R {
        HSDRVDCLEV_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - HSDRVCURINCR
    #[inline(always)]
    pub fn hsdrvcurincr(&self) -> HSDRVCURINCR_R {
        HSDRVCURINCR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - FSDRVRFADJ
    #[inline(always)]
    pub fn fsdrvrfadj(&self) -> FSDRVRFADJ_R {
        FSDRVRFADJ_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - HSDRVRFRED
    #[inline(always)]
    pub fn hsdrvrfred(&self) -> HSDRVRFRED_R {
        HSDRVRFRED_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:12 - HSDRVCHKITRM
    #[inline(always)]
    pub fn hsdrvchkitrm(&self) -> HSDRVCHKITRM_R {
        HSDRVCHKITRM_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    ///Bits 13:14 - HSDRVCHKZTRM
    #[inline(always)]
    pub fn hsdrvchkztrm(&self) -> HSDRVCHKZTRM_R {
        HSDRVCHKZTRM_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bits 15:19 - OTPCOMP
    #[inline(always)]
    pub fn otpcomp(&self) -> OTPCOMP_R {
        OTPCOMP_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 20:21 - SQLCHCTL
    #[inline(always)]
    pub fn sqlchctl(&self) -> SQLCHCTL_R {
        SQLCHCTL_R::new(((self.bits >> 20) & 3) as u8)
    }
    ///Bit 22 - HDRXGNEQEN
    #[inline(always)]
    pub fn hdrxgneqen(&self) -> HDRXGNEQEN_R {
        HDRXGNEQEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bits 23:24 - HSRXOFF
    #[inline(always)]
    pub fn hsrxoff(&self) -> HSRXOFF_R {
        HSRXOFF_R::new(((self.bits >> 23) & 3) as u8)
    }
    ///Bit 25 - HSFALLPREEM
    #[inline(always)]
    pub fn hsfallpreem(&self) -> HSFALLPREEM_R {
        HSFALLPREEM_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - SHTCCTCTLPROT
    #[inline(always)]
    pub fn shtcctctlprot(&self) -> SHTCCTCTLPROT_R {
        SHTCCTCTLPROT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - STAGSEL
    #[inline(always)]
    pub fn stagsel(&self) -> STAGSEL_R {
        STAGSEL_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TUNE1")
            .field("incurren", &self.incurren())
            .field("incurrint", &self.incurrint())
            .field("lfscapen", &self.lfscapen())
            .field("hsdrvslew", &self.hsdrvslew())
            .field("hsdrvdccur", &self.hsdrvdccur())
            .field("hsdrvdclev", &self.hsdrvdclev())
            .field("hsdrvcurincr", &self.hsdrvcurincr())
            .field("fsdrvrfadj", &self.fsdrvrfadj())
            .field("hsdrvrfred", &self.hsdrvrfred())
            .field("hsdrvchkitrm", &self.hsdrvchkitrm())
            .field("hsdrvchkztrm", &self.hsdrvchkztrm())
            .field("otpcomp", &self.otpcomp())
            .field("sqlchctl", &self.sqlchctl())
            .field("hdrxgneqen", &self.hdrxgneqen())
            .field("hsrxoff", &self.hsrxoff())
            .field("hsfallpreem", &self.hsfallpreem())
            .field("shtcctctlprot", &self.shtcctctlprot())
            .field("stagsel", &self.stagsel())
            .finish()
    }
}
impl W {
    ///Bit 0 - INCURREN
    #[inline(always)]
    pub fn incurren(&mut self) -> INCURREN_W<'_, TUNE1rs> {
        INCURREN_W::new(self, 0)
    }
    ///Bit 1 - INCURRINT
    #[inline(always)]
    pub fn incurrint(&mut self) -> INCURRINT_W<'_, TUNE1rs> {
        INCURRINT_W::new(self, 1)
    }
    ///Bit 2 - LFSCAPEN
    #[inline(always)]
    pub fn lfscapen(&mut self) -> LFSCAPEN_W<'_, TUNE1rs> {
        LFSCAPEN_W::new(self, 2)
    }
    ///Bit 3 - HSDRVSLEW
    #[inline(always)]
    pub fn hsdrvslew(&mut self) -> HSDRVSLEW_W<'_, TUNE1rs> {
        HSDRVSLEW_W::new(self, 3)
    }
    ///Bit 4 - HSDRVDCCUR
    #[inline(always)]
    pub fn hsdrvdccur(&mut self) -> HSDRVDCCUR_W<'_, TUNE1rs> {
        HSDRVDCCUR_W::new(self, 4)
    }
    ///Bit 5 - HSDRVDCLEV
    #[inline(always)]
    pub fn hsdrvdclev(&mut self) -> HSDRVDCLEV_W<'_, TUNE1rs> {
        HSDRVDCLEV_W::new(self, 5)
    }
    ///Bit 6 - HSDRVCURINCR
    #[inline(always)]
    pub fn hsdrvcurincr(&mut self) -> HSDRVCURINCR_W<'_, TUNE1rs> {
        HSDRVCURINCR_W::new(self, 6)
    }
    ///Bit 7 - FSDRVRFADJ
    #[inline(always)]
    pub fn fsdrvrfadj(&mut self) -> FSDRVRFADJ_W<'_, TUNE1rs> {
        FSDRVRFADJ_W::new(self, 7)
    }
    ///Bit 8 - HSDRVRFRED
    #[inline(always)]
    pub fn hsdrvrfred(&mut self) -> HSDRVRFRED_W<'_, TUNE1rs> {
        HSDRVRFRED_W::new(self, 8)
    }
    ///Bits 9:12 - HSDRVCHKITRM
    #[inline(always)]
    pub fn hsdrvchkitrm(&mut self) -> HSDRVCHKITRM_W<'_, TUNE1rs> {
        HSDRVCHKITRM_W::new(self, 9)
    }
    ///Bits 13:14 - HSDRVCHKZTRM
    #[inline(always)]
    pub fn hsdrvchkztrm(&mut self) -> HSDRVCHKZTRM_W<'_, TUNE1rs> {
        HSDRVCHKZTRM_W::new(self, 13)
    }
    ///Bits 15:19 - OTPCOMP
    #[inline(always)]
    pub fn otpcomp(&mut self) -> OTPCOMP_W<'_, TUNE1rs> {
        OTPCOMP_W::new(self, 15)
    }
    ///Bits 20:21 - SQLCHCTL
    #[inline(always)]
    pub fn sqlchctl(&mut self) -> SQLCHCTL_W<'_, TUNE1rs> {
        SQLCHCTL_W::new(self, 20)
    }
    ///Bit 22 - HDRXGNEQEN
    #[inline(always)]
    pub fn hdrxgneqen(&mut self) -> HDRXGNEQEN_W<'_, TUNE1rs> {
        HDRXGNEQEN_W::new(self, 22)
    }
    ///Bits 23:24 - HSRXOFF
    #[inline(always)]
    pub fn hsrxoff(&mut self) -> HSRXOFF_W<'_, TUNE1rs> {
        HSRXOFF_W::new(self, 23)
    }
    ///Bit 25 - HSFALLPREEM
    #[inline(always)]
    pub fn hsfallpreem(&mut self) -> HSFALLPREEM_W<'_, TUNE1rs> {
        HSFALLPREEM_W::new(self, 25)
    }
    ///Bit 26 - SHTCCTCTLPROT
    #[inline(always)]
    pub fn shtcctctlprot(&mut self) -> SHTCCTCTLPROT_W<'_, TUNE1rs> {
        SHTCCTCTLPROT_W::new(self, 26)
    }
    ///Bit 27 - STAGSEL
    #[inline(always)]
    pub fn stagsel(&mut self) -> STAGSEL_W<'_, TUNE1rs> {
        STAGSEL_W::new(self, 27)
    }
}
/**This register is used to control the tune interface of the HS PHY, port #x.

You can [`read`](crate::Reg::read) this register and get [`tune1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tune1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#USBPHYC:TUNE1)*/
pub struct TUNE1rs;
impl crate::RegisterSpec for TUNE1rs {
    type Ux = u32;
}
///`read()` method returns [`tune1::R`](R) reader structure
impl crate::Readable for TUNE1rs {}
///`write(|w| ..)` method takes [`tune1::W`](W) writer structure
impl crate::Writable for TUNE1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TUNE1 to value 0x0407_0004
impl crate::Resettable for TUNE1rs {
    const RESET_VALUE: u32 = 0x0407_0004;
}
