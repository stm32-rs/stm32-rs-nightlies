///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CKPSC` reader - HRTIM Timer x Clock prescaler
pub type CKPSC_R = crate::FieldReader;
///Field `CKPSC` writer - HRTIM Timer x Clock prescaler
pub type CKPSC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, u8, crate::Safe>;
///Continuous mode
pub use crate::stm32h755cm7::hrtim_master::cr::CONT;
///Field `CONT` reader - Continuous mode
pub use crate::stm32h755cm7::hrtim_master::cr::CONT_R;
///Field `CONT` writer - Continuous mode
pub use crate::stm32h755cm7::hrtim_master::cr::CONT_W;
///Half mode enable
pub use crate::stm32h755cm7::hrtim_master::cr::HALF;
///Field `HALF` reader - Half mode enable
pub use crate::stm32h755cm7::hrtim_master::cr::HALF_R;
///Field `HALF` writer - Half mode enable
pub use crate::stm32h755cm7::hrtim_master::cr::HALF_W;
///Re-triggerable mode
pub use crate::stm32h755cm7::hrtim_master::cr::RETRIG;
///Field `RETRIG` reader - Re-triggerable mode
pub use crate::stm32h755cm7::hrtim_master::cr::RETRIG_R;
///Field `RETRIG` writer - Re-triggerable mode
pub use crate::stm32h755cm7::hrtim_master::cr::RETRIG_W;
/**Push-Pull mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSHPLL {
    ///0: Push-pull mode disabled
    Disabled = 0,
    ///1: Push-pull mode enabled
    Enabled = 1,
}
impl From<PSHPLL> for bool {
    #[inline(always)]
    fn from(variant: PSHPLL) -> Self {
        variant as u8 != 0
    }
}
///Field `PSHPLL` reader - Push-Pull mode enable
pub type PSHPLL_R = crate::BitReader<PSHPLL>;
impl PSHPLL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSHPLL {
        match self.bits {
            false => PSHPLL::Disabled,
            true => PSHPLL::Enabled,
        }
    }
    ///Push-pull mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PSHPLL::Disabled
    }
    ///Push-pull mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PSHPLL::Enabled
    }
}
///Field `PSHPLL` writer - Push-Pull mode enable
pub type PSHPLL_W<'a, REG> = crate::BitWriter<'a, REG, PSHPLL>;
impl<'a, REG> PSHPLL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Push-pull mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSHPLL::Disabled)
    }
    ///Push-pull mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PSHPLL::Enabled)
    }
}
///Synchronization Resets Timer x
pub use crate::stm32h755cm7::hrtim_master::cr::SYNCRST;
///Field `SYNCRST` reader - Synchronization Resets Timer x
pub use crate::stm32h755cm7::hrtim_master::cr::SYNCRST_R;
///Field `SYNCRST` writer - Synchronization Resets Timer x
pub use crate::stm32h755cm7::hrtim_master::cr::SYNCRST_W;
///Synchronization Starts Timer x
pub use crate::stm32h755cm7::hrtim_master::cr::SYNCSTRT;
///Field `SYNCSTRT` reader - Synchronization Starts Timer x
pub use crate::stm32h755cm7::hrtim_master::cr::SYNCSTRT_R;
///Field `SYNCSTRT` writer - Synchronization Starts Timer x
pub use crate::stm32h755cm7::hrtim_master::cr::SYNCSTRT_W;
/**Delayed CMP2 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELCMP2 {
    ///0: CMP2 register is always active (standard compare mode)
    Standard = 0,
    ///1: CMP2 is recomputed and is active following a capture 1 event
    Capture1 = 1,
    ///2: CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match
    Capture1Compare1 = 2,
    ///3: CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match
    Capture1Compare3 = 3,
}
impl From<DELCMP2> for u8 {
    #[inline(always)]
    fn from(variant: DELCMP2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DELCMP2 {
    type Ux = u8;
}
impl crate::IsEnum for DELCMP2 {}
///Field `DELCMP2` reader - Delayed CMP2 mode
pub type DELCMP2_R = crate::FieldReader<DELCMP2>;
impl DELCMP2_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DELCMP2 {
        match self.bits {
            0 => DELCMP2::Standard,
            1 => DELCMP2::Capture1,
            2 => DELCMP2::Capture1Compare1,
            3 => DELCMP2::Capture1Compare3,
            _ => unreachable!(),
        }
    }
    ///CMP2 register is always active (standard compare mode)
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP2::Standard
    }
    ///CMP2 is recomputed and is active following a capture 1 event
    #[inline(always)]
    pub fn is_capture1(&self) -> bool {
        *self == DELCMP2::Capture1
    }
    ///CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match
    #[inline(always)]
    pub fn is_capture1_compare1(&self) -> bool {
        *self == DELCMP2::Capture1Compare1
    }
    ///CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match
    #[inline(always)]
    pub fn is_capture1_compare3(&self) -> bool {
        *self == DELCMP2::Capture1Compare3
    }
}
///Field `DELCMP2` writer - Delayed CMP2 mode
pub type DELCMP2_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DELCMP2, crate::Safe>;
impl<'a, REG> DELCMP2_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CMP2 register is always active (standard compare mode)
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Standard)
    }
    ///CMP2 is recomputed and is active following a capture 1 event
    #[inline(always)]
    pub fn capture1(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Capture1)
    }
    ///CMP2 is recomputed and is active following a capture 1 event or a Compare 1 match
    #[inline(always)]
    pub fn capture1_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Capture1Compare1)
    }
    ///CMP2 is recomputed and is active following a capture 1 event or a Compare 3 match
    #[inline(always)]
    pub fn capture1_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP2::Capture1Compare3)
    }
}
/**Delayed CMP4 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DELCMP4 {
    ///0: CMP4 register is always active (standard compare mode)
    Standard = 0,
    ///1: CMP4 is recomputed and is active following a capture 2 event
    Capture2 = 1,
    ///2: CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match
    Capture2Compare1 = 2,
    ///3: CMP4 is recomputed and is active following a capture event or a Compare 3 match
    CaptureCompare3 = 3,
}
impl From<DELCMP4> for u8 {
    #[inline(always)]
    fn from(variant: DELCMP4) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DELCMP4 {
    type Ux = u8;
}
impl crate::IsEnum for DELCMP4 {}
///Field `DELCMP4` reader - Delayed CMP4 mode
pub type DELCMP4_R = crate::FieldReader<DELCMP4>;
impl DELCMP4_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DELCMP4 {
        match self.bits {
            0 => DELCMP4::Standard,
            1 => DELCMP4::Capture2,
            2 => DELCMP4::Capture2Compare1,
            3 => DELCMP4::CaptureCompare3,
            _ => unreachable!(),
        }
    }
    ///CMP4 register is always active (standard compare mode)
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == DELCMP4::Standard
    }
    ///CMP4 is recomputed and is active following a capture 2 event
    #[inline(always)]
    pub fn is_capture2(&self) -> bool {
        *self == DELCMP4::Capture2
    }
    ///CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match
    #[inline(always)]
    pub fn is_capture2_compare1(&self) -> bool {
        *self == DELCMP4::Capture2Compare1
    }
    ///CMP4 is recomputed and is active following a capture event or a Compare 3 match
    #[inline(always)]
    pub fn is_capture_compare3(&self) -> bool {
        *self == DELCMP4::CaptureCompare3
    }
}
///Field `DELCMP4` writer - Delayed CMP4 mode
pub type DELCMP4_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DELCMP4, crate::Safe>;
impl<'a, REG> DELCMP4_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CMP4 register is always active (standard compare mode)
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::Standard)
    }
    ///CMP4 is recomputed and is active following a capture 2 event
    #[inline(always)]
    pub fn capture2(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::Capture2)
    }
    ///CMP4 is recomputed and is active following a capture 2 event or a Compare 1 match
    #[inline(always)]
    pub fn capture2_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::Capture2Compare1)
    }
    ///CMP4 is recomputed and is active following a capture event or a Compare 3 match
    #[inline(always)]
    pub fn capture_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(DELCMP4::CaptureCompare3)
    }
}
/**Timer x Repetition update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TREPU {
    ///0: Update by timer x repetition disabled
    Disabled = 0,
    ///1: Update by timer x repetition enabled
    Enabled = 1,
}
impl From<TREPU> for bool {
    #[inline(always)]
    fn from(variant: TREPU) -> Self {
        variant as u8 != 0
    }
}
///Field `TREPU` reader - Timer x Repetition update
pub type TREPU_R = crate::BitReader<TREPU>;
impl TREPU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TREPU {
        match self.bits {
            false => TREPU::Disabled,
            true => TREPU::Enabled,
        }
    }
    ///Update by timer x repetition disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TREPU::Disabled
    }
    ///Update by timer x repetition enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TREPU::Enabled
    }
}
///Field `TREPU` writer - Timer x Repetition update
pub type TREPU_W<'a, REG> = crate::BitWriter<'a, REG, TREPU>;
impl<'a, REG> TREPU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update by timer x repetition disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TREPU::Disabled)
    }
    ///Update by timer x repetition enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TREPU::Enabled)
    }
}
/**Timerx reset update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRSTU {
    ///0: Update by timer x reset/roll-over disabled
    Disabled = 0,
    ///1: Update by timer x reset/roll-over enabled
    Enabled = 1,
}
impl From<TRSTU> for bool {
    #[inline(always)]
    fn from(variant: TRSTU) -> Self {
        variant as u8 != 0
    }
}
///Field `TRSTU` reader - Timerx reset update
pub type TRSTU_R = crate::BitReader<TRSTU>;
impl TRSTU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRSTU {
        match self.bits {
            false => TRSTU::Disabled,
            true => TRSTU::Enabled,
        }
    }
    ///Update by timer x reset/roll-over disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRSTU::Disabled
    }
    ///Update by timer x reset/roll-over enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRSTU::Enabled
    }
}
///Field `TRSTU` writer - Timerx reset update
pub type TRSTU_W<'a, REG> = crate::BitWriter<'a, REG, TRSTU>;
impl<'a, REG> TRSTU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update by timer x reset/roll-over disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRSTU::Disabled)
    }
    ///Update by timer x reset/roll-over enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRSTU::Enabled)
    }
}
/**TBU

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TBU {
    ///0: Update by timer x disabled
    Disabled = 0,
    ///1: Update by timer x enabled
    Enabled = 1,
}
impl From<TBU> for bool {
    #[inline(always)]
    fn from(variant: TBU) -> Self {
        variant as u8 != 0
    }
}
///Field `TBU` reader - TBU
pub type TBU_R = crate::BitReader<TBU>;
impl TBU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TBU {
        match self.bits {
            false => TBU::Disabled,
            true => TBU::Enabled,
        }
    }
    ///Update by timer x disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TBU::Disabled
    }
    ///Update by timer x enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TBU::Enabled
    }
}
///Field `TBU` writer - TBU
pub type TBU_W<'a, REG> = crate::BitWriter<'a, REG, TBU>;
impl<'a, REG> TBU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update by timer x disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBU::Disabled)
    }
    ///Update by timer x enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TBU::Enabled)
    }
}
///Field `TCU` reader - TCU
pub use TBU_R as TCU_R;
///Field `TDU` reader - TDU
pub use TBU_R as TDU_R;
///Field `TEU` reader - TEU
pub use TBU_R as TEU_R;
///Field `TCU` writer - TCU
pub use TBU_W as TCU_W;
///Field `TDU` writer - TDU
pub use TBU_W as TDU_W;
///Field `TEU` writer - TEU
pub use TBU_W as TEU_W;
/**Master Timer update

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTU {
    ///0: Update by master timer disabled
    Disabled = 0,
    ///1: Update by master timer enabled
    Enabled = 1,
}
impl From<MSTU> for bool {
    #[inline(always)]
    fn from(variant: MSTU) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTU` reader - Master Timer update
pub type MSTU_R = crate::BitReader<MSTU>;
impl MSTU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTU {
        match self.bits {
            false => MSTU::Disabled,
            true => MSTU::Enabled,
        }
    }
    ///Update by master timer disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MSTU::Disabled
    }
    ///Update by master timer enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MSTU::Enabled
    }
}
///Field `MSTU` writer - Master Timer update
pub type MSTU_W<'a, REG> = crate::BitWriter<'a, REG, MSTU>;
impl<'a, REG> MSTU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update by master timer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSTU::Disabled)
    }
    ///Update by master timer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MSTU::Enabled)
    }
}
///AC Synchronization
pub use crate::stm32h755cm7::hrtim_master::cr::DACSYNC;
///Field `DACSYNC` reader - AC Synchronization
pub use crate::stm32h755cm7::hrtim_master::cr::DACSYNC_R;
///Field `DACSYNC` writer - AC Synchronization
pub use crate::stm32h755cm7::hrtim_master::cr::DACSYNC_W;
///Preload enable
pub use crate::stm32h755cm7::hrtim_master::cr::PREEN;
///Field `PREEN` reader - Preload enable
pub use crate::stm32h755cm7::hrtim_master::cr::PREEN_R;
///Field `PREEN` writer - Preload enable
pub use crate::stm32h755cm7::hrtim_master::cr::PREEN_W;
/**Update Gating

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPDGAT {
    ///0: Update occurs independently from the DMA burst transfer
    Independent = 0,
    ///1: Update occurs when the DMA burst transfer is completed
    Dmaburst = 1,
    ///2: Update occurs on the update event following DMA burst transfer completion
    DmaburstUpdate = 2,
    ///3: Update occurs on a rising edge of HRTIM update enable input 1
    Input1 = 3,
    ///4: Update occurs on a rising edge of HRTIM update enable input 2
    Input2 = 4,
    ///5: Update occurs on a rising edge of HRTIM update enable input 3
    Input3 = 5,
    ///6: Update occurs on the update event following a rising edge of HRTIM update enable input 1
    Input1Update = 6,
    ///7: Update occurs on the update event following a rising edge of HRTIM update enable input 2
    Input2Update = 7,
    ///8: Update occurs on the update event following a rising edge of HRTIM update enable input 3
    Input3Update = 8,
}
impl From<UPDGAT> for u8 {
    #[inline(always)]
    fn from(variant: UPDGAT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for UPDGAT {
    type Ux = u8;
}
impl crate::IsEnum for UPDGAT {}
///Field `UPDGAT` reader - Update Gating
pub type UPDGAT_R = crate::FieldReader<UPDGAT>;
impl UPDGAT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<UPDGAT> {
        match self.bits {
            0 => Some(UPDGAT::Independent),
            1 => Some(UPDGAT::Dmaburst),
            2 => Some(UPDGAT::DmaburstUpdate),
            3 => Some(UPDGAT::Input1),
            4 => Some(UPDGAT::Input2),
            5 => Some(UPDGAT::Input3),
            6 => Some(UPDGAT::Input1Update),
            7 => Some(UPDGAT::Input2Update),
            8 => Some(UPDGAT::Input3Update),
            _ => None,
        }
    }
    ///Update occurs independently from the DMA burst transfer
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == UPDGAT::Independent
    }
    ///Update occurs when the DMA burst transfer is completed
    #[inline(always)]
    pub fn is_dmaburst(&self) -> bool {
        *self == UPDGAT::Dmaburst
    }
    ///Update occurs on the update event following DMA burst transfer completion
    #[inline(always)]
    pub fn is_dmaburst_update(&self) -> bool {
        *self == UPDGAT::DmaburstUpdate
    }
    ///Update occurs on a rising edge of HRTIM update enable input 1
    #[inline(always)]
    pub fn is_input1(&self) -> bool {
        *self == UPDGAT::Input1
    }
    ///Update occurs on a rising edge of HRTIM update enable input 2
    #[inline(always)]
    pub fn is_input2(&self) -> bool {
        *self == UPDGAT::Input2
    }
    ///Update occurs on a rising edge of HRTIM update enable input 3
    #[inline(always)]
    pub fn is_input3(&self) -> bool {
        *self == UPDGAT::Input3
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 1
    #[inline(always)]
    pub fn is_input1_update(&self) -> bool {
        *self == UPDGAT::Input1Update
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 2
    #[inline(always)]
    pub fn is_input2_update(&self) -> bool {
        *self == UPDGAT::Input2Update
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 3
    #[inline(always)]
    pub fn is_input3_update(&self) -> bool {
        *self == UPDGAT::Input3Update
    }
}
///Field `UPDGAT` writer - Update Gating
pub type UPDGAT_W<'a, REG> = crate::FieldWriter<'a, REG, 4, UPDGAT>;
impl<'a, REG> UPDGAT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Update occurs independently from the DMA burst transfer
    #[inline(always)]
    pub fn independent(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Independent)
    }
    ///Update occurs when the DMA burst transfer is completed
    #[inline(always)]
    pub fn dmaburst(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Dmaburst)
    }
    ///Update occurs on the update event following DMA burst transfer completion
    #[inline(always)]
    pub fn dmaburst_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::DmaburstUpdate)
    }
    ///Update occurs on a rising edge of HRTIM update enable input 1
    #[inline(always)]
    pub fn input1(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input1)
    }
    ///Update occurs on a rising edge of HRTIM update enable input 2
    #[inline(always)]
    pub fn input2(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input2)
    }
    ///Update occurs on a rising edge of HRTIM update enable input 3
    #[inline(always)]
    pub fn input3(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input3)
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 1
    #[inline(always)]
    pub fn input1_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input1Update)
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 2
    #[inline(always)]
    pub fn input2_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input2Update)
    }
    ///Update occurs on the update event following a rising edge of HRTIM update enable input 3
    #[inline(always)]
    pub fn input3_update(self) -> &'a mut crate::W<REG> {
        self.variant(UPDGAT::Input3Update)
    }
}
impl R {
    ///Bits 0:2 - HRTIM Timer x Clock prescaler
    #[inline(always)]
    pub fn ckpsc(&self) -> CKPSC_R {
        CKPSC_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Continuous mode
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&self) -> RETRIG_R {
        RETRIG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&self) -> HALF_R {
        HALF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Push-Pull mode enable
    #[inline(always)]
    pub fn pshpll(&self) -> PSHPLL_R {
        PSHPLL_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 10 - Synchronization Resets Timer x
    #[inline(always)]
    pub fn syncrst(&self) -> SYNCRST_R {
        SYNCRST_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Synchronization Starts Timer x
    #[inline(always)]
    pub fn syncstrt(&self) -> SYNCSTRT_R {
        SYNCSTRT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 12:13 - Delayed CMP2 mode
    #[inline(always)]
    pub fn delcmp2(&self) -> DELCMP2_R {
        DELCMP2_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Delayed CMP4 mode
    #[inline(always)]
    pub fn delcmp4(&self) -> DELCMP4_R {
        DELCMP4_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bit 17 - Timer x Repetition update
    #[inline(always)]
    pub fn trepu(&self) -> TREPU_R {
        TREPU_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Timerx reset update
    #[inline(always)]
    pub fn trstu(&self) -> TRSTU_R {
        TRSTU_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 20 - TBU
    #[inline(always)]
    pub fn tbu(&self) -> TBU_R {
        TBU_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - TCU
    #[inline(always)]
    pub fn tcu(&self) -> TCU_R {
        TCU_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - TDU
    #[inline(always)]
    pub fn tdu(&self) -> TDU_R {
        TDU_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - TEU
    #[inline(always)]
    pub fn teu(&self) -> TEU_R {
        TEU_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - Master Timer update
    #[inline(always)]
    pub fn mstu(&self) -> MSTU_R {
        MSTU_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&self) -> DACSYNC_R {
        DACSYNC_R::new(((self.bits >> 25) & 3) as u8)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&self) -> PREEN_R {
        PREEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31 - Update Gating
    #[inline(always)]
    pub fn updgat(&self) -> UPDGAT_R {
        UPDGAT_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("updgat", &self.updgat())
            .field("preen", &self.preen())
            .field("dacsync", &self.dacsync())
            .field("mstu", &self.mstu())
            .field("tbu", &self.tbu())
            .field("teu", &self.teu())
            .field("tdu", &self.tdu())
            .field("tcu", &self.tcu())
            .field("trstu", &self.trstu())
            .field("trepu", &self.trepu())
            .field("delcmp4", &self.delcmp4())
            .field("delcmp2", &self.delcmp2())
            .field("syncstrt", &self.syncstrt())
            .field("syncrst", &self.syncrst())
            .field("pshpll", &self.pshpll())
            .field("half", &self.half())
            .field("retrig", &self.retrig())
            .field("cont", &self.cont())
            .field("ckpsc", &self.ckpsc())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - HRTIM Timer x Clock prescaler
    #[inline(always)]
    pub fn ckpsc(&mut self) -> CKPSC_W<'_, CRrs> {
        CKPSC_W::new(self, 0)
    }
    ///Bit 3 - Continuous mode
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W<'_, CRrs> {
        CONT_W::new(self, 3)
    }
    ///Bit 4 - Re-triggerable mode
    #[inline(always)]
    pub fn retrig(&mut self) -> RETRIG_W<'_, CRrs> {
        RETRIG_W::new(self, 4)
    }
    ///Bit 5 - Half mode enable
    #[inline(always)]
    pub fn half(&mut self) -> HALF_W<'_, CRrs> {
        HALF_W::new(self, 5)
    }
    ///Bit 6 - Push-Pull mode enable
    #[inline(always)]
    pub fn pshpll(&mut self) -> PSHPLL_W<'_, CRrs> {
        PSHPLL_W::new(self, 6)
    }
    ///Bit 10 - Synchronization Resets Timer x
    #[inline(always)]
    pub fn syncrst(&mut self) -> SYNCRST_W<'_, CRrs> {
        SYNCRST_W::new(self, 10)
    }
    ///Bit 11 - Synchronization Starts Timer x
    #[inline(always)]
    pub fn syncstrt(&mut self) -> SYNCSTRT_W<'_, CRrs> {
        SYNCSTRT_W::new(self, 11)
    }
    ///Bits 12:13 - Delayed CMP2 mode
    #[inline(always)]
    pub fn delcmp2(&mut self) -> DELCMP2_W<'_, CRrs> {
        DELCMP2_W::new(self, 12)
    }
    ///Bits 14:15 - Delayed CMP4 mode
    #[inline(always)]
    pub fn delcmp4(&mut self) -> DELCMP4_W<'_, CRrs> {
        DELCMP4_W::new(self, 14)
    }
    ///Bit 17 - Timer x Repetition update
    #[inline(always)]
    pub fn trepu(&mut self) -> TREPU_W<'_, CRrs> {
        TREPU_W::new(self, 17)
    }
    ///Bit 18 - Timerx reset update
    #[inline(always)]
    pub fn trstu(&mut self) -> TRSTU_W<'_, CRrs> {
        TRSTU_W::new(self, 18)
    }
    ///Bit 20 - TBU
    #[inline(always)]
    pub fn tbu(&mut self) -> TBU_W<'_, CRrs> {
        TBU_W::new(self, 20)
    }
    ///Bit 21 - TCU
    #[inline(always)]
    pub fn tcu(&mut self) -> TCU_W<'_, CRrs> {
        TCU_W::new(self, 21)
    }
    ///Bit 22 - TDU
    #[inline(always)]
    pub fn tdu(&mut self) -> TDU_W<'_, CRrs> {
        TDU_W::new(self, 22)
    }
    ///Bit 23 - TEU
    #[inline(always)]
    pub fn teu(&mut self) -> TEU_W<'_, CRrs> {
        TEU_W::new(self, 23)
    }
    ///Bit 24 - Master Timer update
    #[inline(always)]
    pub fn mstu(&mut self) -> MSTU_W<'_, CRrs> {
        MSTU_W::new(self, 24)
    }
    ///Bits 25:26 - AC Synchronization
    #[inline(always)]
    pub fn dacsync(&mut self) -> DACSYNC_W<'_, CRrs> {
        DACSYNC_W::new(self, 25)
    }
    ///Bit 27 - Preload enable
    #[inline(always)]
    pub fn preen(&mut self) -> PREEN_W<'_, CRrs> {
        PREEN_W::new(self, 27)
    }
    ///Bits 28:31 - Update Gating
    #[inline(always)]
    pub fn updgat(&mut self) -> UPDGAT_W<'_, CRrs> {
        UPDGAT_W::new(self, 28)
    }
}
/**Timerx Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM7.html#HRTIM_TIMA:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
