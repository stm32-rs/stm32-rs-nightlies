#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Field `FRAME_ISC` writer - Capture complete interrupt status clear Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register."]
pub type FRAME_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR_ISC` writer - Overrun interrupt status clear Setting this bit clears the OVR_RIS flag in the DCMI_RIS register."]
pub type OVR_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_ISC` writer - Synchronization error interrupt status clear Setting this bit clears the ERR_RIS flag in the DCMI_RIS register. Note: This bit is available only in embedded synchronization mode."]
pub type ERR_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNC_ISC` writer - Vertical Synchronization interrupt status clear Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register."]
pub type VSYNC_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINE_ISC` writer - line interrupt status clear Setting this bit clears the LINE_RIS flag in the DCMI_RIS register."]
pub type LINE_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Capture complete interrupt status clear Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register."]
    #[inline(always)]
    #[must_use]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W<ICRrs> {
        FRAME_ISC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Overrun interrupt status clear Setting this bit clears the OVR_RIS flag in the DCMI_RIS register."]
    #[inline(always)]
    #[must_use]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<ICRrs> {
        OVR_ISC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Synchronization error interrupt status clear Setting this bit clears the ERR_RIS flag in the DCMI_RIS register. Note: This bit is available only in embedded synchronization mode."]
    #[inline(always)]
    #[must_use]
    pub fn err_isc(&mut self) -> ERR_ISC_W<ICRrs> {
        ERR_ISC_W::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical Synchronization interrupt status clear Setting this bit clears the VSYNC_RIS flag in the DCMI_RIS register."]
    #[inline(always)]
    #[must_use]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W<ICRrs> {
        VSYNC_ISC_W::new(self, 3)
    }
    #[doc = "Bit 4 - line interrupt status clear Setting this bit clears the LINE_RIS flag in the DCMI_RIS register."]
    #[inline(always)]
    #[must_use]
    pub fn line_isc(&mut self) -> LINE_ISC_W<ICRrs> {
        LINE_ISC_W::new(self, 4)
    }
}
#[doc = "DCMI interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
