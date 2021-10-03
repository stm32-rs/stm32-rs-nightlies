#[doc = "Register `TIMEISR` reader"]
pub struct R(crate::R<TIMEISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMEISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMEISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMEISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `O2STAT` reader - Output 2 State"]
pub struct O2STAT_R(crate::FieldReader<bool, bool>);
impl O2STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        O2STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for O2STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `O1STAT` reader - Output 1 State"]
pub struct O1STAT_R(crate::FieldReader<bool, bool>);
impl O1STAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        O1STAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for O1STAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IPPSTAT` reader - Idle Push Pull Status"]
pub struct IPPSTAT_R(crate::FieldReader<bool, bool>);
impl IPPSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IPPSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IPPSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPPSTAT` reader - Current Push Pull Status"]
pub struct CPPSTAT_R(crate::FieldReader<bool, bool>);
impl CPPSTAT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPPSTAT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPPSTAT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DLYPRT` reader - Delayed Protection Flag"]
pub struct DLYPRT_R(crate::FieldReader<bool, bool>);
impl DLYPRT_R {
    pub(crate) fn new(bits: bool) -> Self {
        DLYPRT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DLYPRT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RST` reader - Reset Interrupt Flag"]
pub struct RST_R(crate::FieldReader<bool, bool>);
impl RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTx2` reader - Output 2 Reset Interrupt Flag"]
pub struct RSTX2_R(crate::FieldReader<bool, bool>);
impl RSTX2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTX2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETx2` reader - Output 2 Set Interrupt Flag"]
pub struct SETX2_R(crate::FieldReader<bool, bool>);
impl SETX2_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETX2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETX2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSTx1` reader - Output 1 Reset Interrupt Flag"]
pub struct RSTX1_R(crate::FieldReader<bool, bool>);
impl RSTX1_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSTX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSTX1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SETx1` reader - Output 1 Set Interrupt Flag"]
pub struct SETX1_R(crate::FieldReader<bool, bool>);
impl SETX1_R {
    pub(crate) fn new(bits: bool) -> Self {
        SETX1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SETX1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPT2` reader - Capture2 Interrupt Flag"]
pub struct CPT2_R(crate::FieldReader<bool, bool>);
impl CPT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPT1` reader - Capture1 Interrupt Flag"]
pub struct CPT1_R(crate::FieldReader<bool, bool>);
impl CPT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPD` reader - Update Interrupt Flag"]
pub struct UPD_R(crate::FieldReader<bool, bool>);
impl UPD_R {
    pub(crate) fn new(bits: bool) -> Self {
        UPD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UPD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REP` reader - Repetition Interrupt Flag"]
pub struct REP_R(crate::FieldReader<bool, bool>);
impl REP_R {
    pub(crate) fn new(bits: bool) -> Self {
        REP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP4` reader - Compare 4 Interrupt Flag"]
pub struct CMP4_R(crate::FieldReader<bool, bool>);
impl CMP4_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP4_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP4_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP3` reader - Compare 3 Interrupt Flag"]
pub struct CMP3_R(crate::FieldReader<bool, bool>);
impl CMP3_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP2` reader - Compare 2 Interrupt Flag"]
pub struct CMP2_R(crate::FieldReader<bool, bool>);
impl CMP2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CMP1` reader - Compare 1 Interrupt Flag"]
pub struct CMP1_R(crate::FieldReader<bool, bool>);
impl CMP1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CMP1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CMP1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 19 - Output 2 State"]
    #[inline(always)]
    pub fn o2stat(&self) -> O2STAT_R {
        O2STAT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Output 1 State"]
    #[inline(always)]
    pub fn o1stat(&self) -> O1STAT_R {
        O1STAT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Idle Push Pull Status"]
    #[inline(always)]
    pub fn ippstat(&self) -> IPPSTAT_R {
        IPPSTAT_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Current Push Pull Status"]
    #[inline(always)]
    pub fn cppstat(&self) -> CPPSTAT_R {
        CPPSTAT_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Delayed Protection Flag"]
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Output 2 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx2(&self) -> RSTX2_R {
        RSTX2_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Output 2 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx2(&self) -> SETX2_R {
        SETX2_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Output 1 Reset Interrupt Flag"]
    #[inline(always)]
    pub fn rstx1(&self) -> RSTX1_R {
        RSTX1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Output 1 Set Interrupt Flag"]
    #[inline(always)]
    pub fn setx1(&self) -> SETX1_R {
        SETX1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Capture2 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt2(&self) -> CPT2_R {
        CPT2_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Capture1 Interrupt Flag"]
    #[inline(always)]
    pub fn cpt1(&self) -> CPT1_R {
        CPT1_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Update Interrupt Flag"]
    #[inline(always)]
    pub fn upd(&self) -> UPD_R {
        UPD_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Repetition Interrupt Flag"]
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Compare 4 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Compare 3 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Compare 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Compare 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new((self.bits & 0x01) != 0)
    }
}
#[doc = "Timerx Interrupt Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timeisr](index.html) module"]
pub struct TIMEISR_SPEC;
impl crate::RegisterSpec for TIMEISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timeisr::R](R) reader structure"]
impl crate::Readable for TIMEISR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMEISR to value 0"]
impl crate::Resettable for TIMEISR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
