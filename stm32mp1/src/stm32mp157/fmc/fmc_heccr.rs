#[doc = "Register `FMC_HECCR` reader"]
pub struct R(crate::R<FMC_HECCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_HECCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_HECCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_HECCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HECC` reader - HECC"]
pub struct HECC_R(crate::FieldReader<u32, u32>);
impl HECC_R {
    pub(crate) fn new(bits: u32) -> Self {
        HECC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HECC_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - HECC"]
    #[inline(always)]
    pub fn hecc(&self) -> HECC_R {
        HECC_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "This register contain the current error correction code value computed by the FMC NAND controller Hamming module.When the CPU reads/writes data from/to a NAND Flash memory page at the correct address (refer to Section25.8.6: NAND ECC controller), the data read/written from/to the NAND Flash memory are automatically processed by the Hamming computation module. When X bytes have been read (according to the sector size ECCSS field in the FMC_PCR register), the CPU must read the computed ECC value from the FMC_HECCR register. It then verifies if these computed parity data are the same as the parity value recorded in the spare area, to determine whether a page is valid, and to correct it otherwise. The FMC_HECCR register should be cleared after being read by setting the ECCEN bit to 0. To compute a new data block, the ECCEN bit must be set to 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_heccr](index.html) module"]
pub struct FMC_HECCR_SPEC;
impl crate::RegisterSpec for FMC_HECCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_heccr::R](R) reader structure"]
impl crate::Readable for FMC_HECCR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_HECCR to value 0"]
impl crate::Resettable for FMC_HECCR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
