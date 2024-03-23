#[doc = "Register `DCTRL` reader"]
pub type R = crate::R<DCTRLrs>;
#[doc = "Register `DCTRL` writer"]
pub type W = crate::W<DCTRLrs>;
#[doc = "Field `DTEN` reader - DTEN"]
pub type DTEN_R = crate::BitReader;
#[doc = "Field `DTEN` writer - DTEN"]
pub type DTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDIR` reader - DTDIR"]
pub type DTDIR_R = crate::BitReader;
#[doc = "Field `DTDIR` writer - DTDIR"]
pub type DTDIR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTMODE` reader - DTMODE"]
pub type DTMODE_R = crate::BitReader;
#[doc = "Field `DTMODE` writer - DTMODE"]
pub type DTMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBLOCKSIZE` reader - DBLOCKSIZE"]
pub type DBLOCKSIZE_R = crate::FieldReader;
#[doc = "Field `DBLOCKSIZE` writer - DBLOCKSIZE"]
pub type DBLOCKSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PWSTART` reader - PWSTART"]
pub type PWSTART_R = crate::BitReader;
#[doc = "Field `PWSTART` writer - PWSTART"]
pub type PWSTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PWSTOP` reader - PWSTOP"]
pub type PWSTOP_R = crate::BitReader;
#[doc = "Field `PWSTOP` writer - PWSTOP"]
pub type PWSTOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWMOD` reader - RWMOD"]
pub type RWMOD_R = crate::BitReader;
#[doc = "Field `RWMOD` writer - RWMOD"]
pub type RWMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOEN` reader - SDIOEN"]
pub type SDIOEN_R = crate::BitReader;
#[doc = "Field `SDIOEN` writer - SDIOEN"]
pub type SDIOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    pub fn dtmode(&self) -> DTMODE_R {
        DTMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DBLOCKSIZE_R {
        DBLOCKSIZE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    pub fn pwstart(&self) -> PWSTART_R {
        PWSTART_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    pub fn pwstop(&self) -> PWSTOP_R {
        PWSTOP_R::new(((self.bits >> 9) & 1) != 0)
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
}
impl W {
    #[doc = "Bit 0 - DTEN"]
    #[inline(always)]
    #[must_use]
    pub fn dten(&mut self) -> DTEN_W<DCTRLrs> {
        DTEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DTDIR"]
    #[inline(always)]
    #[must_use]
    pub fn dtdir(&mut self) -> DTDIR_W<DCTRLrs> {
        DTDIR_W::new(self, 1)
    }
    #[doc = "Bit 2 - DTMODE"]
    #[inline(always)]
    #[must_use]
    pub fn dtmode(&mut self) -> DTMODE_W<DCTRLrs> {
        DTMODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<DCTRLrs> {
        DMAEN_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - DBLOCKSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn dblocksize(&mut self) -> DBLOCKSIZE_W<DCTRLrs> {
        DBLOCKSIZE_W::new(self, 4)
    }
    #[doc = "Bit 8 - PWSTART"]
    #[inline(always)]
    #[must_use]
    pub fn pwstart(&mut self) -> PWSTART_W<DCTRLrs> {
        PWSTART_W::new(self, 8)
    }
    #[doc = "Bit 9 - PWSTOP"]
    #[inline(always)]
    #[must_use]
    pub fn pwstop(&mut self) -> PWSTOP_W<DCTRLrs> {
        PWSTOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - RWMOD"]
    #[inline(always)]
    #[must_use]
    pub fn rwmod(&mut self) -> RWMOD_W<DCTRLrs> {
        RWMOD_W::new(self, 10)
    }
    #[doc = "Bit 11 - SDIOEN"]
    #[inline(always)]
    #[must_use]
    pub fn sdioen(&mut self) -> SDIOEN_W<DCTRLrs> {
        SDIOEN_W::new(self, 11)
    }
}
#[doc = "SDIO data control register (SDIO_DCTRL)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCTRLrs;
impl crate::RegisterSpec for DCTRLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctrl::R`](R) reader structure"]
impl crate::Readable for DCTRLrs {}
#[doc = "`write(|w| ..)` method takes [`dctrl::W`](W) writer structure"]
impl crate::Writable for DCTRLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCTRL to value 0"]
impl crate::Resettable for DCTRLrs {
    const RESET_VALUE: u32 = 0;
}
