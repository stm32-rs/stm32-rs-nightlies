///Register `DMACRxCR` reader
pub type R = crate::R<DMACRX_CRrs>;
///Register `DMACRxCR` writer
pub type W = crate::W<DMACRX_CRrs>;
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
///Field `RPF` reader - DMA Rx Channel Packet Flush
pub type RPF_R = crate::BitReader;
///Field `RPF` writer - DMA Rx Channel Packet Flush
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
    ///Bit 31 - DMA Rx Channel Packet Flush
    #[inline(always)]
    pub fn rpf(&self) -> RPF_R {
        RPF_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACRxCR")
            .field("rpf", &self.rpf())
            .field("rxpbl", &self.rxpbl())
            .field("rbsz", &self.rbsz())
            .field("sr", &self.sr())
            .finish()
    }
}
impl W {
    ///Bit 0 - Start or Stop Receive
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W<'_, DMACRX_CRrs> {
        SR_W::new(self, 0)
    }
    ///Bits 1:14 - Receive Buffer size
    #[inline(always)]
    pub fn rbsz(&mut self) -> RBSZ_W<'_, DMACRX_CRrs> {
        RBSZ_W::new(self, 1)
    }
    ///Bits 16:21 - Receive Programmable Burst Length
    #[inline(always)]
    pub fn rxpbl(&mut self) -> RXPBL_W<'_, DMACRX_CRrs> {
        RXPBL_W::new(self, 16)
    }
    ///Bit 31 - DMA Rx Channel Packet Flush
    #[inline(always)]
    pub fn rpf(&mut self) -> RPF_W<'_, DMACRX_CRrs> {
        RPF_W::new(self, 31)
    }
}
/**Channel receive control register

You can [`read`](crate::Reg::read) this register and get [`dmacrx_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#Ethernet_DMA:DMACRxCR)*/
pub struct DMACRX_CRrs;
impl crate::RegisterSpec for DMACRX_CRrs {
    type Ux = u32;
}
///`read()` method returns [`dmacrx_cr::R`](R) reader structure
impl crate::Readable for DMACRX_CRrs {}
///`write(|w| ..)` method takes [`dmacrx_cr::W`](W) writer structure
impl crate::Writable for DMACRX_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACRxCR to value 0
impl crate::Resettable for DMACRX_CRrs {}
