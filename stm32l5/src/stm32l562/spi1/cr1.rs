///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**Clock phase

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA {
    ///0: The first clock transition is the first data capture edge
    FirstEdge = 0,
    ///1: The second clock transition is the first data capture edge
    SecondEdge = 1,
}
impl From<CPHA> for bool {
    #[inline(always)]
    fn from(variant: CPHA) -> Self {
        variant as u8 != 0
    }
}
///Field `CPHA` reader - Clock phase
pub type CPHA_R = crate::BitReader<CPHA>;
impl CPHA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPHA {
        match self.bits {
            false => CPHA::FirstEdge,
            true => CPHA::SecondEdge,
        }
    }
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA::FirstEdge
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA::SecondEdge
    }
}
///Field `CPHA` writer - Clock phase
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::FirstEdge)
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::SecondEdge)
    }
}
/**Clock polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL {
    ///0: CK to 0 when idle
    IdleLow = 0,
    ///1: CK to 1 when idle
    IdleHigh = 1,
}
impl From<CPOL> for bool {
    #[inline(always)]
    fn from(variant: CPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `CPOL` reader - Clock polarity
pub type CPOL_R = crate::BitReader<CPOL>;
impl CPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPOL {
        match self.bits {
            false => CPOL::IdleLow,
            true => CPOL::IdleHigh,
        }
    }
    ///CK to 0 when idle
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL::IdleLow
    }
    ///CK to 1 when idle
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL::IdleHigh
    }
}
///Field `CPOL` writer - Clock polarity
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CK to 0 when idle
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::IdleLow)
    }
    ///CK to 1 when idle
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::IdleHigh)
    }
}
/**Master selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR {
    ///0: Slave configuration
    Slave = 0,
    ///1: Master configuration
    Master = 1,
}
impl From<MSTR> for bool {
    #[inline(always)]
    fn from(variant: MSTR) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTR` reader - Master selection
pub type MSTR_R = crate::BitReader<MSTR>;
impl MSTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTR {
        match self.bits {
            false => MSTR::Slave,
            true => MSTR::Master,
        }
    }
    ///Slave configuration
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTR::Slave
    }
    ///Master configuration
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTR::Master
    }
}
///Field `MSTR` writer - Master selection
pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG, MSTR>;
impl<'a, REG> MSTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave configuration
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR::Slave)
    }
    ///Master configuration
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR::Master)
    }
}
/**Baud rate control

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BR {
    ///0: f_PCLK / 2
    Div2 = 0,
    ///1: f_PCLK / 4
    Div4 = 1,
    ///2: f_PCLK / 8
    Div8 = 2,
    ///3: f_PCLK / 16
    Div16 = 3,
    ///4: f_PCLK / 32
    Div32 = 4,
    ///5: f_PCLK / 64
    Div64 = 5,
    ///6: f_PCLK / 128
    Div128 = 6,
    ///7: f_PCLK / 256
    Div256 = 7,
}
impl From<BR> for u8 {
    #[inline(always)]
    fn from(variant: BR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BR {
    type Ux = u8;
}
impl crate::IsEnum for BR {}
///Field `BR` reader - Baud rate control
pub type BR_R = crate::FieldReader<BR>;
impl BR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BR {
        match self.bits {
            0 => BR::Div2,
            1 => BR::Div4,
            2 => BR::Div8,
            3 => BR::Div16,
            4 => BR::Div32,
            5 => BR::Div64,
            6 => BR::Div128,
            7 => BR::Div256,
            _ => unreachable!(),
        }
    }
    ///f_PCLK / 2
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BR::Div2
    }
    ///f_PCLK / 4
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BR::Div4
    }
    ///f_PCLK / 8
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BR::Div8
    }
    ///f_PCLK / 16
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BR::Div16
    }
    ///f_PCLK / 32
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BR::Div32
    }
    ///f_PCLK / 64
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BR::Div64
    }
    ///f_PCLK / 128
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BR::Div128
    }
    ///f_PCLK / 256
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BR::Div256
    }
}
///Field `BR` writer - Baud rate control
pub type BR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BR, crate::Safe>;
impl<'a, REG> BR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///f_PCLK / 2
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(BR::Div2)
    }
    ///f_PCLK / 4
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(BR::Div4)
    }
    ///f_PCLK / 8
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(BR::Div8)
    }
    ///f_PCLK / 16
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(BR::Div16)
    }
    ///f_PCLK / 32
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(BR::Div32)
    }
    ///f_PCLK / 64
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(BR::Div64)
    }
    ///f_PCLK / 128
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(BR::Div128)
    }
    ///f_PCLK / 256
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(BR::Div256)
    }
}
/**SPI enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE {
    ///0: Peripheral disabled
    Disabled = 0,
    ///1: Peripheral enabled
    Enabled = 1,
}
impl From<SPE> for bool {
    #[inline(always)]
    fn from(variant: SPE) -> Self {
        variant as u8 != 0
    }
}
///Field `SPE` reader - SPI enable
pub type SPE_R = crate::BitReader<SPE>;
impl SPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPE {
        match self.bits {
            false => SPE::Disabled,
            true => SPE::Enabled,
        }
    }
    ///Peripheral disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPE::Disabled
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPE::Enabled
    }
}
///Field `SPE` writer - SPI enable
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG, SPE>;
impl<'a, REG> SPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPE::Disabled)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SPE::Enabled)
    }
}
/**Frame format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFIRST {
    ///0: Data is transmitted/received with the MSB first
    Msbfirst = 0,
    ///1: Data is transmitted/received with the LSB first
    Lsbfirst = 1,
}
impl From<LSBFIRST> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `LSBFIRST` reader - Frame format
pub type LSBFIRST_R = crate::BitReader<LSBFIRST>;
impl LSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSBFIRST {
        match self.bits {
            false => LSBFIRST::Msbfirst,
            true => LSBFIRST::Lsbfirst,
        }
    }
    ///Data is transmitted/received with the MSB first
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFIRST::Msbfirst
    }
    ///Data is transmitted/received with the LSB first
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFIRST::Lsbfirst
    }
}
///Field `LSBFIRST` writer - Frame format
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, LSBFIRST>;
impl<'a, REG> LSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Data is transmitted/received with the MSB first
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST::Msbfirst)
    }
    ///Data is transmitted/received with the LSB first
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST::Lsbfirst)
    }
}
/**Internal slave select

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSI {
    ///0: 0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    SlaveSelected = 0,
    ///1: 1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    SlaveNotSelected = 1,
}
impl From<SSI> for bool {
    #[inline(always)]
    fn from(variant: SSI) -> Self {
        variant as u8 != 0
    }
}
///Field `SSI` reader - Internal slave select
pub type SSI_R = crate::BitReader<SSI>;
impl SSI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSI {
        match self.bits {
            false => SSI::SlaveSelected,
            true => SSI::SlaveNotSelected,
        }
    }
    ///0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SSI::SlaveSelected
    }
    ///1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SSI::SlaveNotSelected
    }
}
///Field `SSI` writer - Internal slave select
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG, SSI>;
impl<'a, REG> SSI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut crate::W<REG> {
        self.variant(SSI::SlaveSelected)
    }
    ///1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut crate::W<REG> {
        self.variant(SSI::SlaveNotSelected)
    }
}
/**Software slave management

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM {
    ///0: Software slave management disabled
    Disabled = 0,
    ///1: Software slave management enabled
    Enabled = 1,
}
impl From<SSM> for bool {
    #[inline(always)]
    fn from(variant: SSM) -> Self {
        variant as u8 != 0
    }
}
///Field `SSM` reader - Software slave management
pub type SSM_R = crate::BitReader<SSM>;
impl SSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSM {
        match self.bits {
            false => SSM::Disabled,
            true => SSM::Enabled,
        }
    }
    ///Software slave management disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM::Disabled
    }
    ///Software slave management enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM::Enabled
    }
}
///Field `SSM` writer - Software slave management
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG, SSM>;
impl<'a, REG> SSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software slave management disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSM::Disabled)
    }
    ///Software slave management enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSM::Enabled)
    }
}
/**Receive only

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXONLY {
    ///0: Full duplex (Transmit and receive)
    FullDuplex = 0,
    ///1: Output disabled (Receive-only mode)
    OutputDisabled = 1,
}
impl From<RXONLY> for bool {
    #[inline(always)]
    fn from(variant: RXONLY) -> Self {
        variant as u8 != 0
    }
}
///Field `RXONLY` reader - Receive only
pub type RXONLY_R = crate::BitReader<RXONLY>;
impl RXONLY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXONLY {
        match self.bits {
            false => RXONLY::FullDuplex,
            true => RXONLY::OutputDisabled,
        }
    }
    ///Full duplex (Transmit and receive)
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == RXONLY::FullDuplex
    }
    ///Output disabled (Receive-only mode)
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == RXONLY::OutputDisabled
    }
}
///Field `RXONLY` writer - Receive only
pub type RXONLY_W<'a, REG> = crate::BitWriter<'a, REG, RXONLY>;
impl<'a, REG> RXONLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Full duplex (Transmit and receive)
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut crate::W<REG> {
        self.variant(RXONLY::FullDuplex)
    }
    ///Output disabled (Receive-only mode)
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXONLY::OutputDisabled)
    }
}
///Field `CRCL` reader - CRC length
pub type CRCL_R = crate::BitReader;
///Field `CRCL` writer - CRC length
pub type CRCL_W<'a, REG> = crate::BitWriter<'a, REG>;
/**CRC transfer next

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCNEXT {
    ///0: Next transmit value is from Tx buffer
    TxBuffer = 0,
    ///1: Next transmit value is from Tx CRC register
    Crc = 1,
}
impl From<CRCNEXT> for bool {
    #[inline(always)]
    fn from(variant: CRCNEXT) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCNEXT` reader - CRC transfer next
pub type CRCNEXT_R = crate::BitReader<CRCNEXT>;
impl CRCNEXT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCNEXT {
        match self.bits {
            false => CRCNEXT::TxBuffer,
            true => CRCNEXT::Crc,
        }
    }
    ///Next transmit value is from Tx buffer
    #[inline(always)]
    pub fn is_tx_buffer(&self) -> bool {
        *self == CRCNEXT::TxBuffer
    }
    ///Next transmit value is from Tx CRC register
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == CRCNEXT::Crc
    }
}
///Field `CRCNEXT` writer - CRC transfer next
pub type CRCNEXT_W<'a, REG> = crate::BitWriter<'a, REG, CRCNEXT>;
impl<'a, REG> CRCNEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Next transmit value is from Tx buffer
    #[inline(always)]
    pub fn tx_buffer(self) -> &'a mut crate::W<REG> {
        self.variant(CRCNEXT::TxBuffer)
    }
    ///Next transmit value is from Tx CRC register
    #[inline(always)]
    pub fn crc(self) -> &'a mut crate::W<REG> {
        self.variant(CRCNEXT::Crc)
    }
}
/**Hardware CRC calculation enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN {
    ///0: CRC calculation disabled
    Disabled = 0,
    ///1: CRC calculation enabled
    Enabled = 1,
}
impl From<CRCEN> for bool {
    #[inline(always)]
    fn from(variant: CRCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCEN` reader - Hardware CRC calculation enable
pub type CRCEN_R = crate::BitReader<CRCEN>;
impl CRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN {
        match self.bits {
            false => CRCEN::Disabled,
            true => CRCEN::Enabled,
        }
    }
    ///CRC calculation disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN::Disabled
    }
    ///CRC calculation enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN::Enabled
    }
}
///Field `CRCEN` writer - Hardware CRC calculation enable
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC calculation disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Disabled)
    }
    ///CRC calculation enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::Enabled)
    }
}
/**Output enable in bidirectional mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIOE {
    ///0: Output disabled (receive-only mode)
    OutputDisabled = 0,
    ///1: Output enabled (transmit-only mode)
    OutputEnabled = 1,
}
impl From<BIDIOE> for bool {
    #[inline(always)]
    fn from(variant: BIDIOE) -> Self {
        variant as u8 != 0
    }
}
///Field `BIDIOE` reader - Output enable in bidirectional mode
pub type BIDIOE_R = crate::BitReader<BIDIOE>;
impl BIDIOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIDIOE {
        match self.bits {
            false => BIDIOE::OutputDisabled,
            true => BIDIOE::OutputEnabled,
        }
    }
    ///Output disabled (receive-only mode)
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == BIDIOE::OutputDisabled
    }
    ///Output enabled (transmit-only mode)
    #[inline(always)]
    pub fn is_output_enabled(&self) -> bool {
        *self == BIDIOE::OutputEnabled
    }
}
///Field `BIDIOE` writer - Output enable in bidirectional mode
pub type BIDIOE_W<'a, REG> = crate::BitWriter<'a, REG, BIDIOE>;
impl<'a, REG> BIDIOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output disabled (receive-only mode)
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIOE::OutputDisabled)
    }
    ///Output enabled (transmit-only mode)
    #[inline(always)]
    pub fn output_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIOE::OutputEnabled)
    }
}
/**Bidirectional data mode enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIMODE {
    ///0: 2-line unidirectional data mode selected
    Unidirectional = 0,
    ///1: 1-line bidirectional data mode selected
    Bidirectional = 1,
}
impl From<BIDIMODE> for bool {
    #[inline(always)]
    fn from(variant: BIDIMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `BIDIMODE` reader - Bidirectional data mode enable
pub type BIDIMODE_R = crate::BitReader<BIDIMODE>;
impl BIDIMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIDIMODE {
        match self.bits {
            false => BIDIMODE::Unidirectional,
            true => BIDIMODE::Bidirectional,
        }
    }
    ///2-line unidirectional data mode selected
    #[inline(always)]
    pub fn is_unidirectional(&self) -> bool {
        *self == BIDIMODE::Unidirectional
    }
    ///1-line bidirectional data mode selected
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BIDIMODE::Bidirectional
    }
}
///Field `BIDIMODE` writer - Bidirectional data mode enable
pub type BIDIMODE_W<'a, REG> = crate::BitWriter<'a, REG, BIDIMODE>;
impl<'a, REG> BIDIMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///2-line unidirectional data mode selected
    #[inline(always)]
    pub fn unidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIMODE::Unidirectional)
    }
    ///1-line bidirectional data mode selected
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIMODE::Bidirectional)
    }
}
impl R {
    ///Bit 0 - Clock phase
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock polarity
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master selection
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - Baud rate control
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - SPI enable
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Frame format
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Internal slave select
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software slave management
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Receive only
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CRC length
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CRC transfer next
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Hardware CRC calculation enable
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output enable in bidirectional mode
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Bidirectional data mode enable
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("bidimode", &self.bidimode())
            .field("bidioe", &self.bidioe())
            .field("crcen", &self.crcen())
            .field("crcnext", &self.crcnext())
            .field("crcl", &self.crcl())
            .field("rxonly", &self.rxonly())
            .field("ssm", &self.ssm())
            .field("ssi", &self.ssi())
            .field("lsbfirst", &self.lsbfirst())
            .field("spe", &self.spe())
            .field("br", &self.br())
            .field("mstr", &self.mstr())
            .field("cpol", &self.cpol())
            .field("cpha", &self.cpha())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock phase
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<'_, CR1rs> {
        CPHA_W::new(self, 0)
    }
    ///Bit 1 - Clock polarity
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<'_, CR1rs> {
        CPOL_W::new(self, 1)
    }
    ///Bit 2 - Master selection
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<'_, CR1rs> {
        MSTR_W::new(self, 2)
    }
    ///Bits 3:5 - Baud rate control
    #[inline(always)]
    pub fn br(&mut self) -> BR_W<'_, CR1rs> {
        BR_W::new(self, 3)
    }
    ///Bit 6 - SPI enable
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<'_, CR1rs> {
        SPE_W::new(self, 6)
    }
    ///Bit 7 - Frame format
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<'_, CR1rs> {
        LSBFIRST_W::new(self, 7)
    }
    ///Bit 8 - Internal slave select
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<'_, CR1rs> {
        SSI_W::new(self, 8)
    }
    ///Bit 9 - Software slave management
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W<'_, CR1rs> {
        SSM_W::new(self, 9)
    }
    ///Bit 10 - Receive only
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W<'_, CR1rs> {
        RXONLY_W::new(self, 10)
    }
    ///Bit 11 - CRC length
    #[inline(always)]
    pub fn crcl(&mut self) -> CRCL_W<'_, CR1rs> {
        CRCL_W::new(self, 11)
    }
    ///Bit 12 - CRC transfer next
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W<'_, CR1rs> {
        CRCNEXT_W::new(self, 12)
    }
    ///Bit 13 - Hardware CRC calculation enable
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, CR1rs> {
        CRCEN_W::new(self, 13)
    }
    ///Bit 14 - Output enable in bidirectional mode
    #[inline(always)]
    pub fn bidioe(&mut self) -> BIDIOE_W<'_, CR1rs> {
        BIDIOE_W::new(self, 14)
    }
    ///Bit 15 - Bidirectional data mode enable
    #[inline(always)]
    pub fn bidimode(&mut self) -> BIDIMODE_W<'_, CR1rs> {
        BIDIMODE_W::new(self, 15)
    }
}
/**control register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L562.html#SPI1:CR1)*/
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
