#[doc = "Register `FMC_PCR` reader"]
pub type R = crate::R<FMC_PCRrs>;
#[doc = "Register `FMC_PCR` writer"]
pub type W = crate::W<FMC_PCRrs>;
#[doc = "Field `PWAITEN` reader - PWAITEN"]
pub type PWAITEN_R = crate::BitReader;
#[doc = "Field `PWAITEN` writer - PWAITEN"]
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBKEN` reader - PBKEN"]
pub type PBKEN_R = crate::BitReader;
#[doc = "Field `PBKEN` writer - PBKEN"]
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWID` reader - PWID"]
pub type PWID_R = crate::FieldReader;
#[doc = "Field `PWID` writer - PWID"]
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECCEN` reader - ECCEN"]
pub type ECCEN_R = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECCEN"]
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCALG` reader - ECCALG"]
pub type ECCALG_R = crate::BitReader;
#[doc = "Field `ECCALG` writer - ECCALG"]
pub type ECCALG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLR` reader - TCLR"]
pub type TCLR_R = crate::FieldReader;
#[doc = "Field `TCLR` writer - TCLR"]
pub type TCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TAR` reader - TAR"]
pub type TAR_R = crate::FieldReader;
#[doc = "Field `TAR` writer - TAR"]
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECCSS` reader - ECCSS"]
pub type ECCSS_R = crate::FieldReader;
#[doc = "Field `ECCSS` writer - ECCSS"]
pub type ECCSS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TCEH` reader - TCEH"]
pub type TCEH_R = crate::FieldReader;
#[doc = "Field `TCEH` writer - TCEH"]
pub type TCEH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `BCHECC` reader - BCHECC"]
pub type BCHECC_R = crate::BitReader;
#[doc = "Field `BCHECC` writer - BCHECC"]
pub type BCHECC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEN` reader - WEN"]
pub type WEN_R = crate::BitReader;
#[doc = "Field `WEN` writer - WEN"]
pub type WEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    pub fn pwaiten(&self) -> PWAITEN_R {
        PWAITEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    pub fn pbken(&self) -> PBKEN_R {
        PBKEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    pub fn pwid(&self) -> PWID_R {
        PWID_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    pub fn eccen(&self) -> ECCEN_R {
        ECCEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - ECCALG"]
    #[inline(always)]
    pub fn eccalg(&self) -> ECCALG_R {
        ECCALG_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    pub fn tclr(&self) -> TCLR_R {
        TCLR_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    pub fn tar(&self) -> TAR_R {
        TAR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bits 17:19 - ECCSS"]
    #[inline(always)]
    pub fn eccss(&self) -> ECCSS_R {
        ECCSS_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:23 - TCEH"]
    #[inline(always)]
    pub fn tceh(&self) -> TCEH_R {
        TCEH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - BCHECC"]
    #[inline(always)]
    pub fn bchecc(&self) -> BCHECC_R {
        BCHECC_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - WEN"]
    #[inline(always)]
    pub fn wen(&self) -> WEN_R {
        WEN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    #[must_use]
    pub fn pwaiten(&mut self) -> PWAITEN_W<FMC_PCRrs> {
        PWAITEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    #[must_use]
    pub fn pbken(&mut self) -> PBKEN_W<FMC_PCRrs> {
        PBKEN_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    #[must_use]
    pub fn pwid(&mut self) -> PWID_W<FMC_PCRrs> {
        PWID_W::new(self, 4)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<FMC_PCRrs> {
        ECCEN_W::new(self, 6)
    }
    #[doc = "Bit 8 - ECCALG"]
    #[inline(always)]
    #[must_use]
    pub fn eccalg(&mut self) -> ECCALG_W<FMC_PCRrs> {
        ECCALG_W::new(self, 8)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<FMC_PCRrs> {
        TCLR_W::new(self, 9)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<FMC_PCRrs> {
        TAR_W::new(self, 13)
    }
    #[doc = "Bits 17:19 - ECCSS"]
    #[inline(always)]
    #[must_use]
    pub fn eccss(&mut self) -> ECCSS_W<FMC_PCRrs> {
        ECCSS_W::new(self, 17)
    }
    #[doc = "Bits 20:23 - TCEH"]
    #[inline(always)]
    #[must_use]
    pub fn tceh(&mut self) -> TCEH_W<FMC_PCRrs> {
        TCEH_W::new(self, 20)
    }
    #[doc = "Bit 24 - BCHECC"]
    #[inline(always)]
    #[must_use]
    pub fn bchecc(&mut self) -> BCHECC_W<FMC_PCRrs> {
        BCHECC_W::new(self, 24)
    }
    #[doc = "Bit 25 - WEN"]
    #[inline(always)]
    #[must_use]
    pub fn wen(&mut self) -> WEN_W<FMC_PCRrs> {
        WEN_W::new(self, 25)
    }
}
#[doc = "NAND Flash Programmable control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmc_pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_PCRrs;
impl crate::RegisterSpec for FMC_PCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_pcr::R`](R) reader structure"]
impl crate::Readable for FMC_PCRrs {}
#[doc = "`write(|w| ..)` method takes [`fmc_pcr::W`](W) writer structure"]
impl crate::Writable for FMC_PCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FMC_PCR to value 0x0007_fe08"]
impl crate::Resettable for FMC_PCRrs {
    const RESET_VALUE: u32 = 0x0007_fe08;
}
