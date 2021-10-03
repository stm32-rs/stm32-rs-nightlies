#[doc = "Register `FDCAN_IR` reader"]
pub struct R(crate::R<FDCAN_IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RF0N` reader - Rx FIFO 0 New Message"]
pub struct RF0N_R(crate::FieldReader<bool, bool>);
impl RF0N_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0W` reader - Rx FIFO 0 Full"]
pub struct RF0W_R(crate::FieldReader<bool, bool>);
impl RF0W_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0F` reader - Rx FIFO 0 Full"]
pub struct RF0F_R(crate::FieldReader<bool, bool>);
impl RF0F_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub struct RF0L_R(crate::FieldReader<bool, bool>);
impl RF0L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1N` reader - Rx FIFO 1 New Message"]
pub struct RF1N_R(crate::FieldReader<bool, bool>);
impl RF1N_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1N_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1N_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1W` reader - Rx FIFO 1 Watermark Reached"]
pub struct RF1W_R(crate::FieldReader<bool, bool>);
impl RF1W_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1W_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1W_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1F` reader - Rx FIFO 1 Watermark Reached"]
pub struct RF1F_R(crate::FieldReader<bool, bool>);
impl RF1F_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub struct RF1L_R(crate::FieldReader<bool, bool>);
impl RF1L_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1L_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1L_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPM` reader - High Priority Message"]
pub struct HPM_R(crate::FieldReader<bool, bool>);
impl HPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TC` reader - Transmission Completed"]
pub struct TC_R(crate::FieldReader<bool, bool>);
impl TC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCF` reader - Transmission Cancellation Finished"]
pub struct TCF_R(crate::FieldReader<bool, bool>);
impl TCF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEF` reader - Tx FIFO Empty"]
pub struct TEF_R(crate::FieldReader<bool, bool>);
impl TEF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFN` reader - Tx Event FIFO New Entry"]
pub struct TEFN_R(crate::FieldReader<bool, bool>);
impl TEFN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFW` reader - Tx Event FIFO Watermark Reached"]
pub struct TEFW_R(crate::FieldReader<bool, bool>);
impl TEFW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFF` reader - Tx Event FIFO Full"]
pub struct TEFF_R(crate::FieldReader<bool, bool>);
impl TEFF_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub struct TEFL_R(crate::FieldReader<bool, bool>);
impl TEFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSW` reader - Timestamp Wraparound"]
pub struct TSW_R(crate::FieldReader<bool, bool>);
impl TSW_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRAF` reader - Message RAM Access Failure"]
pub struct MRAF_R(crate::FieldReader<bool, bool>);
impl MRAF_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRAF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRAF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOO` reader - Timeout Occurred"]
pub struct TOO_R(crate::FieldReader<bool, bool>);
impl TOO_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRX` reader - Message stored to Dedicated Rx Buffer"]
pub struct DRX_R(crate::FieldReader<bool, bool>);
impl DRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELO` reader - Error Logging Overflow"]
pub struct ELO_R(crate::FieldReader<bool, bool>);
impl ELO_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP` reader - Error Passive"]
pub struct EP_R(crate::FieldReader<bool, bool>);
impl EP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EW` reader - Warning Status"]
pub struct EW_R(crate::FieldReader<bool, bool>);
impl EW_R {
    pub(crate) fn new(bits: bool) -> Self {
        EW_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EW_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BO` reader - Bus_Off Status"]
pub struct BO_R(crate::FieldReader<bool, bool>);
impl BO_R {
    pub(crate) fn new(bits: bool) -> Self {
        BO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDI` reader - Watchdog Interrupt"]
pub struct WDI_R(crate::FieldReader<bool, bool>);
impl WDI_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEA` reader - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
pub struct PEA_R(crate::FieldReader<bool, bool>);
impl PEA_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PED` reader - Protocol Error in Data Phase (Data Bit Time is used)"]
pub struct PED_R(crate::FieldReader<bool, bool>);
impl PED_R {
    pub(crate) fn new(bits: bool) -> Self {
        PED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARA` reader - Access to Reserved Address"]
pub struct ARA_R(crate::FieldReader<bool, bool>);
impl ARA_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message"]
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0w(&self) -> RF0W_R {
        RF0W_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message"]
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1w(&self) -> RF1W_R {
        RF1W_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    pub fn tef(&self) -> TEF_R {
        TEF_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub fn tefw(&self) -> TEFW_R {
        TEFW_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DRX_R {
        DRX_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)"]
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address"]
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "FDCAN Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ir](index.html) module"]
pub struct FDCAN_IR_SPEC;
impl crate::RegisterSpec for FDCAN_IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ir::R](R) reader structure"]
impl crate::Readable for FDCAN_IR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_IR to value 0"]
impl crate::Resettable for FDCAN_IR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
