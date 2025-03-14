///Register `DOEPCTL2` reader
pub type R = crate::R<DOEPCTL2rs>;
///Register `DOEPCTL2` writer
pub type W = crate::W<DOEPCTL2rs>;
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
///Field `SNPM` reader - SNPM
pub type SNPM_R = crate::BitReader;
///Field `SNPM` writer - SNPM
pub type SNPM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `Stall` reader - Stall
pub type STALL_R = crate::BitReader;
///Field `Stall` writer - Stall
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNAK` writer - CNAK
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - SNAK
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SD0PID_SEVNFRM` writer - SD0PID/SEVNFRM
pub type SD0PID_SEVNFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SODDFRM` writer - SODDFRM
pub type SODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 20 - SNPM
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Stall
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
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
        f.debug_struct("DOEPCTL2")
            .field("epena", &self.epena())
            .field("epdis", &self.epdis())
            .field("stall", &self.stall())
            .field("snpm", &self.snpm())
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
    pub fn mpsiz(&mut self) -> MPSIZ_W<DOEPCTL2rs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bit 15 - USBAEP
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W<DOEPCTL2rs> {
        USBAEP_W::new(self, 15)
    }
    ///Bits 18:19 - EPTYP
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<DOEPCTL2rs> {
        EPTYP_W::new(self, 18)
    }
    ///Bit 20 - SNPM
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W<DOEPCTL2rs> {
        SNPM_W::new(self, 20)
    }
    ///Bit 21 - Stall
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<DOEPCTL2rs> {
        STALL_W::new(self, 21)
    }
    ///Bit 26 - CNAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<DOEPCTL2rs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - SNAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<DOEPCTL2rs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 28 - SD0PID/SEVNFRM
    #[inline(always)]
    pub fn sd0pid_sevnfrm(&mut self) -> SD0PID_SEVNFRM_W<DOEPCTL2rs> {
        SD0PID_SEVNFRM_W::new(self, 28)
    }
    ///Bit 29 - SODDFRM
    #[inline(always)]
    pub fn soddfrm(&mut self) -> SODDFRM_W<DOEPCTL2rs> {
        SODDFRM_W::new(self, 29)
    }
    ///Bit 30 - EPDIS
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<DOEPCTL2rs> {
        EPDIS_W::new(self, 30)
    }
    ///Bit 31 - EPENA
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<DOEPCTL2rs> {
        EPENA_W::new(self, 31)
    }
}
/**device endpoint-2 control register

You can [`read`](crate::Reg::read) this register and get [`doepctl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4P5.html#OTG_FS_DEVICE:DOEPCTL2)*/
pub struct DOEPCTL2rs;
impl crate::RegisterSpec for DOEPCTL2rs {
    type Ux = u32;
}
///`read()` method returns [`doepctl2::R`](R) reader structure
impl crate::Readable for DOEPCTL2rs {}
///`write(|w| ..)` method takes [`doepctl2::W`](W) writer structure
impl crate::Writable for DOEPCTL2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPCTL2 to value 0
impl crate::Resettable for DOEPCTL2rs {}
