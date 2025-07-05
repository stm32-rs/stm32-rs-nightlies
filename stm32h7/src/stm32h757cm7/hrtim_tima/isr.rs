///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Compare %s Interrupt Flag
pub use crate::stm32h757cm7::hrtim_master::isr::CMP1;
///Field `CMP(1-4)` reader - Compare %s Interrupt Flag
pub use crate::stm32h757cm7::hrtim_master::isr::CMP_R;
///Repetition Interrupt Flag
pub use crate::stm32h757cm7::hrtim_master::isr::REP;
///Field `REP` reader - Repetition Interrupt Flag
pub use crate::stm32h757cm7::hrtim_master::isr::REP_R;
///Update Interrupt Flag
pub use crate::stm32h757cm7::hrtim_master::isr::UPD;
///Field `UPD` reader - Update Interrupt Flag
pub use crate::stm32h757cm7::hrtim_master::isr::UPD_R;
/**Capture%s Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPT1 {
    ///0: No timer x capture reset interrupt occurred
    NoEvent = 0,
    ///1: Timer x capture reset interrupt occurred
    Event = 1,
}
impl From<CPT1> for bool {
    #[inline(always)]
    fn from(variant: CPT1) -> Self {
        variant as u8 != 0
    }
}
///Field `CPT(1-2)` reader - Capture%s Interrupt Flag
pub type CPT_R = crate::BitReader<CPT1>;
impl CPT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPT1 {
        match self.bits {
            false => CPT1::NoEvent,
            true => CPT1::Event,
        }
    }
    ///No timer x capture reset interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CPT1::NoEvent
    }
    ///Timer x capture reset interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CPT1::Event
    }
}
/**Output %s Set Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SET1 {
    ///0: No Tx output set interrupt occurred
    NoEvent = 0,
    ///1: Tx output set interrupt occurred
    Event = 1,
}
impl From<SET1> for bool {
    #[inline(always)]
    fn from(variant: SET1) -> Self {
        variant as u8 != 0
    }
}
///Field `SET(1-2)` reader - Output %s Set Interrupt Flag
pub type SET_R = crate::BitReader<SET1>;
impl SET_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SET1 {
        match self.bits {
            false => SET1::NoEvent,
            true => SET1::Event,
        }
    }
    ///No Tx output set interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == SET1::NoEvent
    }
    ///Tx output set interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == SET1::Event
    }
}
/**Output 1 Reset Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST1 {
    ///0: No Tx output reset interrupt occurred
    NoEvent = 0,
    ///1: Tx output reset interrupt occurred
    Event = 1,
}
impl From<RST1> for bool {
    #[inline(always)]
    fn from(variant: RST1) -> Self {
        variant as u8 != 0
    }
}
///Field `RST1` reader - Output 1 Reset Interrupt Flag
pub type RST1_R = crate::BitReader<RST1>;
impl RST1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RST1 {
        match self.bits {
            false => RST1::NoEvent,
            true => RST1::Event,
        }
    }
    ///No Tx output reset interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RST1::NoEvent
    }
    ///Tx output reset interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == RST1::Event
    }
}
///Field `RST2` reader - Output 2 Reset Interrupt Flag
pub use RST1_R as RST2_R;
/**Reset Interrupt Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST {
    ///0: No TIMx counter reset/roll-over interrupt occurred
    NoEvent = 0,
    ///1: TIMx counter reset/roll-over interrupt occurred
    Event = 1,
}
impl From<RST> for bool {
    #[inline(always)]
    fn from(variant: RST) -> Self {
        variant as u8 != 0
    }
}
///Field `RST` reader - Reset Interrupt Flag
pub type RST_R = crate::BitReader<RST>;
impl RST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RST {
        match self.bits {
            false => RST::NoEvent,
            true => RST::Event,
        }
    }
    ///No TIMx counter reset/roll-over interrupt occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == RST::NoEvent
    }
    ///TIMx counter reset/roll-over interrupt occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == RST::Event
    }
}
/**Delayed Protection Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DLYPRT {
    ///0: Not in delayed idle or balanced idle mode
    Inactive = 0,
    ///1: Delayed idle or balanced idle mode entry
    Active = 1,
}
impl From<DLYPRT> for bool {
    #[inline(always)]
    fn from(variant: DLYPRT) -> Self {
        variant as u8 != 0
    }
}
///Field `DLYPRT` reader - Delayed Protection Flag
pub type DLYPRT_R = crate::BitReader<DLYPRT>;
impl DLYPRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DLYPRT {
        match self.bits {
            false => DLYPRT::Inactive,
            true => DLYPRT::Active,
        }
    }
    ///Not in delayed idle or balanced idle mode
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == DLYPRT::Inactive
    }
    ///Delayed idle or balanced idle mode entry
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == DLYPRT::Active
    }
}
/**Current Push Pull Status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPPSTAT {
    ///0: Signal applied on output 1 and output 2 forced inactive
    Output1active = 0,
    ///1: Signal applied on output 2 and output 1 forced inactive
    Output2active = 1,
}
impl From<CPPSTAT> for bool {
    #[inline(always)]
    fn from(variant: CPPSTAT) -> Self {
        variant as u8 != 0
    }
}
///Field `CPPSTAT` reader - Current Push Pull Status
pub type CPPSTAT_R = crate::BitReader<CPPSTAT>;
impl CPPSTAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPPSTAT {
        match self.bits {
            false => CPPSTAT::Output1active,
            true => CPPSTAT::Output2active,
        }
    }
    ///Signal applied on output 1 and output 2 forced inactive
    #[inline(always)]
    pub fn is_output1active(&self) -> bool {
        *self == CPPSTAT::Output1active
    }
    ///Signal applied on output 2 and output 1 forced inactive
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        *self == CPPSTAT::Output2active
    }
}
/**Idle Push Pull Status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPPSTAT {
    ///0: Protection occurred when the output 1 was active and output 2 forced inactive
    Output1active = 0,
    ///1: Protection occurred when the output 2 was active and output 1 forced inactive
    Output2active = 1,
}
impl From<IPPSTAT> for bool {
    #[inline(always)]
    fn from(variant: IPPSTAT) -> Self {
        variant as u8 != 0
    }
}
///Field `IPPSTAT` reader - Idle Push Pull Status
pub type IPPSTAT_R = crate::BitReader<IPPSTAT>;
impl IPPSTAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IPPSTAT {
        match self.bits {
            false => IPPSTAT::Output1active,
            true => IPPSTAT::Output2active,
        }
    }
    ///Protection occurred when the output 1 was active and output 2 forced inactive
    #[inline(always)]
    pub fn is_output1active(&self) -> bool {
        *self == IPPSTAT::Output1active
    }
    ///Protection occurred when the output 2 was active and output 1 forced inactive
    #[inline(always)]
    pub fn is_output2active(&self) -> bool {
        *self == IPPSTAT::Output2active
    }
}
/**Output 1 State

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum O1STAT {
    ///0: Output was inactive
    Inactive = 0,
    ///1: Output was active
    Active = 1,
}
impl From<O1STAT> for bool {
    #[inline(always)]
    fn from(variant: O1STAT) -> Self {
        variant as u8 != 0
    }
}
///Field `O1STAT` reader - Output 1 State
pub type O1STAT_R = crate::BitReader<O1STAT>;
impl O1STAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> O1STAT {
        match self.bits {
            false => O1STAT::Inactive,
            true => O1STAT::Active,
        }
    }
    ///Output was inactive
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == O1STAT::Inactive
    }
    ///Output was active
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == O1STAT::Active
    }
}
///Field `O2STAT` reader - Output 2 State
pub use O1STAT_R as O2STAT_R;
impl R {
    ///Compare (1-4) Interrupt Flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CMP1` field.</div>
    #[inline(always)]
    pub fn cmp(&self, n: u8) -> CMP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CMP_R::new(((self.bits >> n) & 1) != 0)
    }
    ///Iterator for array of:
    ///Compare (1-4) Interrupt Flag
    #[inline(always)]
    pub fn cmp_iter(&self) -> impl Iterator<Item = CMP_R> + '_ {
        (0..4).map(move |n| CMP_R::new(((self.bits >> n) & 1) != 0))
    }
    ///Bit 0 - Compare 1 Interrupt Flag
    #[inline(always)]
    pub fn cmp1(&self) -> CMP_R {
        CMP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Compare 2 Interrupt Flag
    #[inline(always)]
    pub fn cmp2(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Compare 3 Interrupt Flag
    #[inline(always)]
    pub fn cmp3(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Compare 4 Interrupt Flag
    #[inline(always)]
    pub fn cmp4(&self) -> CMP_R {
        CMP_R::new(((self.bits >> 3) & 1) != 0)
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
    ///Capture(1-2) Interrupt Flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CPT1` field.</div>
    #[inline(always)]
    pub fn cpt(&self, n: u8) -> CPT_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CPT_R::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture(1-2) Interrupt Flag
    #[inline(always)]
    pub fn cpt_iter(&self) -> impl Iterator<Item = CPT_R> + '_ {
        (0..2).map(move |n| CPT_R::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    ///Bit 7 - Capture1 Interrupt Flag
    #[inline(always)]
    pub fn cpt1(&self) -> CPT_R {
        CPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture2 Interrupt Flag
    #[inline(always)]
    pub fn cpt2(&self) -> CPT_R {
        CPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Output (1-2) Set Interrupt Flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `SET1` field.</div>
    #[inline(always)]
    pub fn set_(&self, n: u8) -> SET_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        SET_R::new(((self.bits >> (n * 2 + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Output (1-2) Set Interrupt Flag
    #[inline(always)]
    pub fn set__iter(&self) -> impl Iterator<Item = SET_R> + '_ {
        (0..2).map(move |n| SET_R::new(((self.bits >> (n * 2 + 9)) & 1) != 0))
    }
    ///Bit 9 - Output 1 Set Interrupt Flag
    #[inline(always)]
    pub fn set1(&self) -> SET_R {
        SET_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Output 2 Set Interrupt Flag
    #[inline(always)]
    pub fn set2(&self) -> SET_R {
        SET_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 10 - Output 1 Reset Interrupt Flag
    #[inline(always)]
    pub fn rst1(&self) -> RST1_R {
        RST1_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 12 - Output 2 Reset Interrupt Flag
    #[inline(always)]
    pub fn rst2(&self) -> RST2_R {
        RST2_R::new(((self.bits >> 12) & 1) != 0)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("o1stat", &self.o1stat())
            .field("o2stat", &self.o2stat())
            .field("ippstat", &self.ippstat())
            .field("cppstat", &self.cppstat())
            .field("dlyprt", &self.dlyprt())
            .field("rst", &self.rst())
            .field("rst1", &self.rst1())
            .field("rst2", &self.rst2())
            .field("set1", &self.set1())
            .field("set2", &self.set2())
            .field("cpt1", &self.cpt1())
            .field("cpt2", &self.cpt2())
            .field("upd", &self.upd())
            .field("rep", &self.rep())
            .field("cmp1", &self.cmp1())
            .field("cmp2", &self.cmp2())
            .field("cmp3", &self.cmp3())
            .field("cmp4", &self.cmp4())
            .finish()
    }
}
/**Timerx Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#HRTIM_TIMA:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
