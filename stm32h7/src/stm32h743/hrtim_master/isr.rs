///Register `ISR` reader
pub type R = crate::R<ISRrs>;
/**Master Compare %s Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMP1 {
    ///0: No compare interrupt occurred
    NoEvent = 0,
    ///1: Compare interrupt occurred
    Event = 1,
}
impl From<CMP1> for bool {
    #[inline(always)]
    fn from(variant: CMP1) -> Self {
        variant as u8 != 0
    }
}
///Field `CMP(1-4)` reader - Master Compare %s Interrupt Flag
pub type CMP_R = crate::BitReader<CMP1>;
impl CMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CMP1 {
        match self.bits {
            false => CMP1::NoEvent,
            true => CMP1::Event,
        }
    }
    ///No compare interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CMP1::NoEvent
    }
    ///Compare interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CMP1::Event
    }
}
/**Master Repetition Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REP {
    ///0: No timer repetition interrupt occurred
    NoEvent = 0,
    ///1: Timer repetition interrupt occurred
    Event = 1,
}
impl From<REP> for bool {
    #[inline(always)]
    fn from(variant: REP) -> Self {
        variant as u8 != 0
    }
}
///Field `REP` reader - Master Repetition Interrupt Flag
pub type REP_R = crate::BitReader<REP>;
impl REP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REP {
        match self.bits {
            false => REP::NoEvent,
            true => REP::Event,
        }
    }
    ///No timer repetition interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == REP::NoEvent
    }
    ///Timer repetition interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == REP::Event
    }
}
/**Sync Input Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC {
    ///0: No sync input interrupt occurred
    NoEvent = 0,
    ///1: Sync input interrupt occurred
    Event = 1,
}
impl From<SYNC> for bool {
    #[inline(always)]
    fn from(variant: SYNC) -> Self {
        variant as u8 != 0
    }
}
///Field `SYNC` reader - Sync Input Interrupt Flag
pub type SYNC_R = crate::BitReader<SYNC>;
impl SYNC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SYNC {
        match self.bits {
            false => SYNC::NoEvent,
            true => SYNC::Event,
        }
    }
    ///No sync input interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SYNC::NoEvent
    }
    ///Sync input interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SYNC::Event
    }
}
/**Master Update Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPD {
    ///0: No timer update interrupt occurred
    NoEvent = 0,
    ///1: Timer update interrupt occurred
    Event = 1,
}
impl From<UPD> for bool {
    #[inline(always)]
    fn from(variant: UPD) -> Self {
        variant as u8 != 0
    }
}
///Field `UPD` reader - Master Update Interrupt Flag
pub type UPD_R = crate::BitReader<UPD>;
impl UPD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UPD {
        match self.bits {
            false => UPD::NoEvent,
            true => UPD::Event,
        }
    }
    ///No timer update interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == UPD::NoEvent
    }
    ///Timer update interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == UPD::Event
    }
}
impl R {
    ///Master Compare (1-4) Interrupt Flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1` field.</div>
    #[inline(always)]
    pub fn cmp(&self, n: u8) -> CMP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMP_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Master Compare (1-4) Interrupt Flag
    #[inline(always)]
    pub fn cmp_iter(&self) -> impl Iterator<Item = CMP_R> + '_ {
        (0..4).map(move |n| CMP_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Master Compare 1 Interrupt Flag
    #[inline(always)]
    pub fn cmp1(&self) -> CMP_R {
        CMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Master Compare 2 Interrupt Flag
    #[inline(always)]
    pub fn cmp2(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master Compare 3 Interrupt Flag
    #[inline(always)]
    pub fn cmp3(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Master Compare 4 Interrupt Flag
    #[inline(always)]
    pub fn cmp4(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Master Repetition Interrupt Flag
    #[inline(always)]
    pub fn rep(&self) -> REP_R {
        REP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Sync Input Interrupt Flag
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Master Update Interrupt Flag
    #[inline(always)]
    pub fn upd(&self) -> UPD_R {
        UPD_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("upd", &self.upd())
            .field("sync", &self.sync())
            .field("rep", &self.rep())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("cmp3", &self.cmp3())
            .field("cmp4", &self.cmp4())
            .finish()
    }
}
/**Master Timer Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#HRTIM_Master:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
