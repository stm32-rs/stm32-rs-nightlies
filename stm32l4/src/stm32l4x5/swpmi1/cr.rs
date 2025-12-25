///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `RXDMA` reader - Reception DMA enable
pub type RXDMA_R = crate::BitReader;
///Field `RXDMA` writer - Reception DMA enable
pub type RXDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMA` reader - Transmission DMA enable
pub type TXDMA_R = crate::BitReader;
///Field `TXDMA` writer - Transmission DMA enable
pub type TXDMA_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXMODE` reader - Reception buffering mode
pub type RXMODE_R = crate::BitReader;
///Field `RXMODE` writer - Reception buffering mode
pub type RXMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXMODE` reader - Transmission buffering mode
pub type TXMODE_R = crate::BitReader;
///Field `TXMODE` writer - Transmission buffering mode
pub type TXMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LPBK` reader - Loopback mode enable
pub type LPBK_R = crate::BitReader;
///Field `LPBK` writer - Loopback mode enable
pub type LPBK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWPME` reader - Single wire protocol master interface enable
pub type SWPME_R = crate::BitReader;
///Field `SWPME` writer - Single wire protocol master interface enable
pub type SWPME_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DEACT` reader - Single wire protocol master interface deactivate
pub type DEACT_R = crate::BitReader;
///Field `DEACT` writer - Single wire protocol master interface deactivate
pub type DEACT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Reception DMA enable
    #[inline(always)]
    pub fn rxdma(&self) -> RXDMA_R {
        RXDMA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transmission DMA enable
    #[inline(always)]
    pub fn txdma(&self) -> TXDMA_R {
        TXDMA_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Reception buffering mode
    #[inline(always)]
    pub fn rxmode(&self) -> RXMODE_R {
        RXMODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transmission buffering mode
    #[inline(always)]
    pub fn txmode(&self) -> TXMODE_R {
        TXMODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Loopback mode enable
    #[inline(always)]
    pub fn lpbk(&self) -> LPBK_R {
        LPBK_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Single wire protocol master interface enable
    #[inline(always)]
    pub fn swpme(&self) -> SWPME_R {
        SWPME_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 10 - Single wire protocol master interface deactivate
    #[inline(always)]
    pub fn deact(&self) -> DEACT_R {
        DEACT_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("rxdma", &self.rxdma())
            .field("txdma", &self.txdma())
            .field("rxmode", &self.rxmode())
            .field("txmode", &self.txmode())
            .field("lpbk", &self.lpbk())
            .field("swpme", &self.swpme())
            .field("deact", &self.deact())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reception DMA enable
    #[inline(always)]
    pub fn rxdma(&mut self) -> RXDMA_W<'_, CRrs> {
        RXDMA_W::new(self, 0)
    }
    ///Bit 1 - Transmission DMA enable
    #[inline(always)]
    pub fn txdma(&mut self) -> TXDMA_W<'_, CRrs> {
        TXDMA_W::new(self, 1)
    }
    ///Bit 2 - Reception buffering mode
    #[inline(always)]
    pub fn rxmode(&mut self) -> RXMODE_W<'_, CRrs> {
        RXMODE_W::new(self, 2)
    }
    ///Bit 3 - Transmission buffering mode
    #[inline(always)]
    pub fn txmode(&mut self) -> TXMODE_W<'_, CRrs> {
        TXMODE_W::new(self, 3)
    }
    ///Bit 4 - Loopback mode enable
    #[inline(always)]
    pub fn lpbk(&mut self) -> LPBK_W<'_, CRrs> {
        LPBK_W::new(self, 4)
    }
    ///Bit 5 - Single wire protocol master interface enable
    #[inline(always)]
    pub fn swpme(&mut self) -> SWPME_W<'_, CRrs> {
        SWPME_W::new(self, 5)
    }
    ///Bit 10 - Single wire protocol master interface deactivate
    #[inline(always)]
    pub fn deact(&mut self) -> DEACT_W<'_, CRrs> {
        DEACT_W::new(self, 10)
    }
}
/**SWPMI Configuration/Control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#SWPMI1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
