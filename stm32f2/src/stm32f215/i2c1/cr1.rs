#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Peripheral enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE {
    #[doc = "0: Peripheral disabled"]
    Disabled = 0,
    #[doc = "1: Peripheral enabled"]
    Enabled = 1,
}
impl From<PE> for bool {
    #[inline(always)]
    fn from(variant: PE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Peripheral enable"]
pub type PE_R = crate::BitReader<PE>;
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PE {
        match self.bits {
            false => PE::Disabled,
            true => PE::Enabled,
        }
    }
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PE::Disabled
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PE::Enabled
    }
}
#[doc = "Field `PE` writer - Peripheral enable"]
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG, PE>;
impl<'a, REG> PE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PE::Disabled)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PE::Enabled)
    }
}
#[doc = "SMBus mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBUS {
    #[doc = "0: I2C Mode"]
    I2c = 0,
    #[doc = "1: SMBus"]
    Smbus = 1,
}
impl From<SMBUS> for bool {
    #[inline(always)]
    fn from(variant: SMBUS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBUS` reader - SMBus mode"]
pub type SMBUS_R = crate::BitReader<SMBUS>;
impl SMBUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMBUS {
        match self.bits {
            false => SMBUS::I2c,
            true => SMBUS::Smbus,
        }
    }
    #[doc = "I2C Mode"]
    #[inline(always)]
    pub fn is_i2c(&self) -> bool {
        *self == SMBUS::I2c
    }
    #[doc = "SMBus"]
    #[inline(always)]
    pub fn is_smbus(&self) -> bool {
        *self == SMBUS::Smbus
    }
}
#[doc = "Field `SMBUS` writer - SMBus mode"]
pub type SMBUS_W<'a, REG> = crate::BitWriter<'a, REG, SMBUS>;
impl<'a, REG> SMBUS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C Mode"]
    #[inline(always)]
    pub fn i2c(self) -> &'a mut crate::W<REG> {
        self.variant(SMBUS::I2c)
    }
    #[doc = "SMBus"]
    #[inline(always)]
    pub fn smbus(self) -> &'a mut crate::W<REG> {
        self.variant(SMBUS::Smbus)
    }
}
#[doc = "SMBus type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBTYPE {
    #[doc = "0: SMBus Device"]
    Device = 0,
    #[doc = "1: SMBus Host"]
    Host = 1,
}
impl From<SMBTYPE> for bool {
    #[inline(always)]
    fn from(variant: SMBTYPE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SMBTYPE` reader - SMBus type"]
pub type SMBTYPE_R = crate::BitReader<SMBTYPE>;
impl SMBTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SMBTYPE {
        match self.bits {
            false => SMBTYPE::Device,
            true => SMBTYPE::Host,
        }
    }
    #[doc = "SMBus Device"]
    #[inline(always)]
    pub fn is_device(&self) -> bool {
        *self == SMBTYPE::Device
    }
    #[doc = "SMBus Host"]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == SMBTYPE::Host
    }
}
#[doc = "Field `SMBTYPE` writer - SMBus type"]
pub type SMBTYPE_W<'a, REG> = crate::BitWriter<'a, REG, SMBTYPE>;
impl<'a, REG> SMBTYPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBus Device"]
    #[inline(always)]
    pub fn device(self) -> &'a mut crate::W<REG> {
        self.variant(SMBTYPE::Device)
    }
    #[doc = "SMBus Host"]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(SMBTYPE::Host)
    }
}
#[doc = "ARP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENARP {
    #[doc = "0: ARP disabled"]
    Disabled = 0,
    #[doc = "1: ARP enabled"]
    Enabled = 1,
}
impl From<ENARP> for bool {
    #[inline(always)]
    fn from(variant: ENARP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENARP` reader - ARP enable"]
pub type ENARP_R = crate::BitReader<ENARP>;
impl ENARP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENARP {
        match self.bits {
            false => ENARP::Disabled,
            true => ENARP::Enabled,
        }
    }
    #[doc = "ARP disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENARP::Disabled
    }
    #[doc = "ARP enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENARP::Enabled
    }
}
#[doc = "Field `ENARP` writer - ARP enable"]
pub type ENARP_W<'a, REG> = crate::BitWriter<'a, REG, ENARP>;
impl<'a, REG> ENARP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ARP disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENARP::Disabled)
    }
    #[doc = "ARP enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENARP::Enabled)
    }
}
#[doc = "PEC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENPEC {
    #[doc = "0: PEC calculation disabled"]
    Disabled = 0,
    #[doc = "1: PEC calculation enabled"]
    Enabled = 1,
}
impl From<ENPEC> for bool {
    #[inline(always)]
    fn from(variant: ENPEC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENPEC` reader - PEC enable"]
pub type ENPEC_R = crate::BitReader<ENPEC>;
impl ENPEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENPEC {
        match self.bits {
            false => ENPEC::Disabled,
            true => ENPEC::Enabled,
        }
    }
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENPEC::Disabled
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENPEC::Enabled
    }
}
#[doc = "Field `ENPEC` writer - PEC enable"]
pub type ENPEC_W<'a, REG> = crate::BitWriter<'a, REG, ENPEC>;
impl<'a, REG> ENPEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENPEC::Disabled)
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENPEC::Enabled)
    }
}
#[doc = "General call enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENGC {
    #[doc = "0: General call disabled"]
    Disabled = 0,
    #[doc = "1: General call enabled"]
    Enabled = 1,
}
impl From<ENGC> for bool {
    #[inline(always)]
    fn from(variant: ENGC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENGC` reader - General call enable"]
pub type ENGC_R = crate::BitReader<ENGC>;
impl ENGC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENGC {
        match self.bits {
            false => ENGC::Disabled,
            true => ENGC::Enabled,
        }
    }
    #[doc = "General call disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENGC::Disabled
    }
    #[doc = "General call enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENGC::Enabled
    }
}
#[doc = "Field `ENGC` writer - General call enable"]
pub type ENGC_W<'a, REG> = crate::BitWriter<'a, REG, ENGC>;
impl<'a, REG> ENGC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General call disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENGC::Disabled)
    }
    #[doc = "General call enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENGC::Enabled)
    }
}
#[doc = "Clock stretching disable (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOSTRETCH {
    #[doc = "0: Clock stretching enabled"]
    Enabled = 0,
    #[doc = "1: Clock stretching disabled"]
    Disabled = 1,
}
impl From<NOSTRETCH> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable (Slave mode)"]
pub type NOSTRETCH_R = crate::BitReader<NOSTRETCH>;
impl NOSTRETCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NOSTRETCH {
        match self.bits {
            false => NOSTRETCH::Enabled,
            true => NOSTRETCH::Disabled,
        }
    }
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTRETCH::Enabled
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTRETCH::Disabled
    }
}
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable (Slave mode)"]
pub type NOSTRETCH_W<'a, REG> = crate::BitWriter<'a, REG, NOSTRETCH>;
impl<'a, REG> NOSTRETCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH::Enabled)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(NOSTRETCH::Disabled)
    }
}
#[doc = "Start generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START {
    #[doc = "0: No Start generation"]
    NoStart = 0,
    #[doc = "1: In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    Start = 1,
}
impl From<START> for bool {
    #[inline(always)]
    fn from(variant: START) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start generation"]
pub type START_R = crate::BitReader<START>;
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> START {
        match self.bits {
            false => START::NoStart,
            true => START::Start,
        }
    }
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START::NoStart
    }
    #[doc = "In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START::Start
    }
}
#[doc = "Field `START` writer - Start generation"]
pub type START_W<'a, REG> = crate::BitWriter<'a, REG, START>;
impl<'a, REG> START_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(START::NoStart)
    }
    #[doc = "In master mode: repeated start generation, in slave mode: start generation when bus is free"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(START::Start)
    }
}
#[doc = "Stop generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP {
    #[doc = "0: No Stop generation"]
    NoStop = 0,
    #[doc = "1: In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    Stop = 1,
}
impl From<STOP> for bool {
    #[inline(always)]
    fn from(variant: STOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop generation"]
pub type STOP_R = crate::BitReader<STOP>;
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> STOP {
        match self.bits {
            false => STOP::NoStop,
            true => STOP::Stop,
        }
    }
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOP::NoStop
    }
    #[doc = "In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOP::Stop
    }
}
#[doc = "Field `STOP` writer - Stop generation"]
pub type STOP_W<'a, REG> = crate::BitWriter<'a, REG, STOP>;
impl<'a, REG> STOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::NoStop)
    }
    #[doc = "In master mode: stop generation after current byte/start, in slave mode: release SCL and SDA after current byte"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(STOP::Stop)
    }
}
#[doc = "Acknowledge enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACK {
    #[doc = "0: No acknowledge returned"]
    Nak = 0,
    #[doc = "1: Acknowledge returned after a byte is received"]
    Ack = 1,
}
impl From<ACK> for bool {
    #[inline(always)]
    fn from(variant: ACK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACK` reader - Acknowledge enable"]
pub type ACK_R = crate::BitReader<ACK>;
impl ACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACK {
        match self.bits {
            false => ACK::Nak,
            true => ACK::Ack,
        }
    }
    #[doc = "No acknowledge returned"]
    #[inline(always)]
    pub fn is_nak(&self) -> bool {
        *self == ACK::Nak
    }
    #[doc = "Acknowledge returned after a byte is received"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == ACK::Ack
    }
}
#[doc = "Field `ACK` writer - Acknowledge enable"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG, ACK>;
impl<'a, REG> ACK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No acknowledge returned"]
    #[inline(always)]
    pub fn nak(self) -> &'a mut crate::W<REG> {
        self.variant(ACK::Nak)
    }
    #[doc = "Acknowledge returned after a byte is received"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut crate::W<REG> {
        self.variant(ACK::Ack)
    }
}
#[doc = "Acknowledge/PEC Position (for data reception)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POS {
    #[doc = "0: ACK bit controls the (N)ACK of the current byte being received"]
    Current = 0,
    #[doc = "1: ACK bit controls the (N)ACK of the next byte to be received"]
    Next = 1,
}
impl From<POS> for bool {
    #[inline(always)]
    fn from(variant: POS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POS` reader - Acknowledge/PEC Position (for data reception)"]
pub type POS_R = crate::BitReader<POS>;
impl POS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> POS {
        match self.bits {
            false => POS::Current,
            true => POS::Next,
        }
    }
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    #[inline(always)]
    pub fn is_current(&self) -> bool {
        *self == POS::Current
    }
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    #[inline(always)]
    pub fn is_next(&self) -> bool {
        *self == POS::Next
    }
}
#[doc = "Field `POS` writer - Acknowledge/PEC Position (for data reception)"]
pub type POS_W<'a, REG> = crate::BitWriter<'a, REG, POS>;
impl<'a, REG> POS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ACK bit controls the (N)ACK of the current byte being received"]
    #[inline(always)]
    pub fn current(self) -> &'a mut crate::W<REG> {
        self.variant(POS::Current)
    }
    #[doc = "ACK bit controls the (N)ACK of the next byte to be received"]
    #[inline(always)]
    pub fn next(self) -> &'a mut crate::W<REG> {
        self.variant(POS::Next)
    }
}
#[doc = "Packet error checking\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEC {
    #[doc = "0: No PEC transfer"]
    Disabled = 0,
    #[doc = "1: PEC transfer"]
    Enabled = 1,
}
impl From<PEC> for bool {
    #[inline(always)]
    fn from(variant: PEC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PEC` reader - Packet error checking"]
pub type PEC_R = crate::BitReader<PEC>;
impl PEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PEC {
        match self.bits {
            false => PEC::Disabled,
            true => PEC::Enabled,
        }
    }
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEC::Disabled
    }
    #[doc = "PEC transfer"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEC::Enabled
    }
}
#[doc = "Field `PEC` writer - Packet error checking"]
pub type PEC_W<'a, REG> = crate::BitWriter<'a, REG, PEC>;
impl<'a, REG> PEC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEC::Disabled)
    }
    #[doc = "PEC transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PEC::Enabled)
    }
}
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERT {
    #[doc = "0: SMBA pin released high"]
    Release = 0,
    #[doc = "1: SMBA pin driven low"]
    Drive = 1,
}
impl From<ALERT> for bool {
    #[inline(always)]
    fn from(variant: ALERT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERT` reader - SMBus alert"]
pub type ALERT_R = crate::BitReader<ALERT>;
impl ALERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ALERT {
        match self.bits {
            false => ALERT::Release,
            true => ALERT::Drive,
        }
    }
    #[doc = "SMBA pin released high"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == ALERT::Release
    }
    #[doc = "SMBA pin driven low"]
    #[inline(always)]
    pub fn is_drive(&self) -> bool {
        *self == ALERT::Drive
    }
}
#[doc = "Field `ALERT` writer - SMBus alert"]
pub type ALERT_W<'a, REG> = crate::BitWriter<'a, REG, ALERT>;
impl<'a, REG> ALERT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SMBA pin released high"]
    #[inline(always)]
    pub fn release(self) -> &'a mut crate::W<REG> {
        self.variant(ALERT::Release)
    }
    #[doc = "SMBA pin driven low"]
    #[inline(always)]
    pub fn drive(self) -> &'a mut crate::W<REG> {
        self.variant(ALERT::Drive)
    }
}
#[doc = "Software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRST {
    #[doc = "0: I2C peripheral not under reset"]
    NotReset = 0,
    #[doc = "1: I2C peripheral under reset"]
    Reset = 1,
}
impl From<SWRST> for bool {
    #[inline(always)]
    fn from(variant: SWRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SWRST` reader - Software reset"]
pub type SWRST_R = crate::BitReader<SWRST>;
impl SWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SWRST {
        match self.bits {
            false => SWRST::NotReset,
            true => SWRST::Reset,
        }
    }
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == SWRST::NotReset
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == SWRST::Reset
    }
}
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a, REG> = crate::BitWriter<'a, REG, SWRST>;
impl<'a, REG> SWRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C peripheral not under reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut crate::W<REG> {
        self.variant(SWRST::NotReset)
    }
    #[doc = "I2C peripheral under reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(SWRST::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    pub fn smbus(&self) -> SMBUS_R {
        SMBUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    pub fn smbtype(&self) -> SMBTYPE_R {
        SMBTYPE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    pub fn enarp(&self) -> ENARP_R {
        ENARP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    pub fn enpec(&self) -> ENPEC_R {
        ENPEC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    pub fn engc(&self) -> ENGC_R {
        ENGC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    pub fn pec(&self) -> PEC_R {
        PEC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<CR1rs> {
        PE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SMBus mode"]
    #[inline(always)]
    #[must_use]
    pub fn smbus(&mut self) -> SMBUS_W<CR1rs> {
        SMBUS_W::new(self, 1)
    }
    #[doc = "Bit 3 - SMBus type"]
    #[inline(always)]
    #[must_use]
    pub fn smbtype(&mut self) -> SMBTYPE_W<CR1rs> {
        SMBTYPE_W::new(self, 3)
    }
    #[doc = "Bit 4 - ARP enable"]
    #[inline(always)]
    #[must_use]
    pub fn enarp(&mut self) -> ENARP_W<CR1rs> {
        ENARP_W::new(self, 4)
    }
    #[doc = "Bit 5 - PEC enable"]
    #[inline(always)]
    #[must_use]
    pub fn enpec(&mut self) -> ENPEC_W<CR1rs> {
        ENPEC_W::new(self, 5)
    }
    #[doc = "Bit 6 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn engc(&mut self) -> ENGC_W<CR1rs> {
        ENGC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clock stretching disable (Slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<CR1rs> {
        NOSTRETCH_W::new(self, 7)
    }
    #[doc = "Bit 8 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<CR1rs> {
        START_W::new(self, 8)
    }
    #[doc = "Bit 9 - Stop generation"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<CR1rs> {
        STOP_W::new(self, 9)
    }
    #[doc = "Bit 10 - Acknowledge enable"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<CR1rs> {
        ACK_W::new(self, 10)
    }
    #[doc = "Bit 11 - Acknowledge/PEC Position (for data reception)"]
    #[inline(always)]
    #[must_use]
    pub fn pos(&mut self) -> POS_W<CR1rs> {
        POS_W::new(self, 11)
    }
    #[doc = "Bit 12 - Packet error checking"]
    #[inline(always)]
    #[must_use]
    pub fn pec(&mut self) -> PEC_W<CR1rs> {
        PEC_W::new(self, 12)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    #[must_use]
    pub fn alert(&mut self) -> ALERT_W<CR1rs> {
        ALERT_W::new(self, 13)
    }
    #[doc = "Bit 15 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<CR1rs> {
        SWRST_W::new(self, 15)
    }
}
#[doc = "Control register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
