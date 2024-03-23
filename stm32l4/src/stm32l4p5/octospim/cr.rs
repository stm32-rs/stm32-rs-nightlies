#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Field `MUXEN` reader - Multiplexed mode enable"]
pub type MUXEN_R = crate::BitReader;
#[doc = "Field `MUXEN` writer - Multiplexed mode enable"]
pub type MUXEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REQ2ACK_TIME` reader - REQ to ACK time"]
pub type REQ2ACK_TIME_R = crate::FieldReader;
#[doc = "Field `REQ2ACK_TIME` writer - REQ to ACK time"]
pub type REQ2ACK_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Multiplexed mode enable"]
    #[inline(always)]
    pub fn muxen(&self) -> MUXEN_R {
        MUXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:23 - REQ to ACK time"]
    #[inline(always)]
    pub fn req2ack_time(&self) -> REQ2ACK_TIME_R {
        REQ2ACK_TIME_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Multiplexed mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn muxen(&mut self) -> MUXEN_W<CRrs> {
        MUXEN_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - REQ to ACK time"]
    #[inline(always)]
    #[must_use]
    pub fn req2ack_time(&mut self) -> REQ2ACK_TIME_W<CRrs> {
        REQ2ACK_TIME_W::new(self, 16)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
