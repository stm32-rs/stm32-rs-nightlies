///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**Peripheral enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE {
    ///0: Peripheral disabled
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<PE> for bool {
    #[inline(always)]
    fn from(variant: PE) -> Self {
        variant as u8 != 0
    }
}
///Field `PE` reader - Peripheral enable
pub type PE_R = crate::BitReader<PE>;
impl PE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PE {
        match self.bits {
            false => PE::Disabled,
            true => PE::Enabled,
        }
    }
    ///Peripheral disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PE::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PE::Enabled
    }
}
///Field `PE` writer - Peripheral enable
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG, PE>;
impl<'a, REG> PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PE::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PE::Enabled)
    }
}
/**SMBus mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBUS {
    ///0: I2C Mode
    I2c = 0,
    ///1: SMBus
    Smbus = 1,
}
impl From<SMBUS> for bool {
    #[inline(always)]
    fn from(variant: SMBUS) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBUS` reader - SMBus mode
pub type SMBUS_R = crate::BitReader<SMBUS>;
impl SMBUS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMBUS {
        match self.bits {
            false => SMBUS::I2c,
            true => SMBUS::Smbus,
        }
    }
    ///I2C Mode
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == SMBUS::I2c
    }
    ///SMBus
    #[inline(always)]
    pub fn is_smbus(&self) -> bool {
        *self == SMBUS::Smbus
    }
}
///Field `SMBUS` writer - SMBus mode
pub type SMBUS_W<'a, REG> = crate::BitWriter<'a, REG, SMBUS>;
impl<'a, REG> SMBUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2C Mode
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(SMBUS::I2c)
    }
    ///SMBus
    #[inline(always)]
    pub fn smbus(self) -> &'a mut crate::W<REG> {
        self.variant(SMBUS::Smbus)
    }
}
/**SMBus type

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBTYPE {
    ///0: SMBus Device
    Device = 0,
    ///1: SMBus Host
    Host = 1,
}
impl From<SMBTYPE> for bool {
    #[inline(always)]
    fn from(variant: SMBTYPE) -> Self {
        variant as u8 != 0
    }
}
///Field `SMBTYPE` reader - SMBus type
pub type SMBTYPE_R = crate::BitReader<SMBTYPE>;
impl SMBTYPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SMBTYPE {
        match self.bits {
            false => SMBTYPE::Device,
            true => SMBTYPE::Host,
        }
    }
    ///SMBus Device
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == SMBTYPE::Device
    }
    ///SMBus Host
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == SMBTYPE::Host
    }
}
///Field `SMBTYPE` writer - SMBus type
pub type SMBTYPE_W<'a, REG> = crate::BitWriter<'a, REG, SMBTYPE>;
impl<'a, REG> SMBTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SMBus Device
    #[inline(always)]
    pub fn device(self) -> &'a mut crate::W<REG> {
        self.variant(SMBTYPE::Device)
    }
    ///SMBus Host
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(SMBTYPE::Host)
    }
}
/**ARP enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENARP {
    ///0: ARP disabled
    Disabled = 0,
    ///1: ARP enabled
    Enabled = 1,
}
impl From<ENARP> for bool {
    #[inline(always)]
    fn from(variant: ENARP) -> Self {
        variant as u8 != 0
    }
}
///Field `ENARP` reader - ARP enable
pub type ENARP_R = crate::BitReader<ENARP>;
impl ENARP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENARP {
        match self.bits {
            false => ENARP::Disabled,
            true => ENARP::Enabled,
        }
    }
    ///ARP disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENARP::Disabled
    }
    ///ARP enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENARP::Enabled
    }
}
///Field `ENARP` writer - ARP enable
pub type ENARP_W<'a, REG> = crate::BitWriter<'a, REG, ENARP>;
impl<'a, REG> ENARP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ARP disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENARP::Disabled)
    }
    ///ARP enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENARP::Enabled)
    }
}
/**PEC enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENPEC {
    ///0: PEC calculation disabled
    Disabled = 0,
    ///1: PEC calculation enabled
    Enabled = 1,
}
impl From<ENPEC> for bool {
    #[inline(always)]
    fn from(variant: ENPEC) -> Self {
        variant as u8 != 0
    }
}
///Field `ENPEC` reader - PEC enable
pub type ENPEC_R = crate::BitReader<ENPEC>;
impl ENPEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENPEC {
        match self.bits {
            false => ENPEC::Disabled,
            true => ENPEC::Enabled,
        }
    }
    ///PEC calculation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENPEC::Disabled
    }
    ///PEC calculation enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENPEC::Enabled
    }
}
///Field `ENPEC` writer - PEC enable
pub type ENPEC_W<'a, REG> = crate::BitWriter<'a, REG, ENPEC>;
impl<'a, REG> ENPEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///PEC calculation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENPEC::Disabled)
    }
    ///PEC calculation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENPEC::Enabled)
    }
}
/**General call enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENGC {
    ///0: General call disabled
    Disabled = 0,
    ///1: General call enabled
    Enabled = 1,
}
impl From<ENGC> for bool {
    #[inline(always)]
    fn from(variant: ENGC) -> Self {
        variant as u8 != 0
    }
}
///Field `ENGC` reader - General call enable
pub type ENGC_R = crate::BitReader<ENGC>;
impl ENGC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENGC {
        match self.bits {
            false => ENGC::Disabled,
            true => ENGC::Enabled,
        }
    }
    ///General call disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENGC::Disabled
    }
    ///General call enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENGC::Enabled
    }
}
///Field `ENGC` writer - General call enable
pub type ENGC_W<'a, REG> = crate::BitWriter<'a, REG, ENGC>;
impl<'a, REG> ENGC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///General call disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENGC::Disabled)
    }
    ///General call enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENGC::Enabled)
    }
}
/**Clock stretching disable (Slave mode)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOSTRETCH {
    ///0: Clock stretching enabled
    Enabled = 0,
    ///1: Clock stretching disabled
    Disabled = 1,
}
impl From<NOSTRETCH> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH) -> Self {
        variant as u8 != 0
    }
}
///Field `NOSTRETCH` reader - Clock stretching disable (Slave mode)
pub type NOSTRETCH_R = crate::BitReader<NOSTRETCH>;
impl NOSTRETCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NOSTRETCH {
        match self.bits {
            false => NOSTRETCH::Enabled,
            true => NOSTRETCH::Disabled,
        }
    }
    ///Clock stretching enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTRETCH::Enabled
    }
    ///Clock stretching disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTRETCH::Disabled
    }
}
///Field `NOSTRETCH` writer - Clock stretching disable (Slave mode)
pub type NOSTRETCH_W<'a, REG> = crate::BitWriter<'a, REG, NOSTRETCH>;
impl<'a, REG> NOSTRETCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clock stretching enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH::Enabled)
    }
    ///Clock stretching disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH::Disabled)
    }
}
/**Start generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START {
    ///0: No Start generation
    NoStart = 0,
    ///1: In master mode: repeated start generation, in slave mode: start generation when bus is free
    Start = 1,
}
impl From<START> for bool {
    #[inline(always)]
    fn from(variant: START) -> Self {
        variant as u8 != 0
    }
}
///Field `START` reader - Start generation
pub type START_R = crate::BitReader<START>;
impl START_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> START {
        match self.bits {
            false => START::NoStart,
            true => START::Start,
        }
    }
    ///No Start generation
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START::NoStart
    }
    ///In master mode: repeated start generation, in slave mode: start generation when bus is free
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START::Start
    }
}
///Field `START` writer - Start generation
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Start generation
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(START::NoStart)
    }
    ///In master mode: repeated start generation, in slave mode: start generation when bus is free
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START::Start)
    }
}
/**Stop generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP {
    ///0: No Stop generation
    NoStop = 0,
    ///1: In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte
    Stop = 1,
}
impl From<STOP> for bool {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as u8 != 0
    }
}
///Field `STOP` reader - Stop generation
pub type STOP_R = crate::BitReader<STOP>;
impl STOP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STOP {
        match self.bits {
            false => STOP::NoStop,
            true => STOP::Stop,
        }
    }
    ///No Stop generation
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOP::NoStop
    }
    ///In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOP::Stop
    }
}
///Field `STOP` writer - Stop generation
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG, STOP>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Stop generation
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::NoStop)
    }
    ///In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop)
    }
}
/**Acknowledge enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACK {
    ///0: No acknowledge returned
    Nak = 0,
    ///1: Acknowledge returned after a byte is received
    Ack = 1,
}
impl From<ACK> for bool {
    #[inline(always)]
    fn from(variant: ACK) -> Self {
        variant as u8 != 0
    }
}
///Field `ACK` reader - Acknowledge enable
pub type ACK_R = crate::BitReader<ACK>;
impl ACK_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ACK {
        match self.bits {
            false => ACK::Nak,
            true => ACK::Ack,
        }
    }
    ///No acknowledge returned
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == ACK::Nak
    }
    ///Acknowledge returned after a byte is received
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ACK::Ack
    }
}
///Field `ACK` writer - Acknowledge enable
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG, ACK>;
impl<'a, REG> ACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No acknowledge returned
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(ACK::Nak)
    }
    ///Acknowledge returned after a byte is received
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(ACK::Ack)
    }
}
/**Acknowledge/PEC Position (for data reception)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POS {
    ///0: ACK bit controls the (N)ACK of the current byte being received
    Current = 0,
    ///1: ACK bit controls the (N)ACK of the next byte to be received
    Next = 1,
}
impl From<POS> for bool {
    #[inline(always)]
    fn from(variant: POS) -> Self {
        variant as u8 != 0
    }
}
///Field `POS` reader - Acknowledge/PEC Position (for data reception)
pub type POS_R = crate::BitReader<POS>;
impl POS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> POS {
        match self.bits {
            false => POS::Current,
            true => POS::Next,
        }
    }
    ///ACK bit controls the (N)ACK of the current byte being received
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        *self == POS::Current
    }
    ///ACK bit controls the (N)ACK of the next byte to be received
    #[inline(always)]
    pub fn is_next(&self) -> bool {
        *self == POS::Next
    }
}
///Field `POS` writer - Acknowledge/PEC Position (for data reception)
pub type POS_W<'a, REG> = crate::BitWriter<'a, REG, POS>;
impl<'a, REG> POS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///ACK bit controls the (N)ACK of the current byte being received
    #[inline(always)]
    pub fn current(self) -> &'a mut crate::W<REG> {
        self.variant(POS::Current)
    }
    ///ACK bit controls the (N)ACK of the next byte to be received
    #[inline(always)]
    pub fn next(self) -> &'a mut crate::W<REG> {
        self.variant(POS::Next)
    }
}
/**Packet error checking

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEC {
    ///0: No PEC transfer
    Disabled = 0,
    ///1: PEC transfer
    Enabled = 1,
}
impl From<PEC> for bool {
    #[inline(always)]
    fn from(variant: PEC) -> Self {
        variant as u8 != 0
    }
}
///Field `PEC` reader - Packet error checking
pub type PEC_R = crate::BitReader<PEC>;
impl PEC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PEC {
        match self.bits {
            false => PEC::Disabled,
            true => PEC::Enabled,
        }
    }
    ///No PEC transfer
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEC::Disabled
    }
    ///PEC transfer
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEC::Enabled
    }
}
///Field `PEC` writer - Packet error checking
pub type PEC_W<'a, REG> = crate::BitWriter<'a, REG, PEC>;
impl<'a, REG> PEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No PEC transfer
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEC::Disabled)
    }
    ///PEC transfer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEC::Enabled)
    }
}
/**SMBus alert

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERT {
    ///0: SMBA pin released high
    Release = 0,
    ///1: SMBA pin driven low
    Drive = 1,
}
impl From<ALERT> for bool {
    #[inline(always)]
    fn from(variant: ALERT) -> Self {
        variant as u8 != 0
    }
}
///Field `ALERT` reader - SMBus alert
pub type ALERT_R = crate::BitReader<ALERT>;
impl ALERT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ALERT {
        match self.bits {
            false => ALERT::Release,
            true => ALERT::Drive,
        }
    }
    ///SMBA pin released high
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == ALERT::Release
    }
    ///SMBA pin driven low
    #[inline(always)]
    pub fn is_drive(&self) -> bool {
        *self == ALERT::Drive
    }
}
///Field `ALERT` writer - SMBus alert
pub type ALERT_W<'a, REG> = crate::BitWriter<'a, REG, ALERT>;
impl<'a, REG> ALERT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SMBA pin released high
    #[inline(always)]
    pub fn release(self) -> &'a mut crate::W<REG> {
        self.variant(ALERT::Release)
    }
    ///SMBA pin driven low
    #[inline(always)]
    pub fn drive(self) -> &'a mut crate::W<REG> {
        self.variant(ALERT::Drive)
    }
}
/**Software reset

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRST {
    ///0: I2C peripheral not under reset
    NotReset = 0,
    ///1: I2C peripheral under reset
    Reset = 1,
}
impl From<SWRST> for bool {
    #[inline(always)]
    fn from(variant: SWRST) -> Self {
        variant as u8 != 0
    }
}
///Field `SWRST` reader - Software reset
pub type SWRST_R = crate::BitReader<SWRST>;
impl SWRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SWRST {
        match self.bits {
            false => SWRST::NotReset,
            true => SWRST::Reset,
        }
    }
    ///I2C peripheral not under reset
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == SWRST::NotReset
    }
    ///I2C peripheral under reset
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRST::Reset
    }
}
///Field `SWRST` writer - Software reset
pub type SWRST_W<'a, REG> = crate::BitWriter<'a, REG, SWRST>;
impl<'a, REG> SWRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///I2C peripheral not under reset
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SWRST::NotReset)
    }
    ///I2C peripheral under reset
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SWRST::Reset)
    }
}
impl R {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - SMBus mode
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - SMBus type
    #[inline(always)]
    pub fn smbtype(&self) -> SMBTYPE_R {
        SMBTYPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ARP enable
    #[inline(always)]
    pub fn enarp(&self) -> ENARP_R {
        ENARP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - PEC enable
    #[inline(always)]
    pub fn enpec(&self) -> ENPEC_R {
        ENPEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - General call enable
    #[inline(always)]
    pub fn engc(&self) -> ENGC_R {
        ENGC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Clock stretching disable (Slave mode)
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Start generation
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Stop generation
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Acknowledge enable
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Acknowledge/PEC Position (for data reception)
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Packet error checking
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SMBus alert
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Software reset
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("swrst", &self.swrst())
            .field("alert", &self.alert())
            .field("pec", &self.pec())
            .field("pos", &self.pos())
            .field("ack", &self.ack())
            .field("stop", &self.stop())
            .field("start", &self.start())
            .field("nostretch", &self.nostretch())
            .field("engc", &self.engc())
            .field("enpec", &self.enpec())
            .field("enarp", &self.enarp())
            .field("smbtype", &self.smbtype())
            .field("smbus", &self.smbus())
            .field("pe", &self.pe())
            .finish()
    }
}
impl W {
    ///Bit 0 - Peripheral enable
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<'_, CR1rs> {
        PE_W::new(self, 0)
    }
    ///Bit 1 - SMBus mode
    #[inline(always)]
    pub fn smbus(&mut self) -> SMBUS_W<'_, CR1rs> {
        SMBUS_W::new(self, 1)
    }
    ///Bit 3 - SMBus type
    #[inline(always)]
    pub fn smbtype(&mut self) -> SMBTYPE_W<'_, CR1rs> {
        SMBTYPE_W::new(self, 3)
    }
    ///Bit 4 - ARP enable
    #[inline(always)]
    pub fn enarp(&mut self) -> ENARP_W<'_, CR1rs> {
        ENARP_W::new(self, 4)
    }
    ///Bit 5 - PEC enable
    #[inline(always)]
    pub fn enpec(&mut self) -> ENPEC_W<'_, CR1rs> {
        ENPEC_W::new(self, 5)
    }
    ///Bit 6 - General call enable
    #[inline(always)]
    pub fn engc(&mut self) -> ENGC_W<'_, CR1rs> {
        ENGC_W::new(self, 6)
    }
    ///Bit 7 - Clock stretching disable (Slave mode)
    #[inline(always)]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<'_, CR1rs> {
        NOSTRETCH_W::new(self, 7)
    }
    ///Bit 8 - Start generation
    #[inline(always)]
    pub fn start(&mut self) -> START_W<'_, CR1rs> {
        START_W::new(self, 8)
    }
    ///Bit 9 - Stop generation
    #[inline(always)]
    pub fn stop(&mut self) -> STOP_W<'_, CR1rs> {
        STOP_W::new(self, 9)
    }
    ///Bit 10 - Acknowledge enable
    #[inline(always)]
    pub fn ack(&mut self) -> ACK_W<'_, CR1rs> {
        ACK_W::new(self, 10)
    }
    ///Bit 11 - Acknowledge/PEC Position (for data reception)
    #[inline(always)]
    pub fn pos(&mut self) -> POS_W<'_, CR1rs> {
        POS_W::new(self, 11)
    }
    ///Bit 12 - Packet error checking
    #[inline(always)]
    pub fn pec(&mut self) -> PEC_W<'_, CR1rs> {
        PEC_W::new(self, 12)
    }
    ///Bit 13 - SMBus alert
    #[inline(always)]
    pub fn alert(&mut self) -> ALERT_W<'_, CR1rs> {
        ALERT_W::new(self, 13)
    }
    ///Bit 15 - Software reset
    #[inline(always)]
    pub fn swrst(&mut self) -> SWRST_W<'_, CR1rs> {
        SWRST_W::new(self, 15)
    }
}
/**Control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F413.html#I2C1:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u16;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
