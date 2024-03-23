#[doc = "Register `DMA_HIFCR` writer"]
pub type W = crate::W<DMA_HIFCRrs>;
#[doc = "Field `CFEIF4` writer - CFEIF4"]
pub type CFEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF4` writer - CDMEIF4"]
pub type CDMEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF4` writer - CTEIF4"]
pub type CTEIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF4` writer - CHTIF4"]
pub type CHTIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF4` writer - CTCIF4"]
pub type CTCIF4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF5` writer - CFEIF5"]
pub type CFEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF5` writer - CDMEIF5"]
pub type CDMEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF5` writer - CTEIF5"]
pub type CTEIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF5` writer - CHTIF5"]
pub type CHTIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF5` writer - CTCIF5"]
pub type CTCIF5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF6` writer - CFEIF6"]
pub type CFEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF6` writer - CDMEIF6"]
pub type CDMEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF6` writer - CTEIF6"]
pub type CTEIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF6` writer - CHTIF6"]
pub type CHTIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF6` writer - CTCIF6"]
pub type CTCIF6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFEIF7` writer - CFEIF7"]
pub type CFEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDMEIF7` writer - CDMEIF7"]
pub type CDMEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTEIF7` writer - CTEIF7"]
pub type CTEIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHTIF7` writer - CHTIF7"]
pub type CHTIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIF7` writer - CTCIF7"]
pub type CTCIF7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CFEIF4"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif4(&mut self) -> CFEIF4_W<DMA_HIFCRrs> {
        CFEIF4_W::new(self, 0)
    }
    #[doc = "Bit 2 - CDMEIF4"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif4(&mut self) -> CDMEIF4_W<DMA_HIFCRrs> {
        CDMEIF4_W::new(self, 2)
    }
    #[doc = "Bit 3 - CTEIF4"]
    #[inline(always)]
    #[must_use]
    pub fn cteif4(&mut self) -> CTEIF4_W<DMA_HIFCRrs> {
        CTEIF4_W::new(self, 3)
    }
    #[doc = "Bit 4 - CHTIF4"]
    #[inline(always)]
    #[must_use]
    pub fn chtif4(&mut self) -> CHTIF4_W<DMA_HIFCRrs> {
        CHTIF4_W::new(self, 4)
    }
    #[doc = "Bit 5 - CTCIF4"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif4(&mut self) -> CTCIF4_W<DMA_HIFCRrs> {
        CTCIF4_W::new(self, 5)
    }
    #[doc = "Bit 6 - CFEIF5"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif5(&mut self) -> CFEIF5_W<DMA_HIFCRrs> {
        CFEIF5_W::new(self, 6)
    }
    #[doc = "Bit 8 - CDMEIF5"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif5(&mut self) -> CDMEIF5_W<DMA_HIFCRrs> {
        CDMEIF5_W::new(self, 8)
    }
    #[doc = "Bit 9 - CTEIF5"]
    #[inline(always)]
    #[must_use]
    pub fn cteif5(&mut self) -> CTEIF5_W<DMA_HIFCRrs> {
        CTEIF5_W::new(self, 9)
    }
    #[doc = "Bit 10 - CHTIF5"]
    #[inline(always)]
    #[must_use]
    pub fn chtif5(&mut self) -> CHTIF5_W<DMA_HIFCRrs> {
        CHTIF5_W::new(self, 10)
    }
    #[doc = "Bit 11 - CTCIF5"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif5(&mut self) -> CTCIF5_W<DMA_HIFCRrs> {
        CTCIF5_W::new(self, 11)
    }
    #[doc = "Bit 16 - CFEIF6"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif6(&mut self) -> CFEIF6_W<DMA_HIFCRrs> {
        CFEIF6_W::new(self, 16)
    }
    #[doc = "Bit 18 - CDMEIF6"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif6(&mut self) -> CDMEIF6_W<DMA_HIFCRrs> {
        CDMEIF6_W::new(self, 18)
    }
    #[doc = "Bit 19 - CTEIF6"]
    #[inline(always)]
    #[must_use]
    pub fn cteif6(&mut self) -> CTEIF6_W<DMA_HIFCRrs> {
        CTEIF6_W::new(self, 19)
    }
    #[doc = "Bit 20 - CHTIF6"]
    #[inline(always)]
    #[must_use]
    pub fn chtif6(&mut self) -> CHTIF6_W<DMA_HIFCRrs> {
        CHTIF6_W::new(self, 20)
    }
    #[doc = "Bit 21 - CTCIF6"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif6(&mut self) -> CTCIF6_W<DMA_HIFCRrs> {
        CTCIF6_W::new(self, 21)
    }
    #[doc = "Bit 22 - CFEIF7"]
    #[inline(always)]
    #[must_use]
    pub fn cfeif7(&mut self) -> CFEIF7_W<DMA_HIFCRrs> {
        CFEIF7_W::new(self, 22)
    }
    #[doc = "Bit 24 - CDMEIF7"]
    #[inline(always)]
    #[must_use]
    pub fn cdmeif7(&mut self) -> CDMEIF7_W<DMA_HIFCRrs> {
        CDMEIF7_W::new(self, 24)
    }
    #[doc = "Bit 25 - CTEIF7"]
    #[inline(always)]
    #[must_use]
    pub fn cteif7(&mut self) -> CTEIF7_W<DMA_HIFCRrs> {
        CTEIF7_W::new(self, 25)
    }
    #[doc = "Bit 26 - CHTIF7"]
    #[inline(always)]
    #[must_use]
    pub fn chtif7(&mut self) -> CHTIF7_W<DMA_HIFCRrs> {
        CHTIF7_W::new(self, 26)
    }
    #[doc = "Bit 27 - CTCIF7"]
    #[inline(always)]
    #[must_use]
    pub fn ctcif7(&mut self) -> CTCIF7_W<DMA_HIFCRrs> {
        CTCIF7_W::new(self, 27)
    }
}
#[doc = "DMA high interrupt flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_hifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_HIFCRrs;
impl crate::RegisterSpec for DMA_HIFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_hifcr::W`](W) writer structure"]
impl crate::Writable for DMA_HIFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_HIFCR to value 0"]
impl crate::Resettable for DMA_HIFCRrs {
    const RESET_VALUE: u32 = 0;
}
