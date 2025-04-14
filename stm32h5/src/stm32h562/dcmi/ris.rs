///Register `RIS` reader
pub type R = crate::R<RISrs>;
///Field `FRAME_RIS` reader - Capture complete raw interrupt status This bit is set when a frame or window has been captured. In case of a cropped window, this bit is set at the end of line of the last line in the crop. It is set even if the captured frame is empty (e.g. window cropped outside the frame). The bit is cleared by setting the FRAME_ISC bit of the DCMI_ICR register.
pub type FRAME_RIS_R = crate::BitReader;
///Field `OVR_RIS` reader - Overrun raw interrupt status The bit is cleared by setting the OVR_ISC bit of the DCMI_ICR register.
pub type OVR_RIS_R = crate::BitReader;
///Field `ERR_RIS` reader - Synchronization error raw interrupt status This bit is valid only in the embedded synchronization mode. It is cleared by setting the ERR_ISC bit of the DCMI_ICR register. Note: This bit is available only in embedded synchronization mode.
pub type ERR_RIS_R = crate::BitReader;
///Field `VSYNC_RIS` reader - DCMI_VSYNC raw interrupt status This bit is set when the DCMI_VSYNC signal changes from the inactive state to the active state. In the case of embedded synchronization, this bit is set only if the CAPTURE bit is set in DCMI_CR. It is cleared by setting the VSYNC_ISC bit of the DCMI_ICR register.
pub type VSYNC_RIS_R = crate::BitReader;
///Field `LINE_RIS` reader - Line raw interrupt status This bit gets set when the DCMI_HSYNC signal changes from the inactive state to the active state. It goes high even if the line is not valid. In the case of embedded synchronization, this bit is set only if the CAPTURE bit in DCMI_CR is set. It is cleared by setting the LINE_ISC bit of the DCMI_ICR register.
pub type LINE_RIS_R = crate::BitReader;
impl R {
    ///Bit 0 - Capture complete raw interrupt status This bit is set when a frame or window has been captured. In case of a cropped window, this bit is set at the end of line of the last line in the crop. It is set even if the captured frame is empty (e.g. window cropped outside the frame). The bit is cleared by setting the FRAME_ISC bit of the DCMI_ICR register.
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overrun raw interrupt status The bit is cleared by setting the OVR_ISC bit of the DCMI_ICR register.
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization error raw interrupt status This bit is valid only in the embedded synchronization mode. It is cleared by setting the ERR_ISC bit of the DCMI_ICR register. Note: This bit is available only in embedded synchronization mode.
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DCMI_VSYNC raw interrupt status This bit is set when the DCMI_VSYNC signal changes from the inactive state to the active state. In the case of embedded synchronization, this bit is set only if the CAPTURE bit is set in DCMI_CR. It is cleared by setting the VSYNC_ISC bit of the DCMI_ICR register.
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Line raw interrupt status This bit gets set when the DCMI_HSYNC signal changes from the inactive state to the active state. It goes high even if the line is not valid. In the case of embedded synchronization, this bit is set only if the CAPTURE bit in DCMI_CR is set. It is cleared by setting the LINE_ISC bit of the DCMI_ICR register.
    #[inline(always)]
    pub fn line_ris(&self) -> LINE_RIS_R {
        LINE_RIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RIS")
            .field("frame_ris", &self.frame_ris())
            .field("ovr_ris", &self.ovr_ris())
            .field("err_ris", &self.err_ris())
            .field("vsync_ris", &self.vsync_ris())
            .field("line_ris", &self.line_ris())
            .finish()
    }
}
/**DCMI raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H562.html#DCMI:RIS)*/
pub struct RISrs;
impl crate::RegisterSpec for RISrs {
    type Ux = u32;
}
///`read()` method returns [`ris::R`](R) reader structure
impl crate::Readable for RISrs {}
///`reset()` method sets RIS to value 0
impl crate::Resettable for RISrs {}
