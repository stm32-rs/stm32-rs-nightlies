///Register `P0SR` reader
pub type R = crate::R<P0SRrs>;
///Field `LINEF` reader - Multi-line capture completed raw interrupt status This bit is set when one/more lines have been completed. For the JPEG mode, this bit is raised at the end of the frame. The periodicity of LINEF event is configured by LINEMULT bits into DCMIPP_P0PPCR register. When reaching end of frame, this event is triggered out to allow software action even if the LINEMULT value set is not a multiple of the total lines frame. In case of embedded synchronization, this bit is set only if the CAPTURE bit in the DCMIPP_CR register is set. It is cleared by writing a 1 to the CLINEF bit in the DCMIPP_P0FCR register.
pub type LINEF_R = crate::BitReader;
///Field `FRAMEF` reader - Frame capture completed raw interrupt status This bit is set when all data of a frame or window have been captured. In case of a cropped window, this bit is set at the end of line of the last line in the crop. It is set even if the captured frame is empty (for example window cropped outside the frame). This bit is cleared by writing a 1 to the CFRAMEF bit in DCMIPP_P0FCR.
pub type FRAMEF_R = crate::BitReader;
///Field `VSYNCF` reader - VSYNC raw interrupt status This bit is set when the VSYNC signal changes from the inactive state to the active state. In case of embedded synchronization, this bit is set only if the CAPTURE bit is set in DCMIPP_CR. It is cleared by writing a 1 to the CVSYNCF bit in the DCMIPP_P0FCR register.
pub type VSYNCF_R = crate::BitReader;
///Field `LIMITF` reader - Limit raw interrupt status This bit is set when the data counter DCMIPP_PxDCCNTR reaches its maximum value DCMIPP_PxDCLIMITR. It is cleared by writing a 1 to the CLIMITF bit in the DCMIPP_P0FCR register.
pub type LIMITF_R = crate::BitReader;
///Field `OVRF` reader - Overrun raw interrupt status This bit is cleared by writing a 1 to the COVRF bit in the DCMIPP_P0FCR register.
pub type OVRF_R = crate::BitReader;
///Field `CPTACT` reader - Capture immediate status This bit is automatically reset at the end of frame capture complete event (after all the data of that frame have been captured and the IP-Plug has started to emit the last burst on the AXI, usually before the next VSync).
pub type CPTACT_R = crate::BitReader;
impl R {
    ///Bit 0 - Multi-line capture completed raw interrupt status This bit is set when one/more lines have been completed. For the JPEG mode, this bit is raised at the end of the frame. The periodicity of LINEF event is configured by LINEMULT bits into DCMIPP_P0PPCR register. When reaching end of frame, this event is triggered out to allow software action even if the LINEMULT value set is not a multiple of the total lines frame. In case of embedded synchronization, this bit is set only if the CAPTURE bit in the DCMIPP_CR register is set. It is cleared by writing a 1 to the CLINEF bit in the DCMIPP_P0FCR register.
    #[inline(always)]
    pub fn linef(&self) -> LINEF_R {
        LINEF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Frame capture completed raw interrupt status This bit is set when all data of a frame or window have been captured. In case of a cropped window, this bit is set at the end of line of the last line in the crop. It is set even if the captured frame is empty (for example window cropped outside the frame). This bit is cleared by writing a 1 to the CFRAMEF bit in DCMIPP_P0FCR.
    #[inline(always)]
    pub fn framef(&self) -> FRAMEF_R {
        FRAMEF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - VSYNC raw interrupt status This bit is set when the VSYNC signal changes from the inactive state to the active state. In case of embedded synchronization, this bit is set only if the CAPTURE bit is set in DCMIPP_CR. It is cleared by writing a 1 to the CVSYNCF bit in the DCMIPP_P0FCR register.
    #[inline(always)]
    pub fn vsyncf(&self) -> VSYNCF_R {
        VSYNCF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - Limit raw interrupt status This bit is set when the data counter DCMIPP_PxDCCNTR reaches its maximum value DCMIPP_PxDCLIMITR. It is cleared by writing a 1 to the CLIMITF bit in the DCMIPP_P0FCR register.
    #[inline(always)]
    pub fn limitf(&self) -> LIMITF_R {
        LIMITF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Overrun raw interrupt status This bit is cleared by writing a 1 to the COVRF bit in the DCMIPP_P0FCR register.
    #[inline(always)]
    pub fn ovrf(&self) -> OVRF_R {
        OVRF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 23 - Capture immediate status This bit is automatically reset at the end of frame capture complete event (after all the data of that frame have been captured and the IP-Plug has started to emit the last burst on the AXI, usually before the next VSync).
    #[inline(always)]
    pub fn cptact(&self) -> CPTACT_R {
        CPTACT_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P0SR")
            .field("linef", &self.linef())
            .field("framef", &self.framef())
            .field("vsyncf", &self.vsyncf())
            .field("limitf", &self.limitf())
            .field("ovrf", &self.ovrf())
            .field("cptact", &self.cptact())
            .finish()
    }
}
/**DCMIPP Pipe0 status register

You can [`read`](crate::Reg::read) this register and get [`p0sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7S.html#DCMIPP:P0SR)*/
pub struct P0SRrs;
impl crate::RegisterSpec for P0SRrs {
    type Ux = u32;
}
///`read()` method returns [`p0sr::R`](R) reader structure
impl crate::Readable for P0SRrs {}
///`reset()` method sets P0SR to value 0
impl crate::Resettable for P0SRrs {}
