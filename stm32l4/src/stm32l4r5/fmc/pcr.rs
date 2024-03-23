#[doc = "Register `PCR` reader"]
pub type R = crate::R<PCRrs>;
#[doc = "Register `PCR` writer"]
pub type W = crate::W<PCRrs>;
#[doc = "Field `PWAITEN` reader - PWAITEN"]
pub type PWAITEN_R = crate::BitReader;
#[doc = "Field `PWAITEN` writer - PWAITEN"]
pub type PWAITEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBKEN` reader - PBKEN"]
pub type PBKEN_R = crate::BitReader;
#[doc = "Field `PBKEN` writer - PBKEN"]
pub type PBKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PTYP` reader - PTYP"]
pub type PTYP_R = crate::BitReader;
#[doc = "Field `PTYP` writer - PTYP"]
pub type PTYP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWID` reader - PWID"]
pub type PWID_R = crate::FieldReader;
#[doc = "Field `PWID` writer - PWID"]
pub type PWID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECCEN` reader - ECCEN"]
pub type ECCEN_R = crate::BitReader;
#[doc = "Field `ECCEN` writer - ECCEN"]
pub type ECCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCLR` reader - TCLR"]
pub type TCLR_R = crate::FieldReader;
#[doc = "Field `TCLR` writer - TCLR"]
pub type TCLR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TAR` reader - TAR"]
pub type TAR_R = crate::FieldReader;
#[doc = "Field `TAR` writer - TAR"]
pub type TAR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ECCPS` reader - ECCPS"]
pub type ECCPS_R = crate::FieldReader;
#[doc = "Field `ECCPS` writer - ECCPS"]
pub type ECCPS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
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
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    pub fn ptyp(&self) -> PTYP_R {
        PTYP_R::new(((self.bits >> 3) & 1) != 0)
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
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    pub fn eccps(&self) -> ECCPS_R {
        ECCPS_R::new(((self.bits >> 17) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - PWAITEN"]
    #[inline(always)]
    #[must_use]
    pub fn pwaiten(&mut self) -> PWAITEN_W<PCRrs> {
        PWAITEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - PBKEN"]
    #[inline(always)]
    #[must_use]
    pub fn pbken(&mut self) -> PBKEN_W<PCRrs> {
        PBKEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - PTYP"]
    #[inline(always)]
    #[must_use]
    pub fn ptyp(&mut self) -> PTYP_W<PCRrs> {
        PTYP_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - PWID"]
    #[inline(always)]
    #[must_use]
    pub fn pwid(&mut self) -> PWID_W<PCRrs> {
        PWID_W::new(self, 4)
    }
    #[doc = "Bit 6 - ECCEN"]
    #[inline(always)]
    #[must_use]
    pub fn eccen(&mut self) -> ECCEN_W<PCRrs> {
        ECCEN_W::new(self, 6)
    }
    #[doc = "Bits 9:12 - TCLR"]
    #[inline(always)]
    #[must_use]
    pub fn tclr(&mut self) -> TCLR_W<PCRrs> {
        TCLR_W::new(self, 9)
    }
    #[doc = "Bits 13:16 - TAR"]
    #[inline(always)]
    #[must_use]
    pub fn tar(&mut self) -> TAR_W<PCRrs> {
        TAR_W::new(self, 13)
    }
    #[doc = "Bits 17:19 - ECCPS"]
    #[inline(always)]
    #[must_use]
    pub fn eccps(&mut self) -> ECCPS_W<PCRrs> {
        ECCPS_W::new(self, 17)
    }
}
#[doc = "PC Card/NAND Flash control register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCRrs;
impl crate::RegisterSpec for PCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcr::R`](R) reader structure"]
impl crate::Readable for PCRrs {}
#[doc = "`write(|w| ..)` method takes [`pcr::W`](W) writer structure"]
impl crate::Writable for PCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCR to value 0x18"]
impl crate::Resettable for PCRrs {
    const RESET_VALUE: u32 = 0x18;
}
