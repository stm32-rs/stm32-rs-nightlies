///Register `RIS` reader
pub type R = crate::R<RISrs>;
///Field `FRAME_RIS` reader - Capture complete raw interrupt status
pub type FRAME_RIS_R = crate::BitReader;
///Field `OVR_RIS` reader - Overrun raw interrupt status
pub type OVR_RIS_R = crate::BitReader;
///Field `ERR_RIS` reader - Synchronization error raw interrupt status
pub type ERR_RIS_R = crate::BitReader;
///Field `VSYNC_RIS` reader - DCMI_VSYNC raw interrupt status
pub type VSYNC_RIS_R = crate::BitReader;
///Field `LINE_RIS` reader - Line raw interrupt status
pub type LINE_RIS_R = crate::BitReader;
impl R {
    ///Bit 0 - Capture complete raw interrupt status
    #[inline(always)]
    pub fn frame_ris(&self) -> FRAME_RIS_R {
        FRAME_RIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Overrun raw interrupt status
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Synchronization error raw interrupt status
    #[inline(always)]
    pub fn err_ris(&self) -> ERR_RIS_R {
        ERR_RIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - DCMI_VSYNC raw interrupt status
    #[inline(always)]
    pub fn vsync_ris(&self) -> VSYNC_RIS_R {
        VSYNC_RIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Line raw interrupt status
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

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#DCMI:RIS)*/
pub struct RISrs;
impl crate::RegisterSpec for RISrs {
    type Ux = u32;
}
///`read()` method returns [`ris::R`](R) reader structure
impl crate::Readable for RISrs {}
///`reset()` method sets RIS to value 0
impl crate::Resettable for RISrs {}
