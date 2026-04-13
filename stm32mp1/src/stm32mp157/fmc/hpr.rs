///Register `HPR` reader
pub type R = crate::R<HPRrs>;
///Field `HPR` reader - HPR
pub type HPR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - HPR
    #[inline(always)]
    pub fn hpr(&self) -> HPR_R {
        HPR_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPR").field("hpr", &self.hpr()).finish()
    }
}
/**This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.

You can [`read`](crate::Reg::read) this register and get [`hpr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:HPR)*/
pub struct HPRrs;
impl crate::RegisterSpec for HPRrs {
    type Ux = u32;
}
///`read()` method returns [`hpr::R`](R) reader structure
impl crate::Readable for HPRrs {}
///`reset()` method sets HPR to value 0
impl crate::Resettable for HPRrs {}
