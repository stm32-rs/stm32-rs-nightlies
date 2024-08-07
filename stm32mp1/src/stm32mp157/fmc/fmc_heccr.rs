///Register `FMC_HECCR` reader
pub type R = crate::R<FMC_HECCRrs>;
///Field `HECC` reader - HECC
pub type HECC_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - HECC
    #[inline(always)]
    pub fn hecc(&self) -> HECC_R {
        HECC_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FMC_HECCR")
            .field("hecc", &self.hecc())
            .finish()
    }
}
/**This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.

You can [`read`](crate::Reg::read) this register and get [`fmc_heccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:FMC_HECCR)*/
pub struct FMC_HECCRrs;
impl crate::RegisterSpec for FMC_HECCRrs {
    type Ux = u32;
}
///`read()` method returns [`fmc_heccr::R`](R) reader structure
impl crate::Readable for FMC_HECCRrs {}
///`reset()` method sets FMC_HECCR to value 0
impl crate::Resettable for FMC_HECCRrs {
    const RESET_VALUE: u32 = 0;
}
