///Register `OTG_HS_DOEPCTL2` reader
pub type R = crate::R<OTG_HS_DOEPCTL2rs>;
///Register `OTG_HS_DOEPCTL2` writer
pub type W = crate::W<OTG_HS_DOEPCTL2rs>;
///Field `MPSIZ` reader - Maximum packet size
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - Maximum packet size
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `USBAEP` reader - USB active endpoint
pub type USBAEP_R = crate::BitReader;
///Field `USBAEP` writer - USB active endpoint
pub type USBAEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EONUM_DPID` reader - Even odd frame/Endpoint data PID
pub type EONUM_DPID_R = crate::BitReader;
///Field `NAKSTS` reader - NAK status
pub type NAKSTS_R = crate::BitReader;
///Field `EPTYP` reader - Endpoint type
pub type EPTYP_R = crate::FieldReader;
///Field `EPTYP` writer - Endpoint type
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `SNPM` reader - Snoop mode
pub type SNPM_R = crate::BitReader;
///Field `SNPM` writer - Snoop mode
pub type SNPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `Stall` reader - STALL handshake
pub type STALL_R = crate::BitReader;
///Field `Stall` writer - STALL handshake
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNAK` writer - Clear NAK
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - Set NAK
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD0PID_SEVNFRM` writer - Set DATA0 PID/Set even frame
pub type SD0PID_SEVNFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SODDFRM` writer - Set odd frame
pub type SODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPDIS` reader - Endpoint disable
pub type EPDIS_R = crate::BitReader;
///Field `EPDIS` writer - Endpoint disable
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPENA` reader - Endpoint enable
pub type EPENA_R = crate::BitReader;
///Field `EPENA` writer - Endpoint enable
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 15 - USB active endpoint
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Even odd frame/Endpoint data PID
    #[inline(always)]
    pub fn eonum_dpid(&self) -> EONUM_DPID_R {
        EONUM_DPID_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - NAK status
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bit 20 - Snoop mode
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 30 - Endpoint disable
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Endpoint enable
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTG_HS_DOEPCTL2")
            .field("mpsiz", &self.mpsiz())
            .field("usbaep", &self.usbaep())
            .field("eonum_dpid", &self.eonum_dpid())
            .field("naksts", &self.naksts())
            .field("eptyp", &self.eptyp())
            .field("snpm", &self.snpm())
            .field("stall", &self.stall())
            .field("epdis", &self.epdis())
            .field("epena", &self.epena())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Maximum packet size
    #[inline(always)]
    pub fn mpsiz(&mut self) -> MPSIZ_W<'_, OTG_HS_DOEPCTL2rs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bit 15 - USB active endpoint
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W<'_, OTG_HS_DOEPCTL2rs> {
        USBAEP_W::new(self, 15)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<'_, OTG_HS_DOEPCTL2rs> {
        EPTYP_W::new(self, 18)
    }
    ///Bit 20 - Snoop mode
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W<'_, OTG_HS_DOEPCTL2rs> {
        SNPM_W::new(self, 20)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, OTG_HS_DOEPCTL2rs> {
        STALL_W::new(self, 21)
    }
    ///Bit 26 - Clear NAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<'_, OTG_HS_DOEPCTL2rs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - Set NAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<'_, OTG_HS_DOEPCTL2rs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 28 - Set DATA0 PID/Set even frame
    #[inline(always)]
    pub fn sd0pid_sevnfrm(&mut self) -> SD0PID_SEVNFRM_W<'_, OTG_HS_DOEPCTL2rs> {
        SD0PID_SEVNFRM_W::new(self, 28)
    }
    ///Bit 29 - Set odd frame
    #[inline(always)]
    pub fn soddfrm(&mut self) -> SODDFRM_W<'_, OTG_HS_DOEPCTL2rs> {
        SODDFRM_W::new(self, 29)
    }
    ///Bit 30 - Endpoint disable
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<'_, OTG_HS_DOEPCTL2rs> {
        EPDIS_W::new(self, 30)
    }
    ///Bit 31 - Endpoint enable
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<'_, OTG_HS_DOEPCTL2rs> {
        EPENA_W::new(self, 31)
    }
}
/**OTG device endpoint-2 control register

You can [`read`](crate::Reg::read) this register and get [`otg_hs_doepctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_doepctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_HS_DEVICE:OTG_HS_DOEPCTL2)*/
pub struct OTG_HS_DOEPCTL2rs;
impl crate::RegisterSpec for OTG_HS_DOEPCTL2rs {
    type Ux = u32;
}
///`read()` method returns [`otg_hs_doepctl2::R`](R) reader structure
impl crate::Readable for OTG_HS_DOEPCTL2rs {}
///`write(|w| ..)` method takes [`otg_hs_doepctl2::W`](W) writer structure
impl crate::Writable for OTG_HS_DOEPCTL2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTG_HS_DOEPCTL2 to value 0
impl crate::Resettable for OTG_HS_DOEPCTL2rs {}
