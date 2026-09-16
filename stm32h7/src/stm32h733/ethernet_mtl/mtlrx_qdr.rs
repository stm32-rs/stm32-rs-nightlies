///Register `MTLRxQDR` reader
pub type R = crate::R<MTLRX_QDRrs>;
///Register `MTLRxQDR` writer
pub type W = crate::W<MTLRX_QDRrs>;
///Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status
pub type RWCSTS_R = crate::BitReader;
///Field `RWCSTS` writer - MTL Rx Queue Write Controller Active Status
pub type RWCSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRCSTS` reader - MTL Rx Queue Read Controller State
pub type RRCSTS_R = crate::FieldReader;
///Field `RRCSTS` writer - MTL Rx Queue Read Controller State
pub type RRCSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status
pub type RXQSTS_R = crate::FieldReader;
///Field `RXQSTS` writer - MTL Rx Queue Fill-Level Status
pub type RXQSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PRXQ` reader - Number of Packets in Receive Queue
pub type PRXQ_R = crate::FieldReader<u16>;
///Field `PRXQ` writer - Number of Packets in Receive Queue
pub type PRXQ_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    ///Bit 0 - MTL Rx Queue Write Controller Active Status
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:2 - MTL Rx Queue Read Controller State
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    ///Bits 4:5 - MTL Rx Queue Fill-Level Status
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    ///Bits 16:29 - Number of Packets in Receive Queue
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRxQDR")
            .field("prxq", &self.prxq())
            .field("rxqsts", &self.rxqsts())
            .field("rrcsts", &self.rrcsts())
            .field("rwcsts", &self.rwcsts())
            .finish()
    }
}
impl W {
    ///Bit 0 - MTL Rx Queue Write Controller Active Status
    #[inline(always)]
    pub fn rwcsts(&mut self) -> RWCSTS_W<'_, MTLRX_QDRrs> {
        RWCSTS_W::new(self, 0)
    }
    ///Bits 1:2 - MTL Rx Queue Read Controller State
    #[inline(always)]
    pub fn rrcsts(&mut self) -> RRCSTS_W<'_, MTLRX_QDRrs> {
        RRCSTS_W::new(self, 1)
    }
    ///Bits 4:5 - MTL Rx Queue Fill-Level Status
    #[inline(always)]
    pub fn rxqsts(&mut self) -> RXQSTS_W<'_, MTLRX_QDRrs> {
        RXQSTS_W::new(self, 4)
    }
    ///Bits 16:29 - Number of Packets in Receive Queue
    #[inline(always)]
    pub fn prxq(&mut self) -> PRXQ_W<'_, MTLRX_QDRrs> {
        PRXQ_W::new(self, 16)
    }
}
/**Rx queue debug register

You can [`read`](crate::Reg::read) this register and get [`mtlrx_qdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrx_qdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#Ethernet_MTL:MTLRxQDR)*/
pub struct MTLRX_QDRrs;
impl crate::RegisterSpec for MTLRX_QDRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrx_qdr::R`](R) reader structure
impl crate::Readable for MTLRX_QDRrs {}
///`write(|w| ..)` method takes [`mtlrx_qdr::W`](W) writer structure
impl crate::Writable for MTLRX_QDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLRxQDR to value 0
impl crate::Resettable for MTLRX_QDRrs {}
