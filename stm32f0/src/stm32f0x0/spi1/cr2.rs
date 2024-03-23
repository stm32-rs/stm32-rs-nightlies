#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Rx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN {
    #[doc = "0: Rx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Rx buffer DMA enabled"]
    Enabled = 1,
}
impl From<RXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN>;
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXDMAEN {
        match self.bits {
            false => RXDMAEN::Disabled,
            true => RXDMAEN::Enabled,
        }
    }
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN::Disabled
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN::Enabled
    }
}
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable"]
pub type RXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, RXDMAEN>;
impl<'a, REG> RXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Disabled)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RXDMAEN::Enabled)
    }
}
#[doc = "Tx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN {
    #[doc = "0: Tx buffer DMA disabled"]
    Disabled = 0,
    #[doc = "1: Tx buffer DMA enabled"]
    Enabled = 1,
}
impl From<TXDMAEN> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN>;
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXDMAEN {
        match self.bits {
            false => TXDMAEN::Disabled,
            true => TXDMAEN::Enabled,
        }
    }
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN::Disabled
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN::Enabled
    }
}
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable"]
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG, TXDMAEN>;
impl<'a, REG> TXDMAEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Disabled)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TXDMAEN::Enabled)
    }
}
#[doc = "SS output enable\n\nValue on reset: 0"]
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
#[doc = "Field `SSOE` reader - SS output enable"]
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
#[doc = "Field `SSOE` writer - SS output enable"]
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
#[doc = "NSS pulse management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSSP {
    #[doc = "0: No NSS pulse"]
    NoPulse = 0,
    #[doc = "1: NSS pulse generated"]
    PulseGenerated = 1,
}
impl From<NSSP> for bool {
    #[inline(always)]
    fn from(variant: NSSP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NSSP` reader - NSS pulse management"]
pub type NSSP_R = crate::BitReader<NSSP>;
impl NSSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> NSSP {
        match self.bits {
            false => NSSP::NoPulse,
            true => NSSP::PulseGenerated,
        }
    }
    #[doc = "No NSS pulse"]
    #[inline(always)]
    pub fn is_no_pulse(&self) -> bool {
        *self == NSSP::NoPulse
    }
    #[doc = "NSS pulse generated"]
    #[inline(always)]
    pub fn is_pulse_generated(&self) -> bool {
        *self == NSSP::PulseGenerated
    }
}
#[doc = "Field `NSSP` writer - NSS pulse management"]
pub type NSSP_W<'a, REG> = crate::BitWriter<'a, REG, NSSP>;
impl<'a, REG> NSSP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No NSS pulse"]
    #[inline(always)]
    pub fn no_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP::NoPulse)
    }
    #[doc = "NSS pulse generated"]
    #[inline(always)]
    pub fn pulse_generated(self) -> &'a mut crate::W<REG> {
        self.variant(NSSP::PulseGenerated)
    }
}
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRF {
    #[doc = "0: SPI Motorola mode"]
    Motorola = 0,
    #[doc = "1: SPI TI mode"]
    Ti = 1,
}
impl From<FRF> for bool {
    #[inline(always)]
    fn from(variant: FRF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRF` reader - Frame format"]
pub type FRF_R = crate::BitReader<FRF>;
impl FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRF {
        match self.bits {
            false => FRF::Motorola,
            true => FRF::Ti,
        }
    }
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == FRF::Motorola
    }
    #[doc = "SPI TI mode"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF::Ti
    }
}
#[doc = "Field `FRF` writer - Frame format"]
pub type FRF_W<'a, REG> = crate::BitWriter<'a, REG, FRF>;
impl<'a, REG> FRF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut crate::W<REG> {
        self.variant(FRF::Motorola)
    }
    #[doc = "SPI TI mode"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(FRF::Ti)
    }
}
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE {
    #[doc = "0: Error interrupt masked"]
    Masked = 0,
    #[doc = "1: Error interrupt not masked"]
    NotMasked = 1,
}
impl From<ERRIE> for bool {
    #[inline(always)]
    fn from(variant: ERRIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE>;
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRIE {
        match self.bits {
            false => ERRIE::Masked,
            true => ERRIE::NotMasked,
        }
    }
    #[doc = "Error interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ERRIE::Masked
    }
    #[doc = "Error interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIE::NotMasked
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG, ERRIE>;
impl<'a, REG> ERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::Masked)
    }
    #[doc = "Error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(ERRIE::NotMasked)
    }
}
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE {
    #[doc = "0: RXE interrupt masked"]
    Masked = 0,
    #[doc = "1: RXE interrupt not masked"]
    NotMasked = 1,
}
impl From<RXNEIE> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable"]
pub type RXNEIE_R = crate::BitReader<RXNEIE>;
impl RXNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RXNEIE {
        match self.bits {
            false => RXNEIE::Masked,
            true => RXNEIE::NotMasked,
        }
    }
    #[doc = "RXE interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIE::Masked
    }
    #[doc = "RXE interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIE::NotMasked
    }
}
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable"]
pub type RXNEIE_W<'a, REG> = crate::BitWriter<'a, REG, RXNEIE>;
impl<'a, REG> RXNEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::Masked)
    }
    #[doc = "RXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(RXNEIE::NotMasked)
    }
}
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE {
    #[doc = "0: TXE interrupt masked"]
    Masked = 0,
    #[doc = "1: TXE interrupt not masked"]
    NotMasked = 1,
}
impl From<TXEIE> for bool {
    #[inline(always)]
    fn from(variant: TXEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable"]
pub type TXEIE_R = crate::BitReader<TXEIE>;
impl TXEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TXEIE {
        match self.bits {
            false => TXEIE::Masked,
            true => TXEIE::NotMasked,
        }
    }
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXEIE::Masked
    }
    #[doc = "TXE interrupt not masked"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIE::NotMasked
    }
}
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable"]
pub type TXEIE_W<'a, REG> = crate::BitWriter<'a, REG, TXEIE>;
impl<'a, REG> TXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::Masked)
    }
    #[doc = "TXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut crate::W<REG> {
        self.variant(TXEIE::NotMasked)
    }
}
#[doc = "Data size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DS {
    #[doc = "3: 4-bit"]
    FourBit = 3,
    #[doc = "4: 5-bit"]
    FiveBit = 4,
    #[doc = "5: 6-bit"]
    SixBit = 5,
    #[doc = "6: 7-bit"]
    SevenBit = 6,
    #[doc = "7: 8-bit"]
    EightBit = 7,
    #[doc = "8: 9-bit"]
    NineBit = 8,
    #[doc = "9: 10-bit"]
    TenBit = 9,
    #[doc = "10: 11-bit"]
    ElevenBit = 10,
    #[doc = "11: 12-bit"]
    TwelveBit = 11,
    #[doc = "12: 13-bit"]
    ThirteenBit = 12,
    #[doc = "13: 14-bit"]
    FourteenBit = 13,
    #[doc = "14: 15-bit"]
    FifteenBit = 14,
    #[doc = "15: 16-bit"]
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
#[doc = "Field `DS` reader - Data size"]
pub type DS_R = crate::FieldReader<DS>;
impl DS_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "4-bit"]
    #[inline(always)]
    pub fn is_four_bit(&self) -> bool {
        *self == DS::FourBit
    }
    #[doc = "5-bit"]
    #[inline(always)]
    pub fn is_five_bit(&self) -> bool {
        *self == DS::FiveBit
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == DS::SixBit
    }
    #[doc = "7-bit"]
    #[inline(always)]
    pub fn is_seven_bit(&self) -> bool {
        *self == DS::SevenBit
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == DS::EightBit
    }
    #[doc = "9-bit"]
    #[inline(always)]
    pub fn is_nine_bit(&self) -> bool {
        *self == DS::NineBit
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == DS::TenBit
    }
    #[doc = "11-bit"]
    #[inline(always)]
    pub fn is_eleven_bit(&self) -> bool {
        *self == DS::ElevenBit
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == DS::TwelveBit
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn is_thirteen_bit(&self) -> bool {
        *self == DS::ThirteenBit
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == DS::FourteenBit
    }
    #[doc = "15-bit"]
    #[inline(always)]
    pub fn is_fifteen_bit(&self) -> bool {
        *self == DS::FifteenBit
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DS::SixteenBit
    }
}
#[doc = "Field `DS` writer - Data size"]
pub type DS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, DS>;
impl<'a, REG> DS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-bit"]
    #[inline(always)]
    pub fn four_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::FourBit)
    }
    #[doc = "5-bit"]
    #[inline(always)]
    pub fn five_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::FiveBit)
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::SixBit)
    }
    #[doc = "7-bit"]
    #[inline(always)]
    pub fn seven_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::SevenBit)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::EightBit)
    }
    #[doc = "9-bit"]
    #[inline(always)]
    pub fn nine_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::NineBit)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::TenBit)
    }
    #[doc = "11-bit"]
    #[inline(always)]
    pub fn eleven_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::ElevenBit)
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::TwelveBit)
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn thirteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::ThirteenBit)
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::FourteenBit)
    }
    #[doc = "15-bit"]
    #[inline(always)]
    pub fn fifteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::FifteenBit)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut crate::W<REG> {
        self.variant(DS::SixteenBit)
    }
}
#[doc = "FIFO reception threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRXTH {
    #[doc = "0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    Half = 0,
    #[doc = "1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    Quarter = 1,
}
impl From<FRXTH> for bool {
    #[inline(always)]
    fn from(variant: FRXTH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FRXTH` reader - FIFO reception threshold"]
pub type FRXTH_R = crate::BitReader<FRXTH>;
impl FRXTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FRXTH {
        match self.bits {
            false => FRXTH::Half,
            true => FRXTH::Quarter,
        }
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FRXTH::Half
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FRXTH::Quarter
    }
}
#[doc = "Field `FRXTH` writer - FIFO reception threshold"]
pub type FRXTH_W<'a, REG> = crate::BitWriter<'a, REG, FRXTH>;
impl<'a, REG> FRXTH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(FRXTH::Half)
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(FRXTH::Quarter)
    }
}
#[doc = "Last DMA transfer for reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_RX {
    #[doc = "0: Number of data to transfer for receive is even"]
    Even = 0,
    #[doc = "1: Number of data to transfer for receive is odd"]
    Odd = 1,
}
impl From<LDMA_RX> for bool {
    #[inline(always)]
    fn from(variant: LDMA_RX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDMA_RX` reader - Last DMA transfer for reception"]
pub type LDMA_RX_R = crate::BitReader<LDMA_RX>;
impl LDMA_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LDMA_RX {
        match self.bits {
            false => LDMA_RX::Even,
            true => LDMA_RX::Odd,
        }
    }
    #[doc = "Number of data to transfer for receive is even"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_RX::Even
    }
    #[doc = "Number of data to transfer for receive is odd"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_RX::Odd
    }
}
#[doc = "Field `LDMA_RX` writer - Last DMA transfer for reception"]
pub type LDMA_RX_W<'a, REG> = crate::BitWriter<'a, REG, LDMA_RX>;
impl<'a, REG> LDMA_RX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Number of data to transfer for receive is even"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_RX::Even)
    }
    #[doc = "Number of data to transfer for receive is odd"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_RX::Odd)
    }
}
#[doc = "Last DMA transfer for transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_TX {
    #[doc = "0: Number of data to transfer for transmit is even"]
    Even = 0,
    #[doc = "1: Number of data to transfer for transmit is odd"]
    Odd = 1,
}
impl From<LDMA_TX> for bool {
    #[inline(always)]
    fn from(variant: LDMA_TX) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LDMA_TX` reader - Last DMA transfer for transmission"]
pub type LDMA_TX_R = crate::BitReader<LDMA_TX>;
impl LDMA_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LDMA_TX {
        match self.bits {
            false => LDMA_TX::Even,
            true => LDMA_TX::Odd,
        }
    }
    #[doc = "Number of data to transfer for transmit is even"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_TX::Even
    }
    #[doc = "Number of data to transfer for transmit is odd"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_TX::Odd
    }
}
#[doc = "Field `LDMA_TX` writer - Last DMA transfer for transmission"]
pub type LDMA_TX_W<'a, REG> = crate::BitWriter<'a, REG, LDMA_TX>;
impl<'a, REG> LDMA_TX_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Number of data to transfer for transmit is even"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_TX::Even)
    }
    #[doc = "Number of data to transfer for transmit is odd"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(LDMA_TX::Odd)
    }
}
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<CR2rs> {
        RXDMAEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<CR2rs> {
        TXDMAEN_W::new(self, 1)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<CR2rs> {
        SSOE_W::new(self, 2)
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    #[must_use]
    pub fn nssp(&mut self) -> NSSP_W<CR2rs> {
        NSSP_W::new(self, 3)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<CR2rs> {
        FRF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<CR2rs> {
        ERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<CR2rs> {
        RXNEIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<CR2rs> {
        TXEIE_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<CR2rs> {
        DS_W::new(self, 8)
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    #[must_use]
    pub fn frxth(&mut self) -> FRXTH_W<CR2rs> {
        FRXTH_W::new(self, 12)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    #[must_use]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W<CR2rs> {
        LDMA_RX_W::new(self, 13)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W<CR2rs> {
        LDMA_TX_W::new(self, 14)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
