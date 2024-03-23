#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `FRAME_IE` reader - Capture complete interrupt enable"]
pub type FRAME_IE_R = crate::BitReader;
#[doc = "Field `FRAME_IE` writer - Capture complete interrupt enable"]
pub type FRAME_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR_IE` reader - Overrun interrupt enable"]
pub type OVR_IE_R = crate::BitReader;
#[doc = "Field `OVR_IE` writer - Overrun interrupt enable"]
pub type OVR_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_IE` reader - Synchronization error interrupt enable"]
pub type ERR_IE_R = crate::BitReader;
#[doc = "Field `ERR_IE` writer - Synchronization error interrupt enable"]
pub type ERR_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_IE` reader - VSYNC interrupt enable"]
pub type VSYNC_IE_R = crate::BitReader;
#[doc = "Field `VSYNC_IE` writer - VSYNC interrupt enable"]
pub type VSYNC_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_IE` reader - Line interrupt enable"]
pub type LINE_IE_R = crate::BitReader;
#[doc = "Field `LINE_IE` writer - Line interrupt enable"]
pub type LINE_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn frame_ie(&mut self) -> FRAME_IE_W<IERrs> {
        FRAME_IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<IERrs> {
        OVR_IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn err_ie(&mut self) -> ERR_IE_W<IERrs> {
        ERR_IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - VSYNC interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W<IERrs> {
        VSYNC_IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Line interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn line_ie(&mut self) -> LINE_IE_W<IERrs> {
        LINE_IE_W::new(self, 4)
    }
}
#[doc = "interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
