///Register `DMAC1RXCR` reader
pub type R = crate::R<DMAC1RXCRrs>;
///Register `DMAC1RXCR` writer
pub type W = crate::W<DMAC1RXCRrs>;
///Field `SR` reader - Start or Stop Receive
pub type SR_R = crate::BitReader;
///Field `SR` writer - Start or Stop Receive
pub type SR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RBSZ` reader - Receive Buffer size
pub type RBSZ_R = crate::FieldReader<u16>;
///Field `RBSZ` writer - Receive Buffer size
pub type RBSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
///Field `RXPBL` reader - Receive Programmable Burst Length
pub type RXPBL_R = crate::FieldReader;
///Field `RXPBL` writer - Receive Programmable Burst Length
pub type RXPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `RQOS` reader - Rx AXI4 QOS.
pub type RQOS_R = crate::FieldReader;
///Field `RQOS` writer - Rx AXI4 QOS.
pub type RQOS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `RPF` reader - DMA Rx Channel x Packet Flush
pub type RPF_R = crate::BitReader;
///Field `RPF` writer - DMA Rx Channel x Packet Flush
pub type RPF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Start or Stop Receive
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:14 - Receive Buffer size
    #[inline(always)]
    pub fn rbsz(&self) -> RBSZ_R {
        RBSZ_R::new(((self.bits >> 1) & 0x3fff) as u16)
    }
    ///Bits 16:21 - Receive Programmable Burst Length
    #[inline(always)]
    pub fn rxpbl(&self) -> RXPBL_R {
        RXPBL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    ///Bits 24:27 - Rx AXI4 QOS.
    #[inline(always)]
    pub fn rqos(&self) -> RQOS_R {
        RQOS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bit 31 - DMA Rx Channel x Packet Flush
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMAC1RXCR")
            .field("sr", &self.sr())
            .field("rbsz", &self.rbsz())
            .field("rxpbl", &self.rxpbl())
            .field("rqos", &self.rqos())
            .field("rpf", &self.rpf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start or Stop Receive
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<DMAC1RXCRrs> {
        SR_W::new(self, 0)
    }
    ///Bits 1:14 - Receive Buffer size
    #[inline(always)]
    pub fn rbsz(&mut self) -> RBSZ_W<DMAC1RXCRrs> {
        RBSZ_W::new(self, 1)
    }
    ///Bits 16:21 - Receive Programmable Burst Length
    #[inline(always)]
    pub fn rxpbl(&mut self) -> RXPBL_W<DMAC1RXCRrs> {
        RXPBL_W::new(self, 16)
    }
    ///Bits 24:27 - Rx AXI4 QOS.
    #[inline(always)]
    pub fn rqos(&mut self) -> RQOS_W<DMAC1RXCRrs> {
        RQOS_W::new(self, 24)
    }
    ///Bit 31 - DMA Rx Channel x Packet Flush
    #[inline(always)]
    pub fn rpf(&mut self) -> RPF_W<DMAC1RXCRrs> {
        RPF_W::new(self, 31)
    }
}
/**Channel 1 receive control register

You can [`read`](crate::Reg::read) this register and get [`dmac1rxcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmac1rxcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:DMAC1RXCR)*/
pub struct DMAC1RXCRrs;
impl crate::RegisterSpec for DMAC1RXCRrs {
    type Ux = u32;
}
///`read()` method returns [`dmac1rxcr::R`](R) reader structure
impl crate::Readable for DMAC1RXCRrs {}
///`write(|w| ..)` method takes [`dmac1rxcr::W`](W) writer structure
impl crate::Writable for DMAC1RXCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMAC1RXCR to value 0
impl crate::Resettable for DMAC1RXCRrs {}
