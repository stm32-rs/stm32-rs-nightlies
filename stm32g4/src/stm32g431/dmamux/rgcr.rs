#[doc = "Register `RGCR%s` reader"]
pub type R = crate::R<RGCRrs>;
#[doc = "Register `RGCR%s` writer"]
pub type W = crate::W<RGCRrs>;
#[doc = "Field `SIG_ID` reader - DMA request trigger input selected"]
pub type SIG_ID_R = crate::FieldReader;
#[doc = "Field `SIG_ID` writer - DMA request trigger input selected"]
pub type SIG_ID_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OIE` reader - Interrupt enable at trigger event overrun"]
pub type OIE_R = crate::BitReader;
#[doc = "Field `OIE` writer - Interrupt enable at trigger event overrun"]
pub type OIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GE` reader - DMA request generator channel enable/disable"]
pub type GE_R = crate::BitReader;
#[doc = "Field `GE` writer - DMA request generator channel enable/disable"]
pub type GE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPOL` reader - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_R = crate::FieldReader;
#[doc = "Field `GPOL` writer - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
pub type GPOL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GNBREQ` reader - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_R = crate::FieldReader;
#[doc = "Field `GNBREQ` writer - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
pub type GNBREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    pub fn sig_id(&self) -> SIG_ID_R {
        SIG_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    pub fn oie(&self) -> OIE_R {
        OIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    pub fn ge(&self) -> GE_R {
        GE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    pub fn gpol(&self) -> GPOL_R {
        GPOL_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    pub fn gnbreq(&self) -> GNBREQ_R {
        GNBREQ_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - DMA request trigger input selected"]
    #[inline(always)]
    #[must_use]
    pub fn sig_id(&mut self) -> SIG_ID_W<RGCRrs> {
        SIG_ID_W::new(self, 0)
    }
    #[doc = "Bit 8 - Interrupt enable at trigger event overrun"]
    #[inline(always)]
    #[must_use]
    pub fn oie(&mut self) -> OIE_W<RGCRrs> {
        OIE_W::new(self, 8)
    }
    #[doc = "Bit 16 - DMA request generator channel enable/disable"]
    #[inline(always)]
    #[must_use]
    pub fn ge(&mut self) -> GE_W<RGCRrs> {
        GE_W::new(self, 16)
    }
    #[doc = "Bits 17:18 - DMA request generator trigger event type selection Defines the trigger event on the selected DMA request trigger input"]
    #[inline(always)]
    #[must_use]
    pub fn gpol(&mut self) -> GPOL_W<RGCRrs> {
        GPOL_W::new(self, 17)
    }
    #[doc = "Bits 19:23 - Number of DMA requests to generate Defines the number of DMA requests generated after a trigger event, then stop generating. The actual number of generated DMA requests is GNBREQ+1. Note: This field can only be written when GE bit is reset."]
    #[inline(always)]
    #[must_use]
    pub fn gnbreq(&mut self) -> GNBREQ_W<RGCRrs> {
        GNBREQ_W::new(self, 19)
    }
}
#[doc = "DMAMux - DMA request generator channel x control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGCRrs;
impl crate::RegisterSpec for RGCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgcr::R`](R) reader structure"]
impl crate::Readable for RGCRrs {}
#[doc = "`write(|w| ..)` method takes [`rgcr::W`](W) writer structure"]
impl crate::Writable for RGCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RGCR%s to value 0"]
impl crate::Resettable for RGCRrs {
    const RESET_VALUE: u32 = 0;
}
