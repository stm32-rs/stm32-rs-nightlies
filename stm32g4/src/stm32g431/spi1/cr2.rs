#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable"]
pub type RXDMAEN_R = crate::BitReader;
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable"]
pub type TXDMAEN_R = crate::BitReader;
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSOE` reader - SS output enable"]
pub type SSOE_R = crate::BitReader;
#[doc = "Field `SSOE` writer - SS output enable"]
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NSSP` reader - NSS pulse management"]
pub type NSSP_R = crate::BitReader;
#[doc = "Field `NSSP` writer - NSS pulse management"]
pub type NSSP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRF` reader - Frame format"]
pub type FRF_R = crate::BitReader;
#[doc = "Field `FRF` writer - Frame format"]
pub type FRF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader;
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable"]
pub type RXNEIE_R = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable"]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable"]
pub type TXEIE_R = crate::BitReader;
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable"]
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DS` reader - Data size"]
pub type DS_R = crate::FieldReader;
#[doc = "Field `DS` writer - Data size"]
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FRXTH` reader - FIFO reception threshold"]
pub type FRXTH_R = crate::BitReader;
#[doc = "Field `FRXTH` writer - FIFO reception threshold"]
pub type FRXTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA_RX` reader - Last DMA transfer for reception"]
pub type LDMA_RX_R = crate::BitReader;
#[doc = "Field `LDMA_RX` writer - Last DMA transfer for reception"]
pub type LDMA_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LDMA_TX` reader - Last DMA transfer for transmission"]
pub type LDMA_TX_R = crate::BitReader;
#[doc = "Field `LDMA_TX` writer - Last DMA transfer for transmission"]
pub type LDMA_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CR2rs> {
        RXDMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CR2rs> {
        TXDMAEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<CR2rs> {
        SSOE_W::new(self, 2)
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    #[must_use]
    pub fn nssp(&mut self) -> NSSP_W<CR2rs> {
        NSSP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<CR2rs> {
        FRF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CR2rs> {
        ERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<CR2rs> {
        RXNEIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<CR2rs> {
        TXEIE_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<CR2rs> {
        DS_W::new(self, 8)
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    #[must_use]
    pub fn frxth(&mut self) -> FRXTH_W<CR2rs> {
        FRXTH_W::new(self, 12)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    #[must_use]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W<CR2rs> {
        LDMA_RX_W::new(self, 13)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W<CR2rs> {
        LDMA_TX_W::new(self, 14)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0x0700"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0x0700;
}
