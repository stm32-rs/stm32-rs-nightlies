#[doc = "Register `WPCR1` reader"]
pub type R = crate::R<WPCR1rs>;
#[doc = "Register `WPCR1` writer"]
pub type W = crate::W<WPCR1rs>;
#[doc = "Field `SKEWCL` reader - SKEWCL"]
pub type SKEWCL_R = crate::FieldReader;
#[doc = "Field `SKEWCL` writer - SKEWCL"]
pub type SKEWCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SKEWDL` reader - SKEWDL"]
pub type SKEWDL_R = crate::FieldReader;
#[doc = "Field `SKEWDL` writer - SKEWDL"]
pub type SKEWDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTXSRCL` reader - LPTXSRCL"]
pub type LPTXSRCL_R = crate::FieldReader;
#[doc = "Field `LPTXSRCL` writer - LPTXSRCL"]
pub type LPTXSRCL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTXSRDL` reader - LPTXSRDL"]
pub type LPTXSRDL_R = crate::FieldReader;
#[doc = "Field `LPTXSRDL` writer - LPTXSRDL"]
pub type LPTXSRDL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDDCCL` reader - SDDCCL"]
pub type SDDCCL_R = crate::BitReader;
#[doc = "Field `SDDCCL` writer - SDDCCL"]
pub type SDDCCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDDCDL` reader - SDDCDL"]
pub type SDDCDL_R = crate::BitReader;
#[doc = "Field `SDDCDL` writer - SDDCDL"]
pub type SDDCDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTXSRUCL` reader - HSTXSRUCL"]
pub type HSTXSRUCL_R = crate::BitReader;
#[doc = "Field `HSTXSRUCL` writer - HSTXSRUCL"]
pub type HSTXSRUCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTXSRDCL` reader - HSTXSRDCL"]
pub type HSTXSRDCL_R = crate::BitReader;
#[doc = "Field `HSTXSRDCL` writer - HSTXSRDCL"]
pub type HSTXSRDCL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTXSRUDL` reader - HSTXSRUDL"]
pub type HSTXSRUDL_R = crate::BitReader;
#[doc = "Field `HSTXSRUDL` writer - HSTXSRUDL"]
pub type HSTXSRUDL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTXSRDDL` reader - HSTXSRDDL"]
pub type HSTXSRDDL_R = crate::BitReader;
#[doc = "Field `HSTXSRDDL` writer - HSTXSRDDL"]
pub type HSTXSRDDL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SKEWCL"]
    #[inline(always)]
    pub fn skewcl(&self) -> SKEWCL_R {
        SKEWCL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SKEWDL"]
    #[inline(always)]
    pub fn skewdl(&self) -> SKEWDL_R {
        SKEWDL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - LPTXSRCL"]
    #[inline(always)]
    pub fn lptxsrcl(&self) -> LPTXSRCL_R {
        LPTXSRCL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - LPTXSRDL"]
    #[inline(always)]
    pub fn lptxsrdl(&self) -> LPTXSRDL_R {
        LPTXSRDL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - SDDCCL"]
    #[inline(always)]
    pub fn sddccl(&self) -> SDDCCL_R {
        SDDCCL_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SDDCDL"]
    #[inline(always)]
    pub fn sddcdl(&self) -> SDDCDL_R {
        SDDCDL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - HSTXSRUCL"]
    #[inline(always)]
    pub fn hstxsrucl(&self) -> HSTXSRUCL_R {
        HSTXSRUCL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSTXSRDCL"]
    #[inline(always)]
    pub fn hstxsrdcl(&self) -> HSTXSRDCL_R {
        HSTXSRDCL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSTXSRUDL"]
    #[inline(always)]
    pub fn hstxsrudl(&self) -> HSTXSRUDL_R {
        HSTXSRUDL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSTXSRDDL"]
    #[inline(always)]
    pub fn hstxsrddl(&self) -> HSTXSRDDL_R {
        HSTXSRDDL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SKEWCL"]
    #[inline(always)]
    #[must_use]
    pub fn skewcl(&mut self) -> SKEWCL_W<WPCR1rs> {
        SKEWCL_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - SKEWDL"]
    #[inline(always)]
    #[must_use]
    pub fn skewdl(&mut self) -> SKEWDL_W<WPCR1rs> {
        SKEWDL_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - LPTXSRCL"]
    #[inline(always)]
    #[must_use]
    pub fn lptxsrcl(&mut self) -> LPTXSRCL_W<WPCR1rs> {
        LPTXSRCL_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - LPTXSRDL"]
    #[inline(always)]
    #[must_use]
    pub fn lptxsrdl(&mut self) -> LPTXSRDL_W<WPCR1rs> {
        LPTXSRDL_W::new(self, 8)
    }
    #[doc = "Bit 12 - SDDCCL"]
    #[inline(always)]
    #[must_use]
    pub fn sddccl(&mut self) -> SDDCCL_W<WPCR1rs> {
        SDDCCL_W::new(self, 12)
    }
    #[doc = "Bit 13 - SDDCDL"]
    #[inline(always)]
    #[must_use]
    pub fn sddcdl(&mut self) -> SDDCDL_W<WPCR1rs> {
        SDDCDL_W::new(self, 13)
    }
    #[doc = "Bit 16 - HSTXSRUCL"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrucl(&mut self) -> HSTXSRUCL_W<WPCR1rs> {
        HSTXSRUCL_W::new(self, 16)
    }
    #[doc = "Bit 17 - HSTXSRDCL"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrdcl(&mut self) -> HSTXSRDCL_W<WPCR1rs> {
        HSTXSRDCL_W::new(self, 17)
    }
    #[doc = "Bit 18 - HSTXSRUDL"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrudl(&mut self) -> HSTXSRUDL_W<WPCR1rs> {
        HSTXSRUDL_W::new(self, 18)
    }
    #[doc = "Bit 19 - HSTXSRDDL"]
    #[inline(always)]
    #[must_use]
    pub fn hstxsrddl(&mut self) -> HSTXSRDDL_W<WPCR1rs> {
        HSTXSRDDL_W::new(self, 19)
    }
}
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR1rs;
impl crate::RegisterSpec for WPCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpcr1::R`](R) reader structure"]
impl crate::Readable for WPCR1rs {}
#[doc = "`write(|w| ..)` method takes [`wpcr1::W`](W) writer structure"]
impl crate::Writable for WPCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR1 to value 0"]
impl crate::Resettable for WPCR1rs {
    const RESET_VALUE: u32 = 0;
}
