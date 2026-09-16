///Register `MTLTxQDR` reader
pub type R = crate::R<MTLTX_QDRrs>;
///Register `MTLTxQDR` writer
pub type W = crate::W<MTLTX_QDRrs>;
///Field `TXQPAUSED` reader - Transmit Queue in Pause
pub type TXQPAUSED_R = crate::BitReader;
///Field `TXQPAUSED` writer - Transmit Queue in Pause
pub type TXQPAUSED_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRCSTS` reader - MTL Tx Queue Read Controller Status
pub type TRCSTS_R = crate::FieldReader;
///Field `TRCSTS` writer - MTL Tx Queue Read Controller Status
pub type TRCSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `TWCSTS` reader - MTL Tx Queue Write Controller Status
pub type TWCSTS_R = crate::BitReader;
///Field `TWCSTS` writer - MTL Tx Queue Write Controller Status
pub type TWCSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXQSTS` reader - MTL Tx Queue Not Empty Status
pub type TXQSTS_R = crate::BitReader;
///Field `TXQSTS` writer - MTL Tx Queue Not Empty Status
pub type TXQSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSTSFSTS` reader - MTL Tx Status FIFO Full Status
pub type TXSTSFSTS_R = crate::BitReader;
///Field `TXSTSFSTS` writer - MTL Tx Status FIFO Full Status
pub type TXSTSFSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PTXQ` reader - Number of Packets in the Transmit Queue
pub type PTXQ_R = crate::FieldReader;
///Field `PTXQ` writer - Number of Packets in the Transmit Queue
pub type PTXQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `STXSTSF` reader - Number of Status Words in Tx Status FIFO of Queue
pub type STXSTSF_R = crate::FieldReader;
///Field `STXSTSF` writer - Number of Status Words in Tx Status FIFO of Queue
pub type STXSTSF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bit 0 - Transmit Queue in Pause
    #[inline(always)]
    pub fn txqpaused(&self) -> TXQPAUSED_R {
        TXQPAUSED_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - MTL Tx Queue Read Controller Status
    #[inline(always)]
    pub fn trcsts(&self) -> TRCSTS_R {
        TRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bit 3 - MTL Tx Queue Write Controller Status
    #[inline(always)]
    pub fn twcsts(&self) -> TWCSTS_R {
        TWCSTS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - MTL Tx Queue Not Empty Status
    #[inline(always)]
    pub fn txqsts(&self) -> TXQSTS_R {
        TXQSTS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - MTL Tx Status FIFO Full Status
    #[inline(always)]
    pub fn txstsfsts(&self) -> TXSTSFSTS_R {
        TXSTSFSTS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 16:18 - Number of Packets in the Transmit Queue
    #[inline(always)]
    pub fn ptxq(&self) -> PTXQ_R {
        PTXQ_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue
    #[inline(always)]
    pub fn stxstsf(&self) -> STXSTSF_R {
        STXSTSF_R::new(((self.bits >> 20) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQDR")
            .field("stxstsf", &self.stxstsf())
            .field("ptxq", &self.ptxq())
            .field("txstsfsts", &self.txstsfsts())
            .field("txqsts", &self.txqsts())
            .field("twcsts", &self.twcsts())
            .field("trcsts", &self.trcsts())
            .field("txqpaused", &self.txqpaused())
            .finish()
    }
}
impl W {
    ///Bit 0 - Transmit Queue in Pause
    #[inline(always)]
    pub fn txqpaused(&mut self) -> TXQPAUSED_W<'_, MTLTX_QDRrs> {
        TXQPAUSED_W::new(self, 0)
    }
    ///Bits 1:2 - MTL Tx Queue Read Controller Status
    #[inline(always)]
    pub fn trcsts(&mut self) -> TRCSTS_W<'_, MTLTX_QDRrs> {
        TRCSTS_W::new(self, 1)
    }
    ///Bit 3 - MTL Tx Queue Write Controller Status
    #[inline(always)]
    pub fn twcsts(&mut self) -> TWCSTS_W<'_, MTLTX_QDRrs> {
        TWCSTS_W::new(self, 3)
    }
    ///Bit 4 - MTL Tx Queue Not Empty Status
    #[inline(always)]
    pub fn txqsts(&mut self) -> TXQSTS_W<'_, MTLTX_QDRrs> {
        TXQSTS_W::new(self, 4)
    }
    ///Bit 5 - MTL Tx Status FIFO Full Status
    #[inline(always)]
    pub fn txstsfsts(&mut self) -> TXSTSFSTS_W<'_, MTLTX_QDRrs> {
        TXSTSFSTS_W::new(self, 5)
    }
    ///Bits 16:18 - Number of Packets in the Transmit Queue
    #[inline(always)]
    pub fn ptxq(&mut self) -> PTXQ_W<'_, MTLTX_QDRrs> {
        PTXQ_W::new(self, 16)
    }
    ///Bits 20:22 - Number of Status Words in Tx Status FIFO of Queue
    #[inline(always)]
    pub fn stxstsf(&mut self) -> STXSTSF_W<'_, MTLTX_QDRrs> {
        STXSTSF_W::new(self, 20)
    }
}
/**Tx queue debug Register

You can [`read`](crate::Reg::read) this register and get [`mtltx_qdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_qdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H747_CM7.html#Ethernet_MTL:MTLTxQDR)*/
pub struct MTLTX_QDRrs;
impl crate::RegisterSpec for MTLTX_QDRrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_qdr::R`](R) reader structure
impl crate::Readable for MTLTX_QDRrs {}
///`write(|w| ..)` method takes [`mtltx_qdr::W`](W) writer structure
impl crate::Writable for MTLTX_QDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTxQDR to value 0
impl crate::Resettable for MTLTX_QDRrs {}
