///Register `DSGCR` reader
pub type R = crate::R<DSGCRrs>;
///Register `DSGCR` writer
pub type W = crate::W<DSGCRrs>;
///Field `PUREN` reader - PUREN
pub type PUREN_R = crate::BitReader;
///Field `PUREN` writer - PUREN
pub type PUREN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BDISEN` reader - BDISEN
pub type BDISEN_R = crate::BitReader;
///Field `BDISEN` writer - BDISEN
pub type BDISEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ZUEN` reader - ZUEN
pub type ZUEN_R = crate::BitReader;
///Field `ZUEN` writer - ZUEN
pub type ZUEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPIOPD` reader - LPIOPD
pub type LPIOPD_R = crate::BitReader;
///Field `LPIOPD` writer - LPIOPD
pub type LPIOPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPDLLPD` reader - LPDLLPD
pub type LPDLLPD_R = crate::BitReader;
///Field `LPDLLPD` writer - LPDLLPD
pub type LPDLLPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DQSGX` reader - DQSGX
pub type DQSGX_R = crate::FieldReader;
///Field `DQSGX` writer - DQSGX
pub type DQSGX_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `DQSGE` reader - DQSGE
pub type DQSGE_R = crate::FieldReader;
///Field `DQSGE` writer - DQSGE
pub type DQSGE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `NOBUB` reader - NOBUB
pub type NOBUB_R = crate::BitReader;
///Field `NOBUB` writer - NOBUB
pub type NOBUB_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FXDLAT` reader - FXDLAT
pub type FXDLAT_R = crate::BitReader;
///Field `FXDLAT` writer - FXDLAT
pub type FXDLAT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEPDD` reader - CKEPDD
pub type CKEPDD_R = crate::BitReader;
///Field `CKEPDD` writer - CKEPDD
pub type CKEPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODTPDD` reader - ODTPDD
pub type ODTPDD_R = crate::BitReader;
///Field `ODTPDD` writer - ODTPDD
pub type ODTPDD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NL2PD` reader - NL2PD
pub type NL2PD_R = crate::BitReader;
///Field `NL2PD` writer - NL2PD
pub type NL2PD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NL2OE` reader - NL2OE
pub type NL2OE_R = crate::BitReader;
///Field `NL2OE` writer - NL2OE
pub type NL2OE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TPDPD` reader - TPDPD
pub type TPDPD_R = crate::BitReader;
///Field `TPDPD` writer - TPDPD
pub type TPDPD_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TPDOE` reader - TPDOE
pub type TPDOE_R = crate::BitReader;
///Field `TPDOE` writer - TPDOE
pub type TPDOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKOE` reader - CKOE
pub type CKOE_R = crate::BitReader;
///Field `CKOE` writer - CKOE
pub type CKOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ODTOE` reader - ODTOE
pub type ODTOE_R = crate::BitReader;
///Field `ODTOE` writer - ODTOE
pub type ODTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RSTOE` reader - RSTOE
pub type RSTOE_R = crate::BitReader;
///Field `RSTOE` writer - RSTOE
pub type RSTOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEOE` reader - CKEOE
pub type CKEOE_R = crate::BitReader;
///Field `CKEOE` writer - CKEOE
pub type CKEOE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - PUREN
    #[inline(always)]
    pub fn puren(&self) -> PUREN_R {
        PUREN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BDISEN
    #[inline(always)]
    pub fn bdisen(&self) -> BDISEN_R {
        BDISEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ZUEN
    #[inline(always)]
    pub fn zuen(&self) -> ZUEN_R {
        ZUEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - LPIOPD
    #[inline(always)]
    pub fn lpiopd(&self) -> LPIOPD_R {
        LPIOPD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LPDLLPD
    #[inline(always)]
    pub fn lpdllpd(&self) -> LPDLLPD_R {
        LPDLLPD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bits 5:7 - DQSGX
    #[inline(always)]
    pub fn dqsgx(&self) -> DQSGX_R {
        DQSGX_R::new(((self.bits >> 5) & 7) as u8)
    }
    ///Bits 8:10 - DQSGE
    #[inline(always)]
    pub fn dqsge(&self) -> DQSGE_R {
        DQSGE_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 11 - NOBUB
    #[inline(always)]
    pub fn nobub(&self) -> NOBUB_R {
        NOBUB_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - FXDLAT
    #[inline(always)]
    pub fn fxdlat(&self) -> FXDLAT_R {
        FXDLAT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 16 - CKEPDD
    #[inline(always)]
    pub fn ckepdd(&self) -> CKEPDD_R {
        CKEPDD_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 20 - ODTPDD
    #[inline(always)]
    pub fn odtpdd(&self) -> ODTPDD_R {
        ODTPDD_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 24 - NL2PD
    #[inline(always)]
    pub fn nl2pd(&self) -> NL2PD_R {
        NL2PD_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - NL2OE
    #[inline(always)]
    pub fn nl2oe(&self) -> NL2OE_R {
        NL2OE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - TPDPD
    #[inline(always)]
    pub fn tpdpd(&self) -> TPDPD_R {
        TPDPD_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - TPDOE
    #[inline(always)]
    pub fn tpdoe(&self) -> TPDOE_R {
        TPDOE_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - CKOE
    #[inline(always)]
    pub fn ckoe(&self) -> CKOE_R {
        CKOE_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - ODTOE
    #[inline(always)]
    pub fn odtoe(&self) -> ODTOE_R {
        ODTOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - RSTOE
    #[inline(always)]
    pub fn rstoe(&self) -> RSTOE_R {
        RSTOE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - CKEOE
    #[inline(always)]
    pub fn ckeoe(&self) -> CKEOE_R {
        CKEOE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSGCR")
            .field("puren", &self.puren())
            .field("bdisen", &self.bdisen())
            .field("zuen", &self.zuen())
            .field("lpiopd", &self.lpiopd())
            .field("lpdllpd", &self.lpdllpd())
            .field("dqsgx", &self.dqsgx())
            .field("dqsge", &self.dqsge())
            .field("nobub", &self.nobub())
            .field("fxdlat", &self.fxdlat())
            .field("ckepdd", &self.ckepdd())
            .field("odtpdd", &self.odtpdd())
            .field("nl2pd", &self.nl2pd())
            .field("nl2oe", &self.nl2oe())
            .field("tpdpd", &self.tpdpd())
            .field("tpdoe", &self.tpdoe())
            .field("ckoe", &self.ckoe())
            .field("odtoe", &self.odtoe())
            .field("rstoe", &self.rstoe())
            .field("ckeoe", &self.ckeoe())
            .finish()
    }
}
impl W {
    ///Bit 0 - PUREN
    #[inline(always)]
    pub fn puren(&mut self) -> PUREN_W<'_, DSGCRrs> {
        PUREN_W::new(self, 0)
    }
    ///Bit 1 - BDISEN
    #[inline(always)]
    pub fn bdisen(&mut self) -> BDISEN_W<'_, DSGCRrs> {
        BDISEN_W::new(self, 1)
    }
    ///Bit 2 - ZUEN
    #[inline(always)]
    pub fn zuen(&mut self) -> ZUEN_W<'_, DSGCRrs> {
        ZUEN_W::new(self, 2)
    }
    ///Bit 3 - LPIOPD
    #[inline(always)]
    pub fn lpiopd(&mut self) -> LPIOPD_W<'_, DSGCRrs> {
        LPIOPD_W::new(self, 3)
    }
    ///Bit 4 - LPDLLPD
    #[inline(always)]
    pub fn lpdllpd(&mut self) -> LPDLLPD_W<'_, DSGCRrs> {
        LPDLLPD_W::new(self, 4)
    }
    ///Bits 5:7 - DQSGX
    #[inline(always)]
    pub fn dqsgx(&mut self) -> DQSGX_W<'_, DSGCRrs> {
        DQSGX_W::new(self, 5)
    }
    ///Bits 8:10 - DQSGE
    #[inline(always)]
    pub fn dqsge(&mut self) -> DQSGE_W<'_, DSGCRrs> {
        DQSGE_W::new(self, 8)
    }
    ///Bit 11 - NOBUB
    #[inline(always)]
    pub fn nobub(&mut self) -> NOBUB_W<'_, DSGCRrs> {
        NOBUB_W::new(self, 11)
    }
    ///Bit 12 - FXDLAT
    #[inline(always)]
    pub fn fxdlat(&mut self) -> FXDLAT_W<'_, DSGCRrs> {
        FXDLAT_W::new(self, 12)
    }
    ///Bit 16 - CKEPDD
    #[inline(always)]
    pub fn ckepdd(&mut self) -> CKEPDD_W<'_, DSGCRrs> {
        CKEPDD_W::new(self, 16)
    }
    ///Bit 20 - ODTPDD
    #[inline(always)]
    pub fn odtpdd(&mut self) -> ODTPDD_W<'_, DSGCRrs> {
        ODTPDD_W::new(self, 20)
    }
    ///Bit 24 - NL2PD
    #[inline(always)]
    pub fn nl2pd(&mut self) -> NL2PD_W<'_, DSGCRrs> {
        NL2PD_W::new(self, 24)
    }
    ///Bit 25 - NL2OE
    #[inline(always)]
    pub fn nl2oe(&mut self) -> NL2OE_W<'_, DSGCRrs> {
        NL2OE_W::new(self, 25)
    }
    ///Bit 26 - TPDPD
    #[inline(always)]
    pub fn tpdpd(&mut self) -> TPDPD_W<'_, DSGCRrs> {
        TPDPD_W::new(self, 26)
    }
    ///Bit 27 - TPDOE
    #[inline(always)]
    pub fn tpdoe(&mut self) -> TPDOE_W<'_, DSGCRrs> {
        TPDOE_W::new(self, 27)
    }
    ///Bit 28 - CKOE
    #[inline(always)]
    pub fn ckoe(&mut self) -> CKOE_W<'_, DSGCRrs> {
        CKOE_W::new(self, 28)
    }
    ///Bit 29 - ODTOE
    #[inline(always)]
    pub fn odtoe(&mut self) -> ODTOE_W<'_, DSGCRrs> {
        ODTOE_W::new(self, 29)
    }
    ///Bit 30 - RSTOE
    #[inline(always)]
    pub fn rstoe(&mut self) -> RSTOE_W<'_, DSGCRrs> {
        RSTOE_W::new(self, 30)
    }
    ///Bit 31 - CKEOE
    #[inline(always)]
    pub fn ckeoe(&mut self) -> CKEOE_W<'_, DSGCRrs> {
        CKEOE_W::new(self, 31)
    }
}
/**DDRPHYC DSGC register

You can [`read`](crate::Reg::read) this register and get [`dsgcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsgcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DDRPHYC:DSGCR)*/
pub struct DSGCRrs;
impl crate::RegisterSpec for DSGCRrs {
    type Ux = u32;
}
///`read()` method returns [`dsgcr::R`](R) reader structure
impl crate::Readable for DSGCRrs {}
///`write(|w| ..)` method takes [`dsgcr::W`](W) writer structure
impl crate::Writable for DSGCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DSGCR to value 0xfa00_001f
impl crate::Resettable for DSGCRrs {
    const RESET_VALUE: u32 = 0xfa00_001f;
}
