#[doc = "Register `DMA_LIFCR` writer"]
pub type W = crate::W<DMA_LIFCRrs>;
#[doc = "Field `CFEIF0` writer - CFEIF0"]
pub type CFEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF0` writer - CDMEIF0"]
pub type CDMEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF0` writer - CTEIF0"]
pub type CTEIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF0` writer - CHTIF0"]
pub type CHTIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF0` writer - CTCIF0"]
pub type CTCIF0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF1` writer - CFEIF1"]
pub type CFEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF1` writer - CDMEIF1"]
pub type CDMEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF1` writer - CTEIF1"]
pub type CTEIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF1` writer - CHTIF1"]
pub type CHTIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF1` writer - CTCIF1"]
pub type CTCIF1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF2` writer - CFEIF2"]
pub type CFEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF2` writer - CDMEIF2"]
pub type CDMEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF2` writer - CTEIF2"]
pub type CTEIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF2` writer - CHTIF2"]
pub type CHTIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF2` writer - CTCIF2"]
pub type CTCIF2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF3` writer - CFEIF3"]
pub type CFEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF3` writer - CDMEIF3"]
pub type CDMEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF3` writer - CTEIF3"]
pub type CTEIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF3` writer - CHTIF3"]
pub type CHTIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF3` writer - CTCIF3"]
pub type CTCIF3_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CFEIF0"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif0(&mut self) -> CFEIF0_W<DMA_LIFCRrs> {
        CFEIF0_W::new(self, 0)
    }
    #[doc = "Bit 2 - CDMEIF0"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif0(&mut self) -> CDMEIF0_W<DMA_LIFCRrs> {
        CDMEIF0_W::new(self, 2)
    }
    #[doc = "Bit 3 - CTEIF0"]
    #[inline(always)]
    #[must_use]
    pub fn cteif0(&mut self) -> CTEIF0_W<DMA_LIFCRrs> {
        CTEIF0_W::new(self, 3)
    }
    #[doc = "Bit 4 - CHTIF0"]
    #[inline(always)]
    #[must_use]
    pub fn chtif0(&mut self) -> CHTIF0_W<DMA_LIFCRrs> {
        CHTIF0_W::new(self, 4)
    }
    #[doc = "Bit 5 - CTCIF0"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif0(&mut self) -> CTCIF0_W<DMA_LIFCRrs> {
        CTCIF0_W::new(self, 5)
    }
    #[doc = "Bit 6 - CFEIF1"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif1(&mut self) -> CFEIF1_W<DMA_LIFCRrs> {
        CFEIF1_W::new(self, 6)
    }
    #[doc = "Bit 8 - CDMEIF1"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif1(&mut self) -> CDMEIF1_W<DMA_LIFCRrs> {
        CDMEIF1_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTEIF1"]
    #[inline(always)]
    #[must_use]
    pub fn cteif1(&mut self) -> CTEIF1_W<DMA_LIFCRrs> {
        CTEIF1_W::new(self, 9)
    }
    #[doc = "Bit 10 - CHTIF1"]
    #[inline(always)]
    #[must_use]
    pub fn chtif1(&mut self) -> CHTIF1_W<DMA_LIFCRrs> {
        CHTIF1_W::new(self, 10)
    }
    #[doc = "Bit 11 - CTCIF1"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif1(&mut self) -> CTCIF1_W<DMA_LIFCRrs> {
        CTCIF1_W::new(self, 11)
    }
    #[doc = "Bit 16 - CFEIF2"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif2(&mut self) -> CFEIF2_W<DMA_LIFCRrs> {
        CFEIF2_W::new(self, 16)
    }
    #[doc = "Bit 18 - CDMEIF2"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif2(&mut self) -> CDMEIF2_W<DMA_LIFCRrs> {
        CDMEIF2_W::new(self, 18)
    }
    #[doc = "Bit 19 - CTEIF2"]
    #[inline(always)]
    #[must_use]
    pub fn cteif2(&mut self) -> CTEIF2_W<DMA_LIFCRrs> {
        CTEIF2_W::new(self, 19)
    }
    #[doc = "Bit 20 - CHTIF2"]
    #[inline(always)]
    #[must_use]
    pub fn chtif2(&mut self) -> CHTIF2_W<DMA_LIFCRrs> {
        CHTIF2_W::new(self, 20)
    }
    #[doc = "Bit 21 - CTCIF2"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif2(&mut self) -> CTCIF2_W<DMA_LIFCRrs> {
        CTCIF2_W::new(self, 21)
    }
    #[doc = "Bit 22 - CFEIF3"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif3(&mut self) -> CFEIF3_W<DMA_LIFCRrs> {
        CFEIF3_W::new(self, 22)
    }
    #[doc = "Bit 24 - CDMEIF3"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif3(&mut self) -> CDMEIF3_W<DMA_LIFCRrs> {
        CDMEIF3_W::new(self, 24)
    }
    #[doc = "Bit 25 - CTEIF3"]
    #[inline(always)]
    #[must_use]
    pub fn cteif3(&mut self) -> CTEIF3_W<DMA_LIFCRrs> {
        CTEIF3_W::new(self, 25)
    }
    #[doc = "Bit 26 - CHTIF3"]
    #[inline(always)]
    #[must_use]
    pub fn chtif3(&mut self) -> CHTIF3_W<DMA_LIFCRrs> {
        CHTIF3_W::new(self, 26)
    }
    #[doc = "Bit 27 - CTCIF3"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif3(&mut self) -> CTCIF3_W<DMA_LIFCRrs> {
        CTCIF3_W::new(self, 27)
    }
}
#[doc = "DMA low interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_lifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_LIFCRrs;
impl crate::RegisterSpec for DMA_LIFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_lifcr::W`](W) writer structure"]
impl crate::Writable for DMA_LIFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_LIFCR to value 0"]
impl crate::Resettable for DMA_LIFCRrs {
    const RESET_VALUE: u32 = 0;
}
