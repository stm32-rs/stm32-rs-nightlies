#[doc = "Register `FMC_SR` reader"]
pub struct R(crate::R<FMC_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FMC_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FMC_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FMC_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ISOST` reader - ISOST"]
pub struct ISOST_R(crate::FieldReader<u8, u8>);
impl ISOST_R {
    pub(crate) fn new(bits: u8) -> Self {
        ISOST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ISOST_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEF` reader - PEF"]
pub struct PEF_R(crate::FieldReader<bool, bool>);
impl PEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NWRF` reader - NWRF"]
pub struct NWRF_R(crate::FieldReader<bool, bool>);
impl NWRF_R {
    pub(crate) fn new(bits: bool) -> Self {
        NWRF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NWRF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:1 - ISOST"]
    #[inline(always)]
    pub fn isost(&self) -> ISOST_R {
        ISOST_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 4 - PEF"]
    #[inline(always)]
    pub fn pef(&self) -> PEF_R {
        PEF_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - NWRF"]
    #[inline(always)]
    pub fn nwrf(&self) -> NWRF_R {
        NWRF_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "This register contains information about the AXI interface isolation status and the NAND write requests status. The FMC has to be disabled before modifying some registers. As requests might be pending, it is necessary to wait till the AXI interface is stable and the core of the block is totally isolated from its AXI interface before reconfiguring the registers. The PEF and PNWEF bits indicate the status of the pipe. If Hamming algorithm is used, the ECC is calculated while data are written to the memory. To read the correct ECC, the software must consequently wait untill no write request to the NAND controller are pending, by polling PEF and NWRF bits.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fmc_sr](index.html) module"]
pub struct FMC_SR_SPEC;
impl crate::RegisterSpec for FMC_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fmc_sr::R](R) reader structure"]
impl crate::Readable for FMC_SR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FMC_SR to value 0x40"]
impl crate::Resettable for FMC_SR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
