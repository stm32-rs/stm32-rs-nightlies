#[doc = "Register `DFSDM_FLT0CR2` reader"]
pub type R = crate::R<DFSDM_FLT0CR2rs>;
#[doc = "Register `DFSDM_FLT0CR2` writer"]
pub type W = crate::W<DFSDM_FLT0CR2rs>;
#[doc = "Field `JEOCIE` reader - JEOCIE"]
pub type JEOCIE_R = crate::BitReader;
#[doc = "Field `JEOCIE` writer - JEOCIE"]
pub type JEOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REOCIE` reader - REOCIE"]
pub type REOCIE_R = crate::BitReader;
#[doc = "Field `REOCIE` writer - REOCIE"]
pub type REOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JOVRIE` reader - JOVRIE"]
pub type JOVRIE_R = crate::BitReader;
#[doc = "Field `JOVRIE` writer - JOVRIE"]
pub type JOVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVRIE` reader - ROVRIE"]
pub type ROVRIE_R = crate::BitReader;
#[doc = "Field `ROVRIE` writer - ROVRIE"]
pub type ROVRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDIE` reader - AWDIE"]
pub type AWDIE_R = crate::BitReader;
#[doc = "Field `AWDIE` writer - AWDIE"]
pub type AWDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCDIE` reader - SCDIE"]
pub type SCDIE_R = crate::BitReader;
#[doc = "Field `SCDIE` writer - SCDIE"]
pub type SCDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKABIE` reader - CKABIE"]
pub type CKABIE_R = crate::BitReader;
#[doc = "Field `CKABIE` writer - CKABIE"]
pub type CKABIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCH` reader - EXCH"]
pub type EXCH_R = crate::FieldReader;
#[doc = "Field `EXCH` writer - EXCH"]
pub type EXCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AWDCH` reader - AWDCH"]
pub type AWDCH_R = crate::FieldReader;
#[doc = "Field `AWDCH` writer - AWDCH"]
pub type AWDCH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - JEOCIE"]
    #[inline(always)]
    pub fn jeocie(&self) -> JEOCIE_R {
        JEOCIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - REOCIE"]
    #[inline(always)]
    pub fn reocie(&self) -> REOCIE_R {
        REOCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - JOVRIE"]
    #[inline(always)]
    pub fn jovrie(&self) -> JOVRIE_R {
        JOVRIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ROVRIE"]
    #[inline(always)]
    pub fn rovrie(&self) -> ROVRIE_R {
        ROVRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AWDIE"]
    #[inline(always)]
    pub fn awdie(&self) -> AWDIE_R {
        AWDIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SCDIE"]
    #[inline(always)]
    pub fn scdie(&self) -> SCDIE_R {
        SCDIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CKABIE"]
    #[inline(always)]
    pub fn ckabie(&self) -> CKABIE_R {
        CKABIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - EXCH"]
    #[inline(always)]
    pub fn exch(&self) -> EXCH_R {
        EXCH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - AWDCH"]
    #[inline(always)]
    pub fn awdch(&self) -> AWDCH_R {
        AWDCH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - JEOCIE"]
    #[inline(always)]
    #[must_use]
    pub fn jeocie(&mut self) -> JEOCIE_W<DFSDM_FLT0CR2rs> {
        JEOCIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - REOCIE"]
    #[inline(always)]
    #[must_use]
    pub fn reocie(&mut self) -> REOCIE_W<DFSDM_FLT0CR2rs> {
        REOCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - JOVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn jovrie(&mut self) -> JOVRIE_W<DFSDM_FLT0CR2rs> {
        JOVRIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - ROVRIE"]
    #[inline(always)]
    #[must_use]
    pub fn rovrie(&mut self) -> ROVRIE_W<DFSDM_FLT0CR2rs> {
        ROVRIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - AWDIE"]
    #[inline(always)]
    #[must_use]
    pub fn awdie(&mut self) -> AWDIE_W<DFSDM_FLT0CR2rs> {
        AWDIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - SCDIE"]
    #[inline(always)]
    #[must_use]
    pub fn scdie(&mut self) -> SCDIE_W<DFSDM_FLT0CR2rs> {
        SCDIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - CKABIE"]
    #[inline(always)]
    #[must_use]
    pub fn ckabie(&mut self) -> CKABIE_W<DFSDM_FLT0CR2rs> {
        CKABIE_W::new(self, 6)
    }
    #[doc = "Bits 8:15 - EXCH"]
    #[inline(always)]
    #[must_use]
    pub fn exch(&mut self) -> EXCH_W<DFSDM_FLT0CR2rs> {
        EXCH_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - AWDCH"]
    #[inline(always)]
    #[must_use]
    pub fn awdch(&mut self) -> AWDCH_W<DFSDM_FLT0CR2rs> {
        AWDCH_W::new(self, 16)
    }
}
#[doc = "DFSDM filter 0 control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dfsdm_flt0cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dfsdm_flt0cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DFSDM_FLT0CR2rs;
impl crate::RegisterSpec for DFSDM_FLT0CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0cr2::R`](R) reader structure"]
impl crate::Readable for DFSDM_FLT0CR2rs {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt0cr2::W`](W) writer structure"]
impl crate::Writable for DFSDM_FLT0CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DFSDM_FLT0CR2 to value 0"]
impl crate::Resettable for DFSDM_FLT0CR2rs {
    const RESET_VALUE: u32 = 0;
}
