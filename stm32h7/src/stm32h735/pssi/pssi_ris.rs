///Register `PSSI_RIS` reader
pub type R = crate::R<PSSI_RISrs>;
///Field `OVR_RIS` reader - Data buffer overrun/underrun raw interrupt status This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR.
pub type OVR_RIS_R = crate::BitReader;
impl R {
    ///Bit 1 - Data buffer overrun/underrun raw interrupt status This bit is cleared by writing a 1 to the OVR_ISC bit in PSSI_ICR.
    #[inline(always)]
    pub fn ovr_ris(&self) -> OVR_RIS_R {
        OVR_RIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSSI_RIS")
            .field("ovr_ris", &self.ovr_ris())
            .finish()
    }
}
/**PSSI raw interrupt status register

You can [`read`](crate::Reg::read) this register and get [`pssi_ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#PSSI:PSSI_RIS)*/
pub struct PSSI_RISrs;
impl crate::RegisterSpec for PSSI_RISrs {
    type Ux = u32;
}
///`read()` method returns [`pssi_ris::R`](R) reader structure
impl crate::Readable for PSSI_RISrs {}
///`reset()` method sets PSSI_RIS to value 0
impl crate::Resettable for PSSI_RISrs {
    const RESET_VALUE: u32 = 0;
}
