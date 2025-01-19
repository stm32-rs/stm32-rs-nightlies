///Register `PSSI_MIS` reader
pub type R = crate::R<PSSI_MISrs>;
///Field `OVR_MIS` reader - Data buffer overrun/underrun masked interrupt status This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1.
pub type OVR_MIS_R = crate::BitReader;
impl R {
    ///Bit 1 - Data buffer overrun/underrun masked interrupt status This bit is set to 1 only when PSSI_IER/OVR_IE and PSSI_RIS/OVR_RIS are both set to 1.
    #[inline(always)]
    pub fn ovr_mis(&self) -> OVR_MIS_R {
        OVR_MIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSSI_MIS")
            .field("ovr_mis", &self.ovr_mis())
            .finish()
    }
}
/**PSSI masked interrupt status register

You can [`read`](crate::Reg::read) this register and get [`pssi_mis::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#PSSI:PSSI_MIS)*/
pub struct PSSI_MISrs;
impl crate::RegisterSpec for PSSI_MISrs {
    type Ux = u32;
}
///`read()` method returns [`pssi_mis::R`](R) reader structure
impl crate::Readable for PSSI_MISrs {}
///`reset()` method sets PSSI_MIS to value 0
impl crate::Resettable for PSSI_MISrs {
    const RESET_VALUE: u32 = 0;
}
