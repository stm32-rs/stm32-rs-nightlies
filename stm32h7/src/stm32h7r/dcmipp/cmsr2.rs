///Register `CMSR2` reader
pub type R = crate::R<CMSR2rs>;
///Field `ATXERRF` reader - AXI transfer error interrupt status flag for the IP-Plug. This bit is cleared by writing a 1 to CATXERRF bit in the DCMIPP_CMFCR register.
pub type ATXERRF_R = crate::BitReader;
///Field `PRERRF` reader - Synchronization error raw interrupt status for the parallel interface. This bit is valid only in the embedded synchronization mode. It is cleared by writing a 1 to the CPRERRF bit in the DCMIPP_CMFCR register. This bit is available only in embedded synchronization mode.
pub type PRERRF_R = crate::BitReader;
///Field `P0LINEF` reader - Multi-line capture completed raw interrupt status for Pipe0 This bit is set when one/more lines have been completed. The periodicity of LINEF event is configured by LINEMULT bits into DCMIPP_P0PPCR register. When reaching end of frame, this event is triggered out to allow software action even if the LINEMULT value set is not a multiple of the total lines frame. In the case of embedded synchronization, this bit is set only if the CAPTURE bit in the DCMIPP_CR register is set. It is cleared by writing a 1 to the CP0LINEF bit in the DCMIPP_CMFCR register.
pub type P0LINEF_R = crate::BitReader;
///Field `P0FRAMEF` reader - Frame capture completed raw interrupt status for Pipe0 This bit is set when all data of a frame or window have been captured. In case of a cropped window, this bit is set at the end of line of the last line in the crop, even if the captured frame is empty (for example window cropped outside the frame). This bit is cleared by writing a 1 to the CP0FRAMEF bit in the DCMIPP_CMFCR register.
pub type P0FRAMEF_R = crate::BitReader;
///Field `P0VSYNCF` reader - VSYNC raw interrupt status for Pipe0 This bit is set when the VSYNC signal changes from the inactive state to the active state. In the case of embedded synchronization, this bit is set only if the CAPTURE bit is set in DCMIPP_CR. It is cleared by writing a 1 to the CP0VSYNCF bit in the DCMIPP_CMFCR register.
pub type P0VSYNCF_R = crate::BitReader;
///Field `P0LIMITF` reader - Limit raw interrupt status for Pipe0 This bit is set when the data counter DCMIPP_P0DCCNT reaches its maximum value DCMIPP_P0DCLIMIT. It is cleared by writing a 1 to the CP0LIMITF bit in the DCMIPP_CMFCR register.
pub type P0LIMITF_R = crate::BitReader;
///Field `P0OVRF` reader - Overrun raw interrupt status for Pipe0 This bit is cleared by writing a 1 to the CP0OVRF bit in the DCMIPP_CMFCR register.
pub type P0OVRF_R = crate::BitReader;
impl R {
    ///Bit 5 - AXI transfer error interrupt status flag for the IP-Plug. This bit is cleared by writing a 1 to CATXERRF bit in the DCMIPP_CMFCR register.
    #[inline(always)]
    pub fn atxerrf(&self) -> ATXERRF_R {
        ATXERRF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Synchronization error raw interrupt status for the parallel interface. This bit is valid only in the embedded synchronization mode. It is cleared by writing a 1 to the CPRERRF bit in the DCMIPP_CMFCR register. This bit is available only in embedded synchronization mode.
    #[inline(always)]
    pub fn prerrf(&self) -> PRERRF_R {
        PRERRF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Multi-line capture completed raw interrupt status for Pipe0 This bit is set when one/more lines have been completed. The periodicity of LINEF event is configured by LINEMULT bits into DCMIPP_P0PPCR register. When reaching end of frame, this event is triggered out to allow software action even if the LINEMULT value set is not a multiple of the total lines frame. In the case of embedded synchronization, this bit is set only if the CAPTURE bit in the DCMIPP_CR register is set. It is cleared by writing a 1 to the CP0LINEF bit in the DCMIPP_CMFCR register.
    #[inline(always)]
    pub fn p0linef(&self) -> P0LINEF_R {
        P0LINEF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Frame capture completed raw interrupt status for Pipe0 This bit is set when all data of a frame or window have been captured. In case of a cropped window, this bit is set at the end of line of the last line in the crop, even if the captured frame is empty (for example window cropped outside the frame). This bit is cleared by writing a 1 to the CP0FRAMEF bit in the DCMIPP_CMFCR register.
    #[inline(always)]
    pub fn p0framef(&self) -> P0FRAMEF_R {
        P0FRAMEF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - VSYNC raw interrupt status for Pipe0 This bit is set when the VSYNC signal changes from the inactive state to the active state. In the case of embedded synchronization, this bit is set only if the CAPTURE bit is set in DCMIPP_CR. It is cleared by writing a 1 to the CP0VSYNCF bit in the DCMIPP_CMFCR register.
    #[inline(always)]
    pub fn p0vsyncf(&self) -> P0VSYNCF_R {
        P0VSYNCF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 14 - Limit raw interrupt status for Pipe0 This bit is set when the data counter DCMIPP_P0DCCNT reaches its maximum value DCMIPP_P0DCLIMIT. It is cleared by writing a 1 to the CP0LIMITF bit in the DCMIPP_CMFCR register.
    #[inline(always)]
    pub fn p0limitf(&self) -> P0LIMITF_R {
        P0LIMITF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Overrun raw interrupt status for Pipe0 This bit is cleared by writing a 1 to the CP0OVRF bit in the DCMIPP_CMFCR register.
    #[inline(always)]
    pub fn p0ovrf(&self) -> P0OVRF_R {
        P0OVRF_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMSR2")
            .field("atxerrf", &self.atxerrf())
            .field("prerrf", &self.prerrf())
            .field("p0linef", &self.p0linef())
            .field("p0framef", &self.p0framef())
            .field("p0vsyncf", &self.p0vsyncf())
            .field("p0limitf", &self.p0limitf())
            .field("p0ovrf", &self.p0ovrf())
            .finish()
    }
}
/**DCMIPP common status register 2

You can [`read`](crate::Reg::read) this register and get [`cmsr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#DCMIPP:CMSR2)*/
pub struct CMSR2rs;
impl crate::RegisterSpec for CMSR2rs {
    type Ux = u32;
}
///`read()` method returns [`cmsr2::R`](R) reader structure
impl crate::Readable for CMSR2rs {}
///`reset()` method sets CMSR2 to value 0
impl crate::Resettable for CMSR2rs {}
