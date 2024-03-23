#[doc = "Register `DMA_S7CR` reader"]
pub type R = crate::R<DMA_S7CRrs>;
#[doc = "Register `DMA_S7CR` writer"]
pub type W = crate::W<DMA_S7CRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMEIE` reader - DMEIE"]
pub type DMEIE_R = crate::BitReader;
#[doc = "Field `DMEIE` writer - DMEIE"]
pub type DMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - TEIE"]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - TEIE"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIE` reader - HTIE"]
pub type HTIE_R = crate::BitReader;
#[doc = "Field `HTIE` writer - HTIE"]
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PFCTRL` reader - PFCTRL"]
pub type PFCTRL_R = crate::BitReader;
#[doc = "Field `PFCTRL` writer - PFCTRL"]
pub type PFCTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - DIR"]
pub type DIR_R = crate::FieldReader;
#[doc = "Field `DIR` writer - DIR"]
pub type DIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CIRC` reader - CIRC"]
pub type CIRC_R = crate::BitReader;
#[doc = "Field `CIRC` writer - CIRC"]
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINC` reader - PINC"]
pub type PINC_R = crate::BitReader;
#[doc = "Field `PINC` writer - PINC"]
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINC` reader - MINC"]
pub type MINC_R = crate::BitReader;
#[doc = "Field `MINC` writer - MINC"]
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE` reader - PSIZE"]
pub type PSIZE_R = crate::FieldReader;
#[doc = "Field `PSIZE` writer - PSIZE"]
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MSIZE` reader - MSIZE"]
pub type MSIZE_R = crate::FieldReader;
#[doc = "Field `MSIZE` writer - MSIZE"]
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PINCOS` reader - PINCOS"]
pub type PINCOS_R = crate::BitReader;
#[doc = "Field `PINCOS` writer - PINCOS"]
pub type PINCOS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL` reader - PL"]
pub type PL_R = crate::FieldReader;
#[doc = "Field `PL` writer - PL"]
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBM` reader - DBM"]
pub type DBM_R = crate::BitReader;
#[doc = "Field `DBM` writer - DBM"]
pub type DBM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CT` reader - CT"]
pub type CT_R = crate::BitReader;
#[doc = "Field `CT` writer - CT"]
pub type CT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBURST` reader - PBURST"]
pub type PBURST_R = crate::FieldReader;
#[doc = "Field `PBURST` writer - PBURST"]
pub type PBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MBURST` reader - MBURST"]
pub type MBURST_R = crate::FieldReader;
#[doc = "Field `MBURST` writer - MBURST"]
pub type MBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMEIE"]
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HTIE"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PFCTRL"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - CIRC"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PINC"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MINC"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - PSIZE"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - MSIZE"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - PINCOS"]
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - PL"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - DBM"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CT"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 21:22 - PBURST"]
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - MBURST"]
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<DMA_S7CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMEIE"]
    #[inline(always)]
    #[must_use]
    pub fn dmeie(&mut self) -> DMEIE_W<DMA_S7CRrs> {
        DMEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - TEIE"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<DMA_S7CRrs> {
        TEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - HTIE"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<DMA_S7CRrs> {
        HTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<DMA_S7CRrs> {
        TCIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - PFCTRL"]
    #[inline(always)]
    #[must_use]
    pub fn pfctrl(&mut self) -> PFCTRL_W<DMA_S7CRrs> {
        PFCTRL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - DIR"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<DMA_S7CRrs> {
        DIR_W::new(self, 6)
    }
    #[doc = "Bit 8 - CIRC"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<DMA_S7CRrs> {
        CIRC_W::new(self, 8)
    }
    #[doc = "Bit 9 - PINC"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<DMA_S7CRrs> {
        PINC_W::new(self, 9)
    }
    #[doc = "Bit 10 - MINC"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<DMA_S7CRrs> {
        MINC_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - PSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<DMA_S7CRrs> {
        PSIZE_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - MSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<DMA_S7CRrs> {
        MSIZE_W::new(self, 13)
    }
    #[doc = "Bit 15 - PINCOS"]
    #[inline(always)]
    #[must_use]
    pub fn pincos(&mut self) -> PINCOS_W<DMA_S7CRrs> {
        PINCOS_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - PL"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<DMA_S7CRrs> {
        PL_W::new(self, 16)
    }
    #[doc = "Bit 18 - DBM"]
    #[inline(always)]
    #[must_use]
    pub fn dbm(&mut self) -> DBM_W<DMA_S7CRrs> {
        DBM_W::new(self, 18)
    }
    #[doc = "Bit 19 - CT"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CT_W<DMA_S7CRrs> {
        CT_W::new(self, 19)
    }
    #[doc = "Bits 21:22 - PBURST"]
    #[inline(always)]
    #[must_use]
    pub fn pburst(&mut self) -> PBURST_W<DMA_S7CRrs> {
        PBURST_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - MBURST"]
    #[inline(always)]
    #[must_use]
    pub fn mburst(&mut self) -> MBURST_W<DMA_S7CRrs> {
        MBURST_W::new(self, 23)
    }
}
#[doc = "This register is used to configure the concerned stream.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_s7cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_s7cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_S7CRrs;
impl crate::RegisterSpec for DMA_S7CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_s7cr::R`](R) reader structure"]
impl crate::Readable for DMA_S7CRrs {}
#[doc = "`write(|w| ..)` method takes [`dma_s7cr::W`](W) writer structure"]
impl crate::Writable for DMA_S7CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_S7CR to value 0"]
impl crate::Resettable for DMA_S7CRrs {
    const RESET_VALUE: u32 = 0;
}
