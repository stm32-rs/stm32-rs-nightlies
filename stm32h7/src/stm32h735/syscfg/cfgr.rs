#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CFGRrs>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CFGRrs>;
#[doc = "Field `PVDL` reader - Programmable voltage detector lockup bit"]
pub type PVDL_R = crate::BitReader;
#[doc = "Field `PVDL` writer - Programmable voltage detector lockup bit"]
pub type PVDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHL` reader - FLASH double error lockup bit"]
pub type FLASHL_R = crate::BitReader;
#[doc = "Field `FLASHL` writer - FLASH double error lockup bit"]
pub type FLASHL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM7L` reader - CPU lockup bit"]
pub type CM7L_R = crate::BitReader;
#[doc = "Field `CM7L` writer - CPU lockup bit"]
pub type CM7L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKRAML` reader - Backup RAM Double error lockup bit"]
pub type BKRAML_R = crate::BitReader;
#[doc = "Field `BKRAML` writer - Backup RAM Double error lockup bit"]
pub type BKRAML_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM4L` reader - SRAM4 Double error lockup bit"]
pub type SRAM4L_R = crate::BitReader;
#[doc = "Field `SRAM4L` writer - SRAM4 Double error lockup bit"]
pub type SRAM4L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2L` reader - SRAM2 Double error lockup bit"]
pub type SRAM2L_R = crate::BitReader;
#[doc = "Field `SRAM2L` writer - SRAM2 Double error lockup bit"]
pub type SRAM2L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1L` reader - SRAM1 Double error lockup bit"]
pub type SRAM1L_R = crate::BitReader;
#[doc = "Field `SRAM1L` writer - SRAM1 Double error lockup bit"]
pub type SRAM1L_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCML` reader - DTCM-RAM Double error lockup bit"]
pub type DTCML_R = crate::BitReader;
#[doc = "Field `DTCML` writer - DTCM-RAM Double error lockup bit"]
pub type DTCML_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITCML` reader - ITCM-RAM Double error lockup bit"]
pub type ITCML_R = crate::BitReader;
#[doc = "Field `ITCML` writer - ITCM-RAM Double error lockup bit"]
pub type ITCML_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXIRAML` reader - AXISRAM Double error lockup bit"]
pub type AXIRAML_R = crate::BitReader;
#[doc = "Field `AXIRAML` writer - AXISRAM Double error lockup bit"]
pub type AXIRAML_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Programmable voltage detector lockup bit"]
    #[inline(always)]
    pub fn pvdl(&self) -> PVDL_R {
        PVDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLASH double error lockup bit"]
    #[inline(always)]
    pub fn flashl(&self) -> FLASHL_R {
        FLASHL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU lockup bit"]
    #[inline(always)]
    pub fn cm7l(&self) -> CM7L_R {
        CM7L_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Backup RAM Double error lockup bit"]
    #[inline(always)]
    pub fn bkraml(&self) -> BKRAML_R {
        BKRAML_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM4 Double error lockup bit"]
    #[inline(always)]
    pub fn sram4l(&self) -> SRAM4L_R {
        SRAM4L_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM2 Double error lockup bit"]
    #[inline(always)]
    pub fn sram2l(&self) -> SRAM2L_R {
        SRAM2L_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM1 Double error lockup bit"]
    #[inline(always)]
    pub fn sram1l(&self) -> SRAM1L_R {
        SRAM1L_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DTCM-RAM Double error lockup bit"]
    #[inline(always)]
    pub fn dtcml(&self) -> DTCML_R {
        DTCML_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ITCM-RAM Double error lockup bit"]
    #[inline(always)]
    pub fn itcml(&self) -> ITCML_R {
        ITCML_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AXISRAM Double error lockup bit"]
    #[inline(always)]
    pub fn axiraml(&self) -> AXIRAML_R {
        AXIRAML_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Programmable voltage detector lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvdl(&mut self) -> PVDL_W<CFGRrs> {
        PVDL_W::new(self, 2)
    }
    #[doc = "Bit 3 - FLASH double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn flashl(&mut self) -> FLASHL_W<CFGRrs> {
        FLASHL_W::new(self, 3)
    }
    #[doc = "Bit 6 - CPU lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn cm7l(&mut self) -> CM7L_W<CFGRrs> {
        CM7L_W::new(self, 6)
    }
    #[doc = "Bit 7 - Backup RAM Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn bkraml(&mut self) -> BKRAML_W<CFGRrs> {
        BKRAML_W::new(self, 7)
    }
    #[doc = "Bit 9 - SRAM4 Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram4l(&mut self) -> SRAM4L_W<CFGRrs> {
        SRAM4L_W::new(self, 9)
    }
    #[doc = "Bit 11 - SRAM2 Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram2l(&mut self) -> SRAM2L_W<CFGRrs> {
        SRAM2L_W::new(self, 11)
    }
    #[doc = "Bit 12 - SRAM1 Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram1l(&mut self) -> SRAM1L_W<CFGRrs> {
        SRAM1L_W::new(self, 12)
    }
    #[doc = "Bit 13 - DTCM-RAM Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn dtcml(&mut self) -> DTCML_W<CFGRrs> {
        DTCML_W::new(self, 13)
    }
    #[doc = "Bit 14 - ITCM-RAM Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn itcml(&mut self) -> ITCML_W<CFGRrs> {
        ITCML_W::new(self, 14)
    }
    #[doc = "Bit 15 - AXISRAM Double error lockup bit"]
    #[inline(always)]
    #[must_use]
    pub fn axiraml(&mut self) -> AXIRAML_W<CFGRrs> {
        AXIRAML_W::new(self, 15)
    }
}
#[doc = "Timer break lockup register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFGRrs;
impl crate::RegisterSpec for CFGRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CFGRrs {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGRrs {
    const RESET_VALUE: u32 = 0;
}
