///Register `SPI_CR2` reader
pub type R = crate::R<SPI_CR2rs>;
///Register `SPI_CR2` writer
pub type W = crate::W<SPI_CR2rs>;
/**Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    ///0: Rx buffer DMA disabled
    B0x0 = 0,
    ///1: Rx buffer DMA enabled
    B0x1 = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAEN` reader - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::B0x0,
            true => RXDMAEN::B0x1,
        }
    }
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXDMAEN::B0x0
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXDMAEN::B0x1
    }
}
///Field `RXDMAEN` writer - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::B0x0)
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::B0x1)
    }
}
/**Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    ///0: Tx buffer DMA disabled
    B0x0 = 0,
    ///1: Tx buffer DMA enabled
    B0x1 = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAEN` reader - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::B0x0,
            true => TXDMAEN::B0x1,
        }
    }
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXDMAEN::B0x0
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXDMAEN::B0x1
    }
}
///Field `TXDMAEN` writer - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::B0x0)
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::B0x1)
    }
}
/**SS output enable Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE {
    ///0: SS output is disabled in master mode and the SPI interface can work in multimaster configuration
    B0x0 = 0,
    ///1: SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment.
    B0x1 = 1,
}
impl From<SSOE> for bool {
    #[inline(always)]
    fn from(variant: SSOE) -> Self {
        variant as u8 != 0
    }
}
///Field `SSOE` reader - SS output enable Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type SSOE_R = crate::BitReader<SSOE>;
impl SSOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSOE {
        match self.bits {
            false => SSOE::B0x0,
            true => SSOE::B0x1,
        }
    }
    ///SS output is disabled in master mode and the SPI interface can work in multimaster configuration
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == SSOE::B0x0
    }
    ///SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == SSOE::B0x1
    }
}
///Field `SSOE` writer - SS output enable Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG, SSOE>;
impl<'a, REG> SSOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SS output is disabled in master mode and the SPI interface can work in multimaster configuration
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::B0x0)
    }
    ///SS output is enabled in master mode and when the SPI interface is enabled. The SPI interface cannot work in a multimaster environment.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::B0x1)
    }
}
/**NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1 , or FRF = 1 . Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSSP {
    ///0: No NSS pulse
    B0x0 = 0,
    ///1: NSS pulse generated
    B0x1 = 1,
}
impl From<NSSP> for bool {
    #[inline(always)]
    fn from(variant: NSSP) -> Self {
        variant as u8 != 0
    }
}
///Field `NSSP` reader - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1 , or FRF = 1 . Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type NSSP_R = crate::BitReader<NSSP>;
impl NSSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NSSP {
        match self.bits {
            false => NSSP::B0x0,
            true => NSSP::B0x1,
        }
    }
    ///No NSS pulse
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == NSSP::B0x0
    }
    ///NSS pulse generated
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == NSSP::B0x1
    }
}
///Field `NSSP` writer - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1 , or FRF = 1 . Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
pub type NSSP_W<'a, REG> = crate::BitWriter<'a, REG, NSSP>;
impl<'a, REG> NSSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NSS pulse
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP::B0x0)
    }
    ///NSS pulse generated
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP::B0x1)
    }
}
/**Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRF {
    ///0: SPI Motorola mode
    B0x0 = 0,
}
impl From<FRF> for bool {
    #[inline(always)]
    fn from(variant: FRF) -> Self {
        variant as u8 != 0
    }
}
///Field `FRF` reader - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). Note: This bit is not used in I<sup>2</sup>S mode.
pub type FRF_R = crate::BitReader<FRF>;
impl FRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<FRF> {
        match self.bits {
            false => Some(FRF::B0x0),
            _ => None,
        }
    }
    ///SPI Motorola mode
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FRF::B0x0
    }
}
///Field `FRF` writer - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). Note: This bit is not used in I<sup>2</sup>S mode.
pub type FRF_W<'a, REG> = crate::BitWriter<'a, REG, FRF>;
impl<'a, REG> FRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI Motorola mode
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FRF::B0x0)
    }
}
/**Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I<sup>2</sup>S mode).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    ///0: Error interrupt is masked
    B0x0 = 0,
    ///1: Error interrupt is enabled
    B0x1 = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I<sup>2</sup>S mode).
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::B0x0,
            true => ERRIE::B0x1,
        }
    }
    ///Error interrupt is masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ERRIE::B0x0
    }
    ///Error interrupt is enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ERRIE::B0x1
    }
}
///Field `ERRIE` writer - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I<sup>2</sup>S mode).
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error interrupt is masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::B0x0)
    }
    ///Error interrupt is enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::B0x1)
    }
}
/**RX buffer not empty interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE {
    ///0: RXNE interrupt masked
    B0x0 = 0,
    ///1: RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set.
    B0x1 = 1,
}
impl From<RXNEIE> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `RXNEIE` reader - RX buffer not empty interrupt enable
pub type RXNEIE_R = crate::BitReader<RXNEIE>;
impl RXNEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXNEIE {
        match self.bits {
            false => RXNEIE::B0x0,
            true => RXNEIE::B0x1,
        }
    }
    ///RXNE interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RXNEIE::B0x0
    }
    ///RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RXNEIE::B0x1
    }
}
///Field `RXNEIE` writer - RX buffer not empty interrupt enable
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RXNE interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::B0x0)
    }
    ///RXNE interrupt not masked. Used to generate an interrupt request when the RXNE flag is set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::B0x1)
    }
}
/**Tx buffer empty interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE {
    ///0: TXE interrupt masked
    B0x0 = 0,
    ///1: TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set.
    B0x1 = 1,
}
impl From<TXEIE> for bool {
    #[inline(always)]
    fn from(variant: TXEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TXEIE` reader - Tx buffer empty interrupt enable
pub type TXEIE_R = crate::BitReader<TXEIE>;
impl TXEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXEIE {
        match self.bits {
            false => TXEIE::B0x0,
            true => TXEIE::B0x1,
        }
    }
    ///TXE interrupt masked
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TXEIE::B0x0
    }
    ///TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TXEIE::B0x1
    }
}
///Field `TXEIE` writer - Tx buffer empty interrupt enable
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TXE interrupt masked
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::B0x0)
    }
    ///TXE interrupt not masked. Used to generate an interrupt request when the TXE flag is set.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::B0x1)
    }
}
/**Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit) Note: These bits are not used in I<sup>2</sup>S mode.

Value on reset: 7*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DS {
    ///0: Not used
    B0x0 = 0,
    ///1: Not used
    B0x1 = 1,
    ///2: Not used
    B0x2 = 2,
    ///3: 4-bit
    B0x3 = 3,
    ///4: 5-bit
    B0x4 = 4,
    ///5: 6-bit
    B0x5 = 5,
    ///6: 7-bit
    B0x6 = 6,
    ///7: 8-bit
    B0x7 = 7,
    ///8: 9-bit
    B0x8 = 8,
    ///9: 10-bit
    B0x9 = 9,
    ///10: 11-bit
    B0xA = 10,
    ///11: 12-bit
    B0xB = 11,
    ///12: 13-bit
    B0xC = 12,
    ///13: 14-bit
    B0xD = 13,
    ///14: 15-bit
    B0xE = 14,
    ///15: 16-bit
    B0xF = 15,
}
impl From<DS> for u8 {
    #[inline(always)]
    fn from(variant: DS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DS {
    type Ux = u8;
}
impl crate::IsEnum for DS {}
///Field `DS` reader - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit) Note: These bits are not used in I<sup>2</sup>S mode.
pub type DS_R = crate::FieldReader<DS>;
impl DS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DS {
        match self.bits {
            0 => DS::B0x0,
            1 => DS::B0x1,
            2 => DS::B0x2,
            3 => DS::B0x3,
            4 => DS::B0x4,
            5 => DS::B0x5,
            6 => DS::B0x6,
            7 => DS::B0x7,
            8 => DS::B0x8,
            9 => DS::B0x9,
            10 => DS::B0xA,
            11 => DS::B0xB,
            12 => DS::B0xC,
            13 => DS::B0xD,
            14 => DS::B0xE,
            15 => DS::B0xF,
            _ => unreachable!(),
        }
    }
    ///Not used
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DS::B0x0
    }
    ///Not used
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DS::B0x1
    }
    ///Not used
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == DS::B0x2
    }
    ///4-bit
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == DS::B0x3
    }
    ///5-bit
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == DS::B0x4
    }
    ///6-bit
    #[inline(always)]
    pub fn is_b_0x5(&self) -> bool {
        *self == DS::B0x5
    }
    ///7-bit
    #[inline(always)]
    pub fn is_b_0x6(&self) -> bool {
        *self == DS::B0x6
    }
    ///8-bit
    #[inline(always)]
    pub fn is_b_0x7(&self) -> bool {
        *self == DS::B0x7
    }
    ///9-bit
    #[inline(always)]
    pub fn is_b_0x8(&self) -> bool {
        *self == DS::B0x8
    }
    ///10-bit
    #[inline(always)]
    pub fn is_b_0x9(&self) -> bool {
        *self == DS::B0x9
    }
    ///11-bit
    #[inline(always)]
    pub fn is_b_0x_a(&self) -> bool {
        *self == DS::B0xA
    }
    ///12-bit
    #[inline(always)]
    pub fn is_b_0x_b(&self) -> bool {
        *self == DS::B0xB
    }
    ///13-bit
    #[inline(always)]
    pub fn is_b_0x_c(&self) -> bool {
        *self == DS::B0xC
    }
    ///14-bit
    #[inline(always)]
    pub fn is_b_0x_d(&self) -> bool {
        *self == DS::B0xD
    }
    ///15-bit
    #[inline(always)]
    pub fn is_b_0x_e(&self) -> bool {
        *self == DS::B0xE
    }
    ///16-bit
    #[inline(always)]
    pub fn is_b_0x_f(&self) -> bool {
        *self == DS::B0xF
    }
}
///Field `DS` writer - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit) Note: These bits are not used in I<sup>2</sup>S mode.
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DS, crate::Safe>;
impl<'a, REG> DS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Not used
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x0)
    }
    ///Not used
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x1)
    }
    ///Not used
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x2)
    }
    ///4-bit
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x3)
    }
    ///5-bit
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x4)
    }
    ///6-bit
    #[inline(always)]
    pub fn b_0x5(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x5)
    }
    ///7-bit
    #[inline(always)]
    pub fn b_0x6(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x6)
    }
    ///8-bit
    #[inline(always)]
    pub fn b_0x7(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x7)
    }
    ///9-bit
    #[inline(always)]
    pub fn b_0x8(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x8)
    }
    ///10-bit
    #[inline(always)]
    pub fn b_0x9(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0x9)
    }
    ///11-bit
    #[inline(always)]
    pub fn b_0x_a(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0xA)
    }
    ///12-bit
    #[inline(always)]
    pub fn b_0x_b(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0xB)
    }
    ///13-bit
    #[inline(always)]
    pub fn b_0x_c(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0xC)
    }
    ///14-bit
    #[inline(always)]
    pub fn b_0x_d(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0xD)
    }
    ///15-bit
    #[inline(always)]
    pub fn b_0x_e(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0xE)
    }
    ///16-bit
    #[inline(always)]
    pub fn b_0x_f(self) -> &'a mut crate::W<REG> {
        self.variant(DS::B0xF)
    }
}
/**FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I<sup>2</sup>S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRXTH {
    ///0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)
    B0x0 = 0,
    ///1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)
    B0x1 = 1,
}
impl From<FRXTH> for bool {
    #[inline(always)]
    fn from(variant: FRXTH) -> Self {
        variant as u8 != 0
    }
}
///Field `FRXTH` reader - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I<sup>2</sup>S mode.
pub type FRXTH_R = crate::BitReader<FRXTH>;
impl FRXTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRXTH {
        match self.bits {
            false => FRXTH::B0x0,
            true => FRXTH::B0x1,
        }
    }
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == FRXTH::B0x0
    }
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == FRXTH::B0x1
    }
}
///Field `FRXTH` writer - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I<sup>2</sup>S mode.
pub type FRXTH_W<'a, REG> = crate::BitWriter<'a, REG, FRXTH>;
impl<'a, REG> FRXTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(FRXTH::B0x0)
    }
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(FRXTH::B0x1)
    }
}
/**Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_RX {
    ///0: Number of data to transfer is even
    B0x0 = 0,
    ///1: Number of data to transfer is odd
    B0x1 = 1,
}
impl From<LDMA_RX> for bool {
    #[inline(always)]
    fn from(variant: LDMA_RX) -> Self {
        variant as u8 != 0
    }
}
///Field `LDMA_RX` reader - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.
pub type LDMA_RX_R = crate::BitReader<LDMA_RX>;
impl LDMA_RX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LDMA_RX {
        match self.bits {
            false => LDMA_RX::B0x0,
            true => LDMA_RX::B0x1,
        }
    }
    ///Number of data to transfer is even
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LDMA_RX::B0x0
    }
    ///Number of data to transfer is odd
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LDMA_RX::B0x1
    }
}
///Field `LDMA_RX` writer - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.
pub type LDMA_RX_W<'a, REG> = crate::BitWriter<'a, REG, LDMA_RX>;
impl<'a, REG> LDMA_RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Number of data to transfer is even
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_RX::B0x0)
    }
    ///Number of data to transfer is odd
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_RX::B0x1)
    }
}
/**Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_TX {
    ///0: Number of data to transfer is even
    B0x0 = 0,
    ///1: Number of data to transfer is odd
    B0x1 = 1,
}
impl From<LDMA_TX> for bool {
    #[inline(always)]
    fn from(variant: LDMA_TX) -> Self {
        variant as u8 != 0
    }
}
///Field `LDMA_TX` reader - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.
pub type LDMA_TX_R = crate::BitReader<LDMA_TX>;
impl LDMA_TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LDMA_TX {
        match self.bits {
            false => LDMA_TX::B0x0,
            true => LDMA_TX::B0x1,
        }
    }
    ///Number of data to transfer is even
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LDMA_TX::B0x0
    }
    ///Number of data to transfer is odd
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LDMA_TX::B0x1
    }
}
///Field `LDMA_TX` writer - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.
pub type LDMA_TX_W<'a, REG> = crate::BitWriter<'a, REG, LDMA_TX>;
impl<'a, REG> LDMA_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Number of data to transfer is even
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_TX::B0x0)
    }
    ///Number of data to transfer is odd
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_TX::B0x1)
    }
}
impl R {
    ///Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SS output enable Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1 , or FRF = 1 . Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I<sup>2</sup>S mode).
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit) Note: These bits are not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_CR2")
            .field("rxdmaen", &self.rxdmaen())
            .field("txdmaen", &self.txdmaen())
            .field("ssoe", &self.ssoe())
            .field("nssp", &self.nssp())
            .field("frf", &self.frf())
            .field("errie", &self.errie())
            .field("rxneie", &self.rxneie())
            .field("txeie", &self.txeie())
            .field("ds", &self.ds())
            .field("frxth", &self.frxth())
            .field("ldma_rx", &self.ldma_rx())
            .field("ldma_tx", &self.ldma_tx())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx buffer DMA enable When this bit is set, a DMA request is generated whenever the RXNE flag is set.
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, SPI_CR2rs> {
        RXDMAEN_W::new(self, 0)
    }
    ///Bit 1 - Tx buffer DMA enable When this bit is set, a DMA request is generated whenever the TXE flag is set.
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, SPI_CR2rs> {
        TXDMAEN_W::new(self, 1)
    }
    ///Bit 2 - SS output enable Note: This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W<'_, SPI_CR2rs> {
        SSOE_W::new(self, 2)
    }
    ///Bit 3 - NSS pulse management This bit is used in master mode only. it allows the SPI to generate an NSS pulse between two consecutive data when doing continuous transfers. In the case of a single data transfer, it forces the NSS pin high level after the transfer. It has no meaning if CPHA = 1 , or FRF = 1 . Note: 1. This bit must be written only when the SPI is disabled (SPE=0). Note: 2. This bit is not used in I<sup>2</sup>S mode and SPI TI mode.
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W<'_, SPI_CR2rs> {
        NSSP_W::new(self, 3)
    }
    ///Bit 4 - Frame format 1 SPI TI mode Note: This bit must be written only when the SPI is disabled (SPE=0). Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W<'_, SPI_CR2rs> {
        FRF_W::new(self, 4)
    }
    ///Bit 5 - Error interrupt enable This bit controls the generation of an interrupt when an error condition occurs (CRCERR, OVR, MODF in SPI mode, FRE at TI mode and UDR, OVR, and FRE in I<sup>2</sup>S mode).
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, SPI_CR2rs> {
        ERRIE_W::new(self, 5)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<'_, SPI_CR2rs> {
        RXNEIE_W::new(self, 6)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<'_, SPI_CR2rs> {
        TXEIE_W::new(self, 7)
    }
    ///Bits 8:11 - Data size These bits configure the data length for SPI transfers. If software attempts to write one of the Not used values, they are forced to the value 0111 (8-bit) Note: These bits are not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W<'_, SPI_CR2rs> {
        DS_W::new(self, 8)
    }
    ///Bit 12 - FIFO reception threshold This bit is used to set the threshold of the RXFIFO that triggers an RXNE event Note: This bit is not used in I<sup>2</sup>S mode.
    #[inline(always)]
    pub fn frxth(&mut self) -> FRXTH_W<'_, SPI_CR2rs> {
        FRXTH_W::new(self, 12)
    }
    ///Bit 13 - Last DMA transfer for reception This bit is used in data packing mode, to define if the total number of data to receive by DMA is odd or even. It has significance only if the RXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.
    #[inline(always)]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W<'_, SPI_CR2rs> {
        LDMA_RX_W::new(self, 13)
    }
    ///Bit 14 - Last DMA transfer for transmission This bit is used in data packing mode, to define if the total number of data to transmit by DMA is odd or even. It has significance only if the TXDMAEN bit in the SPIx_CR2 register is set and if packing mode is used (data length =< 8-bit and write access to SPIx_DR is 16-bit wide). It has to be written when the SPI is disabled (SPE = 0 in the SPIx_CR1 register). Note: Refer to Procedure for disabling the SPI on page 789 if the CRCEN bit is set. Note: This bit is not used in I S mode.
    #[inline(always)]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W<'_, SPI_CR2rs> {
        LDMA_TX_W::new(self, 14)
    }
}
/**SPI control register 2

You can [`read`](crate::Reg::read) this register and get [`spi_cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#SPI1:SPI_CR2)*/
pub struct SPI_CR2rs;
impl crate::RegisterSpec for SPI_CR2rs {
    type Ux = u16;
}
///`read()` method returns [`spi_cr2::R`](R) reader structure
impl crate::Readable for SPI_CR2rs {}
///`write(|w| ..)` method takes [`spi_cr2::W`](W) writer structure
impl crate::Writable for SPI_CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SPI_CR2 to value 0x0700
impl crate::Resettable for SPI_CR2rs {
    const RESET_VALUE: u16 = 0x0700;
}
