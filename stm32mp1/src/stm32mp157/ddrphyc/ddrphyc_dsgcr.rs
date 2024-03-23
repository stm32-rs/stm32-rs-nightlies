#[doc = "Register `DDRPHYC_DSGCR` reader"]
pub type R = crate::R<DDRPHYC_DSGCRrs>;
#[doc = "Register `DDRPHYC_DSGCR` writer"]
pub type W = crate::W<DDRPHYC_DSGCRrs>;
#[doc = "Field `PUREN` reader - PUREN"]
pub type PUREN_R = crate::BitReader;
#[doc = "Field `PUREN` writer - PUREN"]
pub type PUREN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDISEN` reader - BDISEN"]
pub type BDISEN_R = crate::BitReader;
#[doc = "Field `BDISEN` writer - BDISEN"]
pub type BDISEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZUEN` reader - ZUEN"]
pub type ZUEN_R = crate::BitReader;
#[doc = "Field `ZUEN` writer - ZUEN"]
pub type ZUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPIOPD` reader - LPIOPD"]
pub type LPIOPD_R = crate::BitReader;
#[doc = "Field `LPIOPD` writer - LPIOPD"]
pub type LPIOPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPDLLPD` reader - LPDLLPD"]
pub type LPDLLPD_R = crate::BitReader;
#[doc = "Field `LPDLLPD` writer - LPDLLPD"]
pub type LPDLLPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSGX` reader - DQSGX"]
pub type DQSGX_R = crate::FieldReader;
#[doc = "Field `DQSGX` writer - DQSGX"]
pub type DQSGX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DQSGE` reader - DQSGE"]
pub type DQSGE_R = crate::FieldReader;
#[doc = "Field `DQSGE` writer - DQSGE"]
pub type DQSGE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NOBUB` reader - NOBUB"]
pub type NOBUB_R = crate::BitReader;
#[doc = "Field `NOBUB` writer - NOBUB"]
pub type NOBUB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FXDLAT` reader - FXDLAT"]
pub type FXDLAT_R = crate::BitReader;
#[doc = "Field `FXDLAT` writer - FXDLAT"]
pub type FXDLAT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEPDD` reader - CKEPDD"]
pub type CKEPDD_R = crate::BitReader;
#[doc = "Field `CKEPDD` writer - CKEPDD"]
pub type CKEPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODTPDD` reader - ODTPDD"]
pub type ODTPDD_R = crate::BitReader;
#[doc = "Field `ODTPDD` writer - ODTPDD"]
pub type ODTPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NL2PD` reader - NL2PD"]
pub type NL2PD_R = crate::BitReader;
#[doc = "Field `NL2PD` writer - NL2PD"]
pub type NL2PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NL2OE` reader - NL2OE"]
pub type NL2OE_R = crate::BitReader;
#[doc = "Field `NL2OE` writer - NL2OE"]
pub type NL2OE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPDPD` reader - TPDPD"]
pub type TPDPD_R = crate::BitReader;
#[doc = "Field `TPDPD` writer - TPDPD"]
pub type TPDPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPDOE` reader - TPDOE"]
pub type TPDOE_R = crate::BitReader;
#[doc = "Field `TPDOE` writer - TPDOE"]
pub type TPDOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKOE` reader - CKOE"]
pub type CKOE_R = crate::BitReader;
#[doc = "Field `CKOE` writer - CKOE"]
pub type CKOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODTOE` reader - ODTOE"]
pub type ODTOE_R = crate::BitReader;
#[doc = "Field `ODTOE` writer - ODTOE"]
pub type ODTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSTOE` reader - RSTOE"]
pub type RSTOE_R = crate::BitReader;
#[doc = "Field `RSTOE` writer - RSTOE"]
pub type RSTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEOE` reader - CKEOE"]
pub type CKEOE_R = crate::BitReader;
#[doc = "Field `CKEOE` writer - CKEOE"]
pub type CKEOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PUREN"]
    #[inline(always)]
    pub fn puren(&self) -> PUREN_R {
        PUREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BDISEN"]
    #[inline(always)]
    pub fn bdisen(&self) -> BDISEN_R {
        BDISEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ZUEN"]
    #[inline(always)]
    pub fn zuen(&self) -> ZUEN_R {
        ZUEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LPIOPD"]
    #[inline(always)]
    pub fn lpiopd(&self) -> LPIOPD_R {
        LPIOPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LPDLLPD"]
    #[inline(always)]
    pub fn lpdllpd(&self) -> LPDLLPD_R {
        LPDLLPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - DQSGX"]
    #[inline(always)]
    pub fn dqsgx(&self) -> DQSGX_R {
        DQSGX_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:10 - DQSGE"]
    #[inline(always)]
    pub fn dqsge(&self) -> DQSGE_R {
        DQSGE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - NOBUB"]
    #[inline(always)]
    pub fn nobub(&self) -> NOBUB_R {
        NOBUB_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FXDLAT"]
    #[inline(always)]
    pub fn fxdlat(&self) -> FXDLAT_R {
        FXDLAT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - CKEPDD"]
    #[inline(always)]
    pub fn ckepdd(&self) -> CKEPDD_R {
        CKEPDD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 20 - ODTPDD"]
    #[inline(always)]
    pub fn odtpdd(&self) -> ODTPDD_R {
        ODTPDD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 24 - NL2PD"]
    #[inline(always)]
    pub fn nl2pd(&self) -> NL2PD_R {
        NL2PD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - NL2OE"]
    #[inline(always)]
    pub fn nl2oe(&self) -> NL2OE_R {
        NL2OE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - TPDPD"]
    #[inline(always)]
    pub fn tpdpd(&self) -> TPDPD_R {
        TPDPD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TPDOE"]
    #[inline(always)]
    pub fn tpdoe(&self) -> TPDOE_R {
        TPDOE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - CKOE"]
    #[inline(always)]
    pub fn ckoe(&self) -> CKOE_R {
        CKOE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - ODTOE"]
    #[inline(always)]
    pub fn odtoe(&self) -> ODTOE_R {
        ODTOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - RSTOE"]
    #[inline(always)]
    pub fn rstoe(&self) -> RSTOE_R {
        RSTOE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - CKEOE"]
    #[inline(always)]
    pub fn ckeoe(&self) -> CKEOE_R {
        CKEOE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PUREN"]
    #[inline(always)]
    #[must_use]
    pub fn puren(&mut self) -> PUREN_W<DDRPHYC_DSGCRrs> {
        PUREN_W::new(self, 0)
    }
    #[doc = "Bit 1 - BDISEN"]
    #[inline(always)]
    #[must_use]
    pub fn bdisen(&mut self) -> BDISEN_W<DDRPHYC_DSGCRrs> {
        BDISEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - ZUEN"]
    #[inline(always)]
    #[must_use]
    pub fn zuen(&mut self) -> ZUEN_W<DDRPHYC_DSGCRrs> {
        ZUEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - LPIOPD"]
    #[inline(always)]
    #[must_use]
    pub fn lpiopd(&mut self) -> LPIOPD_W<DDRPHYC_DSGCRrs> {
        LPIOPD_W::new(self, 3)
    }
    #[doc = "Bit 4 - LPDLLPD"]
    #[inline(always)]
    #[must_use]
    pub fn lpdllpd(&mut self) -> LPDLLPD_W<DDRPHYC_DSGCRrs> {
        LPDLLPD_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - DQSGX"]
    #[inline(always)]
    #[must_use]
    pub fn dqsgx(&mut self) -> DQSGX_W<DDRPHYC_DSGCRrs> {
        DQSGX_W::new(self, 5)
    }
    #[doc = "Bits 8:10 - DQSGE"]
    #[inline(always)]
    #[must_use]
    pub fn dqsge(&mut self) -> DQSGE_W<DDRPHYC_DSGCRrs> {
        DQSGE_W::new(self, 8)
    }
    #[doc = "Bit 11 - NOBUB"]
    #[inline(always)]
    #[must_use]
    pub fn nobub(&mut self) -> NOBUB_W<DDRPHYC_DSGCRrs> {
        NOBUB_W::new(self, 11)
    }
    #[doc = "Bit 12 - FXDLAT"]
    #[inline(always)]
    #[must_use]
    pub fn fxdlat(&mut self) -> FXDLAT_W<DDRPHYC_DSGCRrs> {
        FXDLAT_W::new(self, 12)
    }
    #[doc = "Bit 16 - CKEPDD"]
    #[inline(always)]
    #[must_use]
    pub fn ckepdd(&mut self) -> CKEPDD_W<DDRPHYC_DSGCRrs> {
        CKEPDD_W::new(self, 16)
    }
    #[doc = "Bit 20 - ODTPDD"]
    #[inline(always)]
    #[must_use]
    pub fn odtpdd(&mut self) -> ODTPDD_W<DDRPHYC_DSGCRrs> {
        ODTPDD_W::new(self, 20)
    }
    #[doc = "Bit 24 - NL2PD"]
    #[inline(always)]
    #[must_use]
    pub fn nl2pd(&mut self) -> NL2PD_W<DDRPHYC_DSGCRrs> {
        NL2PD_W::new(self, 24)
    }
    #[doc = "Bit 25 - NL2OE"]
    #[inline(always)]
    #[must_use]
    pub fn nl2oe(&mut self) -> NL2OE_W<DDRPHYC_DSGCRrs> {
        NL2OE_W::new(self, 25)
    }
    #[doc = "Bit 26 - TPDPD"]
    #[inline(always)]
    #[must_use]
    pub fn tpdpd(&mut self) -> TPDPD_W<DDRPHYC_DSGCRrs> {
        TPDPD_W::new(self, 26)
    }
    #[doc = "Bit 27 - TPDOE"]
    #[inline(always)]
    #[must_use]
    pub fn tpdoe(&mut self) -> TPDOE_W<DDRPHYC_DSGCRrs> {
        TPDOE_W::new(self, 27)
    }
    #[doc = "Bit 28 - CKOE"]
    #[inline(always)]
    #[must_use]
    pub fn ckoe(&mut self) -> CKOE_W<DDRPHYC_DSGCRrs> {
        CKOE_W::new(self, 28)
    }
    #[doc = "Bit 29 - ODTOE"]
    #[inline(always)]
    #[must_use]
    pub fn odtoe(&mut self) -> ODTOE_W<DDRPHYC_DSGCRrs> {
        ODTOE_W::new(self, 29)
    }
    #[doc = "Bit 30 - RSTOE"]
    #[inline(always)]
    #[must_use]
    pub fn rstoe(&mut self) -> RSTOE_W<DDRPHYC_DSGCRrs> {
        RSTOE_W::new(self, 30)
    }
    #[doc = "Bit 31 - CKEOE"]
    #[inline(always)]
    #[must_use]
    pub fn ckeoe(&mut self) -> CKEOE_W<DDRPHYC_DSGCRrs> {
        CKEOE_W::new(self, 31)
    }
}
#[doc = "DDRPHYC DSGC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddrphyc_dsgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddrphyc_dsgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DDRPHYC_DSGCRrs;
impl crate::RegisterSpec for DDRPHYC_DSGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddrphyc_dsgcr::R`](R) reader structure"]
impl crate::Readable for DDRPHYC_DSGCRrs {}
#[doc = "`write(|w| ..)` method takes [`ddrphyc_dsgcr::W`](W) writer structure"]
impl crate::Writable for DDRPHYC_DSGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDRPHYC_DSGCR to value 0xfa00_001f"]
impl crate::Resettable for DDRPHYC_DSGCRrs {
    const RESET_VALUE: u32 = 0xfa00_001f;
}
