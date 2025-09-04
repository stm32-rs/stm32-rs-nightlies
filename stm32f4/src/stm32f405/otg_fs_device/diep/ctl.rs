///Register `CTL` reader
pub type R = crate::R<CTLrs>;
///Register `CTL` writer
pub type W = crate::W<CTLrs>;
///Field `MPSIZ` reader - MPSIZ
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - MPSIZ
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `USBAEP` reader - USBAEP
pub type USBAEP_R = crate::BitReader;
///Field `USBAEP` writer - USBAEP
pub type USBAEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EONUM_DPID` reader - EONUM/DPID
pub type EONUM_DPID_R = crate::BitReader;
///Field `NAKSTS` reader - NAKSTS
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYP` reader - EPTYP
pub type EPTYP_R = crate::FieldReader;
///Field `EPTYP` writer - EPTYP
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `STALL` reader - STALL handshake
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL handshake
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFNUM` reader - TXFNUM
pub type TXFNUM_R = crate::FieldReader;
///Field `TXFNUM` writer - TXFNUM
pub type TXFNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `CNAK` writer - CNAK
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - SNAK
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD0PID_SEVNFRM` writer - SD0PID/SEVNFRM
pub type SD0PID_SEVNFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SODDFRM_SD1PID` writer - SODDFRM/SD1PID
pub type SODDFRM_SD1PID_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - EPDIS
pub type EPDIS_R = crate::BitReader;
///Field `EPDIS` writer - EPDIS
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPENA` reader - EPENA
pub type EPENA_R = crate::BitReader;
///Field `EPENA` writer - EPENA
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - EONUM/DPID
    #[inline(always)]
    pub fn eonum_dpid(&self) -> EONUM_DPID_R {
        EONUM_DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NAKSTS
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bits 22:25 - TXFNUM
    #[inline(always)]
    pub fn txfnum(&self) -> TXFNUM_R {
        TXFNUM_R::new(((self.bits >> 22) & 0x0f) as u8)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTL")
            .field("epena", &self.epena())
            .field("epdis", &self.epdis())
            .field("txfnum", &self.txfnum())
            .field("stall", &self.stall())
            .field("eptyp", &self.eptyp())
            .field("naksts", &self.naksts())
            .field("eonum_dpid", &self.eonum_dpid())
            .field("usbaep", &self.usbaep())
            .field("mpsiz", &self.mpsiz())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - MPSIZ
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<CTLrs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W<CTLrs> {
        USBAEP_W::new(self, 15)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<CTLrs> {
        EPTYP_W::new(self, 18)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<CTLrs> {
        STALL_W::new(self, 21)
    }
    ///Bits 22:25 - TXFNUM
    #[inline(always)]
    pub fn txfnum(&mut self) -> TXFNUM_W<CTLrs> {
        TXFNUM_W::new(self, 22)
    }
    ///Bit 26 - CNAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<CTLrs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - SNAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<CTLrs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 28 - SD0PID/SEVNFRM
    #[inline(always)]
    pub fn sd0pid_sevnfrm(&mut self) -> SD0PID_SEVNFRM_W<CTLrs> {
        SD0PID_SEVNFRM_W::new(self, 28)
    }
    ///Bit 29 - SODDFRM/SD1PID
    #[inline(always)]
    pub fn soddfrm_sd1pid(&mut self) -> SODDFRM_SD1PID_W<CTLrs> {
        SODDFRM_SD1PID_W::new(self, 29)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<CTLrs> {
        EPDIS_W::new(self, 30)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<CTLrs> {
        EPENA_W::new(self, 31)
    }
}
/**OTG device endpoint-1 control register

You can [`read`](crate::Reg::read) this register and get [`ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CTLrs;
impl crate::RegisterSpec for CTLrs {
    type Ux = u32;
}
///`read()` method returns [`ctl::R`](R) reader structure
impl crate::Readable for CTLrs {}
///`write(|w| ..)` method takes [`ctl::W`](W) writer structure
impl crate::Writable for CTLrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CTL to value 0
impl crate::Resettable for CTLrs {}
