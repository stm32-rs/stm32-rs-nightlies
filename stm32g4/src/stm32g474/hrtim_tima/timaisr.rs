///Register `TIMAISR` reader
pub type R = crate::R<TIMAISRrs>;
///Field `CMP1` reader - Compare 1 Interrupt Flag
pub type CMP1_R = crate::BitReader;
///Field `CMP2` reader - Compare 2 Interrupt Flag
pub type CMP2_R = crate::BitReader;
///Field `CMP3` reader - Compare 3 Interrupt Flag
pub type CMP3_R = crate::BitReader;
///Field `CMP4` reader - Compare 4 Interrupt Flag
pub type CMP4_R = crate::BitReader;
///Field `REP` reader - Repetition Interrupt Flag
pub type REP_R = crate::BitReader;
///Field `UPD` reader - Update Interrupt Flag
pub type UPD_R = crate::BitReader;
///Field `CPT1` reader - Capture1 Interrupt Flag
pub type CPT1_R = crate::BitReader;
///Field `CPT2` reader - Capture2 Interrupt Flag
pub type CPT2_R = crate::BitReader;
///Field `SETx1` reader - Output 1 Set Interrupt Flag
pub type SETX1_R = crate::BitReader;
///Field `RSTx1` reader - Output 1 Reset Interrupt Flag
pub type RSTX1_R = crate::BitReader;
///Field `SETx2` reader - Output 2 Set Interrupt Flag
pub type SETX2_R = crate::BitReader;
///Field `RSTx2` reader - Output 2 Reset Interrupt Flag
pub type RSTX2_R = crate::BitReader;
///Field `RST` reader - Reset Interrupt Flag
pub type RST_R = crate::BitReader;
///Field `DLYPRT` reader - Delayed Protection Flag
pub type DLYPRT_R = crate::BitReader;
///Field `CPPSTAT` reader - Current Push Pull Status
pub type CPPSTAT_R = crate::BitReader;
///Field `IPPSTAT` reader - Idle Push Pull Status
pub type IPPSTAT_R = crate::BitReader;
///Field `O1STAT` reader - Output 1 State
pub type O1STAT_R = crate::BitReader;
///Field `O2STAT` reader - Output 2 State
pub type O2STAT_R = crate::BitReader;
///Field `O1CPY` reader - Output 1 Copy
pub type O1CPY_R = crate::BitReader;
///Field `O2CPY` reader - Output 2 Copy
pub type O2CPY_R = crate::BitReader;
impl R {
    ///Bit 0 - Compare 1 Interrupt Flag
    #[inline(always)]
    pub fn cmp1(&self) -> CMP1_R {
        CMP1_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare 2 Interrupt Flag
    #[inline(always)]
    pub fn cmp2(&self) -> CMP2_R {
        CMP2_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare 3 Interrupt Flag
    #[inline(always)]
    pub fn cmp3(&self) -> CMP3_R {
        CMP3_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare 4 Interrupt Flag
    #[inline(always)]
    pub fn cmp4(&self) -> CMP4_R {
        CMP4_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Repetition Interrupt Flag
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Update Interrupt Flag
    #[inline(always)]
    pub fn upd(&self) -> UPD_R {
        UPD_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Capture1 Interrupt Flag
    #[inline(always)]
    pub fn cpt1(&self) -> CPT1_R {
        CPT1_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture2 Interrupt Flag
    #[inline(always)]
    pub fn cpt2(&self) -> CPT2_R {
        CPT2_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Output 1 Set Interrupt Flag
    #[inline(always)]
    pub fn setx1(&self) -> SETX1_R {
        SETX1_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Output 1 Reset Interrupt Flag
    #[inline(always)]
    pub fn rstx1(&self) -> RSTX1_R {
        RSTX1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Output 2 Set Interrupt Flag
    #[inline(always)]
    pub fn setx2(&self) -> SETX2_R {
        SETX2_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Output 2 Reset Interrupt Flag
    #[inline(always)]
    pub fn rstx2(&self) -> RSTX2_R {
        RSTX2_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Reset Interrupt Flag
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Delayed Protection Flag
    #[inline(always)]
    pub fn dlyprt(&self) -> DLYPRT_R {
        DLYPRT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 16 - Current Push Pull Status
    #[inline(always)]
    pub fn cppstat(&self) -> CPPSTAT_R {
        CPPSTAT_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Idle Push Pull Status
    #[inline(always)]
    pub fn ippstat(&self) -> IPPSTAT_R {
        IPPSTAT_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Output 1 State
    #[inline(always)]
    pub fn o1stat(&self) -> O1STAT_R {
        O1STAT_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Output 2 State
    #[inline(always)]
    pub fn o2stat(&self) -> O2STAT_R {
        O2STAT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output 1 Copy
    #[inline(always)]
    pub fn o1cpy(&self) -> O1CPY_R {
        O1CPY_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Output 2 Copy
    #[inline(always)]
    pub fn o2cpy(&self) -> O2CPY_R {
        O2CPY_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMAISR")
            .field("o2cpy", &self.o2cpy())
            .field("o1cpy", &self.o1cpy())
            .field("o2stat", &self.o2stat())
            .field("o1stat", &self.o1stat())
            .field("ippstat", &self.ippstat())
            .field("cppstat", &self.cppstat())
            .field("dlyprt", &self.dlyprt())
            .field("rst", &self.rst())
            .field("rstx2", &self.rstx2())
            .field("setx2", &self.setx2())
            .field("rstx1", &self.rstx1())
            .field("setx1", &self.setx1())
            .field("cpt2", &self.cpt2())
            .field("cpt1", &self.cpt1())
            .field("upd", &self.upd())
            .field("rep", &self.rep())
            .field("cmp4", &self.cmp4())
            .field("cmp3", &self.cmp3())
            .field("cmp2", &self.cmp2())
            .field("cmp1", &self.cmp1())
            .finish()
    }
}
/**Timerx Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`timaisr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474xx.html#HRTIM_TIMA:TIMAISR)*/
pub struct TIMAISRrs;
impl crate::RegisterSpec for TIMAISRrs {
    type Ux = u32;
}
///`read()` method returns [`timaisr::R`](R) reader structure
impl crate::Readable for TIMAISRrs {}
///`reset()` method sets TIMAISR to value 0
impl crate::Resettable for TIMAISRrs {
    const RESET_VALUE: u32 = 0;
}
