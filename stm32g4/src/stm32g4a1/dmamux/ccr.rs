#[doc = "Register `CCR%s` reader"]
pub type R = crate::R<CCRrs>;
#[doc = "Register `CCR%s` writer"]
pub type W = crate::W<CCRrs>;
#[doc = "Field `DMAREQ_ID` reader - Input DMA request line selected"]
pub type DMAREQ_ID_R = crate::FieldReader;
#[doc = "Field `DMAREQ_ID` writer - Input DMA request line selected"]
pub type DMAREQ_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SOIE` reader - Interrupt enable at synchronization event overrun"]
pub type SOIE_R = crate::BitReader;
#[doc = "Field `SOIE` writer - Interrupt enable at synchronization event overrun"]
pub type SOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EGE` reader - Event generation enable/disable"]
pub type EGE_R = crate::BitReader;
#[doc = "Field `EGE` writer - Event generation enable/disable"]
pub type EGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE` reader - Synchronous operating mode enable/disable"]
pub type SE_R = crate::BitReader;
#[doc = "Field `SE` writer - Synchronous operating mode enable/disable"]
pub type SE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPOL` reader - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SPOL_R = crate::FieldReader;
#[doc = "Field `SPOL` writer - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
pub type SPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `NBREQ` reader - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NBREQ_R = crate::FieldReader;
#[doc = "Field `NBREQ` writer - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
pub type NBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SYNC_ID` reader - Synchronization input selected"]
pub type SYNC_ID_R = crate::FieldReader;
#[doc = "Field `SYNC_ID` writer - Synchronization input selected"]
pub type SYNC_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:6 - Input DMA request line selected"]
    #[inline(always)]
    pub fn dmareq_id(&self) -> DMAREQ_ID_R {
        DMAREQ_ID_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    pub fn soie(&self) -> SOIE_R {
        SOIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    pub fn ege(&self) -> EGE_R {
        EGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    pub fn se(&self) -> SE_R {
        SE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    pub fn spol(&self) -> SPOL_R {
        SPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    pub fn nbreq(&self) -> NBREQ_R {
        NBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    pub fn sync_id(&self) -> SYNC_ID_R {
        SYNC_ID_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Input DMA request line selected"]
    #[inline(always)]
    #[must_use]
    pub fn dmareq_id(&mut self) -> DMAREQ_ID_W<CCRrs> {
        DMAREQ_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable at synchronization event overrun"]
    #[inline(always)]
    #[must_use]
    pub fn soie(&mut self) -> SOIE_W<CCRrs> {
        SOIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Event generation enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn ege(&mut self) -> EGE_W<CCRrs> {
        EGE_W::new(self, 9)
    }
    #[doc = "Bit 16 - Synchronous operating mode enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SE_W<CCRrs> {
        SE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - Synchronization event type selector Defines the synchronization event on the selected synchronization input:"]
    #[inline(always)]
    #[must_use]
    pub fn spol(&mut self) -> SPOL_W<CCRrs> {
        SPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to forward Defines the number of DMA requests forwarded before output event is generated. In synchronous mode, it also defines the number of DMA requests to forward after a synchronization event, then stop forwarding. The actual number of DMA requests forwarded is NBREQ+1. Note: This field can only be written when both SE and EGE bits are reset."]
    #[inline(always)]
    #[must_use]
    pub fn nbreq(&mut self) -> NBREQ_W<CCRrs> {
        NBREQ_W::new(self, 19)
    }
    #[doc = "Bits 24:28 - Synchronization input selected"]
    #[inline(always)]
    #[must_use]
    pub fn sync_id(&mut self) -> SYNC_ID_W<CCRrs> {
        SYNC_ID_W::new(self, 24)
    }
}
#[doc = "DMA Multiplexer Channel %s Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CCRrs {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR%s to value 0"]
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0;
}
