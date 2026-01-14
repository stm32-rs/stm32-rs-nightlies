///Register `DOEPCTL3_ALTERNATE` reader
pub type R = crate::R<DOEPCTL3_ALTERNATErs>;
///Register `DOEPCTL3_ALTERNATE` writer
pub type W = crate::W<DOEPCTL3_ALTERNATErs>;
///Field `MPSIZ` reader - Maximum packet size
pub type MPSIZ_R = crate::FieldReader<u16>;
///Field `MPSIZ` writer - Maximum packet size
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `USBAEP` reader - USB active endpoint
pub type USBAEP_R = crate::BitReader;
///Field `USBAEP` writer - USB active endpoint
pub type USBAEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EONUM` reader - Even/odd frame
pub type EONUM_R = crate::BitReader;
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
///Field `STALL` reader - STALL handshake
pub type STALL_R = crate::BitReader;
///Field `STALL` writer - STALL handshake
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CNAK` writer - Clear NAK
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SNAK` writer - Set NAK
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEVNFRM` writer - Set even frame
pub type SEVNFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 16 - Even/odd frame
    #[inline(always)]
    pub fn eonum(&self) -> EONUM_R {
        EONUM_R::new(((self.bits >> 16) & 1) != 0)
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
        f.debug_struct("DOEPCTL3_ALTERNATE")
            .field("mpsiz", &self.mpsiz())
            .field("usbaep", &self.usbaep())
            .field("eonum", &self.eonum())
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
    pub fn mpsiz(&mut self) -> MPSIZ_W<'_, DOEPCTL3_ALTERNATErs> {
        MPSIZ_W::new(self, 0)
    }
    ///Bit 15 - USB active endpoint
    #[inline(always)]
    pub fn usbaep(&mut self) -> USBAEP_W<'_, DOEPCTL3_ALTERNATErs> {
        USBAEP_W::new(self, 15)
    }
    ///Bits 18:19 - Endpoint type
    #[inline(always)]
    pub fn eptyp(&mut self) -> EPTYP_W<'_, DOEPCTL3_ALTERNATErs> {
        EPTYP_W::new(self, 18)
    }
    ///Bit 20 - Snoop mode
    #[inline(always)]
    pub fn snpm(&mut self) -> SNPM_W<'_, DOEPCTL3_ALTERNATErs> {
        SNPM_W::new(self, 20)
    }
    ///Bit 21 - STALL handshake
    #[inline(always)]
    pub fn stall(&mut self) -> STALL_W<'_, DOEPCTL3_ALTERNATErs> {
        STALL_W::new(self, 21)
    }
    ///Bit 26 - Clear NAK
    #[inline(always)]
    pub fn cnak(&mut self) -> CNAK_W<'_, DOEPCTL3_ALTERNATErs> {
        CNAK_W::new(self, 26)
    }
    ///Bit 27 - Set NAK
    #[inline(always)]
    pub fn snak(&mut self) -> SNAK_W<'_, DOEPCTL3_ALTERNATErs> {
        SNAK_W::new(self, 27)
    }
    ///Bit 28 - Set even frame
    #[inline(always)]
    pub fn sevnfrm(&mut self) -> SEVNFRM_W<'_, DOEPCTL3_ALTERNATErs> {
        SEVNFRM_W::new(self, 28)
    }
    ///Bit 29 - Set odd frame
    #[inline(always)]
    pub fn soddfrm(&mut self) -> SODDFRM_W<'_, DOEPCTL3_ALTERNATErs> {
        SODDFRM_W::new(self, 29)
    }
    ///Bit 30 - Endpoint disable
    #[inline(always)]
    pub fn epdis(&mut self) -> EPDIS_W<'_, DOEPCTL3_ALTERNATErs> {
        EPDIS_W::new(self, 30)
    }
    ///Bit 31 - Endpoint enable
    #[inline(always)]
    pub fn epena(&mut self) -> EPENA_W<'_, DOEPCTL3_ALTERNATErs> {
        EPENA_W::new(self, 31)
    }
}
/**OTG device OUT endpoint 3 control register \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`doepctl3_alternate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doepctl3_alternate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#OTG1:DOEPCTL3_ALTERNATE)*/
pub struct DOEPCTL3_ALTERNATErs;
impl crate::RegisterSpec for DOEPCTL3_ALTERNATErs {
    type Ux = u32;
}
///`read()` method returns [`doepctl3_alternate::R`](R) reader structure
impl crate::Readable for DOEPCTL3_ALTERNATErs {}
///`write(|w| ..)` method takes [`doepctl3_alternate::W`](W) writer structure
impl crate::Writable for DOEPCTL3_ALTERNATErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DOEPCTL3_ALTERNATE to value 0
impl crate::Resettable for DOEPCTL3_ALTERNATErs {}
