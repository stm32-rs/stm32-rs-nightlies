///Register `DMAC0RxCR` reader
pub type R = crate::R<DMAC0RX_CRrs>;
///Register `DMAC0RxCR` writer
pub type W = crate::W<DMAC0RX_CRrs>;
///Field `SR` reader - Start or Stop Receive Command
pub type SR_R = crate::BitReader;
///Field `SR` writer - Start or Stop Receive Command
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBSZ` reader - Receive Buffer size
pub type RBSZ_R = crate::FieldReader<u16>;
///Field `RBSZ` writer - Receive Buffer size
pub type RBSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `RXPBL` reader - RXPBL
pub type RXPBL_R = crate::FieldReader;
///Field `RXPBL` writer - RXPBL
pub type RXPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RQOS` reader - RQOS
pub type RQOS_R = crate::FieldReader;
///Field `RQOS` writer - RQOS
pub type RQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RPF` reader - DMA Rx Channel Packet Flush
pub type RPF_R = crate::BitReader;
///Field `RPF` writer - DMA Rx Channel Packet Flush
pub type RPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Start or Stop Receive Command
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:14 - Receive Buffer size
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    ///Bits 16:21 - RXPBL
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:27 - RQOS
    #[inline(always)]
    pub fn rqos(&self) -> RQOS_R {
        RQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 31 - DMA Rx Channel Packet Flush
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC0RxCR")
            .field("sr", &self.sr())
            .field("rbsz", &self.rbsz())
            .field("rxpbl", &self.rxpbl())
            .field("rqos", &self.rqos())
            .field("rpf", &self.rpf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start or Stop Receive Command
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<'_, DMAC0RX_CRrs> {
        SR_W::new(self, 0)
    }
    ///Bits 1:14 - Receive Buffer size
    #[inline(always)]
    pub fn rbsz(&mut self) -> RBSZ_W<'_, DMAC0RX_CRrs> {
        RBSZ_W::new(self, 1)
    }
    ///Bits 16:21 - RXPBL
    #[inline(always)]
    pub fn rxpbl(&mut self) -> RXPBL_W<'_, DMAC0RX_CRrs> {
        RXPBL_W::new(self, 16)
    }
    ///Bits 24:27 - RQOS
    #[inline(always)]
    pub fn rqos(&mut self) -> RQOS_W<'_, DMAC0RX_CRrs> {
        RQOS_W::new(self, 24)
    }
    ///Bit 31 - DMA Rx Channel Packet Flush
    #[inline(always)]
    pub fn rpf(&mut self) -> RPF_W<'_, DMAC0RX_CRrs> {
        RPF_W::new(self, 31)
    }
}
/**Channel receive control register

You can [`read`](crate::Reg::read) this register and get [`dmac0rx_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac0rx_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#ETH_DMA:DMAC0RxCR)*/
pub struct DMAC0RX_CRrs;
impl crate::RegisterSpec for DMAC0RX_CRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac0rx_cr::R`](R) reader structure
impl crate::Readable for DMAC0RX_CRrs {}
///`write(|w| ..)` method takes [`dmac0rx_cr::W`](W) writer structure
impl crate::Writable for DMAC0RX_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC0RxCR to value 0x8000
impl crate::Resettable for DMAC0RX_CRrs {
    const RESET_VALUE: u32 = 0x8000;
}
