///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**Rx buffer DMA enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    ///0: Rx buffer DMA disabled
    Disabled = 0,
    ///1: Rx buffer DMA enabled
    Enabled = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `RXDMAEN` reader - Rx buffer DMA enable
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::Disabled,
            true => RXDMAEN::Enabled,
        }
    }
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN::Disabled
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN::Enabled
    }
}
///Field `RXDMAEN` writer - Rx buffer DMA enable
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Disabled)
    }
    ///Rx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Enabled)
    }
}
/**Tx buffer DMA enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    ///0: Tx buffer DMA disabled
    Disabled = 0,
    ///1: Tx buffer DMA enabled
    Enabled = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
///Field `TXDMAEN` reader - Tx buffer DMA enable
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::Disabled,
            true => TXDMAEN::Enabled,
        }
    }
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN::Disabled
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN::Enabled
    }
}
///Field `TXDMAEN` writer - Tx buffer DMA enable
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx buffer DMA disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Disabled)
    }
    ///Tx buffer DMA enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Enabled)
    }
}
/**SS output enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE {
    ///0: SS output is disabled in master mode
    Disabled = 0,
    ///1: SS output is enabled in master mode
    Enabled = 1,
}
impl From<SSOE> for bool {
    #[inline(always)]
    fn from(variant: SSOE) -> Self {
        variant as u8 != 0
    }
}
///Field `SSOE` reader - SS output enable
pub type SSOE_R = crate::BitReader<SSOE>;
impl SSOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SSOE {
        match self.bits {
            false => SSOE::Disabled,
            true => SSOE::Enabled,
        }
    }
    ///SS output is disabled in master mode
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE::Disabled
    }
    ///SS output is enabled in master mode
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE::Enabled
    }
}
///Field `SSOE` writer - SS output enable
pub type SSOE_W<'a, REG> = crate::BitWriter<'a, REG, SSOE>;
impl<'a, REG> SSOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SS output is disabled in master mode
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Disabled)
    }
    ///SS output is enabled in master mode
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(SSOE::Enabled)
    }
}
/**NSS pulse management

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSSP {
    ///0: No NSS pulse
    NoPulse = 0,
    ///1: NSS pulse generated
    PulseGenerated = 1,
}
impl From<NSSP> for bool {
    #[inline(always)]
    fn from(variant: NSSP) -> Self {
        variant as u8 != 0
    }
}
///Field `NSSP` reader - NSS pulse management
pub type NSSP_R = crate::BitReader<NSSP>;
impl NSSP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> NSSP {
        match self.bits {
            false => NSSP::NoPulse,
            true => NSSP::PulseGenerated,
        }
    }
    ///No NSS pulse
    #[inline(always)]
    pub fn is_no_pulse(&self) -> bool {
        *self == NSSP::NoPulse
    }
    ///NSS pulse generated
    #[inline(always)]
    pub fn is_pulse_generated(&self) -> bool {
        *self == NSSP::PulseGenerated
    }
}
///Field `NSSP` writer - NSS pulse management
pub type NSSP_W<'a, REG> = crate::BitWriter<'a, REG, NSSP>;
impl<'a, REG> NSSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No NSS pulse
    #[inline(always)]
    pub fn no_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP::NoPulse)
    }
    ///NSS pulse generated
    #[inline(always)]
    pub fn pulse_generated(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP::PulseGenerated)
    }
}
/**Frame format

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRF {
    ///0: SPI Motorola mode
    Motorola = 0,
    ///1: SPI TI mode
    Ti = 1,
}
impl From<FRF> for bool {
    #[inline(always)]
    fn from(variant: FRF) -> Self {
        variant as u8 != 0
    }
}
///Field `FRF` reader - Frame format
pub type FRF_R = crate::BitReader<FRF>;
impl FRF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRF {
        match self.bits {
            false => FRF::Motorola,
            true => FRF::Ti,
        }
    }
    ///SPI Motorola mode
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == FRF::Motorola
    }
    ///SPI TI mode
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF::Ti
    }
}
///Field `FRF` writer - Frame format
pub type FRF_W<'a, REG> = crate::BitWriter<'a, REG, FRF>;
impl<'a, REG> FRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///SPI Motorola mode
    #[inline(always)]
    pub fn motorola(self) -> &'a mut crate::W<REG> {
        self.variant(FRF::Motorola)
    }
    ///SPI TI mode
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(FRF::Ti)
    }
}
/**Error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    ///0: Error interrupt masked
    Masked = 0,
    ///1: Error interrupt not masked
    NotMasked = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ERRIE` reader - Error interrupt enable
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Masked,
            true => ERRIE::NotMasked,
        }
    }
    ///Error interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ERRIE::Masked
    }
    ///Error interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIE::NotMasked
    }
}
///Field `ERRIE` writer - Error interrupt enable
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Masked)
    }
    ///Error interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::NotMasked)
    }
}
/**RX buffer not empty interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE {
    ///0: RXE interrupt masked
    Masked = 0,
    ///1: RXE interrupt not masked
    NotMasked = 1,
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
            false => RXNEIE::Masked,
            true => RXNEIE::NotMasked,
        }
    }
    ///RXE interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIE::Masked
    }
    ///RXE interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIE::NotMasked
    }
}
///Field `RXNEIE` writer - RX buffer not empty interrupt enable
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RXE interrupt masked
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Masked)
    }
    ///RXE interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::NotMasked)
    }
}
/**Tx buffer empty interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE {
    ///0: TXE interrupt masked
    Masked = 0,
    ///1: TXE interrupt not masked
    NotMasked = 1,
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
            false => TXEIE::Masked,
            true => TXEIE::NotMasked,
        }
    }
    ///TXE interrupt masked
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXEIE::Masked
    }
    ///TXE interrupt not masked
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIE::NotMasked
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
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Masked)
    }
    ///TXE interrupt not masked
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::NotMasked)
    }
}
/**Data size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DS {
    ///3: 4-bit
    FourBit = 3,
    ///4: 5-bit
    FiveBit = 4,
    ///5: 6-bit
    SixBit = 5,
    ///6: 7-bit
    SevenBit = 6,
    ///7: 8-bit
    EightBit = 7,
    ///8: 9-bit
    NineBit = 8,
    ///9: 10-bit
    TenBit = 9,
    ///10: 11-bit
    ElevenBit = 10,
    ///11: 12-bit
    TwelveBit = 11,
    ///12: 13-bit
    ThirteenBit = 12,
    ///13: 14-bit
    FourteenBit = 13,
    ///14: 15-bit
    FifteenBit = 14,
    ///15: 16-bit
    SixteenBit = 15,
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
///Field `DS` reader - Data size
pub type DS_R = crate::FieldReader<DS>;
impl DS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DS> {
        match self.bits {
            3 => Some(DS::FourBit),
            4 => Some(DS::FiveBit),
            5 => Some(DS::SixBit),
            6 => Some(DS::SevenBit),
            7 => Some(DS::EightBit),
            8 => Some(DS::NineBit),
            9 => Some(DS::TenBit),
            10 => Some(DS::ElevenBit),
            11 => Some(DS::TwelveBit),
            12 => Some(DS::ThirteenBit),
            13 => Some(DS::FourteenBit),
            14 => Some(DS::FifteenBit),
            15 => Some(DS::SixteenBit),
            _ => None,
        }
    }
    ///4-bit
    #[inline(always)]
    pub fn is_four_bit(&self) -> bool {
        *self == DS::FourBit
    }
    ///5-bit
    #[inline(always)]
    pub fn is_five_bit(&self) -> bool {
        *self == DS::FiveBit
    }
    ///6-bit
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == DS::SixBit
    }
    ///7-bit
    #[inline(always)]
    pub fn is_seven_bit(&self) -> bool {
        *self == DS::SevenBit
    }
    ///8-bit
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == DS::EightBit
    }
    ///9-bit
    #[inline(always)]
    pub fn is_nine_bit(&self) -> bool {
        *self == DS::NineBit
    }
    ///10-bit
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == DS::TenBit
    }
    ///11-bit
    #[inline(always)]
    pub fn is_eleven_bit(&self) -> bool {
        *self == DS::ElevenBit
    }
    ///12-bit
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == DS::TwelveBit
    }
    ///13-bit
    #[inline(always)]
    pub fn is_thirteen_bit(&self) -> bool {
        *self == DS::ThirteenBit
    }
    ///14-bit
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == DS::FourteenBit
    }
    ///15-bit
    #[inline(always)]
    pub fn is_fifteen_bit(&self) -> bool {
        *self == DS::FifteenBit
    }
    ///16-bit
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DS::SixteenBit
    }
}
///Field `DS` writer - Data size
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DS>;
impl<'a, REG> DS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///4-bit
    #[inline(always)]
    pub fn four_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::FourBit)
    }
    ///5-bit
    #[inline(always)]
    pub fn five_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::FiveBit)
    }
    ///6-bit
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::SixBit)
    }
    ///7-bit
    #[inline(always)]
    pub fn seven_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::SevenBit)
    }
    ///8-bit
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::EightBit)
    }
    ///9-bit
    #[inline(always)]
    pub fn nine_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::NineBit)
    }
    ///10-bit
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::TenBit)
    }
    ///11-bit
    #[inline(always)]
    pub fn eleven_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::ElevenBit)
    }
    ///12-bit
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::TwelveBit)
    }
    ///13-bit
    #[inline(always)]
    pub fn thirteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::ThirteenBit)
    }
    ///14-bit
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::FourteenBit)
    }
    ///15-bit
    #[inline(always)]
    pub fn fifteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::FifteenBit)
    }
    ///16-bit
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::SixteenBit)
    }
}
/**FIFO reception threshold

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRXTH {
    ///0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)
    Half = 0,
    ///1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)
    Quarter = 1,
}
impl From<FRXTH> for bool {
    #[inline(always)]
    fn from(variant: FRXTH) -> Self {
        variant as u8 != 0
    }
}
///Field `FRXTH` reader - FIFO reception threshold
pub type FRXTH_R = crate::BitReader<FRXTH>;
impl FRXTH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FRXTH {
        match self.bits {
            false => FRXTH::Half,
            true => FRXTH::Quarter,
        }
    }
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FRXTH::Half
    }
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FRXTH::Quarter
    }
}
///Field `FRXTH` writer - FIFO reception threshold
pub type FRXTH_W<'a, REG> = crate::BitWriter<'a, REG, FRXTH>;
impl<'a, REG> FRXTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(FRXTH::Half)
    }
    ///RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(FRXTH::Quarter)
    }
}
/**Last DMA transfer for reception

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_RX {
    ///0: Number of data to transfer for receive is even
    Even = 0,
    ///1: Number of data to transfer for receive is odd
    Odd = 1,
}
impl From<LDMA_RX> for bool {
    #[inline(always)]
    fn from(variant: LDMA_RX) -> Self {
        variant as u8 != 0
    }
}
///Field `LDMA_RX` reader - Last DMA transfer for reception
pub type LDMA_RX_R = crate::BitReader<LDMA_RX>;
impl LDMA_RX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LDMA_RX {
        match self.bits {
            false => LDMA_RX::Even,
            true => LDMA_RX::Odd,
        }
    }
    ///Number of data to transfer for receive is even
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_RX::Even
    }
    ///Number of data to transfer for receive is odd
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_RX::Odd
    }
}
///Field `LDMA_RX` writer - Last DMA transfer for reception
pub type LDMA_RX_W<'a, REG> = crate::BitWriter<'a, REG, LDMA_RX>;
impl<'a, REG> LDMA_RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Number of data to transfer for receive is even
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_RX::Even)
    }
    ///Number of data to transfer for receive is odd
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_RX::Odd)
    }
}
/**Last DMA transfer for transmission

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_TX {
    ///0: Number of data to transfer for transmit is even
    Even = 0,
    ///1: Number of data to transfer for transmit is odd
    Odd = 1,
}
impl From<LDMA_TX> for bool {
    #[inline(always)]
    fn from(variant: LDMA_TX) -> Self {
        variant as u8 != 0
    }
}
///Field `LDMA_TX` reader - Last DMA transfer for transmission
pub type LDMA_TX_R = crate::BitReader<LDMA_TX>;
impl LDMA_TX_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LDMA_TX {
        match self.bits {
            false => LDMA_TX::Even,
            true => LDMA_TX::Odd,
        }
    }
    ///Number of data to transfer for transmit is even
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_TX::Even
    }
    ///Number of data to transfer for transmit is odd
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_TX::Odd
    }
}
///Field `LDMA_TX` writer - Last DMA transfer for transmission
pub type LDMA_TX_W<'a, REG> = crate::BitWriter<'a, REG, LDMA_TX>;
impl<'a, REG> LDMA_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Number of data to transfer for transmit is even
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_TX::Even)
    }
    ///Number of data to transfer for transmit is odd
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_TX::Odd)
    }
}
impl R {
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - NSS pulse management
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Error interrupt enable
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
    ///Bits 8:11 - Data size
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - FIFO reception threshold
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Last DMA transfer for reception
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Last DMA transfer for transmission
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
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
    ///Bit 0 - Rx buffer DMA enable
    #[inline(always)]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<'_, CR2rs> {
        RXDMAEN_W::new(self, 0)
    }
    ///Bit 1 - Tx buffer DMA enable
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, CR2rs> {
        TXDMAEN_W::new(self, 1)
    }
    ///Bit 2 - SS output enable
    #[inline(always)]
    pub fn ssoe(&mut self) -> SSOE_W<'_, CR2rs> {
        SSOE_W::new(self, 2)
    }
    ///Bit 3 - NSS pulse management
    #[inline(always)]
    pub fn nssp(&mut self) -> NSSP_W<'_, CR2rs> {
        NSSP_W::new(self, 3)
    }
    ///Bit 4 - Frame format
    #[inline(always)]
    pub fn frf(&mut self) -> FRF_W<'_, CR2rs> {
        FRF_W::new(self, 4)
    }
    ///Bit 5 - Error interrupt enable
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, CR2rs> {
        ERRIE_W::new(self, 5)
    }
    ///Bit 6 - RX buffer not empty interrupt enable
    #[inline(always)]
    pub fn rxneie(&mut self) -> RXNEIE_W<'_, CR2rs> {
        RXNEIE_W::new(self, 6)
    }
    ///Bit 7 - Tx buffer empty interrupt enable
    #[inline(always)]
    pub fn txeie(&mut self) -> TXEIE_W<'_, CR2rs> {
        TXEIE_W::new(self, 7)
    }
    ///Bits 8:11 - Data size
    #[inline(always)]
    pub fn ds(&mut self) -> DS_W<'_, CR2rs> {
        DS_W::new(self, 8)
    }
    ///Bit 12 - FIFO reception threshold
    #[inline(always)]
    pub fn frxth(&mut self) -> FRXTH_W<'_, CR2rs> {
        FRXTH_W::new(self, 12)
    }
    ///Bit 13 - Last DMA transfer for reception
    #[inline(always)]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W<'_, CR2rs> {
        LDMA_RX_W::new(self, 13)
    }
    ///Bit 14 - Last DMA transfer for transmission
    #[inline(always)]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W<'_, CR2rs> {
        LDMA_TX_W::new(self, 14)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G030.html#SPI1:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
