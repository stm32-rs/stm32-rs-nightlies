#[doc = "Register `BDMUPDR` reader"]
pub type R = crate::R<BDMUPDRrs>;
#[doc = "Register `BDMUPDR` writer"]
pub type W = crate::W<BDMUPDRrs>;
#[doc = "Field `MCR` reader - MCR"]
pub type MCR_R = crate::BitReader;
#[doc = "Field `MCR` writer - MCR"]
pub type MCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MICR` reader - MICR"]
pub type MICR_R = crate::BitReader;
#[doc = "Field `MICR` writer - MICR"]
pub type MICR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIER` reader - MDIER"]
pub type MDIER_R = crate::BitReader;
#[doc = "Field `MDIER` writer - MDIER"]
pub type MDIER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCNT` reader - MCNT"]
pub type MCNT_R = crate::BitReader;
#[doc = "Field `MCNT` writer - MCNT"]
pub type MCNT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPER` reader - MPER"]
pub type MPER_R = crate::BitReader;
#[doc = "Field `MPER` writer - MPER"]
pub type MPER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MREP` reader - MREP"]
pub type MREP_R = crate::BitReader;
#[doc = "Field `MREP` writer - MREP"]
pub type MREP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP1` reader - MCMP1"]
pub type MCMP1_R = crate::BitReader;
#[doc = "Field `MCMP1` writer - MCMP1"]
pub type MCMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP2` reader - MCMP2"]
pub type MCMP2_R = crate::BitReader;
#[doc = "Field `MCMP2` writer - MCMP2"]
pub type MCMP2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP3` reader - MCMP3"]
pub type MCMP3_R = crate::BitReader;
#[doc = "Field `MCMP3` writer - MCMP3"]
pub type MCMP3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCMP4` reader - MCMP4"]
pub type MCMP4_R = crate::BitReader;
#[doc = "Field `MCMP4` writer - MCMP4"]
pub type MCMP4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    pub fn mcr(&self) -> MCR_R {
        MCR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    pub fn micr(&self) -> MICR_R {
        MICR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    pub fn mdier(&self) -> MDIER_R {
        MDIER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    pub fn mper(&self) -> MPER_R {
        MPER_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    pub fn mrep(&self) -> MREP_R {
        MREP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    pub fn mcmp1(&self) -> MCMP1_R {
        MCMP1_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    pub fn mcmp2(&self) -> MCMP2_R {
        MCMP2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    pub fn mcmp3(&self) -> MCMP3_R {
        MCMP3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    pub fn mcmp4(&self) -> MCMP4_R {
        MCMP4_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MCR"]
    #[inline(always)]
    #[must_use]
    pub fn mcr(&mut self) -> MCR_W<BDMUPDRrs> {
        MCR_W::new(self, 0)
    }
    #[doc = "Bit 1 - MICR"]
    #[inline(always)]
    #[must_use]
    pub fn micr(&mut self) -> MICR_W<BDMUPDRrs> {
        MICR_W::new(self, 1)
    }
    #[doc = "Bit 2 - MDIER"]
    #[inline(always)]
    #[must_use]
    pub fn mdier(&mut self) -> MDIER_W<BDMUPDRrs> {
        MDIER_W::new(self, 2)
    }
    #[doc = "Bit 3 - MCNT"]
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> MCNT_W<BDMUPDRrs> {
        MCNT_W::new(self, 3)
    }
    #[doc = "Bit 4 - MPER"]
    #[inline(always)]
    #[must_use]
    pub fn mper(&mut self) -> MPER_W<BDMUPDRrs> {
        MPER_W::new(self, 4)
    }
    #[doc = "Bit 5 - MREP"]
    #[inline(always)]
    #[must_use]
    pub fn mrep(&mut self) -> MREP_W<BDMUPDRrs> {
        MREP_W::new(self, 5)
    }
    #[doc = "Bit 6 - MCMP1"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp1(&mut self) -> MCMP1_W<BDMUPDRrs> {
        MCMP1_W::new(self, 6)
    }
    #[doc = "Bit 7 - MCMP2"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp2(&mut self) -> MCMP2_W<BDMUPDRrs> {
        MCMP2_W::new(self, 7)
    }
    #[doc = "Bit 8 - MCMP3"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp3(&mut self) -> MCMP3_W<BDMUPDRrs> {
        MCMP3_W::new(self, 8)
    }
    #[doc = "Bit 9 - MCMP4"]
    #[inline(always)]
    #[must_use]
    pub fn mcmp4(&mut self) -> MCMP4_W<BDMUPDRrs> {
        MCMP4_W::new(self, 9)
    }
}
#[doc = "BDMUPDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bdmupdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bdmupdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BDMUPDRrs;
impl crate::RegisterSpec for BDMUPDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bdmupdr::R`](R) reader structure"]
impl crate::Readable for BDMUPDRrs {}
#[doc = "`write(|w| ..)` method takes [`bdmupdr::W`](W) writer structure"]
impl crate::Writable for BDMUPDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BDMUPDR to value 0"]
impl crate::Resettable for BDMUPDRrs {
    const RESET_VALUE: u32 = 0;
}
