#[doc = "Register `SDMMC_DCTRL` reader"]
pub type R = crate::R<SDMMC_DCTRLrs>;
#[doc = "Register `SDMMC_DCTRL` writer"]
pub type W = crate::W<SDMMC_DCTRLrs>;
#[doc = "Field `DTEN` reader - DTEN"]
pub type DTEN_R = crate::BitReader;
#[doc = "Field `DTEN` writer - DTEN"]
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDIR` reader - DTDIR"]
pub type DTDIR_R = crate::BitReader;
#[doc = "Field `DTDIR` writer - DTDIR"]
pub type DTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTMODE` reader - DTMODE"]
pub type DTMODE_R = crate::FieldReader;
#[doc = "Field `DTMODE` writer - DTMODE"]
pub type DTMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBLOCKSIZE` reader - DBLOCKSIZE"]
pub type DBLOCKSIZE_R = crate::FieldReader;
#[doc = "Field `DBLOCKSIZE` writer - DBLOCKSIZE"]
pub type DBLOCKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWSTART` reader - RWSTART"]
pub type RWSTART_R = crate::BitReader;
#[doc = "Field `RWSTART` writer - RWSTART"]
pub type RWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWSTOP` reader - RWSTOP"]
pub type RWSTOP_R = crate::BitReader;
#[doc = "Field `RWSTOP` writer - RWSTOP"]
pub type RWSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWMOD` reader - RWMOD"]
pub type RWMOD_R = crate::BitReader;
#[doc = "Field `RWMOD` writer - RWMOD"]
pub type RWMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOEN` reader - SDIOEN"]
pub type SDIOEN_R = crate::BitReader;
#[doc = "Field `SDIOEN` writer - SDIOEN"]
pub type SDIOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACKEN` reader - BOOTACKEN"]
pub type BOOTACKEN_R = crate::BitReader;
#[doc = "Field `BOOTACKEN` writer - BOOTACKEN"]
pub type BOOTACKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFORST` reader - FIFORST"]
pub type FIFORST_R = crate::BitReader;
#[doc = "Field `FIFORST` writer - FIFORST"]
pub type FIFORST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    pub fn dten(&self) -> DTEN_R {
        DTEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    pub fn dtdir(&self) -> DTDIR_R {
        DTDIR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - RWSTART"]
    #[inline(always)]
    pub fn rwstart(&self) -> RWSTART_R {
        RWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - RWSTOP"]
    #[inline(always)]
    pub fn rwstop(&self) -> RWSTOP_R {
        RWSTOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    pub fn rwmod(&self) -> RWMOD_R {
        RWMOD_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    pub fn sdioen(&self) -> SDIOEN_R {
        SDIOEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BOOTACKEN"]
    #[inline(always)]
    pub fn bootacken(&self) -> BOOTACKEN_R {
        BOOTACKEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FIFORST"]
    #[inline(always)]
    pub fn fiforst(&self) -> FIFORST_R {
        FIFORST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<SDMMC_DCTRLrs> {
        DTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<SDMMC_DCTRLrs> {
        DTDIR_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - DTMODE"]
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<SDMMC_DCTRLrs> {
        DTMODE_W::new(self, 2)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<SDMMC_DCTRLrs> {
        DBLOCKSIZE_W::new(self, 4)
    }
    #[doc = "Bit 8 - RWSTART"]
    #[inline(always)]
    #[must_use]
    pub fn rwstart(&mut self) -> RWSTART_W<SDMMC_DCTRLrs> {
        RWSTART_W::new(self, 8)
    }
    #[doc = "Bit 9 - RWSTOP"]
    #[inline(always)]
    #[must_use]
    pub fn rwstop(&mut self) -> RWSTOP_W<SDMMC_DCTRLrs> {
        RWSTOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<SDMMC_DCTRLrs> {
        RWMOD_W::new(self, 10)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<SDMMC_DCTRLrs> {
        SDIOEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - BOOTACKEN"]
    #[inline(always)]
    #[must_use]
    pub fn bootacken(&mut self) -> BOOTACKEN_W<SDMMC_DCTRLrs> {
        BOOTACKEN_W::new(self, 12)
    }
    #[doc = "Bit 13 - FIFORST"]
    #[inline(always)]
    #[must_use]
    pub fn fiforst(&mut self) -> FIFORST_W<SDMMC_DCTRLrs> {
        FIFORST_W::new(self, 13)
    }
}
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmmc_dctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmmc_dctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDMMC_DCTRLrs;
impl crate::RegisterSpec for SDMMC_DCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmmc_dctrl::R`](R) reader structure"]
impl crate::Readable for SDMMC_DCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`sdmmc_dctrl::W`](W) writer structure"]
impl crate::Writable for SDMMC_DCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDMMC_DCTRL to value 0"]
impl crate::Resettable for SDMMC_DCTRLrs {
    const RESET_VALUE: u32 = 0;
}
