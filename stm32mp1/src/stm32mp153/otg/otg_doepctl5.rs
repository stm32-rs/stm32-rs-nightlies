#[doc = "Register `OTG_DOEPCTL5` reader"]
pub type R = crate::R<OTG_DOEPCTL5rs>;
#[doc = "Register `OTG_DOEPCTL5` writer"]
pub type W = crate::W<OTG_DOEPCTL5rs>;
#[doc = "Field `MPSIZ` reader - MPSIZ"]
pub type MPSIZ_R = crate::FieldReader<u16>;
#[doc = "Field `MPSIZ` writer - MPSIZ"]
pub type MPSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `USBAEP` reader - USBAEP"]
pub type USBAEP_R = crate::BitReader;
#[doc = "Field `USBAEP` writer - USBAEP"]
pub type USBAEP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EONUM_DPIP` reader - EONUM_DPIP"]
pub type EONUM_DPIP_R = crate::BitReader;
#[doc = "Field `NAKSTS` reader - NAKSTS"]
pub type NAKSTS_R = crate::BitReader;
#[doc = "Field `EPTYP` reader - EPTYP"]
pub type EPTYP_R = crate::FieldReader;
#[doc = "Field `EPTYP` writer - EPTYP"]
pub type EPTYP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SNPM` reader - SNPM"]
pub type SNPM_R = crate::BitReader;
#[doc = "Field `SNPM` writer - SNPM"]
pub type SNPM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL"]
pub type STALL_R = crate::BitReader;
#[doc = "Field `STALL` writer - STALL"]
pub type STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNAK` writer - CNAK"]
pub type CNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAK` writer - SNAK"]
pub type SNAK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD0PID_SEVNFRM` writer - SD0PID_SEVNFRM"]
pub type SD0PID_SEVNFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SD1PID_SODDFRM` writer - SD1PID_SODDFRM"]
pub type SD1PID_SODDFRM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPDIS` reader - EPDIS"]
pub type EPDIS_R = crate::BitReader;
#[doc = "Field `EPDIS` writer - EPDIS"]
pub type EPDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPENA` reader - EPENA"]
pub type EPENA_R = crate::BitReader;
#[doc = "Field `EPENA` writer - EPENA"]
pub type EPENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - MPSIZ"]
    #[inline(always)]
    pub fn mpsiz(&self) -> MPSIZ_R {
        MPSIZ_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - USBAEP"]
    #[inline(always)]
    pub fn usbaep(&self) -> USBAEP_R {
        USBAEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - EONUM_DPIP"]
    #[inline(always)]
    pub fn eonum_dpip(&self) -> EONUM_DPIP_R {
        EONUM_DPIP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NAKSTS"]
    #[inline(always)]
    pub fn naksts(&self) -> NAKSTS_R {
        NAKSTS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    pub fn eptyp(&self) -> EPTYP_R {
        EPTYP_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    pub fn snpm(&self) -> SNPM_R {
        SNPM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - STALL"]
    #[inline(always)]
    pub fn stall(&self) -> STALL_R {
        STALL_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 30 - EPDIS"]
    #[inline(always)]
    pub fn epdis(&self) -> EPDIS_R {
        EPDIS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - EPENA"]
    #[inline(always)]
    pub fn epena(&self) -> EPENA_R {
        EPENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - MPSIZ"]
    #[inline(always)]
    #[must_use]
    pub fn mpsiz(&mut self) -> MPSIZ_W<OTG_DOEPCTL5rs> {
        MPSIZ_W::new(self, 0)
    }
    #[doc = "Bit 15 - USBAEP"]
    #[inline(always)]
    #[must_use]
    pub fn usbaep(&mut self) -> USBAEP_W<OTG_DOEPCTL5rs> {
        USBAEP_W::new(self, 15)
    }
    #[doc = "Bits 18:19 - EPTYP"]
    #[inline(always)]
    #[must_use]
    pub fn eptyp(&mut self) -> EPTYP_W<OTG_DOEPCTL5rs> {
        EPTYP_W::new(self, 18)
    }
    #[doc = "Bit 20 - SNPM"]
    #[inline(always)]
    #[must_use]
    pub fn snpm(&mut self) -> SNPM_W<OTG_DOEPCTL5rs> {
        SNPM_W::new(self, 20)
    }
    #[doc = "Bit 21 - STALL"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self) -> STALL_W<OTG_DOEPCTL5rs> {
        STALL_W::new(self, 21)
    }
    #[doc = "Bit 26 - CNAK"]
    #[inline(always)]
    #[must_use]
    pub fn cnak(&mut self) -> CNAK_W<OTG_DOEPCTL5rs> {
        CNAK_W::new(self, 26)
    }
    #[doc = "Bit 27 - SNAK"]
    #[inline(always)]
    #[must_use]
    pub fn snak(&mut self) -> SNAK_W<OTG_DOEPCTL5rs> {
        SNAK_W::new(self, 27)
    }
    #[doc = "Bit 28 - SD0PID_SEVNFRM"]
    #[inline(always)]
    #[must_use]
    pub fn sd0pid_sevnfrm(&mut self) -> SD0PID_SEVNFRM_W<OTG_DOEPCTL5rs> {
        SD0PID_SEVNFRM_W::new(self, 28)
    }
    #[doc = "Bit 29 - SD1PID_SODDFRM"]
    #[inline(always)]
    #[must_use]
    pub fn sd1pid_soddfrm(&mut self) -> SD1PID_SODDFRM_W<OTG_DOEPCTL5rs> {
        SD1PID_SODDFRM_W::new(self, 29)
    }
    #[doc = "Bit 30 - EPDIS"]
    #[inline(always)]
    #[must_use]
    pub fn epdis(&mut self) -> EPDIS_W<OTG_DOEPCTL5rs> {
        EPDIS_W::new(self, 30)
    }
    #[doc = "Bit 31 - EPENA"]
    #[inline(always)]
    #[must_use]
    pub fn epena(&mut self) -> EPENA_W<OTG_DOEPCTL5rs> {
        EPENA_W::new(self, 31)
    }
}
#[doc = "The application uses this register to control the behavior of each logical endpoint other than endpoint 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`otg_doepctl5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`otg_doepctl5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OTG_DOEPCTL5rs;
impl crate::RegisterSpec for OTG_DOEPCTL5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_doepctl5::R`](R) reader structure"]
impl crate::Readable for OTG_DOEPCTL5rs {}
#[doc = "`write(|w| ..)` method takes [`otg_doepctl5::W`](W) writer structure"]
impl crate::Writable for OTG_DOEPCTL5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OTG_DOEPCTL5 to value 0"]
impl crate::Resettable for OTG_DOEPCTL5rs {
    const RESET_VALUE: u32 = 0;
}
