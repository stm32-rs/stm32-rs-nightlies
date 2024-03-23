#[doc = "Register `FMC_HPR` reader"]
pub type R = crate::R<FMC_HPRrs>;
#[doc = "Field `HPR` reader - HPR"]
pub type HPR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - HPR"]
    #[inline(always)]
    pub fn hpr(&self) -> HPR_R {
        HPR_R::new(self.bits)
    }
}
#[doc = "This register is used during read accesses in conjunction with the FMC sequencer. It contains the current error correction code value computed by the FMC NAND controller Hamming module. When the FMC sequencer reads data from a NAND Flash memory page at the correct address, the data read are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area and stored in the and the FMC_HPR, to determine whether a page is valid, and to correct it otherwise. The FMC_HPR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmc_hpr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FMC_HPRrs;
impl crate::RegisterSpec for FMC_HPRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmc_hpr::R`](R) reader structure"]
impl crate::Readable for FMC_HPRrs {}
#[doc = "`reset()` method sets FMC_HPR to value 0"]
impl crate::Resettable for FMC_HPRrs {
    const RESET_VALUE: u32 = 0;
}
