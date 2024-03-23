#[doc = "Register `MTLRxQDR` reader"]
pub type R = crate::R<MTLRX_QDRrs>;
#[doc = "Register `MTLRxQDR` writer"]
pub type W = crate::W<MTLRX_QDRrs>;
#[doc = "Field `RWCSTS` reader - MTL Rx Queue Write Controller Active Status"]
pub type RWCSTS_R = crate::BitReader;
#[doc = "Field `RWCSTS` writer - MTL Rx Queue Write Controller Active Status"]
pub type RWCSTS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRCSTS` reader - MTL Rx Queue Read Controller State"]
pub type RRCSTS_R = crate::FieldReader;
#[doc = "Field `RRCSTS` writer - MTL Rx Queue Read Controller State"]
pub type RRCSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RXQSTS` reader - MTL Rx Queue Fill-Level Status"]
pub type RXQSTS_R = crate::FieldReader;
#[doc = "Field `RXQSTS` writer - MTL Rx Queue Fill-Level Status"]
pub type RXQSTS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRXQ` reader - Number of Packets in Receive Queue"]
pub type PRXQ_R = crate::FieldReader<u16>;
#[doc = "Field `PRXQ` writer - Number of Packets in Receive Queue"]
pub type PRXQ_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RWCSTS_R {
        RWCSTS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RRCSTS_R {
        RRCSTS_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RXQSTS_R {
        RXQSTS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    pub fn prxq(&self) -> PRXQ_R {
        PRXQ_R::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - MTL Rx Queue Write Controller Active Status"]
    #[inline(always)]
    #[must_use]
    pub fn rwcsts(&mut self) -> RWCSTS_W<MTLRX_QDRrs> {
        RWCSTS_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - MTL Rx Queue Read Controller State"]
    #[inline(always)]
    #[must_use]
    pub fn rrcsts(&mut self) -> RRCSTS_W<MTLRX_QDRrs> {
        RRCSTS_W::new(self, 1)
    }
    #[doc = "Bits 4:5 - MTL Rx Queue Fill-Level Status"]
    #[inline(always)]
    #[must_use]
    pub fn rxqsts(&mut self) -> RXQSTS_W<MTLRX_QDRrs> {
        RXQSTS_W::new(self, 4)
    }
    #[doc = "Bits 16:29 - Number of Packets in Receive Queue"]
    #[inline(always)]
    #[must_use]
    pub fn prxq(&mut self) -> PRXQ_W<MTLRX_QDRrs> {
        PRXQ_W::new(self, 16)
    }
}
#[doc = "Rx queue debug register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtlrx_qdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtlrx_qdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLRX_QDRrs;
impl crate::RegisterSpec for MTLRX_QDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qdr::R`](R) reader structure"]
impl crate::Readable for MTLRX_QDRrs {}
#[doc = "`write(|w| ..)` method takes [`mtlrx_qdr::W`](W) writer structure"]
impl crate::Writable for MTLRX_QDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTLRxQDR to value 0"]
impl crate::Resettable for MTLRX_QDRrs {
    const RESET_VALUE: u32 = 0;
}
