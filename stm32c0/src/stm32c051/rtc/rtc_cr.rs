///Register `RTC_CR` reader
pub type R = crate::R<RTC_CRrs>;
///Register `RTC_CR` writer
pub type W = crate::W<RTC_CRrs>;
/**Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEDGE {
    ///0: RTC_TS input rising edge generates a timestamp event
    B0x0 = 0,
    ///1: RTC_TS input falling edge generates a timestamp event
    B0x1 = 1,
}
impl From<TSEDGE> for bool {
    #[inline(always)]
    fn from(variant: TSEDGE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSEDGE` reader - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
pub type TSEDGE_R = crate::BitReader<TSEDGE>;
impl TSEDGE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSEDGE {
        match self.bits {
            false => TSEDGE::B0x0,
            true => TSEDGE::B0x1,
        }
    }
    ///RTC_TS input rising edge generates a timestamp event
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSEDGE::B0x0
    }
    ///RTC_TS input falling edge generates a timestamp event
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSEDGE::B0x1
    }
}
///Field `TSEDGE` writer - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
pub type TSEDGE_W<'a, REG> = crate::BitWriter<'a, REG, TSEDGE>;
impl<'a, REG> TSEDGE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC_TS input rising edge generates a timestamp event
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE::B0x0)
    }
    ///RTC_TS input falling edge generates a timestamp event
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSEDGE::B0x1)
    }
}
/**RTC_REFIN reference clock detection enable (50 or 60 Hz) Note: PREDIV_S must be 0x00FF.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REFCKON {
    ///0: RTC_REFIN detection disabled
    B0x0 = 0,
    ///1: RTC_REFIN detection enabled
    B0x1 = 1,
}
impl From<REFCKON> for bool {
    #[inline(always)]
    fn from(variant: REFCKON) -> Self {
        variant as u8 != 0
    }
}
///Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60 Hz) Note: PREDIV_S must be 0x00FF.
pub type REFCKON_R = crate::BitReader<REFCKON>;
impl REFCKON_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> REFCKON {
        match self.bits {
            false => REFCKON::B0x0,
            true => REFCKON::B0x1,
        }
    }
    ///RTC_REFIN detection disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == REFCKON::B0x0
    }
    ///RTC_REFIN detection enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == REFCKON::B0x1
    }
}
///Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60 Hz) Note: PREDIV_S must be 0x00FF.
pub type REFCKON_W<'a, REG> = crate::BitWriter<'a, REG, REFCKON>;
impl<'a, REG> REFCKON_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RTC_REFIN detection disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON::B0x0)
    }
    ///RTC_REFIN detection enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(REFCKON::B0x1)
    }
}
/**Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BYPSHAD {
    ///0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles.
    B0x0 = 0,
    ///1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
    B0x1 = 1,
}
impl From<BYPSHAD> for bool {
    #[inline(always)]
    fn from(variant: BYPSHAD) -> Self {
        variant as u8 != 0
    }
}
///Field `BYPSHAD` reader - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
pub type BYPSHAD_R = crate::BitReader<BYPSHAD>;
impl BYPSHAD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BYPSHAD {
        match self.bits {
            false => BYPSHAD::B0x0,
            true => BYPSHAD::B0x1,
        }
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BYPSHAD::B0x0
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BYPSHAD::B0x1
    }
}
///Field `BYPSHAD` writer - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
pub type BYPSHAD_W<'a, REG> = crate::BitWriter<'a, REG, BYPSHAD>;
impl<'a, REG> BYPSHAD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from the shadow registers, which are updated once every two RTCCLK cycles.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSHAD::B0x0)
    }
    ///Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken directly from the calendar counters.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BYPSHAD::B0x1)
    }
}
/**Hour format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FMT {
    ///0: 24 hour/day format
    B0x0 = 0,
    ///1: AM/PM hour format
    B0x1 = 1,
}
impl From<FMT> for bool {
    #[inline(always)]
    fn from(variant: FMT) -> Self {
        variant as u8 != 0
    }
}
///Field `FMT` reader - Hour format
pub type FMT_R = crate::BitReader<FMT>;
impl FMT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FMT {
        match self.bits {
            false => FMT::B0x0,
            true => FMT::B0x1,
        }
    }
    ///24 hour/day format
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FMT::B0x0
    }
    ///AM/PM hour format
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FMT::B0x1
    }
}
///Field `FMT` writer - Hour format
pub type FMT_W<'a, REG> = crate::BitWriter<'a, REG, FMT>;
impl<'a, REG> FMT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///24 hour/day format
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FMT::B0x0)
    }
    ///AM/PM hour format
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FMT::B0x1)
    }
}
/**Alarm A enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAE {
    ///0: Alarm A disabled
    B0x0 = 0,
    ///1: Alarm A enabled
    B0x1 = 1,
}
impl From<ALRAE> for bool {
    #[inline(always)]
    fn from(variant: ALRAE) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAE` reader - Alarm A enable
pub type ALRAE_R = crate::BitReader<ALRAE>;
impl ALRAE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRAE {
        match self.bits {
            false => ALRAE::B0x0,
            true => ALRAE::B0x1,
        }
    }
    ///Alarm A disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALRAE::B0x0
    }
    ///Alarm A enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALRAE::B0x1
    }
}
///Field `ALRAE` writer - Alarm A enable
pub type ALRAE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAE>;
impl<'a, REG> ALRAE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm A disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE::B0x0)
    }
    ///Alarm A enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAE::B0x1)
    }
}
/**timestamp enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSE {
    ///0: timestamp disable
    B0x0 = 0,
    ///1: timestamp enable
    B0x1 = 1,
}
impl From<TSE> for bool {
    #[inline(always)]
    fn from(variant: TSE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSE` reader - timestamp enable
pub type TSE_R = crate::BitReader<TSE>;
impl TSE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSE {
        match self.bits {
            false => TSE::B0x0,
            true => TSE::B0x1,
        }
    }
    ///timestamp disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSE::B0x0
    }
    ///timestamp enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSE::B0x1
    }
}
///Field `TSE` writer - timestamp enable
pub type TSE_W<'a, REG> = crate::BitWriter<'a, REG, TSE>;
impl<'a, REG> TSE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///timestamp disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSE::B0x0)
    }
    ///timestamp enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSE::B0x1)
    }
}
/**Alarm A interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALRAIE {
    ///0: Alarm A interrupt disabled
    B0x0 = 0,
    ///1: Alarm A interrupt enabled
    B0x1 = 1,
}
impl From<ALRAIE> for bool {
    #[inline(always)]
    fn from(variant: ALRAIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ALRAIE` reader - Alarm A interrupt enable
pub type ALRAIE_R = crate::BitReader<ALRAIE>;
impl ALRAIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALRAIE {
        match self.bits {
            false => ALRAIE::B0x0,
            true => ALRAIE::B0x1,
        }
    }
    ///Alarm A interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ALRAIE::B0x0
    }
    ///Alarm A interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ALRAIE::B0x1
    }
}
///Field `ALRAIE` writer - Alarm A interrupt enable
pub type ALRAIE_W<'a, REG> = crate::BitWriter<'a, REG, ALRAIE>;
impl<'a, REG> ALRAIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Alarm A interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE::B0x0)
    }
    ///Alarm A interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ALRAIE::B0x1)
    }
}
/**Timestamp interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSIE {
    ///0: Timestamp interrupt disable
    B0x0 = 0,
    ///1: Timestamp interrupt enable
    B0x1 = 1,
}
impl From<TSIE> for bool {
    #[inline(always)]
    fn from(variant: TSIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSIE` reader - Timestamp interrupt enable
pub type TSIE_R = crate::BitReader<TSIE>;
impl TSIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSIE {
        match self.bits {
            false => TSIE::B0x0,
            true => TSIE::B0x1,
        }
    }
    ///Timestamp interrupt disable
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSIE::B0x0
    }
    ///Timestamp interrupt enable
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSIE::B0x1
    }
}
///Field `TSIE` writer - Timestamp interrupt enable
pub type TSIE_W<'a, REG> = crate::BitWriter<'a, REG, TSIE>;
impl<'a, REG> TSIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timestamp interrupt disable
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE::B0x0)
    }
    ///Timestamp interrupt enable
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSIE::B0x1)
    }
}
/**Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1H {
    ///0: No effect
    B0x0 = 0,
    ///1: Adds 1 hour to the current time. This can be used for summer time change
    B0x1 = 1,
}
impl From<ADD1H> for bool {
    #[inline(always)]
    fn from(variant: ADD1H) -> Self {
        variant as u8 != 0
    }
}
///Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.
pub type ADD1H_W<'a, REG> = crate::BitWriter<'a, REG, ADD1H>;
impl<'a, REG> ADD1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1H::B0x0)
    }
    ///Adds 1 hour to the current time. This can be used for summer time change
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1H::B0x1)
    }
}
/**Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SUB1H {
    ///0: No effect
    B0x0 = 0,
    ///1: Subtracts 1 hour to the current time. This can be used for winter time change.
    B0x1 = 1,
}
impl From<SUB1H> for bool {
    #[inline(always)]
    fn from(variant: SUB1H) -> Self {
        variant as u8 != 0
    }
}
///Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.
pub type SUB1H_W<'a, REG> = crate::BitWriter<'a, REG, SUB1H>;
impl<'a, REG> SUB1H_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No effect
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SUB1H::B0x0)
    }
    ///Subtracts 1 hour to the current time. This can be used for winter time change.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SUB1H::B0x1)
    }
}
///Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
pub type BKP_R = crate::BitReader;
///Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
pub type BKP_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 24.3.14: Calibration clock output.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSEL {
    ///0: Calibration output is 512 Hz
    B0x0 = 0,
    ///1: Calibration output is 1 Hz
    B0x1 = 1,
}
impl From<COSEL> for bool {
    #[inline(always)]
    fn from(variant: COSEL) -> Self {
        variant as u8 != 0
    }
}
///Field `COSEL` reader - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 24.3.14: Calibration clock output.
pub type COSEL_R = crate::BitReader<COSEL>;
impl COSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COSEL {
        match self.bits {
            false => COSEL::B0x0,
            true => COSEL::B0x1,
        }
    }
    ///Calibration output is 512 Hz
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COSEL::B0x0
    }
    ///Calibration output is 1 Hz
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COSEL::B0x1
    }
}
///Field `COSEL` writer - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 24.3.14: Calibration clock output.
pub type COSEL_W<'a, REG> = crate::BitWriter<'a, REG, COSEL>;
impl<'a, REG> COSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration output is 512 Hz
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COSEL::B0x0)
    }
    ///Calibration output is 1 Hz
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COSEL::B0x1)
    }
}
/**Output polarity This bit is used to configure the polarity of TAMPALRM output.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POL {
    ///0: The pin is high when ALRAF is asserted (depending on OSEL\[1:0\]).
    B0x0 = 0,
    ///1: The pin is low when ALRAF is asserted (depending on OSEL\[1:0\]).
    B0x1 = 1,
}
impl From<POL> for bool {
    #[inline(always)]
    fn from(variant: POL) -> Self {
        variant as u8 != 0
    }
}
///Field `POL` reader - Output polarity This bit is used to configure the polarity of TAMPALRM output.
pub type POL_R = crate::BitReader<POL>;
impl POL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> POL {
        match self.bits {
            false => POL::B0x0,
            true => POL::B0x1,
        }
    }
    ///The pin is high when ALRAF is asserted (depending on OSEL\[1:0\]).
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == POL::B0x0
    }
    ///The pin is low when ALRAF is asserted (depending on OSEL\[1:0\]).
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == POL::B0x1
    }
}
///Field `POL` writer - Output polarity This bit is used to configure the polarity of TAMPALRM output.
pub type POL_W<'a, REG> = crate::BitWriter<'a, REG, POL>;
impl<'a, REG> POL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The pin is high when ALRAF is asserted (depending on OSEL\[1:0\]).
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(POL::B0x0)
    }
    ///The pin is low when ALRAF is asserted (depending on OSEL\[1:0\]).
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(POL::B0x1)
    }
}
/**Output selection These bits are used to select the flag to be routed to TAMPALRM output.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OSEL {
    ///0: Output disabled
    B0x0 = 0,
    ///1: Alarm A output enabled
    B0x1 = 1,
}
impl From<OSEL> for u8 {
    #[inline(always)]
    fn from(variant: OSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for OSEL {
    type Ux = u8;
}
impl crate::IsEnum for OSEL {}
///Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to TAMPALRM output.
pub type OSEL_R = crate::FieldReader<OSEL>;
impl OSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<OSEL> {
        match self.bits {
            0 => Some(OSEL::B0x0),
            1 => Some(OSEL::B0x1),
            _ => None,
        }
    }
    ///Output disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == OSEL::B0x0
    }
    ///Alarm A output enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == OSEL::B0x1
    }
}
///Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to TAMPALRM output.
pub type OSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, OSEL>;
impl<'a, REG> OSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Output disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::B0x0)
    }
    ///Alarm A output enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(OSEL::B0x1)
    }
}
/**Calibration output enable This bit enables the CALIB output

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COE {
    ///0: Calibration output disabled
    B0x0 = 0,
    ///1: Calibration output enabled
    B0x1 = 1,
}
impl From<COE> for bool {
    #[inline(always)]
    fn from(variant: COE) -> Self {
        variant as u8 != 0
    }
}
///Field `COE` reader - Calibration output enable This bit enables the CALIB output
pub type COE_R = crate::BitReader<COE>;
impl COE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COE {
        match self.bits {
            false => COE::B0x0,
            true => COE::B0x1,
        }
    }
    ///Calibration output disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COE::B0x0
    }
    ///Calibration output enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COE::B0x1
    }
}
///Field `COE` writer - Calibration output enable This bit enables the CALIB output
pub type COE_W<'a, REG> = crate::BitWriter<'a, REG, COE>;
impl<'a, REG> COE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Calibration output disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COE::B0x0)
    }
    ///Calibration output enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COE::B0x1)
    }
}
/**TAMPALRM pull-up enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPALRM_PU {
    ///0: No pull-up is applied on TAMPALRM output
    B0x0 = 0,
    ///1: A pull-up is applied on TAMPALRM output
    B0x1 = 1,
}
impl From<TAMPALRM_PU> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_PU) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPALRM_PU` reader - TAMPALRM pull-up enable
pub type TAMPALRM_PU_R = crate::BitReader<TAMPALRM_PU>;
impl TAMPALRM_PU_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPALRM_PU {
        match self.bits {
            false => TAMPALRM_PU::B0x0,
            true => TAMPALRM_PU::B0x1,
        }
    }
    ///No pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMPALRM_PU::B0x0
    }
    ///A pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMPALRM_PU::B0x1
    }
}
///Field `TAMPALRM_PU` writer - TAMPALRM pull-up enable
pub type TAMPALRM_PU_W<'a, REG> = crate::BitWriter<'a, REG, TAMPALRM_PU>;
impl<'a, REG> TAMPALRM_PU_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_PU::B0x0)
    }
    ///A pull-up is applied on TAMPALRM output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_PU::B0x1)
    }
}
/**TAMPALRM output type

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMPALRM_TYPE {
    ///0: TAMPALRM is push-pull output
    B0x0 = 0,
    ///1: TAMPALRM is open-drain output
    B0x1 = 1,
}
impl From<TAMPALRM_TYPE> for bool {
    #[inline(always)]
    fn from(variant: TAMPALRM_TYPE) -> Self {
        variant as u8 != 0
    }
}
///Field `TAMPALRM_TYPE` reader - TAMPALRM output type
pub type TAMPALRM_TYPE_R = crate::BitReader<TAMPALRM_TYPE>;
impl TAMPALRM_TYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TAMPALRM_TYPE {
        match self.bits {
            false => TAMPALRM_TYPE::B0x0,
            true => TAMPALRM_TYPE::B0x1,
        }
    }
    ///TAMPALRM is push-pull output
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TAMPALRM_TYPE::B0x0
    }
    ///TAMPALRM is open-drain output
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TAMPALRM_TYPE::B0x1
    }
}
///Field `TAMPALRM_TYPE` writer - TAMPALRM output type
pub type TAMPALRM_TYPE_W<'a, REG> = crate::BitWriter<'a, REG, TAMPALRM_TYPE>;
impl<'a, REG> TAMPALRM_TYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TAMPALRM is push-pull output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_TYPE::B0x0)
    }
    ///TAMPALRM is open-drain output
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMPALRM_TYPE::B0x1)
    }
}
///Field `OUT2EN` reader - RTC_OUT2 output enable
pub type OUT2EN_R = crate::BitReader;
///Field `OUT2EN` writer - RTC_OUT2 output enable
pub type OUT2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
    #[inline(always)]
    pub fn tsedge(&self) -> TSEDGE_R {
        TSEDGE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz) Note: PREDIV_S must be 0x00FF.
    #[inline(always)]
    pub fn refckon(&self) -> REFCKON_R {
        REFCKON_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
    #[inline(always)]
    pub fn bypshad(&self) -> BYPSHAD_R {
        BYPSHAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&self) -> FMT_R {
        FMT_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&self) -> ALRAE_R {
        ALRAE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&self) -> TSE_R {
        TSE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&self) -> ALRAIE_R {
        ALRAIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 15 - Timestamp interrupt enable
    #[inline(always)]
    pub fn tsie(&self) -> TSIE_R {
        TSIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
    #[inline(always)]
    pub fn bkp(&self) -> BKP_R {
        BKP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 24.3.14: Calibration clock output.
    #[inline(always)]
    pub fn cosel(&self) -> COSEL_R {
        COSEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output.
    #[inline(always)]
    pub fn pol(&self) -> POL_R {
        POL_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output.
    #[inline(always)]
    pub fn osel(&self) -> OSEL_R {
        OSEL_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bit 23 - Calibration output enable This bit enables the CALIB output
    #[inline(always)]
    pub fn coe(&self) -> COE_R {
        COE_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 29 - TAMPALRM pull-up enable
    #[inline(always)]
    pub fn tampalrm_pu(&self) -> TAMPALRM_PU_R {
        TAMPALRM_PU_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - TAMPALRM output type
    #[inline(always)]
    pub fn tampalrm_type(&self) -> TAMPALRM_TYPE_R {
        TAMPALRM_TYPE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - RTC_OUT2 output enable
    #[inline(always)]
    pub fn out2en(&self) -> OUT2EN_R {
        OUT2EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC_CR")
            .field("tsedge", &self.tsedge())
            .field("refckon", &self.refckon())
            .field("bypshad", &self.bypshad())
            .field("fmt", &self.fmt())
            .field("alrae", &self.alrae())
            .field("tse", &self.tse())
            .field("alraie", &self.alraie())
            .field("tsie", &self.tsie())
            .field("bkp", &self.bkp())
            .field("cosel", &self.cosel())
            .field("pol", &self.pol())
            .field("osel", &self.osel())
            .field("coe", &self.coe())
            .field("tampalrm_pu", &self.tampalrm_pu())
            .field("tampalrm_type", &self.tampalrm_type())
            .field("out2en", &self.out2en())
            .finish()
    }
}
impl W {
    ///Bit 3 - Timestamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
    #[inline(always)]
    pub fn tsedge(&mut self) -> TSEDGE_W<'_, RTC_CRrs> {
        TSEDGE_W::new(self, 3)
    }
    ///Bit 4 - RTC_REFIN reference clock detection enable (50 or 60 Hz) Note: PREDIV_S must be 0x00FF.
    #[inline(always)]
    pub fn refckon(&mut self) -> REFCKON_W<'_, RTC_CRrs> {
        REFCKON_W::new(self, 4)
    }
    ///Bit 5 - Bypass the shadow registers Note: If the frequency of the APB1 clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1.
    #[inline(always)]
    pub fn bypshad(&mut self) -> BYPSHAD_W<'_, RTC_CRrs> {
        BYPSHAD_W::new(self, 5)
    }
    ///Bit 6 - Hour format
    #[inline(always)]
    pub fn fmt(&mut self) -> FMT_W<'_, RTC_CRrs> {
        FMT_W::new(self, 6)
    }
    ///Bit 8 - Alarm A enable
    #[inline(always)]
    pub fn alrae(&mut self) -> ALRAE_W<'_, RTC_CRrs> {
        ALRAE_W::new(self, 8)
    }
    ///Bit 11 - timestamp enable
    #[inline(always)]
    pub fn tse(&mut self) -> TSE_W<'_, RTC_CRrs> {
        TSE_W::new(self, 11)
    }
    ///Bit 12 - Alarm A interrupt enable
    #[inline(always)]
    pub fn alraie(&mut self) -> ALRAIE_W<'_, RTC_CRrs> {
        ALRAIE_W::new(self, 12)
    }
    ///Bit 15 - Timestamp interrupt enable
    #[inline(always)]
    pub fn tsie(&mut self) -> TSIE_W<'_, RTC_CRrs> {
        TSIE_W::new(self, 15)
    }
    ///Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0.
    #[inline(always)]
    pub fn add1h(&mut self) -> ADD1H_W<'_, RTC_CRrs> {
        ADD1H_W::new(self, 16)
    }
    ///Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0.
    #[inline(always)]
    pub fn sub1h(&mut self) -> SUB1H_W<'_, RTC_CRrs> {
        SUB1H_W::new(self, 17)
    }
    ///Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not.
    #[inline(always)]
    pub fn bkp(&mut self) -> BKP_W<'_, RTC_CRrs> {
        BKP_W::new(self, 18)
    }
    ///Bit 19 - Calibration output selection When COE = 1, this bit selects which signal is output on CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 24.3.14: Calibration clock output.
    #[inline(always)]
    pub fn cosel(&mut self) -> COSEL_W<'_, RTC_CRrs> {
        COSEL_W::new(self, 19)
    }
    ///Bit 20 - Output polarity This bit is used to configure the polarity of TAMPALRM output.
    #[inline(always)]
    pub fn pol(&mut self) -> POL_W<'_, RTC_CRrs> {
        POL_W::new(self, 20)
    }
    ///Bits 21:22 - Output selection These bits are used to select the flag to be routed to TAMPALRM output.
    #[inline(always)]
    pub fn osel(&mut self) -> OSEL_W<'_, RTC_CRrs> {
        OSEL_W::new(self, 21)
    }
    ///Bit 23 - Calibration output enable This bit enables the CALIB output
    #[inline(always)]
    pub fn coe(&mut self) -> COE_W<'_, RTC_CRrs> {
        COE_W::new(self, 23)
    }
    ///Bit 29 - TAMPALRM pull-up enable
    #[inline(always)]
    pub fn tampalrm_pu(&mut self) -> TAMPALRM_PU_W<'_, RTC_CRrs> {
        TAMPALRM_PU_W::new(self, 29)
    }
    ///Bit 30 - TAMPALRM output type
    #[inline(always)]
    pub fn tampalrm_type(&mut self) -> TAMPALRM_TYPE_W<'_, RTC_CRrs> {
        TAMPALRM_TYPE_W::new(self, 30)
    }
    ///Bit 31 - RTC_OUT2 output enable
    #[inline(always)]
    pub fn out2en(&mut self) -> OUT2EN_W<'_, RTC_CRrs> {
        OUT2EN_W::new(self, 31)
    }
}
/**RTC control register

You can [`read`](crate::Reg::read) this register and get [`rtc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#RTC:RTC_CR)*/
pub struct RTC_CRrs;
impl crate::RegisterSpec for RTC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`rtc_cr::R`](R) reader structure
impl crate::Readable for RTC_CRrs {}
///`write(|w| ..)` method takes [`rtc_cr::W`](W) writer structure
impl crate::Writable for RTC_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RTC_CR to value 0
impl crate::Resettable for RTC_CRrs {}
