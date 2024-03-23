#[doc = "Register `MDMA_C4CR` reader"]
pub type R = crate::R<MDMA_C4CRrs>;
#[doc = "Register `MDMA_C4CR` writer"]
pub type W = crate::W<MDMA_C4CRrs>;
#[doc = "Field `EN` reader - EN"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - TEIE"]
pub type TEIE_R = crate::BitReader;
#[doc = "Field `TEIE` writer - TEIE"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIE` reader - CTCIE"]
pub type CTCIE_R = crate::BitReader;
#[doc = "Field `CTCIE` writer - CTCIE"]
pub type CTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRTIE` reader - BRTIE"]
pub type BRTIE_R = crate::BitReader;
#[doc = "Field `BRTIE` writer - BRTIE"]
pub type BRTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIE` reader - BTIE"]
pub type BTIE_R = crate::BitReader;
#[doc = "Field `BTIE` writer - BTIE"]
pub type BTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - TCIE"]
pub type TCIE_R = crate::BitReader;
#[doc = "Field `TCIE` writer - TCIE"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL` reader - PL"]
pub type PL_R = crate::FieldReader;
#[doc = "Field `PL` writer - PL"]
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BEX` reader - BEX"]
pub type BEX_R = crate::BitReader;
#[doc = "Field `BEX` writer - BEX"]
pub type BEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEX` reader - HEX"]
pub type HEX_R = crate::BitReader;
#[doc = "Field `HEX` writer - HEX"]
pub type HEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEX` reader - WEX"]
pub type WEX_R = crate::BitReader;
#[doc = "Field `WEX` writer - WEX"]
pub type WEX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRQ` writer - SWRQ"]
pub type SWRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTCIE"]
    #[inline(always)]
    pub fn ctcie(&self) -> CTCIE_R {
        CTCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - BRTIE"]
    #[inline(always)]
    pub fn brtie(&self) -> BRTIE_R {
        BRTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - BTIE"]
    #[inline(always)]
    pub fn btie(&self) -> BTIE_R {
        BTIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TCIE"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PL"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 12 - BEX"]
    #[inline(always)]
    pub fn bex(&self) -> BEX_R {
        BEX_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - HEX"]
    #[inline(always)]
    pub fn hex(&self) -> HEX_R {
        HEX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - WEX"]
    #[inline(always)]
    pub fn wex(&self) -> WEX_R {
        WEX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<MDMA_C4CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - TEIE"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<MDMA_C4CRrs> {
        TEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - CTCIE"]
    #[inline(always)]
    #[must_use]
    pub fn ctcie(&mut self) -> CTCIE_W<MDMA_C4CRrs> {
        CTCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - BRTIE"]
    #[inline(always)]
    #[must_use]
    pub fn brtie(&mut self) -> BRTIE_W<MDMA_C4CRrs> {
        BRTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - BTIE"]
    #[inline(always)]
    #[must_use]
    pub fn btie(&mut self) -> BTIE_W<MDMA_C4CRrs> {
        BTIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - TCIE"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<MDMA_C4CRrs> {
        TCIE_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - PL"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<MDMA_C4CRrs> {
        PL_W::new(self, 6)
    }
    #[doc = "Bit 12 - BEX"]
    #[inline(always)]
    #[must_use]
    pub fn bex(&mut self) -> BEX_W<MDMA_C4CRrs> {
        BEX_W::new(self, 12)
    }
    #[doc = "Bit 13 - HEX"]
    #[inline(always)]
    #[must_use]
    pub fn hex(&mut self) -> HEX_W<MDMA_C4CRrs> {
        HEX_W::new(self, 13)
    }
    #[doc = "Bit 14 - WEX"]
    #[inline(always)]
    #[must_use]
    pub fn wex(&mut self) -> WEX_W<MDMA_C4CRrs> {
        WEX_W::new(self, 14)
    }
    #[doc = "Bit 16 - SWRQ"]
    #[inline(always)]
    #[must_use]
    pub fn swrq(&mut self) -> SWRQ_W<MDMA_C4CRrs> {
        SWRQ_W::new(self, 16)
    }
}
#[doc = "This register is used to control the concerned channel.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mdma_c4cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mdma_c4cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MDMA_C4CRrs;
impl crate::RegisterSpec for MDMA_C4CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c4cr::R`](R) reader structure"]
impl crate::Readable for MDMA_C4CRrs {}
#[doc = "`write(|w| ..)` method takes [`mdma_c4cr::W`](W) writer structure"]
impl crate::Writable for MDMA_C4CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MDMA_C4CR to value 0"]
impl crate::Resettable for MDMA_C4CRrs {
    const RESET_VALUE: u32 = 0;
}
