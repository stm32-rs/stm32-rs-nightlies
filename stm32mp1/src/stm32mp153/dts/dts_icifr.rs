#[doc = "Register `DTS_ICIFR` reader"]
pub type R = crate::R<DTS_ICIFRrs>;
#[doc = "Register `DTS_ICIFR` writer"]
pub type W = crate::W<DTS_ICIFRrs>;
#[doc = "Field `TS1_CITEF` reader - TS1_CITEF"]
pub type TS1_CITEF_R = crate::BitReader;
#[doc = "Field `TS1_CITEF` writer - TS1_CITEF"]
pub type TS1_CITEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CITLF` reader - TS1_CITLF"]
pub type TS1_CITLF_R = crate::BitReader;
#[doc = "Field `TS1_CITLF` writer - TS1_CITLF"]
pub type TS1_CITLF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CITHF` reader - TS1_CITHF"]
pub type TS1_CITHF_R = crate::BitReader;
#[doc = "Field `TS1_CITHF` writer - TS1_CITHF"]
pub type TS1_CITHF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CAITEF` reader - TS1_CAITEF"]
pub type TS1_CAITEF_R = crate::BitReader;
#[doc = "Field `TS1_CAITEF` writer - TS1_CAITEF"]
pub type TS1_CAITEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CAITLF` reader - TS1_CAITLF"]
pub type TS1_CAITLF_R = crate::BitReader;
#[doc = "Field `TS1_CAITLF` writer - TS1_CAITLF"]
pub type TS1_CAITLF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1_CAITHF` reader - TS1_CAITHF"]
pub type TS1_CAITHF_R = crate::BitReader;
#[doc = "Field `TS1_CAITHF` writer - TS1_CAITHF"]
pub type TS1_CAITHF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TS1_CITEF"]
    #[inline(always)]
    pub fn ts1_citef(&self) -> TS1_CITEF_R {
        TS1_CITEF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TS1_CITLF"]
    #[inline(always)]
    pub fn ts1_citlf(&self) -> TS1_CITLF_R {
        TS1_CITLF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TS1_CITHF"]
    #[inline(always)]
    pub fn ts1_cithf(&self) -> TS1_CITHF_R {
        TS1_CITHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - TS1_CAITEF"]
    #[inline(always)]
    pub fn ts1_caitef(&self) -> TS1_CAITEF_R {
        TS1_CAITEF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TS1_CAITLF"]
    #[inline(always)]
    pub fn ts1_caitlf(&self) -> TS1_CAITLF_R {
        TS1_CAITLF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TS1_CAITHF"]
    #[inline(always)]
    pub fn ts1_caithf(&self) -> TS1_CAITHF_R {
        TS1_CAITHF_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TS1_CITEF"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_citef(&mut self) -> TS1_CITEF_W<DTS_ICIFRrs> {
        TS1_CITEF_W::new(self, 0)
    }
    #[doc = "Bit 1 - TS1_CITLF"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_citlf(&mut self) -> TS1_CITLF_W<DTS_ICIFRrs> {
        TS1_CITLF_W::new(self, 1)
    }
    #[doc = "Bit 2 - TS1_CITHF"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_cithf(&mut self) -> TS1_CITHF_W<DTS_ICIFRrs> {
        TS1_CITHF_W::new(self, 2)
    }
    #[doc = "Bit 4 - TS1_CAITEF"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_caitef(&mut self) -> TS1_CAITEF_W<DTS_ICIFRrs> {
        TS1_CAITEF_W::new(self, 4)
    }
    #[doc = "Bit 5 - TS1_CAITLF"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_caitlf(&mut self) -> TS1_CAITLF_W<DTS_ICIFRrs> {
        TS1_CAITLF_W::new(self, 5)
    }
    #[doc = "Bit 6 - TS1_CAITHF"]
    #[inline(always)]
    #[must_use]
    pub fn ts1_caithf(&mut self) -> TS1_CAITHF_W<DTS_ICIFRrs> {
        TS1_CAITHF_W::new(self, 6)
    }
}
#[doc = "DTS_ICIFR is the control register for the interrupt flags.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dts_icifr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dts_icifr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DTS_ICIFRrs;
impl crate::RegisterSpec for DTS_ICIFRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dts_icifr::R`](R) reader structure"]
impl crate::Readable for DTS_ICIFRrs {}
#[doc = "`write(|w| ..)` method takes [`dts_icifr::W`](W) writer structure"]
impl crate::Writable for DTS_ICIFRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DTS_ICIFR to value 0"]
impl crate::Resettable for DTS_ICIFRrs {
    const RESET_VALUE: u32 = 0;
}
