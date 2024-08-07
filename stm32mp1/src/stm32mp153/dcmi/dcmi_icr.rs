///Register `DCMI_ICR` writer
pub type W = crate::W<DCMI_ICRrs>;
///Field `FRAME_ISC` writer - FRAME_ISC
pub type FRAME_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVR_ISC` writer - OVR_ISC
pub type OVR_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERR_ISC` writer - ERR_ISC
pub type ERR_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSYNC_ISC` writer - VSYNC_ISC
pub type VSYNC_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LINE_ISC` writer - LINE_ISC
pub type LINE_ISC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<DCMI_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - FRAME_ISC
    #[inline(always)]
    #[must_use]
    pub fn frame_isc(&mut self) -> FRAME_ISC_W<DCMI_ICRrs> {
        FRAME_ISC_W::new(self, 0)
    }
    ///Bit 1 - OVR_ISC
    #[inline(always)]
    #[must_use]
    pub fn ovr_isc(&mut self) -> OVR_ISC_W<DCMI_ICRrs> {
        OVR_ISC_W::new(self, 1)
    }
    ///Bit 2 - ERR_ISC
    #[inline(always)]
    #[must_use]
    pub fn err_isc(&mut self) -> ERR_ISC_W<DCMI_ICRrs> {
        ERR_ISC_W::new(self, 2)
    }
    ///Bit 3 - VSYNC_ISC
    #[inline(always)]
    #[must_use]
    pub fn vsync_isc(&mut self) -> VSYNC_ISC_W<DCMI_ICRrs> {
        VSYNC_ISC_W::new(self, 3)
    }
    ///Bit 4 - LINE_ISC
    #[inline(always)]
    #[must_use]
    pub fn line_isc(&mut self) -> LINE_ISC_W<DCMI_ICRrs> {
        LINE_ISC_W::new(self, 4)
    }
}
/**The DCMI_ICR register is write-only.

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcmi_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DCMI:DCMI_ICR)*/
pub struct DCMI_ICRrs;
impl crate::RegisterSpec for DCMI_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`dcmi_icr::W`](W) writer structure
impl crate::Writable for DCMI_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DCMI_ICR to value 0
impl crate::Resettable for DCMI_ICRrs {
    const RESET_VALUE: u32 = 0;
}
