///Register `MIS` reader
pub type R = crate::R<MISrs>;
///Field `FRAME_MIS` reader - FRAME_MIS
pub type FRAME_MIS_R = crate::BitReader;
///Field `OVR_MIS` reader - OVR_MIS
pub type OVR_MIS_R = crate::BitReader;
///Field `ERR_MIS` reader - ERR_MIS
pub type ERR_MIS_R = crate::BitReader;
///Field `VSYNC_MIS` reader - VSYNC_MIS
pub type VSYNC_MIS_R = crate::BitReader;
///Field `LINE_MIS` reader - LINE_MIS
pub type LINE_MIS_R = crate::BitReader;
impl R {
    ///Bit 0 - FRAME_MIS
    #[inline(always)]
    pub fn frame_mis(&self) -> FRAME_MIS_R {
        FRAME_MIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - OVR_MIS
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ERR_MIS
    #[inline(always)]
    pub fn err_mis(&self) -> ERR_MIS_R {
        ERR_MIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - VSYNC_MIS
    #[inline(always)]
    pub fn vsync_mis(&self) -> VSYNC_MIS_R {
        VSYNC_MIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - LINE_MIS
    #[inline(always)]
    pub fn line_mis(&self) -> LINE_MIS_R {
        LINE_MIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MIS")
            .field("frame_mis", &self.frame_mis())
            .field("ovr_mis", &self.ovr_mis())
            .field("err_mis", &self.err_mis())
            .field("vsync_mis", &self.vsync_mis())
            .field("line_mis", &self.line_mis())
            .finish()
    }
}
/**This DCMI_MIS register is a read-only register. When read, it returns the current masked status value (depending on the value in DCMI_IER) of the corresponding interrupt. A bit in this register is set if the corresponding enable bit in DCMI_IER is set and the corresponding bit in DCMI_RIS is set.

You can [`read`](crate::Reg::read) this register and get [`mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DCMI:MIS)*/
pub struct MISrs;
impl crate::RegisterSpec for MISrs {
    type Ux = u32;
}
///`read()` method returns [`mis::R`](R) reader structure
impl crate::Readable for MISrs {}
///`reset()` method sets MIS to value 0
impl crate::Resettable for MISrs {}
