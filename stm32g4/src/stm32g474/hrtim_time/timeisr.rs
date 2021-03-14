#[doc = "Reader of register TIMEISR"]
pub type R = crate::R<u32, super::TIMEISR>;
#[doc = "Reader of field `O2CPY`"]
pub type O2CPY_R = crate::R<bool, bool>;
#[doc = "Reader of field `O1CPY`"]
pub type O1CPY_R = crate::R<bool, bool>;
#[doc = "Reader of field `O2STAT`"]
pub type O2STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `O1STAT`"]
pub type O1STAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `IPPSTAT`"]
pub type IPPSTAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `CPPSTAT`"]
pub type CPPSTAT_R = crate::R<bool, bool>;
#[doc = "Reader of field `DLYPRT`"]
pub type DLYPRT_R = crate::R<bool, bool>;
#[doc = "Reader of field `RST`"]
pub type RST_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTx2`"]
pub type RSTX2_R = crate::R<bool, bool>;
#[doc = "Reader of field `SETx2`"]
pub type SETX2_R = crate::R<bool, bool>;
#[doc = "Reader of field `RSTx1`"]
pub type RSTX1_R = crate::R<bool, bool>;
#[doc = "Reader of field `SETx1`"]
pub type SETX1_R = crate::R<bool, bool>;
#[doc = "Reader of field `CPT2`"]
pub type CPT2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CPT1`"]
pub type CPT1_R = crate::R<bool, bool>;
#[doc = "Reader of field `UPD`"]
pub type UPD_R = crate::R<bool, bool>;
#[doc = "Reader of field `REP`"]
pub type REP_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP4`"]
pub type CMP4_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP3`"]
pub type CMP3_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP2`"]
pub type CMP2_R = crate::R<bool, bool>;
#[doc = "Reader of field `CMP1`"]
pub type CMP1_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 21 - Output 2 Copy"]
    #[inline(always)]
    pub fn o2cpy(&self) -> O2CPY_R {
        O2CPY_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Output 1 Copy"]
    #[inline(always)]
    pub fn o1cpy(&self) -> O1CPY_R {
        O1CPY_R::new(((self.bits >> 20) & 0x01) != 0)
    }
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
