#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `RXDMA` reader - Reception DMA enable"]
pub type RXDMA_R = crate::BitReader;
#[doc = "Field `RXDMA` writer - Reception DMA enable"]
pub type RXDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMA` reader - Transmission DMA enable"]
pub type TXDMA_R = crate::BitReader;
#[doc = "Field `TXDMA` writer - Transmission DMA enable"]
pub type TXDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMODE` reader - Reception buffering mode"]
pub type RXMODE_R = crate::BitReader;
#[doc = "Field `RXMODE` writer - Reception buffering mode"]
pub type RXMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMODE` reader - Transmission buffering mode"]
pub type TXMODE_R = crate::BitReader;
#[doc = "Field `TXMODE` writer - Transmission buffering mode"]
pub type TXMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPBK` reader - Loopback mode enable"]
pub type LPBK_R = crate::BitReader;
#[doc = "Field `LPBK` writer - Loopback mode enable"]
pub type LPBK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPME` reader - Single wire protocol master interface enable"]
pub type SWPME_R = crate::BitReader;
#[doc = "Field `SWPME` writer - Single wire protocol master interface enable"]
pub type SWPME_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEACT` reader - Single wire protocol master interface deactivate"]
pub type DEACT_R = crate::BitReader;
#[doc = "Field `DEACT` writer - Single wire protocol master interface deactivate"]
pub type DEACT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Single wire protocol master interface enable"]
    #[inline(always)]
    pub fn swpme(&self) -> SWPME_R {
        SWPME_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reception DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdma(&mut self) -> RXDMA_W<CRrs> {
        RXDMA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmission DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdma(&mut self) -> TXDMA_W<CRrs> {
        TXDMA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Reception buffering mode"]
    #[inline(always)]
    #[must_use]
    pub fn rxmode(&mut self) -> RXMODE_W<CRrs> {
        RXMODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transmission buffering mode"]
    #[inline(always)]
    #[must_use]
    pub fn txmode(&mut self) -> TXMODE_W<CRrs> {
        TXMODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Loopback mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpbk(&mut self) -> LPBK_W<CRrs> {
        LPBK_W::new(self, 4)
    }
    #[doc = "Bit 5 - Single wire protocol master interface enable"]
    #[inline(always)]
    #[must_use]
    pub fn swpme(&mut self) -> SWPME_W<CRrs> {
        SWPME_W::new(self, 5)
    }
    #[doc = "Bit 10 - Single wire protocol master interface deactivate"]
    #[inline(always)]
    #[must_use]
    pub fn deact(&mut self) -> DEACT_W<CRrs> {
        DEACT_W::new(self, 10)
    }
}
#[doc = "SWPMI Configuration/Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
