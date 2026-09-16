///Register `SPI_CR1` reader
pub type R = crate::R<SPI_CR1rs>;
///Register `SPI_CR1` writer
pub type W = crate::W<SPI_CR1rs>;
/**Clock phase Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA {
    ///0: The first clock transition is the first data capture edge
    B0x0 = 0,
    ///1: The second clock transition is the first data capture edge
    B0x1 = 1,
}
impl From<CPHA> for bool {
    #[inline(always)]
    fn from(variant: CPHA) -> Self {
        variant as u8 != 0
    }
}
///Field `CPHA` reader - Clock phase Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.
pub type CPHA_R = crate::BitReader<CPHA>;
impl CPHA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPHA {
        match self.bits {
            false => CPHA::B0x0,
            true => CPHA::B0x1,
        }
    }
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CPHA::B0x0
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CPHA::B0x1
    }
}
///Field `CPHA` writer - Clock phase Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.
pub type CPHA_W<'a, REG> = crate::BitWriter<'a, REG, CPHA>;
impl<'a, REG> CPHA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The first clock transition is the first data capture edge
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::B0x0)
    }
    ///The second clock transition is the first data capture edge
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CPHA::B0x1)
    }
}
/**Clock polarity Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL {
    ///0: CK to 0 when idle
    B0x0 = 0,
    ///1: CK to 1 when idle
    B0x1 = 1,
}
impl From<CPOL> for bool {
    #[inline(always)]
    fn from(variant: CPOL) -> Self {
        variant as u8 != 0
    }
}
///Field `CPOL` reader - Clock polarity Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.
pub type CPOL_R = crate::BitReader<CPOL>;
impl CPOL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CPOL {
        match self.bits {
            false => CPOL::B0x0,
            true => CPOL::B0x1,
        }
    }
    ///CK to 0 when idle
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CPOL::B0x0
    }
    ///CK to 1 when idle
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CPOL::B0x1
    }
}
///Field `CPOL` writer - Clock polarity Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.
pub type CPOL_W<'a, REG> = crate::BitWriter<'a, REG, CPOL>;
impl<'a, REG> CPOL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CK to 0 when idle
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::B0x0)
    }
    ///CK to 1 when idle
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CPOL::B0x1)
    }
}
/**Master selection Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR {
    ///0: Slave configuration
    B0x0 = 0,
    ///1: Master configuration
    B0x1 = 1,
}
impl From<MSTR> for bool {
    #[inline(always)]
    fn from(variant: MSTR) -> Self {
        variant as u8 != 0
    }
}
///Field `MSTR` reader - Master selection Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode.
pub type MSTR_R = crate::BitReader<MSTR>;
impl MSTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MSTR {
        match self.bits {
            false => MSTR::B0x0,
            true => MSTR::B0x1,
        }
    }
    ///Slave configuration
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSTR::B0x0
    }
    ///Master configuration
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSTR::B0x1
    }
}
///Field `MSTR` writer - Master selection Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode.
pub type MSTR_W<'a, REG> = crate::BitWriter<'a, REG, MSTR>;
impl<'a, REG> MSTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Slave configuration
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR::B0x0)
    }
    ///Master configuration
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSTR::B0x1)
    }
}
/**Baud rate control Note: These bits should not be changed when communication is ongoing. Note: These bits are not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BR {
    ///0: f<sub>PCLK</sub>/2
    B0x0 = 0,
    ///1: f<sub>PCLK</sub>/4
    B0x1 = 1,
    ///2: f<sub>PCLK</sub>/8
    B0x2 = 2,
    ///3: f<sub>PCLK</sub>/16
    B0x3 = 3,
    ///4: f<sub>PCLK</sub>/32
    B0x4 = 4,
    ///5: f<sub>PCLK</sub>/64
    B0x5 = 5,
    ///6: f<sub>PCLK</sub>/128
    B0x6 = 6,
    ///7: f<sub>PCLK</sub>/256
    B0x7 = 7,
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
///Field `BR` reader - Baud rate control Note: These bits should not be changed when communication is ongoing. Note: These bits are not used in I<sup>2</sup>S mode.
pub type BR_R = crate::FieldReader<BR>;
impl BR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BR {
        match self.bits {
            0 => BR::B0x0,
            1 => BR::B0x1,
            2 => BR::B0x2,
            3 => BR::B0x3,
            4 => BR::B0x4,
            5 => BR::B0x5,
            6 => BR::B0x6,
            7 => BR::B0x7,
            _ => unreachable!(),
        }
    }
    ///f<sub>PCLK</sub>/2
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BR::B0x0
    }
    ///f<sub>PCLK</sub>/4
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BR::B0x1
    }
    ///f<sub>PCLK</sub>/8
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == BR::B0x2
    }
    ///f<sub>PCLK</sub>/16
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == BR::B0x3
    }
    ///f<sub>PCLK</sub>/32
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == BR::B0x4
    }
    ///f<sub>PCLK</sub>/64
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == BR::B0x5
    }
    ///f<sub>PCLK</sub>/128
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == BR::B0x6
    }
    ///f<sub>PCLK</sub>/256
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == BR::B0x7
    }
}
///Field `BR` writer - Baud rate control Note: These bits should not be changed when communication is ongoing. Note: These bits are not used in I<sup>2</sup>S mode.
pub type BR_W<'a, REG> = crate::FieldWriter<'a, REG, 3, BR, crate::Safe>;
impl<'a, REG> BR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///f<sub>PCLK</sub>/2
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BR::B0x0)
    }
    ///f<sub>PCLK</sub>/4
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BR::B0x1)
    }
    ///f<sub>PCLK</sub>/8
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(BR::B0x2)
    }
    ///f<sub>PCLK</sub>/16
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(BR::B0x3)
    }
    ///f<sub>PCLK</sub>/32
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(BR::B0x4)
    }
    ///f<sub>PCLK</sub>/64
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(BR::B0x5)
    }
    ///f<sub>PCLK</sub>/128
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(BR::B0x6)
    }
    ///f<sub>PCLK</sub>/256
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(BR::B0x7)
    }
}
/**SPI enable Note: When disabling the SPI, follow the procedure described in Procedure for disabling the SPI on page 789. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE {
    ///0: Peripheral disabled
    B0x0 = 0,
    ///1: Peripheral enabled
    B0x1 = 1,
}
impl From<SPE> for bool {
    #[inline(always)]
    fn from(variant: SPE) -> Self {
        variant as u8 != 0
    }
}
///Field `SPE` reader - SPI enable Note: When disabling the SPI, follow the procedure described in Procedure for disabling the SPI on page 789. Note: This bit is not used in I<sup>2</sup>S mode.
pub type SPE_R = crate::BitReader<SPE>;
impl SPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SPE {
        match self.bits {
            false => SPE::B0x0,
            true => SPE::B0x1,
        }
    }
    ///Peripheral disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SPE::B0x0
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SPE::B0x1
    }
}
///Field `SPE` writer - SPI enable Note: When disabling the SPI, follow the procedure described in Procedure for disabling the SPI on page 789. Note: This bit is not used in I<sup>2</sup>S mode.
pub type SPE_W<'a, REG> = crate::BitWriter<'a, REG, SPE>;
impl<'a, REG> SPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SPE::B0x0)
    }
    ///Peripheral enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SPE::B0x1)
    }
}
/**Frame format Note: 1. This bit should not be changed when communication is ongoing. Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFIRST {
    ///0: data is transmitted / received with the MSB first
    B0x0 = 0,
    ///1: data is transmitted / received with the LSB first
    B0x1 = 1,
}
impl From<LSBFIRST> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST) -> Self {
        variant as u8 != 0
    }
}
///Field `LSBFIRST` reader - Frame format Note: 1. This bit should not be changed when communication is ongoing. Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type LSBFIRST_R = crate::BitReader<LSBFIRST>;
impl LSBFIRST_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LSBFIRST {
        match self.bits {
            false => LSBFIRST::B0x0,
            true => LSBFIRST::B0x1,
        }
    }
    ///data is transmitted / received with the MSB first
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LSBFIRST::B0x0
    }
    ///data is transmitted / received with the LSB first
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LSBFIRST::B0x1
    }
}
///Field `LSBFIRST` writer - Frame format Note: 1. This bit should not be changed when communication is ongoing. Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type LSBFIRST_W<'a, REG> = crate::BitWriter<'a, REG, LSBFIRST>;
impl<'a, REG> LSBFIRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///data is transmitted / received with the MSB first
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST::B0x0)
    }
    ///data is transmitted / received with the LSB first
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LSBFIRST::B0x1)
    }
}
///Field `SSI` reader - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type SSI_R = crate::BitReader;
///Field `SSI` writer - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type SSI_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM {
    ///0: Software slave management disabled
    B0x0 = 0,
    ///1: Software slave management enabled
    B0x1 = 1,
}
impl From<SSM> for bool {
    #[inline(always)]
    fn from(variant: SSM) -> Self {
        variant as u8 != 0
    }
}
///Field `SSM` reader - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type SSM_R = crate::BitReader<SSM>;
impl SSM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSM {
        match self.bits {
            false => SSM::B0x0,
            true => SSM::B0x1,
        }
    }
    ///Software slave management disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SSM::B0x0
    }
    ///Software slave management enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SSM::B0x1
    }
}
///Field `SSM` writer - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type SSM_W<'a, REG> = crate::BitWriter<'a, REG, SSM>;
impl<'a, REG> SSM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Software slave management disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SSM::B0x0)
    }
    ///Software slave management enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SSM::B0x1)
    }
}
/**Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXONLY {
    ///0: Full-duplex (Transmit and receive)
    B0x0 = 0,
    ///1: Output disabled (Receive-only mode)
    B0x1 = 1,
}
impl From<RXONLY> for bool {
    #[inline(always)]
    fn from(variant: RXONLY) -> Self {
        variant as u8 != 0
    }
}
///Field `RXONLY` reader - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I<sup>2</sup>S mode.
pub type RXONLY_R = crate::BitReader<RXONLY>;
impl RXONLY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXONLY {
        match self.bits {
            false => RXONLY::B0x0,
            true => RXONLY::B0x1,
        }
    }
    ///Full-duplex (Transmit and receive)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXONLY::B0x0
    }
    ///Output disabled (Receive-only mode)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXONLY::B0x1
    }
}
///Field `RXONLY` writer - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I<sup>2</sup>S mode.
pub type RXONLY_W<'a, REG> = crate::BitWriter<'a, REG, RXONLY>;
impl<'a, REG> RXONLY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Full-duplex (Transmit and receive)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXONLY::B0x0)
    }
    ///Output disabled (Receive-only mode)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXONLY::B0x1)
    }
}
/**CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCL {
    ///0: 8-bit CRC length
    B0x0 = 0,
    ///1: 16-bit CRC length
    B0x1 = 1,
}
impl From<CRCL> for bool {
    #[inline(always)]
    fn from(variant: CRCL) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCL` reader - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.
pub type CRCL_R = crate::BitReader<CRCL>;
impl CRCL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCL {
        match self.bits {
            false => CRCL::B0x0,
            true => CRCL::B0x1,
        }
    }
    ///8-bit CRC length
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCL::B0x0
    }
    ///16-bit CRC length
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCL::B0x1
    }
}
///Field `CRCL` writer - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.
pub type CRCL_W<'a, REG> = crate::BitWriter<'a, REG, CRCL>;
impl<'a, REG> CRCL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///8-bit CRC length
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCL::B0x0)
    }
    ///16-bit CRC length
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCL::B0x1)
    }
}
/**Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCNEXT {
    ///0: Next transmit value is from Tx buffer.
    B0x0 = 0,
    ///1: Next transmit value is from Tx CRC register.
    B0x1 = 1,
}
impl From<CRCNEXT> for bool {
    #[inline(always)]
    fn from(variant: CRCNEXT) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCNEXT` reader - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. Note: This bit is not used in I<sup>2</sup>S mode.
pub type CRCNEXT_R = crate::BitReader<CRCNEXT>;
impl CRCNEXT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCNEXT {
        match self.bits {
            false => CRCNEXT::B0x0,
            true => CRCNEXT::B0x1,
        }
    }
    ///Next transmit value is from Tx buffer.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCNEXT::B0x0
    }
    ///Next transmit value is from Tx CRC register.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCNEXT::B0x1
    }
}
///Field `CRCNEXT` writer - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. Note: This bit is not used in I<sup>2</sup>S mode.
pub type CRCNEXT_W<'a, REG> = crate::BitWriter<'a, REG, CRCNEXT>;
impl<'a, REG> CRCNEXT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Next transmit value is from Tx buffer.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCNEXT::B0x0)
    }
    ///Next transmit value is from Tx CRC register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCNEXT::B0x1)
    }
}
/**Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN {
    ///0: CRC calculation disabled
    B0x0 = 0,
    ///1: CRC calculation enabled
    B0x1 = 1,
}
impl From<CRCEN> for bool {
    #[inline(always)]
    fn from(variant: CRCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCEN` reader - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.
pub type CRCEN_R = crate::BitReader<CRCEN>;
impl CRCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCEN {
        match self.bits {
            false => CRCEN::B0x0,
            true => CRCEN::B0x1,
        }
    }
    ///CRC calculation disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCEN::B0x0
    }
    ///CRC calculation enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCEN::B0x1
    }
}
///Field `CRCEN` writer - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.
pub type CRCEN_W<'a, REG> = crate::BitWriter<'a, REG, CRCEN>;
impl<'a, REG> CRCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CRC calculation disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::B0x0)
    }
    ///CRC calculation enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEN::B0x1)
    }
}
/**Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIOE {
    ///0: Output disabled (receive-only mode)
    B0x0 = 0,
    ///1: Output enabled (transmit-only mode)
    B0x1 = 1,
}
impl From<BIDIOE> for bool {
    #[inline(always)]
    fn from(variant: BIDIOE) -> Self {
        variant as u8 != 0
    }
}
///Field `BIDIOE` reader - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. Note: This bit is not used in I<sup>2</sup>S mode.
pub type BIDIOE_R = crate::BitReader<BIDIOE>;
impl BIDIOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIDIOE {
        match self.bits {
            false => BIDIOE::B0x0,
            true => BIDIOE::B0x1,
        }
    }
    ///Output disabled (receive-only mode)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BIDIOE::B0x0
    }
    ///Output enabled (transmit-only mode)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BIDIOE::B0x1
    }
}
///Field `BIDIOE` writer - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. Note: This bit is not used in I<sup>2</sup>S mode.
pub type BIDIOE_W<'a, REG> = crate::BitWriter<'a, REG, BIDIOE>;
impl<'a, REG> BIDIOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output disabled (receive-only mode)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIOE::B0x0)
    }
    ///Output enabled (transmit-only mode)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIOE::B0x1)
    }
}
/**Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIMODE {
    ///0: 2-line unidirectional data mode selected
    B0x0 = 0,
    ///1: 1-line bidirectional data mode selected
    B0x1 = 1,
}
impl From<BIDIMODE> for bool {
    #[inline(always)]
    fn from(variant: BIDIMODE) -> Self {
        variant as u8 != 0
    }
}
///Field `BIDIMODE` reader - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I<sup>2</sup>S mode.
pub type BIDIMODE_R = crate::BitReader<BIDIMODE>;
impl BIDIMODE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIDIMODE {
        match self.bits {
            false => BIDIMODE::B0x0,
            true => BIDIMODE::B0x1,
        }
    }
    ///2-line unidirectional data mode selected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BIDIMODE::B0x0
    }
    ///1-line bidirectional data mode selected
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BIDIMODE::B0x1
    }
}
///Field `BIDIMODE` writer - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I<sup>2</sup>S mode.
pub type BIDIMODE_W<'a, REG> = crate::BitWriter<'a, REG, BIDIMODE>;
impl<'a, REG> BIDIMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///2-line unidirectional data mode selected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIMODE::B0x0)
    }
    ///1-line bidirectional data mode selected
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BIDIMODE::B0x1)
    }
}
impl R {
    ///Bit 0 - Clock phase Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clock polarity Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Master selection Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:5 - Baud rate control Note: These bits should not be changed when communication is ongoing. Note: These bits are not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    ///Bit 6 - SPI enable Note: When disabling the SPI, follow the procedure described in Procedure for disabling the SPI on page 789. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Frame format Note: 1. This bit should not be changed when communication is ongoing. Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CR1")
            .field("cpha", &self.cpha())
            .field("cpol", &self.cpol())
            .field("mstr", &self.mstr())
            .field("br", &self.br())
            .field("spe", &self.spe())
            .field("lsbfirst", &self.lsbfirst())
            .field("ssi", &self.ssi())
            .field("ssm", &self.ssm())
            .field("rxonly", &self.rxonly())
            .field("crcl", &self.crcl())
            .field("crcnext", &self.crcnext())
            .field("crcen", &self.crcen())
            .field("bidioe", &self.bidioe())
            .field("bidimode", &self.bidimode())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clock phase Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.
    #[inline(always)]
    pub fn cpha(&mut self) -> CPHA_W<'_, SPI_CR1rs> {
        CPHA_W::new(self, 0)
    }
    ///Bit 1 - Clock polarity Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode except the case when CRC is applied at TI mode.
    #[inline(always)]
    pub fn cpol(&mut self) -> CPOL_W<'_, SPI_CR1rs> {
        CPOL_W::new(self, 1)
    }
    ///Bit 2 - Master selection Note: This bit should not be changed when communication is ongoing. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn mstr(&mut self) -> MSTR_W<'_, SPI_CR1rs> {
        MSTR_W::new(self, 2)
    }
    ///Bits 3:5 - Baud rate control Note: These bits should not be changed when communication is ongoing. Note: These bits are not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn br(&mut self) -> BR_W<'_, SPI_CR1rs> {
        BR_W::new(self, 3)
    }
    ///Bit 6 - SPI enable Note: When disabling the SPI, follow the procedure described in Procedure for disabling the SPI on page 789. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn spe(&mut self) -> SPE_W<'_, SPI_CR1rs> {
        SPE_W::new(self, 6)
    }
    ///Bit 7 - Frame format Note: 1. This bit should not be changed when communication is ongoing. Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<'_, SPI_CR1rs> {
        LSBFIRST_W::new(self, 7)
    }
    ///Bit 8 - Internal slave select This bit has an effect only when the SSM bit is set. The value of this bit is forced onto the NSS pin and the I/O value of the NSS pin is ignored. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssi(&mut self) -> SSI_W<'_, SPI_CR1rs> {
        SSI_W::new(self, 8)
    }
    ///Bit 9 - Software slave management When the SSM bit is set, the NSS pin input is replaced with the value from the SSI bit. Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssm(&mut self) -> SSM_W<'_, SPI_CR1rs> {
        SSM_W::new(self, 9)
    }
    ///Bit 10 - Receive only mode enabled. This bit enables simplex communication using a single unidirectional line to receive data exclusively. Keep BIDIMODE bit clear when receive only mode is active.This bit is also useful in a multislave system in which this particular slave is not accessed, the output from the accessed slave is not corrupted. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn rxonly(&mut self) -> RXONLY_W<'_, SPI_CR1rs> {
        RXONLY_W::new(self, 10)
    }
    ///Bit 11 - CRC length This bit is set and cleared by software to select the CRC length. Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn crcl(&mut self) -> CRCL_W<'_, SPI_CR1rs> {
        CRCL_W::new(self, 11)
    }
    ///Bit 12 - Transmit CRC next Note: This bit has to be written as soon as the last data is written in the SPIx_DR register. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn crcnext(&mut self) -> CRCNEXT_W<'_, SPI_CR1rs> {
        CRCNEXT_W::new(self, 12)
    }
    ///Bit 13 - Hardware CRC calculation enable Note: This bit should be written only when SPI is disabled (SPE = 0 ) for correct operation. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn crcen(&mut self) -> CRCEN_W<'_, SPI_CR1rs> {
        CRCEN_W::new(self, 13)
    }
    ///Bit 14 - Output enable in bidirectional mode This bit combined with the BIDIMODE bit selects the direction of transfer in bidirectional mode. Note: In master mode, the MOSI pin is used and in slave mode, the MISO pin is used. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn bidioe(&mut self) -> BIDIOE_W<'_, SPI_CR1rs> {
        BIDIOE_W::new(self, 14)
    }
    ///Bit 15 - Bidirectional data mode enable. This bit enables half-duplex communication using common single bidirectional data line. Keep RXONLY bit clear when bidirectional mode is active. Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn bidimode(&mut self) -> BIDIMODE_W<'_, SPI_CR1rs> {
        BIDIMODE_W::new(self, 15)
    }
}
/**SPI control register 1

You can [`read`](crate::Reg::read) this register and get [`spi_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#SPI1:SPI_CR1)*/
pub struct SPI_CR1rs;
impl crate::RegisterSpec for SPI_CR1rs {
    type Ux = u16;
}
///`read()` method returns [`spi_cr1::R`](R) reader structure
impl crate::Readable for SPI_CR1rs {}
///`write(|w| ..)` method takes [`spi_cr1::W`](W) writer structure
impl crate::Writable for SPI_CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_CR1 to value 0
impl crate::Resettable for SPI_CR1rs {}
