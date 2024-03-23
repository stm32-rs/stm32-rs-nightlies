#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `JCEN` reader - JPEG Core Enable"]
pub type JCEN_R = crate::BitReader;
#[doc = "Field `JCEN` writer - JPEG Core Enable"]
pub type JCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFTIE` reader - Input FIFO Threshold Interrupt Enable"]
pub type IFTIE_R = crate::BitReader;
#[doc = "Field `IFTIE` writer - Input FIFO Threshold Interrupt Enable"]
pub type IFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFNFIE` reader - Input FIFO Not Full Interrupt Enable"]
pub type IFNFIE_R = crate::BitReader;
#[doc = "Field `IFNFIE` writer - Input FIFO Not Full Interrupt Enable"]
pub type IFNFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFTIE` reader - Output FIFO Threshold Interrupt Enable"]
pub type OFTIE_R = crate::BitReader;
#[doc = "Field `OFTIE` writer - Output FIFO Threshold Interrupt Enable"]
pub type OFTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OFNEIE` reader - Output FIFO Not Empty Interrupt Enable"]
pub type OFNEIE_R = crate::BitReader;
#[doc = "Field `OFNEIE` writer - Output FIFO Not Empty Interrupt Enable"]
pub type OFNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - End of Conversion Interrupt Enable"]
pub type EOCIE_R = crate::BitReader;
#[doc = "Field `EOCIE` writer - End of Conversion Interrupt Enable"]
pub type EOCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPDIE` reader - Header Parsing Done Interrupt Enable"]
pub type HPDIE_R = crate::BitReader;
#[doc = "Field `HPDIE` writer - Header Parsing Done Interrupt Enable"]
pub type HPDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMAEN` reader - Input DMA Enable"]
pub type IDMAEN_R = crate::BitReader;
#[doc = "Field `IDMAEN` writer - Input DMA Enable"]
pub type IDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ODMAEN` reader - Output DMA Enable"]
pub type ODMAEN_R = crate::BitReader;
#[doc = "Field `ODMAEN` writer - Output DMA Enable"]
pub type ODMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFF` reader - Input FIFO Flush"]
pub type IFF_R = crate::BitReader;
#[doc = "Field `OFF` reader - Output FIFO Flush"]
pub type OFF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - JPEG Core Enable"]
    #[inline(always)]
    pub fn jcen(&self) -> JCEN_R {
        JCEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Input FIFO Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn iftie(&self) -> IFTIE_R {
        IFTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Interrupt Enable"]
    #[inline(always)]
    pub fn ifnfie(&self) -> IFNFIE_R {
        IFNFIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Interrupt Enable"]
    #[inline(always)]
    pub fn oftie(&self) -> OFTIE_R {
        OFTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Interrupt Enable"]
    #[inline(always)]
    pub fn ofneie(&self) -> OFNEIE_R {
        OFNEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable"]
    #[inline(always)]
    pub fn eocie(&self) -> EOCIE_R {
        EOCIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Header Parsing Done Interrupt Enable"]
    #[inline(always)]
    pub fn hpdie(&self) -> HPDIE_R {
        HPDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 11 - Input DMA Enable"]
    #[inline(always)]
    pub fn idmaen(&self) -> IDMAEN_R {
        IDMAEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Output DMA Enable"]
    #[inline(always)]
    pub fn odmaen(&self) -> ODMAEN_R {
        ODMAEN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Input FIFO Flush"]
    #[inline(always)]
    pub fn iff(&self) -> IFF_R {
        IFF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output FIFO Flush"]
    #[inline(always)]
    pub fn off(&self) -> OFF_R {
        OFF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - JPEG Core Enable"]
    #[inline(always)]
    #[must_use]
    pub fn jcen(&mut self) -> JCEN_W<CRrs> {
        JCEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Input FIFO Threshold Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iftie(&mut self) -> IFTIE_W<CRrs> {
        IFTIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Input FIFO Not Full Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ifnfie(&mut self) -> IFNFIE_W<CRrs> {
        IFNFIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Output FIFO Threshold Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn oftie(&mut self) -> OFTIE_W<CRrs> {
        OFTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Output FIFO Not Empty Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ofneie(&mut self) -> OFNEIE_W<CRrs> {
        OFNEIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - End of Conversion Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eocie(&mut self) -> EOCIE_W<CRrs> {
        EOCIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Header Parsing Done Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hpdie(&mut self) -> HPDIE_W<CRrs> {
        HPDIE_W::new(self, 6)
    }
    #[doc = "Bit 11 - Input DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn idmaen(&mut self) -> IDMAEN_W<CRrs> {
        IDMAEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Output DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn odmaen(&mut self) -> ODMAEN_W<CRrs> {
        ODMAEN_W::new(self, 12)
    }
}
#[doc = "JPEG control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
