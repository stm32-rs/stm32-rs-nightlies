#[doc = "Register `CFG2` reader"]
pub type R = crate::R<CFG2rs>;
#[doc = "Register `CFG2` writer"]
pub type W = crate::W<CFG2rs>;
#[doc = "Field `MSSI` reader - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
pub type MSSI_R = crate::FieldReader;
#[doc = "Field `MSSI` writer - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
pub type MSSI_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `MIDI` reader - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
pub type MIDI_R = crate::FieldReader;
#[doc = "Field `MIDI` writer - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
pub type MIDI_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4>;
#[doc = "Field `RDIOM` reader - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
pub type RDIOM_R = crate::BitReader;
#[doc = "Field `RDIOM` writer - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
pub type RDIOM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDIOP` reader - RDY signal input/output polarity"]
pub type RDIOP_R = crate::BitReader;
#[doc = "Field `RDIOP` writer - RDY signal input/output polarity"]
pub type RDIOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IOSWP {
    #[doc = "0: MISO and MOSI not swapped"]
    Disabled = 0,
    #[doc = "1: MISO and MOSI swapped"]
    Enabled = 1,
}
impl From<IOSWP> for bool {
    #[inline(always)]
    fn from(variant: IOSWP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IOSWP` reader - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins."]
pub type IOSWP_R = crate::BitReader<IOSWP>;
impl IOSWP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IOSWP {
        match self.bits {
            false => IOSWP::Disabled,
            true => IOSWP::Enabled,
        }
    }
    #[doc = "MISO and MOSI not swapped"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IOSWP::Disabled
    }
    #[doc = "MISO and MOSI swapped"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IOSWP::Enabled
    }
}
#[doc = "Field `IOSWP` writer - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins."]
pub type IOSWP_W<'a, REG> = crate::BitWriter<'a, REG, IOSWP>;
impl<'a, REG> IOSWP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MISO and MOSI not swapped"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOSWP::Disabled)
    }
    #[doc = "MISO and MOSI swapped"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IOSWP::Enabled)
    }
}
#[doc = "SPI Communication Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum COMM {
    #[doc = "0: Full duplex"]
    FullDuplex = 0,
    #[doc = "1: Simplex transmitter only"]
    Transmitter = 1,
    #[doc = "2: Simplex receiver only"]
    Receiver = 2,
    #[doc = "3: Half duplex"]
    HalfDuplex = 3,
}
impl From<COMM> for u8 {
    #[inline(always)]
    fn from(variant: COMM) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for COMM {
    type Ux = u8;
}
#[doc = "Field `COMM` reader - SPI Communication Mode"]
pub type COMM_R = crate::FieldReader<COMM>;
impl COMM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMM {
        match self.bits {
            0 => COMM::FullDuplex,
            1 => COMM::Transmitter,
            2 => COMM::Receiver,
            3 => COMM::HalfDuplex,
            _ => unreachable!(),
        }
    }
    #[doc = "Full duplex"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == COMM::FullDuplex
    }
    #[doc = "Simplex transmitter only"]
    #[inline(always)]
    pub fn is_transmitter(&self) -> bool {
        *self == COMM::Transmitter
    }
    #[doc = "Simplex receiver only"]
    #[inline(always)]
    pub fn is_receiver(&self) -> bool {
        *self == COMM::Receiver
    }
    #[doc = "Half duplex"]
    #[inline(always)]
    pub fn is_half_duplex(&self) -> bool {
        *self == COMM::HalfDuplex
    }
}
#[doc = "Field `COMM` writer - SPI Communication Mode"]
pub type COMM_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, COMM>;
impl<'a, REG> COMM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Full duplex"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(COMM::FullDuplex)
    }
    #[doc = "Simplex transmitter only"]
    #[inline(always)]
    pub fn transmitter(self) -> &'a mut crate::W<REG> {
        self.variant(COMM::Transmitter)
    }
    #[doc = "Simplex receiver only"]
    #[inline(always)]
    pub fn receiver(self) -> &'a mut crate::W<REG> {
        self.variant(COMM::Receiver)
    }
    #[doc = "Half duplex"]
    #[inline(always)]
    pub fn half_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(COMM::HalfDuplex)
    }
}
#[doc = "serial protocol others: reserved, must not be used\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SP {
    #[doc = "0: Motorola SPI protocol"]
    Motorola = 0,
    #[doc = "1: TI SPI protocol"]
    Ti = 1,
}
impl From<SP> for u8 {
    #[inline(always)]
    fn from(variant: SP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SP {
    type Ux = u8;
}
#[doc = "Field `SP` reader - serial protocol others: reserved, must not be used"]
pub type SP_R = crate::FieldReader<SP>;
impl SP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SP> {
        match self.bits {
            0 => Some(SP::Motorola),
            1 => Some(SP::Ti),
            _ => None,
        }
    }
    #[doc = "Motorola SPI protocol"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == SP::Motorola
    }
    #[doc = "TI SPI protocol"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == SP::Ti
    }
}
#[doc = "Field `SP` writer - serial protocol others: reserved, must not be used"]
pub type SP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, SP>;
impl<'a, REG> SP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Motorola SPI protocol"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut crate::W<REG> {
        self.variant(SP::Motorola)
    }
    #[doc = "TI SPI protocol"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(SP::Ti)
    }
}
#[doc = "SPI Master\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MASTER {
    #[doc = "0: Slave configuration"]
    Slave = 0,
    #[doc = "1: Master configuration"]
    Master = 1,
}
impl From<MASTER> for bool {
    #[inline(always)]
    fn from(variant: MASTER) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MASTER` reader - SPI Master"]
pub type MASTER_R = crate::BitReader<MASTER>;
impl MASTER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MASTER {
        match self.bits {
            false => MASTER::Slave,
            true => MASTER::Master,
        }
    }
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MASTER::Slave
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MASTER::Master
    }
}
#[doc = "Field `MASTER` writer - SPI Master"]
pub type MASTER_W<'a, REG> = crate::BitWriter<'a, REG, MASTER>;
impl<'a, REG> MASTER_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MASTER::Slave)
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(MASTER::Master)
    }
}
#[doc = "data frame format Note: This bit can be also used in PCM and I2S modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFRST {
    #[doc = "0: Data is transmitted/received with the MSB first"]
    Msbfirst = 0,
    #[doc = "1: Data is transmitted/received with the LSB first"]
    Lsbfirst = 1,
}
impl From<LSBFRST> for bool {
    #[inline(always)]
    fn from(variant: LSBFRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LSBFRST` reader - data frame format Note: This bit can be also used in PCM and I2S modes."]
pub type LSBFRST_R = crate::BitReader<LSBFRST>;
impl LSBFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LSBFRST {
        match self.bits {
            false => LSBFRST::Msbfirst,
            true => LSBFRST::Lsbfirst,
        }
    }
    #[doc = "Data is transmitted/received with the MSB first"]
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFRST::Msbfirst
    }
    #[doc = "Data is transmitted/received with the LSB first"]
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFRST::Lsbfirst
    }
}
#[doc = "Field `LSBFRST` writer - data frame format Note: This bit can be also used in PCM and I2S modes."]
pub type LSBFRST_W<'a, REG> = crate::BitWriter<'a, REG, LSBFRST>;
impl<'a, REG> LSBFRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Data is transmitted/received with the MSB first"]
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFRST::Msbfirst)
    }
    #[doc = "Data is transmitted/received with the LSB first"]
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFRST::Lsbfirst)
    }
}
#[doc = "clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA {
    #[doc = "0: The first clock transition is the first data capture edge"]
    FirstEdge = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    SecondEdge = 1,
}
impl From<CPHA> for bool {
    #[inline(always)]
    fn from(variant: CPHA) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - clock phase"]
pub type CPHA_R = crate::BitReader<CPHA>;
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPHA {
        match self.bits {
            false => CPHA::FirstEdge,
            true => CPHA::SecondEdge,
        }
    }
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA::FirstEdge
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA::SecondEdge
    }
}
#[doc = "Field `CPHA` writer - clock phase"]
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::FirstEdge)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::SecondEdge)
    }
}
#[doc = "clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL {
    #[doc = "0: CK to 0 when idle"]
    IdleLow = 0,
    #[doc = "1: CK to 1 when idle"]
    IdleHigh = 1,
}
impl From<CPOL> for bool {
    #[inline(always)]
    fn from(variant: CPOL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - clock polarity"]
pub type CPOL_R = crate::BitReader<CPOL>;
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CPOL {
        match self.bits {
            false => CPOL::IdleLow,
            true => CPOL::IdleHigh,
        }
    }
    #[doc = "CK to 0 when idle"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL::IdleLow
    }
    #[doc = "CK to 1 when idle"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL::IdleHigh
    }
}
#[doc = "Field `CPOL` writer - clock polarity"]
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CK to 0 when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::IdleLow)
    }
    #[doc = "CK to 1 when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::IdleHigh)
    }
}
#[doc = "software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM {
    #[doc = "0: Software slave management disabled"]
    Disabled = 0,
    #[doc = "1: Software slave management enabled"]
    Enabled = 1,
}
impl From<SSM> for bool {
    #[inline(always)]
    fn from(variant: SSM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSM` reader - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
pub type SSM_R = crate::BitReader<SSM>;
impl SSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSM {
        match self.bits {
            false => SSM::Disabled,
            true => SSM::Enabled,
        }
    }
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM::Disabled
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM::Enabled
    }
}
#[doc = "Field `SSM` writer - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG, SSM>;
impl<'a, REG> SSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSM::Disabled)
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSM::Enabled)
    }
}
#[doc = "SS input/output polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSIOP {
    #[doc = "0: Low level is active for SS signal"]
    ActiveLow = 0,
    #[doc = "1: High level is active for SS signal"]
    ActiveHigh = 1,
}
impl From<SSIOP> for bool {
    #[inline(always)]
    fn from(variant: SSIOP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSIOP` reader - SS input/output polarity"]
pub type SSIOP_R = crate::BitReader<SSIOP>;
impl SSIOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSIOP {
        match self.bits {
            false => SSIOP::ActiveLow,
            true => SSIOP::ActiveHigh,
        }
    }
    #[doc = "Low level is active for SS signal"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == SSIOP::ActiveLow
    }
    #[doc = "High level is active for SS signal"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == SSIOP::ActiveHigh
    }
}
#[doc = "Field `SSIOP` writer - SS input/output polarity"]
pub type SSIOP_W<'a, REG> = crate::BitWriter<'a, REG, SSIOP>;
impl<'a, REG> SSIOP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low level is active for SS signal"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(SSIOP::ActiveLow)
    }
    #[doc = "High level is active for SS signal"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(SSIOP::ActiveHigh)
    }
}
#[doc = "SS output enable This bit is taken into account in Master mode only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE {
    #[doc = "0: SS output is disabled in master mode"]
    Disabled = 0,
    #[doc = "1: SS output is enabled in master mode"]
    Enabled = 1,
}
impl From<SSOE> for bool {
    #[inline(always)]
    fn from(variant: SSOE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOE` reader - SS output enable This bit is taken into account in Master mode only"]
pub type SSOE_R = crate::BitReader<SSOE>;
impl SSOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSOE {
        match self.bits {
            false => SSOE::Disabled,
            true => SSOE::Enabled,
        }
    }
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE::Disabled
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE::Enabled
    }
}
#[doc = "Field `SSOE` writer - SS output enable This bit is taken into account in Master mode only"]
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG, SSOE>;
impl<'a, REG> SSOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Disabled)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Enabled)
    }
}
#[doc = "SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOM {
    #[doc = "0: SS is asserted until data transfer complete"]
    Asserted = 0,
    #[doc = "1: Data frames interleaved with SS not asserted during MIDI"]
    NotAsserted = 1,
}
impl From<SSOM> for bool {
    #[inline(always)]
    fn from(variant: SSOM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSOM` reader - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
pub type SSOM_R = crate::BitReader<SSOM>;
impl SSOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SSOM {
        match self.bits {
            false => SSOM::Asserted,
            true => SSOM::NotAsserted,
        }
    }
    #[doc = "SS is asserted until data transfer complete"]
    #[inline(always)]
    pub fn is_asserted(&self) -> bool {
        *self == SSOM::Asserted
    }
    #[doc = "Data frames interleaved with SS not asserted during MIDI"]
    #[inline(always)]
    pub fn is_not_asserted(&self) -> bool {
        *self == SSOM::NotAsserted
    }
}
#[doc = "Field `SSOM` writer - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
pub type SSOM_W<'a, REG> = crate::BitWriter<'a, REG, SSOM>;
impl<'a, REG> SSOM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SS is asserted until data transfer complete"]
    #[inline(always)]
    pub fn asserted(self) -> &'a mut crate::W<REG> {
        self.variant(SSOM::Asserted)
    }
    #[doc = "Data frames interleaved with SS not asserted during MIDI"]
    #[inline(always)]
    pub fn not_asserted(self) -> &'a mut crate::W<REG> {
        self.variant(SSOM::NotAsserted)
    }
}
#[doc = "alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AFCNTR {
    #[doc = "0: Peripheral takes no control of GPIOs while disabled"]
    NotControlled = 0,
    #[doc = "1: Peripheral controls GPIOs while disabled"]
    Controlled = 1,
}
impl From<AFCNTR> for bool {
    #[inline(always)]
    fn from(variant: AFCNTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFCNTR` reader - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode."]
pub type AFCNTR_R = crate::BitReader<AFCNTR>;
impl AFCNTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AFCNTR {
        match self.bits {
            false => AFCNTR::NotControlled,
            true => AFCNTR::Controlled,
        }
    }
    #[doc = "Peripheral takes no control of GPIOs while disabled"]
    #[inline(always)]
    pub fn is_not_controlled(&self) -> bool {
        *self == AFCNTR::NotControlled
    }
    #[doc = "Peripheral controls GPIOs while disabled"]
    #[inline(always)]
    pub fn is_controlled(&self) -> bool {
        *self == AFCNTR::Controlled
    }
}
#[doc = "Field `AFCNTR` writer - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode."]
pub type AFCNTR_W<'a, REG> = crate::BitWriter<'a, REG, AFCNTR>;
impl<'a, REG> AFCNTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Peripheral takes no control of GPIOs while disabled"]
    #[inline(always)]
    pub fn not_controlled(self) -> &'a mut crate::W<REG> {
        self.variant(AFCNTR::NotControlled)
    }
    #[doc = "Peripheral controls GPIOs while disabled"]
    #[inline(always)]
    pub fn controlled(self) -> &'a mut crate::W<REG> {
        self.variant(AFCNTR::Controlled)
    }
}
impl R {
    #[doc = "Bits 0:3 - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
    #[inline(always)]
    pub fn mssi(&self) -> MSSI_R {
        MSSI_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
    #[inline(always)]
    pub fn midi(&self) -> MIDI_R {
        MIDI_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
    #[inline(always)]
    pub fn rdiom(&self) -> RDIOM_R {
        RDIOM_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - RDY signal input/output polarity"]
    #[inline(always)]
    pub fn rdiop(&self) -> RDIOP_R {
        RDIOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins."]
    #[inline(always)]
    pub fn ioswp(&self) -> IOSWP_R {
        IOSWP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    pub fn comm(&self) -> COMM_R {
        COMM_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 19:21 - serial protocol others: reserved, must not be used"]
    #[inline(always)]
    pub fn sp(&self) -> SP_R {
        SP_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - data frame format Note: This bit can be also used in PCM and I2S modes."]
    #[inline(always)]
    pub fn lsbfrst(&self) -> LSBFRST_R {
        LSBFRST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    pub fn ssiop(&self) -> SSIOP_R {
        SSIOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SS output enable This bit is taken into account in Master mode only"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
    #[inline(always)]
    pub fn ssom(&self) -> SSOM_R {
        SSOM_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode."]
    #[inline(always)]
    pub fn afcntr(&self) -> AFCNTR_R {
        AFCNTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Master SS Idleness Specifies an extra delay, expressed in number of SPI clock cycle periods, inserted additionally between active edge of SS opening a session and the beginning of the first data frame of the session in Master mode when SSOE is enabled. ... Note: This feature is not supported in TI mode. To include the delay, the SPI must be disabled and re-enabled between sessions."]
    #[inline(always)]
    #[must_use]
    pub fn mssi(&mut self) -> MSSI_W<CFG2rs> {
        MSSI_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - master Inter-Data Idleness Specifies minimum time delay (expressed in SPI clock cycles periods) inserted between two consecutive data frames in Master mode. ... Note: This feature is not supported in TI mode."]
    #[inline(always)]
    #[must_use]
    pub fn midi(&mut self) -> MIDI_W<CFG2rs> {
        MIDI_W::new(self, 4)
    }
    #[doc = "Bit 13 - RDY signal input/output management Note: When DSIZE at the SPI_CFG1 register is configured shorter than 8-bit, the RDIOM bit has to be kept at zero."]
    #[inline(always)]
    #[must_use]
    pub fn rdiom(&mut self) -> RDIOM_W<CFG2rs> {
        RDIOM_W::new(self, 13)
    }
    #[doc = "Bit 14 - RDY signal input/output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn rdiop(&mut self) -> RDIOP_W<CFG2rs> {
        RDIOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - swap functionality of MISO and MOSI pins When this bit is set, the function of MISO and MOSI pins alternate functions are inverted. Original MISO pin becomes MOSI and original MOSI pin becomes MISO. Note: This bit can be also used in PCM and I2S modes to swap SDO and SDI pins."]
    #[inline(always)]
    #[must_use]
    pub fn ioswp(&mut self) -> IOSWP_W<CFG2rs> {
        IOSWP_W::new(self, 15)
    }
    #[doc = "Bits 17:18 - SPI Communication Mode"]
    #[inline(always)]
    #[must_use]
    pub fn comm(&mut self) -> COMM_W<CFG2rs> {
        COMM_W::new(self, 17)
    }
    #[doc = "Bits 19:21 - serial protocol others: reserved, must not be used"]
    #[inline(always)]
    #[must_use]
    pub fn sp(&mut self) -> SP_W<CFG2rs> {
        SP_W::new(self, 19)
    }
    #[doc = "Bit 22 - SPI Master"]
    #[inline(always)]
    #[must_use]
    pub fn master(&mut self) -> MASTER_W<CFG2rs> {
        MASTER_W::new(self, 22)
    }
    #[doc = "Bit 23 - data frame format Note: This bit can be also used in PCM and I2S modes."]
    #[inline(always)]
    #[must_use]
    pub fn lsbfrst(&mut self) -> LSBFRST_W<CFG2rs> {
        LSBFRST_W::new(self, 23)
    }
    #[doc = "Bit 24 - clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<CFG2rs> {
        CPHA_W::new(self, 24)
    }
    #[doc = "Bit 25 - clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<CFG2rs> {
        CPOL_W::new(self, 25)
    }
    #[doc = "Bit 26 - software management of SS signal input When master uses hardware SS output (SSM=0 and SSOE=1) the SS signal input is forced to not active state internally to prevent master mode fault error."]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<CFG2rs> {
        SSM_W::new(self, 26)
    }
    #[doc = "Bit 28 - SS input/output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn ssiop(&mut self) -> SSIOP_W<CFG2rs> {
        SSIOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - SS output enable This bit is taken into account in Master mode only"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<CFG2rs> {
        SSOE_W::new(self, 29)
    }
    #[doc = "Bit 30 - SS output management in Master mode This bit is taken into account in Master mode when SSOE is enabled. It allows the SS output to be configured between two consecutive data transfers."]
    #[inline(always)]
    #[must_use]
    pub fn ssom(&mut self) -> SSOM_W<CFG2rs> {
        SSOM_W::new(self, 30)
    }
    #[doc = "Bit 31 - alternate function GPIOs control This bit is taken into account when SPE=0 only When SPI has to be disabled temporary for a specific configuration reason (e.g. CRC reset, CPHA or HDDIR change) setting this bit prevents any glitches on the associated outputs configured at alternate function mode by keeping them forced at state corresponding the current SPI configuration. Note: This bit can be also used in PCM and I2S modes. Note: The bit AFCNTR must not be set to 1, when the block is in slave mode."]
    #[inline(always)]
    #[must_use]
    pub fn afcntr(&mut self) -> AFCNTR_W<CFG2rs> {
        AFCNTR_W::new(self, 31)
    }
}
#[doc = "SPI/I2S configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG2rs;
impl crate::RegisterSpec for CFG2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg2::R`](R) reader structure"]
impl crate::Readable for CFG2rs {}
#[doc = "`write(|w| ..)` method takes [`cfg2::W`](W) writer structure"]
impl crate::Writable for CFG2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CFG2 to value 0"]
impl crate::Resettable for CFG2rs {
    const RESET_VALUE: u32 = 0;
}
