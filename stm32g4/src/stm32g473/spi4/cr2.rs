///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `RXDMAEN` reader - Rx buffer DMA enable
pub type RXDMAEN_R = crate::BitReader;
///Field `RXDMAEN` writer - Rx buffer DMA enable
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMAEN` reader - Tx buffer DMA enable
pub type TXDMAEN_R = crate::BitReader;
///Field `TXDMAEN` writer - Tx buffer DMA enable
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSOE` reader - SS output enable
pub type SSOE_R = crate::BitReader;
///Field `SSOE` writer - SS output enable
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NSSP` reader - NSS pulse management
pub type NSSP_R = crate::BitReader;
///Field `NSSP` writer - NSS pulse management
pub type NSSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FRF` reader - Frame format
pub type FRF_R = crate::BitReader;
///Field `FRF` writer - Frame format
pub type FRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXNEIE` reader - RX buffer not empty interrupt enable
pub type RXNEIE_R = crate::BitReader;
///Field `RXNEIE` writer - RX buffer not empty interrupt enable
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEIE` reader - Tx buffer empty interrupt enable
pub type TXEIE_R = crate::BitReader;
///Field `TXEIE` writer - Tx buffer empty interrupt enable
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DS` reader - Data size
pub type DS_R = crate::FieldReader;
///Field `DS` writer - Data size
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `FRXTH` reader - FIFO reception threshold
pub type FRXTH_R = crate::BitReader;
///Field `FRXTH` writer - FIFO reception threshold
pub type FRXTH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDMA_RX` reader - Last DMA transfer for reception
pub type LDMA_RX_R = crate::BitReader;
///Field `LDMA_RX` writer - Last DMA transfer for reception
pub type LDMA_RX_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LDMA_TX` reader - Last DMA transfer for transmission
pub type LDMA_TX_R = crate::BitReader;
///Field `LDMA_TX` writer - Last DMA transfer for transmission
pub type LDMA_TX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NSS pulse management
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - Data size
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - FIFO reception threshold
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Last DMA transfer for reception
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Last DMA transfer for transmission
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("rxdmaen", &self.rxdmaen())
            .field("txdmaen", &self.txdmaen())
            .field("ssoe", &self.ssoe())
            .field("nssp", &self.nssp())
            .field("frf", &self.frf())
            .field("errie", &self.errie())
            .field("rxneie", &self.rxneie())
            .field("txeie", &self.txeie())
            .field("ds", &self.ds())
            .field("frxth", &self.frxth())
            .field("ldma_rx", &self.ldma_rx())
            .field("ldma_tx", &self.ldma_tx())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CR2rs> {
        RXDMAEN_W::new(self, 0)
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CR2rs> {
        TXDMAEN_W::new(self, 1)
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W<CR2rs> {
        SSOE_W::new(self, 2)
    }
    ///Bit 3 - NSS pulse management
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W<CR2rs> {
        NSSP_W::new(self, 3)
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W<CR2rs> {
        FRF_W::new(self, 4)
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<CR2rs> {
        ERRIE_W::new(self, 5)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<CR2rs> {
        RXNEIE_W::new(self, 6)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<CR2rs> {
        TXEIE_W::new(self, 7)
    }
    ///Bits 8:11 - Data size
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W<CR2rs> {
        DS_W::new(self, 8)
    }
    ///Bit 12 - FIFO reception threshold
    #[inline(always)]
    pub fn frxth(&mut self) -> FRXTH_W<CR2rs> {
        FRXTH_W::new(self, 12)
    }
    ///Bit 13 - Last DMA transfer for reception
    #[inline(always)]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W<CR2rs> {
        LDMA_RX_W::new(self, 13)
    }
    ///Bit 14 - Last DMA transfer for transmission
    #[inline(always)]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W<CR2rs> {
        LDMA_TX_W::new(self, 14)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#SPI4:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u16 = 0;
}
