#[doc = "Register `QUADSPI_CR` reader"]
pub type R = crate::R<QUADSPI_CRrs>;
#[doc = "Register `QUADSPI_CR` writer"]
pub type W = crate::W<QUADSPI_CRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - ABORT"]
pub type ABORT_R = crate::BitReader;
#[doc = "Field `ABORT` writer - ABORT"]
pub type ABORT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMAEN"]
pub type DMAEN_R = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMAEN"]
pub type DMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCEN` reader - TCEN"]
pub type TCEN_R = crate::BitReader;
#[doc = "Field `TCEN` writer - TCEN"]
pub type TCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSHIFT` reader - SSHIFT"]
pub type SSHIFT_R = crate::BitReader;
#[doc = "Field `SSHIFT` writer - SSHIFT"]
pub type SSHIFT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFM` reader - DFM"]
pub type DFM_R = crate::BitReader;
#[doc = "Field `DFM` writer - DFM"]
pub type DFM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSEL` reader - FSEL"]
pub type FSEL_R = crate::BitReader;
#[doc = "Field `FSEL` writer - FSEL"]
pub type FSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTHRES` reader - FTHRES"]
pub type FTHRES_R = crate::FieldReader;
#[doc = "Field `FTHRES` writer - FTHRES"]
pub type FTHRES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TEIE` reader - TEIE"]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - TEIE"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTIE` reader - FTIE"]
pub type FTIE_R = crate::BitReader;
#[doc = "Field `FTIE` writer - FTIE"]
pub type FTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMIE` reader - SMIE"]
pub type SMIE_R = crate::BitReader;
#[doc = "Field `SMIE` writer - SMIE"]
pub type SMIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOIE` reader - TOIE"]
pub type TOIE_R = crate::BitReader;
#[doc = "Field `TOIE` writer - TOIE"]
pub type TOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APMS` reader - APMS"]
pub type APMS_R = crate::BitReader;
#[doc = "Field `APMS` writer - APMS"]
pub type APMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMM` reader - PMM"]
pub type PMM_R = crate::BitReader;
#[doc = "Field `PMM` writer - PMM"]
pub type PMM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCALER` reader - PRESCALER"]
pub type PRESCALER_R = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - PRESCALER"]
pub type PRESCALER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ABORT"]
    #[inline(always)]
    pub fn abort(&self) -> ABORT_R {
        ABORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAEN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TCEN"]
    #[inline(always)]
    pub fn tcen(&self) -> TCEN_R {
        TCEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SSHIFT"]
    #[inline(always)]
    pub fn sshift(&self) -> SSHIFT_R {
        SSHIFT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - DFM"]
    #[inline(always)]
    pub fn dfm(&self) -> DFM_R {
        DFM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FSEL"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - FTHRES"]
    #[inline(always)]
    pub fn fthres(&self) -> FTHRES_R {
        FTHRES_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FTIE"]
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - SMIE"]
    #[inline(always)]
    pub fn smie(&self) -> SMIE_R {
        SMIE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TOIE"]
    #[inline(always)]
    pub fn toie(&self) -> TOIE_R {
        TOIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - APMS"]
    #[inline(always)]
    pub fn apms(&self) -> APMS_R {
        APMS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PMM"]
    #[inline(always)]
    pub fn pmm(&self) -> PMM_R {
        PMM_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - PRESCALER"]
    #[inline(always)]
    pub fn prescaler(&self) -> PRESCALER_R {
        PRESCALER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<QUADSPI_CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - ABORT"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> ABORT_W<QUADSPI_CRrs> {
        ABORT_W::new(self, 1)
    }
    #[doc = "Bit 2 - DMAEN"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<QUADSPI_CRrs> {
        DMAEN_W::new(self, 2)
    }
    #[doc = "Bit 3 - TCEN"]
    #[inline(always)]
    #[must_use]
    pub fn tcen(&mut self) -> TCEN_W<QUADSPI_CRrs> {
        TCEN_W::new(self, 3)
    }
    #[doc = "Bit 4 - SSHIFT"]
    #[inline(always)]
    #[must_use]
    pub fn sshift(&mut self) -> SSHIFT_W<QUADSPI_CRrs> {
        SSHIFT_W::new(self, 4)
    }
    #[doc = "Bit 6 - DFM"]
    #[inline(always)]
    #[must_use]
    pub fn dfm(&mut self) -> DFM_W<QUADSPI_CRrs> {
        DFM_W::new(self, 6)
    }
    #[doc = "Bit 7 - FSEL"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<QUADSPI_CRrs> {
        FSEL_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - FTHRES"]
    #[inline(always)]
    #[must_use]
    pub fn fthres(&mut self) -> FTHRES_W<QUADSPI_CRrs> {
        FTHRES_W::new(self, 8)
    }
    #[doc = "Bit 16 - TEIE"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<QUADSPI_CRrs> {
        TEIE_W::new(self, 16)
    }
    #[doc = "Bit 17 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<QUADSPI_CRrs> {
        TCIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - FTIE"]
    #[inline(always)]
    #[must_use]
    pub fn ftie(&mut self) -> FTIE_W<QUADSPI_CRrs> {
        FTIE_W::new(self, 18)
    }
    #[doc = "Bit 19 - SMIE"]
    #[inline(always)]
    #[must_use]
    pub fn smie(&mut self) -> SMIE_W<QUADSPI_CRrs> {
        SMIE_W::new(self, 19)
    }
    #[doc = "Bit 20 - TOIE"]
    #[inline(always)]
    #[must_use]
    pub fn toie(&mut self) -> TOIE_W<QUADSPI_CRrs> {
        TOIE_W::new(self, 20)
    }
    #[doc = "Bit 22 - APMS"]
    #[inline(always)]
    #[must_use]
    pub fn apms(&mut self) -> APMS_W<QUADSPI_CRrs> {
        APMS_W::new(self, 22)
    }
    #[doc = "Bit 23 - PMM"]
    #[inline(always)]
    #[must_use]
    pub fn pmm(&mut self) -> PMM_W<QUADSPI_CRrs> {
        PMM_W::new(self, 23)
    }
    #[doc = "Bits 24:31 - PRESCALER"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PRESCALER_W<QUADSPI_CRrs> {
        PRESCALER_W::new(self, 24)
    }
}
#[doc = "QUADSPI control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`quadspi_cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`quadspi_cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QUADSPI_CRrs;
impl crate::RegisterSpec for QUADSPI_CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`quadspi_cr::R`](R) reader structure"]
impl crate::Readable for QUADSPI_CRrs {}
#[doc = "`write(|w| ..)` method takes [`quadspi_cr::W`](W) writer structure"]
impl crate::Writable for QUADSPI_CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QUADSPI_CR to value 0"]
impl crate::Resettable for QUADSPI_CRrs {
    const RESET_VALUE: u32 = 0;
}
