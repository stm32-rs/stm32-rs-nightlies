#[doc = "Register `FDCAN_ILS` reader"]
pub struct R(crate::R<FDCAN_ILS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FDCAN_ILS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FDCAN_ILS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FDCAN_ILS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RF0NL` reader - Rx FIFO 0 New Message Interrupt Line"]
pub struct RF0NL_R(crate::FieldReader<bool, bool>);
impl RF0NL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0NL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0NL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0WL` reader - Rx FIFO 0 Watermark Reached Interrupt Line"]
pub struct RF0WL_R(crate::FieldReader<bool, bool>);
impl RF0WL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0WL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0WL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0FL` reader - Rx FIFO 0 Full Interrupt Line"]
pub struct RF0FL_R(crate::FieldReader<bool, bool>);
impl RF0FL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0FL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF0LL` reader - Rx FIFO 0 Message Lost Interrupt Line"]
pub struct RF0LL_R(crate::FieldReader<bool, bool>);
impl RF0LL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF0LL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF0LL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1NL` reader - Rx FIFO 1 New Message Interrupt Line"]
pub struct RF1NL_R(crate::FieldReader<bool, bool>);
impl RF1NL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1NL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1NL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1WL` reader - Rx FIFO 1 Watermark Reached Interrupt Line"]
pub struct RF1WL_R(crate::FieldReader<bool, bool>);
impl RF1WL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1WL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1WL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1FL` reader - Rx FIFO 1 Full Interrupt Line"]
pub struct RF1FL_R(crate::FieldReader<bool, bool>);
impl RF1FL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1FL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1FL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RF1LL` reader - Rx FIFO 1 Message Lost Interrupt Line"]
pub struct RF1LL_R(crate::FieldReader<bool, bool>);
impl RF1LL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RF1LL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RF1LL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HPML` reader - High Priority Message Interrupt Line"]
pub struct HPML_R(crate::FieldReader<bool, bool>);
impl HPML_R {
    pub(crate) fn new(bits: bool) -> Self {
        HPML_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HPML_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCL` reader - Transmission Completed Interrupt Line"]
pub struct TCL_R(crate::FieldReader<bool, bool>);
impl TCL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TCFL` reader - Transmission Cancellation Finished Interrupt Line"]
pub struct TCFL_R(crate::FieldReader<bool, bool>);
impl TCFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TCFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TCFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFL` reader - Tx FIFO Empty Interrupt Line"]
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
#[doc = "Field `TEFNL` reader - Tx Event FIFO New Entry Interrupt Line"]
pub struct TEFNL_R(crate::FieldReader<bool, bool>);
impl TEFNL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFNL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFNL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFWL` reader - Tx Event FIFO Watermark Reached Interrupt Line"]
pub struct TEFWL_R(crate::FieldReader<bool, bool>);
impl TEFWL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFWL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFFL` reader - Tx Event FIFO Full Interrupt Line"]
pub struct TEFFL_R(crate::FieldReader<bool, bool>);
impl TEFFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEFLL` reader - Tx Event FIFO Element Lost Interrupt Line"]
pub struct TEFLL_R(crate::FieldReader<bool, bool>);
impl TEFLL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEFLL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEFLL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TSWL` reader - Timestamp Wraparound Interrupt Line"]
pub struct TSWL_R(crate::FieldReader<bool, bool>);
impl TSWL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TSWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TSWL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MRAFL` reader - Message RAM Access Failure Interrupt Line"]
pub struct MRAFL_R(crate::FieldReader<bool, bool>);
impl MRAFL_R {
    pub(crate) fn new(bits: bool) -> Self {
        MRAFL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MRAFL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOOL` reader - Timeout Occurred Interrupt Line"]
pub struct TOOL_R(crate::FieldReader<bool, bool>);
impl TOOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TOOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DRXL` reader - Message stored to Dedicated Rx Buffer Interrupt Line"]
pub struct DRXL_R(crate::FieldReader<bool, bool>);
impl DRXL_R {
    pub(crate) fn new(bits: bool) -> Self {
        DRXL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DRXL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BECL` reader - Bit Error Corrected Interrupt Line"]
pub struct BECL_R(crate::FieldReader<bool, bool>);
impl BECL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BECL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BECL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BEUL` reader - Bit Error Uncorrected Interrupt Line"]
pub struct BEUL_R(crate::FieldReader<bool, bool>);
impl BEUL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BEUL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BEUL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ELOL` reader - Error Logging Overflow Interrupt Line"]
pub struct ELOL_R(crate::FieldReader<bool, bool>);
impl ELOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ELOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ELOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EPL` reader - Error Passive Interrupt Line"]
pub struct EPL_R(crate::FieldReader<bool, bool>);
impl EPL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EPL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EPL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EWL` reader - Warning Status Interrupt Line"]
pub struct EWL_R(crate::FieldReader<bool, bool>);
impl EWL_R {
    pub(crate) fn new(bits: bool) -> Self {
        EWL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EWL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOL` reader - Bus_Off Status"]
pub struct BOL_R(crate::FieldReader<bool, bool>);
impl BOL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDIL` reader - Watchdog Interrupt Line"]
pub struct WDIL_R(crate::FieldReader<bool, bool>);
impl WDIL_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDIL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDIL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEAL` reader - Protocol Error in Arbitration Phase Line"]
pub struct PEAL_R(crate::FieldReader<bool, bool>);
impl PEAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PEDL` reader - Protocol Error in Data Phase Line"]
pub struct PEDL_R(crate::FieldReader<bool, bool>);
impl PEDL_R {
    pub(crate) fn new(bits: bool) -> Self {
        PEDL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PEDL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ARAL` reader - Access to Reserved Address Line"]
pub struct ARAL_R(crate::FieldReader<bool, bool>);
impl ARAL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ARAL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ARAL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Interrupt Line"]
    #[inline(always)]
    pub fn rf0nl(&self) -> RF0NL_R {
        RF0NL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn rf0wl(&self) -> RF0WL_R {
        RF0WL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Interrupt Line"]
    #[inline(always)]
    pub fn rf0fl(&self) -> RF0FL_R {
        RF0FL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn rf0ll(&self) -> RF0LL_R {
        RF0LL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Interrupt Line"]
    #[inline(always)]
    pub fn rf1nl(&self) -> RF1NL_R {
        RF1NL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn rf1wl(&self) -> RF1WL_R {
        RF1WL_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Interrupt Line"]
    #[inline(always)]
    pub fn rf1fl(&self) -> RF1FL_R {
        RF1FL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn rf1ll(&self) -> RF1LL_R {
        RF1LL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Interrupt Line"]
    #[inline(always)]
    pub fn hpml(&self) -> HPML_R {
        HPML_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Interrupt Line"]
    #[inline(always)]
    pub fn tcl(&self) -> TCL_R {
        TCL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Interrupt Line"]
    #[inline(always)]
    pub fn tcfl(&self) -> TCFL_R {
        TCFL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Interrupt Line"]
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Interrupt Line"]
    #[inline(always)]
    pub fn tefnl(&self) -> TEFNL_R {
        TEFNL_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn tefwl(&self) -> TEFWL_R {
        TEFWL_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Interrupt Line"]
    #[inline(always)]
    pub fn teffl(&self) -> TEFFL_R {
        TEFFL_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Interrupt Line"]
    #[inline(always)]
    pub fn tefll(&self) -> TEFLL_R {
        TEFLL_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Interrupt Line"]
    #[inline(always)]
    pub fn tswl(&self) -> TSWL_R {
        TSWL_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Interrupt Line"]
    #[inline(always)]
    pub fn mrafl(&self) -> MRAFL_R {
        MRAFL_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Interrupt Line"]
    #[inline(always)]
    pub fn tool(&self) -> TOOL_R {
        TOOL_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line"]
    #[inline(always)]
    pub fn drxl(&self) -> DRXL_R {
        DRXL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Line"]
    #[inline(always)]
    pub fn becl(&self) -> BECL_R {
        BECL_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Line"]
    #[inline(always)]
    pub fn beul(&self) -> BEUL_R {
        BEUL_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Interrupt Line"]
    #[inline(always)]
    pub fn elol(&self) -> ELOL_R {
        ELOL_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Error Passive Interrupt Line"]
    #[inline(always)]
    pub fn epl(&self) -> EPL_R {
        EPL_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Warning Status Interrupt Line"]
    #[inline(always)]
    pub fn ewl(&self) -> EWL_R {
        EWL_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bol(&self) -> BOL_R {
        BOL_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn wdil(&self) -> WDIL_R {
        WDIL_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    pub fn peal(&self) -> PEAL_R {
        PEAL_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Line"]
    #[inline(always)]
    pub fn pedl(&self) -> PEDL_R {
        PEDL_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Line"]
    #[inline(always)]
    pub fn aral(&self) -> ARAL_R {
        ARAL_R::new(((self.bits >> 29) & 0x01) != 0)
    }
}
#[doc = "FDCAN Interrupt Line Select Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fdcan_ils](index.html) module"]
pub struct FDCAN_ILS_SPEC;
impl crate::RegisterSpec for FDCAN_ILS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [fdcan_ils::R](R) reader structure"]
impl crate::Readable for FDCAN_ILS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets FDCAN_ILS to value 0"]
impl crate::Resettable for FDCAN_ILS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
