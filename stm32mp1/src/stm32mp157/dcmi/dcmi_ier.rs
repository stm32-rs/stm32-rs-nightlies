#[doc = "Register `DCMI_IER` reader"]
pub type R = crate::R<DCMI_IERrs>;
#[doc = "Register `DCMI_IER` writer"]
pub type W = crate::W<DCMI_IERrs>;
#[doc = "Field `FRAME_IE` reader - FRAME_IE"]
pub type FRAME_IE_R = crate::BitReader;
#[doc = "Field `FRAME_IE` writer - FRAME_IE"]
pub type FRAME_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR_IE` reader - OVR_IE"]
pub type OVR_IE_R = crate::BitReader;
#[doc = "Field `OVR_IE` writer - OVR_IE"]
pub type OVR_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_IE` reader - ERR_IE"]
pub type ERR_IE_R = crate::BitReader;
#[doc = "Field `ERR_IE` writer - ERR_IE"]
pub type ERR_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_IE` reader - VSYNC_IE"]
pub type VSYNC_IE_R = crate::BitReader;
#[doc = "Field `VSYNC_IE` writer - VSYNC_IE"]
pub type VSYNC_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_IE` reader - LINE_IE"]
pub type LINE_IE_R = crate::BitReader;
#[doc = "Field `LINE_IE` writer - LINE_IE"]
pub type LINE_IE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FRAME_IE"]
    #[inline(always)]
    pub fn frame_ie(&self) -> FRAME_IE_R {
        FRAME_IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - OVR_IE"]
    #[inline(always)]
    pub fn ovr_ie(&self) -> OVR_IE_R {
        OVR_IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ERR_IE"]
    #[inline(always)]
    pub fn err_ie(&self) -> ERR_IE_R {
        ERR_IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - VSYNC_IE"]
    #[inline(always)]
    pub fn vsync_ie(&self) -> VSYNC_IE_R {
        VSYNC_IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - LINE_IE"]
    #[inline(always)]
    pub fn line_ie(&self) -> LINE_IE_R {
        LINE_IE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FRAME_IE"]
    #[inline(always)]
    #[must_use]
    pub fn frame_ie(&mut self) -> FRAME_IE_W<DCMI_IERrs> {
        FRAME_IE_W::new(self, 0)
    }
    #[doc = "Bit 1 - OVR_IE"]
    #[inline(always)]
    #[must_use]
    pub fn ovr_ie(&mut self) -> OVR_IE_W<DCMI_IERrs> {
        OVR_IE_W::new(self, 1)
    }
    #[doc = "Bit 2 - ERR_IE"]
    #[inline(always)]
    #[must_use]
    pub fn err_ie(&mut self) -> ERR_IE_W<DCMI_IERrs> {
        ERR_IE_W::new(self, 2)
    }
    #[doc = "Bit 3 - VSYNC_IE"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_ie(&mut self) -> VSYNC_IE_W<DCMI_IERrs> {
        VSYNC_IE_W::new(self, 3)
    }
    #[doc = "Bit 4 - LINE_IE"]
    #[inline(always)]
    #[must_use]
    pub fn line_ie(&mut self) -> LINE_IE_W<DCMI_IERrs> {
        LINE_IE_W::new(self, 4)
    }
}
#[doc = "The DCMI_IER register is used to enable interrupts. When one of the DCMI_IER bits is set, the corresponding interrupt is enabled. This register is accessible in both read and write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcmi_ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcmi_ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCMI_IERrs;
impl crate::RegisterSpec for DCMI_IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcmi_ier::R`](R) reader structure"]
impl crate::Readable for DCMI_IERrs {}
#[doc = "`write(|w| ..)` method takes [`dcmi_ier::W`](W) writer structure"]
impl crate::Writable for DCMI_IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCMI_IER to value 0"]
impl crate::Resettable for DCMI_IERrs {
    const RESET_VALUE: u32 = 0;
}
