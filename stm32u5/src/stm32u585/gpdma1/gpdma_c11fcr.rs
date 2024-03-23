#[doc = "Register `GPDMA_C11FCR` writer"]
pub type W = crate::W<GPDMA_C11FCRrs>;
#[doc = "Field `TCF` writer - transfer complete flag clear"]
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTF` writer - half transfer flag clear"]
pub type HTF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEF` writer - data transfer error flag clear"]
pub type DTEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULEF` writer - update link transfer error flag clear"]
pub type ULEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USEF` writer - user setting error flag clear"]
pub type USEF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPF` writer - completed suspension flag clear"]
pub type SUSPF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOF` writer - trigger overrun flag clear"]
pub type TOF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 8 - transfer complete flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tcf(&mut self) -> TCF_W<GPDMA_C11FCRrs> {
        TCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - half transfer flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn htf(&mut self) -> HTF_W<GPDMA_C11FCRrs> {
        HTF_W::new(self, 9)
    }
    #[doc = "Bit 10 - data transfer error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn dtef(&mut self) -> DTEF_W<GPDMA_C11FCRrs> {
        DTEF_W::new(self, 10)
    }
    #[doc = "Bit 11 - update link transfer error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ulef(&mut self) -> ULEF_W<GPDMA_C11FCRrs> {
        ULEF_W::new(self, 11)
    }
    #[doc = "Bit 12 - user setting error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn usef(&mut self) -> USEF_W<GPDMA_C11FCRrs> {
        USEF_W::new(self, 12)
    }
    #[doc = "Bit 13 - completed suspension flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspf(&mut self) -> SUSPF_W<GPDMA_C11FCRrs> {
        SUSPF_W::new(self, 13)
    }
    #[doc = "Bit 14 - trigger overrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tof(&mut self) -> TOF_W<GPDMA_C11FCRrs> {
        TOF_W::new(self, 14)
    }
}
#[doc = "GPDMA channel 11 flag clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpdma_c11fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPDMA_C11FCRrs;
impl crate::RegisterSpec for GPDMA_C11FCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`gpdma_c11fcr::W`](W) writer structure"]
impl crate::Writable for GPDMA_C11FCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPDMA_C11FCR to value 0"]
impl crate::Resettable for GPDMA_C11FCRrs {
    const RESET_VALUE: u32 = 0;
}
