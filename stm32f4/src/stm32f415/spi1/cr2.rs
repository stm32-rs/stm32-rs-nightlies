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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("txeie", &self.txeie())
            .field("rxneie", &self.rxneie())
            .field("errie", &self.errie())
            .field("frf", &self.frf())
            .field("ssoe", &self.ssoe())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, CR2rs> {
        RXDMAEN_W::new(self, 0)
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, CR2rs> {
        TXDMAEN_W::new(self, 1)
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W<'_, CR2rs> {
        SSOE_W::new(self, 2)
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W<'_, CR2rs> {
        FRF_W::new(self, 4)
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CR2rs> {
        ERRIE_W::new(self, 5)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<'_, CR2rs> {
        RXNEIE_W::new(self, 6)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<'_, CR2rs> {
        TXEIE_W::new(self, 7)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SPI1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
