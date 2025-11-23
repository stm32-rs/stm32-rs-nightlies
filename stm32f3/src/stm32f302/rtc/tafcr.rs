///Register `TAFCR` reader
pub type R = crate::R<TAFCRrs>;
///Register `TAFCR` writer
pub type W = crate::W<TAFCRrs>;
/**Tamper 1 detection enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1E {
    ///0: RTC_TAMPx input detection disabled
    Disabled = 0,
    ///1: RTC_TAMPx input detection enabled
    Enabled = 1,
}
impl From<TAMP1E> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1E` reader - Tamper 1 detection enable
pub type TAMP1E_R = crate::BitReader<TAMP1E>;
impl TAMP1E_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1E {
        match self.bits {
            false => TAMP1E::Disabled,
            true => TAMP1E::Enabled,
        }
    }
    ///RTC_TAMPx input detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMP1E::Disabled
    }
    ///RTC_TAMPx input detection enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMP1E::Enabled
    }
}
///Field `TAMP1E` writer - Tamper 1 detection enable
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1E>;
impl<'a, REG> TAMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC_TAMPx input detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E::Disabled)
    }
    ///RTC_TAMPx input detection enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E::Enabled)
    }
}
/**Active level for tamper 1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1TRG {
    ///0: If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input staying low triggers a tamper detection event.
    RisingEdge = 0,
    ///1: If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input falling edge triggers a tamper detection event
    FallingEdge = 1,
}
impl From<TAMP1TRG> for bool {
    #[inline(always)]
    fn from(variant: TAMP1TRG) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMP1TRG` reader - Active level for tamper 1
pub type TAMP1TRG_R = crate::BitReader<TAMP1TRG>;
impl TAMP1TRG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1TRG {
        match self.bits {
            false => TAMP1TRG::RisingEdge,
            true => TAMP1TRG::FallingEdge,
        }
    }
    ///If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input staying low triggers a tamper detection event.
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == TAMP1TRG::RisingEdge
    }
    ///If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input falling edge triggers a tamper detection event
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == TAMP1TRG::FallingEdge
    }
}
///Field `TAMP1TRG` writer - Active level for tamper 1
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1TRG>;
impl<'a, REG> TAMP1TRG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///If TAMPFLT = 00: RTC_TAMPx input rising edge triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input staying low triggers a tamper detection event.
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1TRG::RisingEdge)
    }
    ///If TAMPFLT = 00: RTC_TAMPx input staying high triggers a tamper detection event. If TAMPFLT ≠ 00: RTC_TAMPx input falling edge triggers a tamper detection event
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1TRG::FallingEdge)
    }
}
/**Tamper interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPIE {
    ///0: Tamper interrupt disabled
    Disabled = 0,
    ///1: Tamper interrupt enabled
    Enabled = 1,
}
impl From<TAMPIE> for bool {
    #[inline(always)]
    fn from(variant: TAMPIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPIE` reader - Tamper interrupt enable
pub type TAMPIE_R = crate::BitReader<TAMPIE>;
impl TAMPIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPIE {
        match self.bits {
            false => TAMPIE::Disabled,
            true => TAMPIE::Enabled,
        }
    }
    ///Tamper interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPIE::Disabled
    }
    ///Tamper interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPIE::Enabled
    }
}
///Field `TAMPIE` writer - Tamper interrupt enable
pub type TAMPIE_W<'a, REG> = crate::BitWriter<'a, REG, TAMPIE>;
impl<'a, REG> TAMPIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPIE::Disabled)
    }
    ///Tamper interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPIE::Enabled)
    }
}
///Field `TAMP2E` reader - Tamper 2 detection enable
pub use TAMP1E_R as TAMP2E_R;
///Field `TAMP3E` reader - Tamper 3 detection enable
pub use TAMP1E_R as TAMP3E_R;
///Field `TAMP2E` writer - Tamper 2 detection enable
pub use TAMP1E_W as TAMP2E_W;
///Field `TAMP3E` writer - Tamper 3 detection enable
pub use TAMP1E_W as TAMP3E_W;
///Field `TAMP2TRG` reader - Active level for tamper 2
pub use TAMP1TRG_R as TAMP2TRG_R;
///Field `TAMP3TRG` reader - Active level for tamper 3
pub use TAMP1TRG_R as TAMP3TRG_R;
///Field `TAMP2TRG` writer - Active level for tamper 2
pub use TAMP1TRG_W as TAMP2TRG_W;
///Field `TAMP3TRG` writer - Active level for tamper 3
pub use TAMP1TRG_W as TAMP3TRG_W;
/**Activate timestamp on tamper detection event

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPTS {
    ///0: Tamper detection event does not cause a timestamp to be saved
    NoSave = 0,
    ///1: Save timestamp on tamper detection event
    Save = 1,
}
impl From<TAMPTS> for bool {
    #[inline(always)]
    fn from(variant: TAMPTS) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPTS` reader - Activate timestamp on tamper detection event
pub type TAMPTS_R = crate::BitReader<TAMPTS>;
impl TAMPTS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPTS {
        match self.bits {
            false => TAMPTS::NoSave,
            true => TAMPTS::Save,
        }
    }
    ///Tamper detection event does not cause a timestamp to be saved
    #[inline(always)]
    pub fn is_no_save(&self) -> bool {
        *self == TAMPTS::NoSave
    }
    ///Save timestamp on tamper detection event
    #[inline(always)]
    pub fn is_save(&self) -> bool {
        *self == TAMPTS::Save
    }
}
///Field `TAMPTS` writer - Activate timestamp on tamper detection event
pub type TAMPTS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPTS>;
impl<'a, REG> TAMPTS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tamper detection event does not cause a timestamp to be saved
    #[inline(always)]
    pub fn no_save(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS::NoSave)
    }
    ///Save timestamp on tamper detection event
    #[inline(always)]
    pub fn save(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPTS::Save)
    }
}
/**Tamper sampling frequency

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFREQ {
    ///0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    Div32768 = 0,
    ///1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    Div16384 = 1,
    ///2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    Div8192 = 2,
    ///3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    Div4096 = 3,
    ///4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    Div2048 = 4,
    ///5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    Div1024 = 5,
    ///6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    Div512 = 6,
    ///7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    Div256 = 7,
}
impl From<TAMPFREQ> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFREQ) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFREQ {
    type Ux = u8;
}
impl crate::IsEnum for TAMPFREQ {}
///Field `TAMPFREQ` reader - Tamper sampling frequency
pub type TAMPFREQ_R = crate::FieldReader<TAMPFREQ>;
impl TAMPFREQ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFREQ {
        match self.bits {
            0 => TAMPFREQ::Div32768,
            1 => TAMPFREQ::Div16384,
            2 => TAMPFREQ::Div8192,
            3 => TAMPFREQ::Div4096,
            4 => TAMPFREQ::Div2048,
            5 => TAMPFREQ::Div1024,
            6 => TAMPFREQ::Div512,
            7 => TAMPFREQ::Div256,
            _ => unreachable!(),
        }
    }
    ///RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div32768(&self) -> bool {
        *self == TAMPFREQ::Div32768
    }
    ///RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div16384(&self) -> bool {
        *self == TAMPFREQ::Div16384
    }
    ///RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div8192(&self) -> bool {
        *self == TAMPFREQ::Div8192
    }
    ///RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div4096(&self) -> bool {
        *self == TAMPFREQ::Div4096
    }
    ///RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div2048(&self) -> bool {
        *self == TAMPFREQ::Div2048
    }
    ///RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == TAMPFREQ::Div1024
    }
    ///RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == TAMPFREQ::Div512
    }
    ///RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == TAMPFREQ::Div256
    }
}
///Field `TAMPFREQ` writer - Tamper sampling frequency
pub type TAMPFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TAMPFREQ, crate::Safe>;
impl<'a, REG> TAMPFREQ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div32768(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div32768)
    }
    ///RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div16384(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div16384)
    }
    ///RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div8192(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div8192)
    }
    ///RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div4096(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div4096)
    }
    ///RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div2048(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div2048)
    }
    ///RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div1024)
    }
    ///RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div512)
    }
    ///RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFREQ::Div256)
    }
}
/**Tamper filter count

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPFLT {
    ///0: Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)
    Immediate = 0,
    ///1: Tamper event is activated after 2 consecutive samples at the active level
    Samples2 = 1,
    ///2: Tamper event is activated after 4 consecutive samples at the active level
    Samples4 = 2,
    ///3: Tamper event is activated after 8 consecutive samples at the active level
    Samples8 = 3,
}
impl From<TAMPFLT> for u8 {
    #[inline(always)]
    fn from(variant: TAMPFLT) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPFLT {
    type Ux = u8;
}
impl crate::IsEnum for TAMPFLT {}
///Field `TAMPFLT` reader - Tamper filter count
pub type TAMPFLT_R = crate::FieldReader<TAMPFLT>;
impl TAMPFLT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPFLT {
        match self.bits {
            0 => TAMPFLT::Immediate,
            1 => TAMPFLT::Samples2,
            2 => TAMPFLT::Samples4,
            3 => TAMPFLT::Samples8,
            _ => unreachable!(),
        }
    }
    ///Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)
    #[inline(always)]
    pub fn is_immediate(&self) -> bool {
        *self == TAMPFLT::Immediate
    }
    ///Tamper event is activated after 2 consecutive samples at the active level
    #[inline(always)]
    pub fn is_samples2(&self) -> bool {
        *self == TAMPFLT::Samples2
    }
    ///Tamper event is activated after 4 consecutive samples at the active level
    #[inline(always)]
    pub fn is_samples4(&self) -> bool {
        *self == TAMPFLT::Samples4
    }
    ///Tamper event is activated after 8 consecutive samples at the active level
    #[inline(always)]
    pub fn is_samples8(&self) -> bool {
        *self == TAMPFLT::Samples8
    }
}
///Field `TAMPFLT` writer - Tamper filter count
pub type TAMPFLT_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TAMPFLT, crate::Safe>;
impl<'a, REG> TAMPFLT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Tamper event is activated on edge of RTC_TAMPx input transitions to the active level (no internal pull-up on RTC_TAMPx input)
    #[inline(always)]
    pub fn immediate(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Immediate)
    }
    ///Tamper event is activated after 2 consecutive samples at the active level
    #[inline(always)]
    pub fn samples2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Samples2)
    }
    ///Tamper event is activated after 4 consecutive samples at the active level
    #[inline(always)]
    pub fn samples4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Samples4)
    }
    ///Tamper event is activated after 8 consecutive samples at the active level
    #[inline(always)]
    pub fn samples8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPFLT::Samples8)
    }
}
/**Tamper precharge duration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TAMPPRCH {
    ///0: 1 RTCCLK cycle
    Cycles1 = 0,
    ///1: 2 RTCCLK cycles
    Cycles2 = 1,
    ///2: 4 RTCCLK cycles
    Cycles4 = 2,
    ///3: 8 RTCCLK cycles
    Cycles8 = 3,
}
impl From<TAMPPRCH> for u8 {
    #[inline(always)]
    fn from(variant: TAMPPRCH) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TAMPPRCH {
    type Ux = u8;
}
impl crate::IsEnum for TAMPPRCH {}
///Field `TAMPPRCH` reader - Tamper precharge duration
pub type TAMPPRCH_R = crate::FieldReader<TAMPPRCH>;
impl TAMPPRCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPRCH {
        match self.bits {
            0 => TAMPPRCH::Cycles1,
            1 => TAMPPRCH::Cycles2,
            2 => TAMPPRCH::Cycles4,
            3 => TAMPPRCH::Cycles8,
            _ => unreachable!(),
        }
    }
    ///1 RTCCLK cycle
    #[inline(always)]
    pub fn is_cycles1(&self) -> bool {
        *self == TAMPPRCH::Cycles1
    }
    ///2 RTCCLK cycles
    #[inline(always)]
    pub fn is_cycles2(&self) -> bool {
        *self == TAMPPRCH::Cycles2
    }
    ///4 RTCCLK cycles
    #[inline(always)]
    pub fn is_cycles4(&self) -> bool {
        *self == TAMPPRCH::Cycles4
    }
    ///8 RTCCLK cycles
    #[inline(always)]
    pub fn is_cycles8(&self) -> bool {
        *self == TAMPPRCH::Cycles8
    }
}
///Field `TAMPPRCH` writer - Tamper precharge duration
pub type TAMPPRCH_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TAMPPRCH, crate::Safe>;
impl<'a, REG> TAMPPRCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///1 RTCCLK cycle
    #[inline(always)]
    pub fn cycles1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles1)
    }
    ///2 RTCCLK cycles
    #[inline(always)]
    pub fn cycles2(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles2)
    }
    ///4 RTCCLK cycles
    #[inline(always)]
    pub fn cycles4(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles4)
    }
    ///8 RTCCLK cycles
    #[inline(always)]
    pub fn cycles8(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPRCH::Cycles8)
    }
}
/**TAMPER pull-up disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPPUDIS {
    ///0: Precharge RTC_TAMPx pins before sampling (enable internal pull-up)
    Enabled = 0,
    ///1: Disable precharge of RTC_TAMPx pins
    Disabled = 1,
}
impl From<TAMPPUDIS> for bool {
    #[inline(always)]
    fn from(variant: TAMPPUDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPPUDIS` reader - TAMPER pull-up disable
pub type TAMPPUDIS_R = crate::BitReader<TAMPPUDIS>;
impl TAMPPUDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPPUDIS {
        match self.bits {
            false => TAMPPUDIS::Enabled,
            true => TAMPPUDIS::Disabled,
        }
    }
    ///Precharge RTC_TAMPx pins before sampling (enable internal pull-up)
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TAMPPUDIS::Enabled
    }
    ///Disable precharge of RTC_TAMPx pins
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TAMPPUDIS::Disabled
    }
}
///Field `TAMPPUDIS` writer - TAMPER pull-up disable
pub type TAMPPUDIS_W<'a, REG> = crate::BitWriter<'a, REG, TAMPPUDIS>;
impl<'a, REG> TAMPPUDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Precharge RTC_TAMPx pins before sampling (enable internal pull-up)
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS::Enabled)
    }
    ///Disable precharge of RTC_TAMPx pins
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPPUDIS::Disabled)
    }
}
/**PC13 value

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PC13VALUE {
    ///0: If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic low
    Low = 0,
    ///1: If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic high
    High = 1,
}
impl From<PC13VALUE> for bool {
    #[inline(always)]
    fn from(variant: PC13VALUE) -> Self {
        variant as u8 != 0
    }
}
///Field `PC13VALUE` reader - PC13 value
pub type PC13VALUE_R = crate::BitReader<PC13VALUE>;
impl PC13VALUE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PC13VALUE {
        match self.bits {
            false => PC13VALUE::Low,
            true => PC13VALUE::High,
        }
    }
    ///If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PC13VALUE::Low
    }
    ///If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic high
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PC13VALUE::High
    }
}
///Field `PC13VALUE` writer - PC13 value
pub type PC13VALUE_W<'a, REG> = crate::BitWriter<'a, REG, PC13VALUE>;
impl<'a, REG> PC13VALUE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(PC13VALUE::Low)
    }
    ///If the LSE is disabled and PCxMODE = 1, set PCxVALUE to logic high
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(PC13VALUE::High)
    }
}
/**PC13 mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PC13MODE {
    ///0: PCx is controlled by the GPIO configuration Register. Consequently PC15 is floating in Standby mode
    Floating = 0,
    ///1: PCx is forced to push-pull output if LSE is disabled
    PushPull = 1,
}
impl From<PC13MODE> for bool {
    #[inline(always)]
    fn from(variant: PC13MODE) -> Self {
        variant as u8 != 0
    }
}
///Field `PC13MODE` reader - PC13 mode
pub type PC13MODE_R = crate::BitReader<PC13MODE>;
impl PC13MODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PC13MODE {
        match self.bits {
            false => PC13MODE::Floating,
            true => PC13MODE::PushPull,
        }
    }
    ///PCx is controlled by the GPIO configuration Register. Consequently PC15 is floating in Standby mode
    #[inline(always)]
    pub fn is_floating(&self) -> bool {
        *self == PC13MODE::Floating
    }
    ///PCx is forced to push-pull output if LSE is disabled
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == PC13MODE::PushPull
    }
}
///Field `PC13MODE` writer - PC13 mode
pub type PC13MODE_W<'a, REG> = crate::BitWriter<'a, REG, PC13MODE>;
impl<'a, REG> PC13MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PCx is controlled by the GPIO configuration Register. Consequently PC15 is floating in Standby mode
    #[inline(always)]
    pub fn floating(self) -> &'a mut crate::W<REG> {
        self.variant(PC13MODE::Floating)
    }
    ///PCx is forced to push-pull output if LSE is disabled
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(PC13MODE::PushPull)
    }
}
///Field `PC14MODE` reader - PC 14 mode
pub use PC13MODE_R as PC14MODE_R;
///Field `PC15MODE` reader - PC15 mode
pub use PC13MODE_R as PC15MODE_R;
///Field `PC14MODE` writer - PC 14 mode
pub use PC13MODE_W as PC14MODE_W;
///Field `PC15MODE` writer - PC15 mode
pub use PC13MODE_W as PC15MODE_W;
///Field `PC14VALUE` reader - PC14 value
pub use PC13VALUE_R as PC14VALUE_R;
///Field `PC15VALUE` reader - PC15 value
pub use PC13VALUE_R as PC15VALUE_R;
///Field `PC14VALUE` writer - PC14 value
pub use PC13VALUE_W as PC14VALUE_W;
///Field `PC15VALUE` writer - PC15 value
pub use PC13VALUE_W as PC15VALUE_W;
impl R {
    ///Bit 0 - Tamper 1 detection enable
    #[inline(always)]
    pub fn tamp1e(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Active level for tamper 1
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&self) -> TAMPIE_R {
        TAMPIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Tamper 2 detection enable
    #[inline(always)]
    pub fn tamp2e(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Active level for tamper 2
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Tamper 3 detection enable
    #[inline(always)]
    pub fn tamp3e(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Active level for tamper 3
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&self) -> TAMPTS_R {
        TAMPTS_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&self) -> TAMPFREQ_R {
        TAMPFREQ_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bits 11:12 - Tamper filter count
    #[inline(always)]
    pub fn tampflt(&self) -> TAMPFLT_R {
        TAMPFLT_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - Tamper precharge duration
    #[inline(always)]
    pub fn tampprch(&self) -> TAMPPRCH_R {
        TAMPPRCH_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - TAMPER pull-up disable
    #[inline(always)]
    pub fn tamppudis(&self) -> TAMPPUDIS_R {
        TAMPPUDIS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - PC13 value
    #[inline(always)]
    pub fn pc13value(&self) -> PC13VALUE_R {
        PC13VALUE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - PC13 mode
    #[inline(always)]
    pub fn pc13mode(&self) -> PC13MODE_R {
        PC13MODE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - PC14 value
    #[inline(always)]
    pub fn pc14value(&self) -> PC14VALUE_R {
        PC14VALUE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - PC 14 mode
    #[inline(always)]
    pub fn pc14mode(&self) -> PC14MODE_R {
        PC14MODE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - PC15 value
    #[inline(always)]
    pub fn pc15value(&self) -> PC15VALUE_R {
        PC15VALUE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PC15 mode
    #[inline(always)]
    pub fn pc15mode(&self) -> PC15MODE_R {
        PC15MODE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TAFCR")
            .field("tamp1e", &self.tamp1e())
            .field("tamp1trg", &self.tamp1trg())
            .field("tampie", &self.tampie())
            .field("tamp2e", &self.tamp2e())
            .field("tamp2trg", &self.tamp2trg())
            .field("tamp3e", &self.tamp3e())
            .field("tamp3trg", &self.tamp3trg())
            .field("tampts", &self.tampts())
            .field("tampfreq", &self.tampfreq())
            .field("tampflt", &self.tampflt())
            .field("tampprch", &self.tampprch())
            .field("tamppudis", &self.tamppudis())
            .field("pc13value", &self.pc13value())
            .field("pc13mode", &self.pc13mode())
            .field("pc14value", &self.pc14value())
            .field("pc14mode", &self.pc14mode())
            .field("pc15value", &self.pc15value())
            .field("pc15mode", &self.pc15mode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper 1 detection enable
    #[inline(always)]
    pub fn tamp1e(&mut self) -> TAMP1E_W<'_, TAFCRrs> {
        TAMP1E_W::new(self, 0)
    }
    ///Bit 1 - Active level for tamper 1
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<'_, TAFCRrs> {
        TAMP1TRG_W::new(self, 1)
    }
    ///Bit 2 - Tamper interrupt enable
    #[inline(always)]
    pub fn tampie(&mut self) -> TAMPIE_W<'_, TAFCRrs> {
        TAMPIE_W::new(self, 2)
    }
    ///Bit 3 - Tamper 2 detection enable
    #[inline(always)]
    pub fn tamp2e(&mut self) -> TAMP2E_W<'_, TAFCRrs> {
        TAMP2E_W::new(self, 3)
    }
    ///Bit 4 - Active level for tamper 2
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<'_, TAFCRrs> {
        TAMP2TRG_W::new(self, 4)
    }
    ///Bit 5 - Tamper 3 detection enable
    #[inline(always)]
    pub fn tamp3e(&mut self) -> TAMP3E_W<'_, TAFCRrs> {
        TAMP3E_W::new(self, 5)
    }
    ///Bit 6 - Active level for tamper 3
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<'_, TAFCRrs> {
        TAMP3TRG_W::new(self, 6)
    }
    ///Bit 7 - Activate timestamp on tamper detection event
    #[inline(always)]
    pub fn tampts(&mut self) -> TAMPTS_W<'_, TAFCRrs> {
        TAMPTS_W::new(self, 7)
    }
    ///Bits 8:10 - Tamper sampling frequency
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TAMPFREQ_W<'_, TAFCRrs> {
        TAMPFREQ_W::new(self, 8)
    }
    ///Bits 11:12 - Tamper filter count
    #[inline(always)]
    pub fn tampflt(&mut self) -> TAMPFLT_W<'_, TAFCRrs> {
        TAMPFLT_W::new(self, 11)
    }
    ///Bits 13:14 - Tamper precharge duration
    #[inline(always)]
    pub fn tampprch(&mut self) -> TAMPPRCH_W<'_, TAFCRrs> {
        TAMPPRCH_W::new(self, 13)
    }
    ///Bit 15 - TAMPER pull-up disable
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TAMPPUDIS_W<'_, TAFCRrs> {
        TAMPPUDIS_W::new(self, 15)
    }
    ///Bit 18 - PC13 value
    #[inline(always)]
    pub fn pc13value(&mut self) -> PC13VALUE_W<'_, TAFCRrs> {
        PC13VALUE_W::new(self, 18)
    }
    ///Bit 19 - PC13 mode
    #[inline(always)]
    pub fn pc13mode(&mut self) -> PC13MODE_W<'_, TAFCRrs> {
        PC13MODE_W::new(self, 19)
    }
    ///Bit 20 - PC14 value
    #[inline(always)]
    pub fn pc14value(&mut self) -> PC14VALUE_W<'_, TAFCRrs> {
        PC14VALUE_W::new(self, 20)
    }
    ///Bit 21 - PC 14 mode
    #[inline(always)]
    pub fn pc14mode(&mut self) -> PC14MODE_W<'_, TAFCRrs> {
        PC14MODE_W::new(self, 21)
    }
    ///Bit 22 - PC15 value
    #[inline(always)]
    pub fn pc15value(&mut self) -> PC15VALUE_W<'_, TAFCRrs> {
        PC15VALUE_W::new(self, 22)
    }
    ///Bit 23 - PC15 mode
    #[inline(always)]
    pub fn pc15mode(&mut self) -> PC15MODE_W<'_, TAFCRrs> {
        PC15MODE_W::new(self, 23)
    }
}
/**tamper and alternate function configuration register

You can [`read`](crate::Reg::read) this register and get [`tafcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tafcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#RTC:TAFCR)*/
pub struct TAFCRrs;
impl crate::RegisterSpec for TAFCRrs {
    type Ux = u32;
}
///`read()` method returns [`tafcr::R`](R) reader structure
impl crate::Readable for TAFCRrs {}
///`write(|w| ..)` method takes [`tafcr::W`](W) writer structure
impl crate::Writable for TAFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TAFCR to value 0
impl crate::Resettable for TAFCRrs {}
