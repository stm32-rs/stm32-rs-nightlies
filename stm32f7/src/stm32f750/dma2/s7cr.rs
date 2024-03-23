#[doc = "Register `S7CR` reader"]
pub type R = crate::R<S7CRrs>;
#[doc = "Register `S7CR` writer"]
pub type W = crate::W<S7CRrs>;
#[doc = "Stream enable / flag stream ready when read low\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    #[doc = "0: Stream disabled"]
    Disabled = 0,
    #[doc = "1: Stream enabled"]
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Stream enable / flag stream ready when read low"]
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    #[doc = "Stream disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    #[doc = "Stream enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
#[doc = "Field `EN` writer - Stream enable / flag stream ready when read low"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Stream disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    #[doc = "Stream enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
#[doc = "Direct mode error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMEIE {
    #[doc = "0: DME interrupt disabled"]
    Disabled = 0,
    #[doc = "1: DME interrupt enabled"]
    Enabled = 1,
}
impl From<DMEIE> for bool {
    #[inline(always)]
    fn from(variant: DMEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMEIE` reader - Direct mode error interrupt enable"]
pub type DMEIE_R = crate::BitReader<DMEIE>;
impl DMEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMEIE {
        match self.bits {
            false => DMEIE::Disabled,
            true => DMEIE::Enabled,
        }
    }
    #[doc = "DME interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMEIE::Disabled
    }
    #[doc = "DME interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMEIE::Enabled
    }
}
#[doc = "Field `DMEIE` writer - Direct mode error interrupt enable"]
pub type DMEIE_W<'a, REG> = crate::BitWriter<'a, REG, DMEIE>;
impl<'a, REG> DMEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DME interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMEIE::Disabled)
    }
    #[doc = "DME interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMEIE::Enabled)
    }
}
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    #[doc = "0: TE interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TE interrupt enabled"]
    Enabled = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader<TEIE>;
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TEIE {
        match self.bits {
            false => TEIE::Disabled,
            true => TEIE::Enabled,
        }
    }
    #[doc = "TE interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE::Disabled
    }
    #[doc = "TE interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE::Enabled
    }
}
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TE interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Disabled)
    }
    #[doc = "TE interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Enabled)
    }
}
#[doc = "Half transfer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE {
    #[doc = "0: HT interrupt disabled"]
    Disabled = 0,
    #[doc = "1: HT interrupt enabled"]
    Enabled = 1,
}
impl From<HTIE> for bool {
    #[inline(always)]
    fn from(variant: HTIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIE` reader - Half transfer interrupt enable"]
pub type HTIE_R = crate::BitReader<HTIE>;
impl HTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HTIE {
        match self.bits {
            false => HTIE::Disabled,
            true => HTIE::Enabled,
        }
    }
    #[doc = "HT interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTIE::Disabled
    }
    #[doc = "HT interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTIE::Enabled
    }
}
#[doc = "Field `HTIE` writer - Half transfer interrupt enable"]
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG, HTIE>;
impl<'a, REG> HTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "HT interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::Disabled)
    }
    #[doc = "HT interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::Enabled)
    }
}
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    #[doc = "0: TC interrupt disabled"]
    Disabled = 0,
    #[doc = "1: TC interrupt enabled"]
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    #[doc = "TC interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
#[doc = "Peripheral flow controller\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFCTRL {
    #[doc = "0: The DMA is the flow controller"]
    Dma = 0,
    #[doc = "1: The peripheral is the flow controller"]
    Peripheral = 1,
}
impl From<PFCTRL> for bool {
    #[inline(always)]
    fn from(variant: PFCTRL) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PFCTRL` reader - Peripheral flow controller"]
pub type PFCTRL_R = crate::BitReader<PFCTRL>;
impl PFCTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PFCTRL {
        match self.bits {
            false => PFCTRL::Dma,
            true => PFCTRL::Peripheral,
        }
    }
    #[doc = "The DMA is the flow controller"]
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == PFCTRL::Dma
    }
    #[doc = "The peripheral is the flow controller"]
    #[inline(always)]
    pub fn is_peripheral(&self) -> bool {
        *self == PFCTRL::Peripheral
    }
}
#[doc = "Field `PFCTRL` writer - Peripheral flow controller"]
pub type PFCTRL_W<'a, REG> = crate::BitWriter<'a, REG, PFCTRL>;
impl<'a, REG> PFCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The DMA is the flow controller"]
    #[inline(always)]
    pub fn dma(self) -> &'a mut crate::W<REG> {
        self.variant(PFCTRL::Dma)
    }
    #[doc = "The peripheral is the flow controller"]
    #[inline(always)]
    pub fn peripheral(self) -> &'a mut crate::W<REG> {
        self.variant(PFCTRL::Peripheral)
    }
}
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIR {
    #[doc = "0: Peripheral-to-memory"]
    PeripheralToMemory = 0,
    #[doc = "1: Memory-to-peripheral"]
    MemoryToPeripheral = 1,
    #[doc = "2: Memory-to-memory"]
    MemoryToMemory = 2,
}
impl From<DIR> for u8 {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DIR {
    type Ux = u8;
}
#[doc = "Field `DIR` reader - Data transfer direction"]
pub type DIR_R = crate::FieldReader<DIR>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIR> {
        match self.bits {
            0 => Some(DIR::PeripheralToMemory),
            1 => Some(DIR::MemoryToPeripheral),
            2 => Some(DIR::MemoryToMemory),
            _ => None,
        }
    }
    #[doc = "Peripheral-to-memory"]
    #[inline(always)]
    pub fn is_peripheral_to_memory(&self) -> bool {
        *self == DIR::PeripheralToMemory
    }
    #[doc = "Memory-to-peripheral"]
    #[inline(always)]
    pub fn is_memory_to_peripheral(&self) -> bool {
        *self == DIR::MemoryToPeripheral
    }
    #[doc = "Memory-to-memory"]
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == DIR::MemoryToMemory
    }
}
#[doc = "Field `DIR` writer - Data transfer direction"]
pub type DIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DIR>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Peripheral-to-memory"]
    #[inline(always)]
    pub fn peripheral_to_memory(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::PeripheralToMemory)
    }
    #[doc = "Memory-to-peripheral"]
    #[inline(always)]
    pub fn memory_to_peripheral(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::MemoryToPeripheral)
    }
    #[doc = "Memory-to-memory"]
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::MemoryToMemory)
    }
}
#[doc = "Circular mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRC {
    #[doc = "0: Circular mode disabled"]
    Disabled = 0,
    #[doc = "1: Circular mode enabled"]
    Enabled = 1,
}
impl From<CIRC> for bool {
    #[inline(always)]
    fn from(variant: CIRC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIRC` reader - Circular mode"]
pub type CIRC_R = crate::BitReader<CIRC>;
impl CIRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CIRC {
        match self.bits {
            false => CIRC::Disabled,
            true => CIRC::Enabled,
        }
    }
    #[doc = "Circular mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRC::Disabled
    }
    #[doc = "Circular mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRC::Enabled
    }
}
#[doc = "Field `CIRC` writer - Circular mode"]
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG, CIRC>;
impl<'a, REG> CIRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Circular mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::Disabled)
    }
    #[doc = "Circular mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::Enabled)
    }
}
#[doc = "Peripheral increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINC {
    #[doc = "0: Address pointer is fixed"]
    Fixed = 0,
    #[doc = "1: Address pointer is incremented after each data transfer"]
    Incremented = 1,
}
impl From<PINC> for bool {
    #[inline(always)]
    fn from(variant: PINC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINC` reader - Peripheral increment mode"]
pub type PINC_R = crate::BitReader<PINC>;
impl PINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINC {
        match self.bits {
            false => PINC::Fixed,
            true => PINC::Incremented,
        }
    }
    #[doc = "Address pointer is fixed"]
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == PINC::Fixed
    }
    #[doc = "Address pointer is incremented after each data transfer"]
    #[inline(always)]
    pub fn is_incremented(&self) -> bool {
        *self == PINC::Incremented
    }
}
#[doc = "Field `PINC` writer - Peripheral increment mode"]
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG, PINC>;
impl<'a, REG> PINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Address pointer is fixed"]
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::Fixed)
    }
    #[doc = "Address pointer is incremented after each data transfer"]
    #[inline(always)]
    pub fn incremented(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::Incremented)
    }
}
#[doc = "Field `MINC` reader - Memory increment mode"]
pub use PINC_R as MINC_R;
#[doc = "Field `MINC` writer - Memory increment mode"]
pub use PINC_W as MINC_W;
#[doc = "Peripheral data size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE {
    #[doc = "0: Byte (8-bit)"]
    Bits8 = 0,
    #[doc = "1: Half-word (16-bit)"]
    Bits16 = 1,
    #[doc = "2: Word (32-bit)"]
    Bits32 = 2,
}
impl From<PSIZE> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSIZE {
    type Ux = u8;
}
#[doc = "Field `PSIZE` reader - Peripheral data size"]
pub type PSIZE_R = crate::FieldReader<PSIZE>;
impl PSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PSIZE> {
        match self.bits {
            0 => Some(PSIZE::Bits8),
            1 => Some(PSIZE::Bits16),
            2 => Some(PSIZE::Bits32),
            _ => None,
        }
    }
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PSIZE::Bits8
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PSIZE::Bits16
    }
    #[doc = "Word (32-bit)"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PSIZE::Bits32
    }
}
#[doc = "Field `PSIZE` writer - Peripheral data size"]
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PSIZE>;
impl<'a, REG> PSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Byte (8-bit)"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits8)
    }
    #[doc = "Half-word (16-bit)"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits16)
    }
    #[doc = "Word (32-bit)"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits32)
    }
}
#[doc = "Field `MSIZE` reader - Memory data size"]
pub use PSIZE_R as MSIZE_R;
#[doc = "Field `MSIZE` writer - Memory data size"]
pub use PSIZE_W as MSIZE_W;
#[doc = "Peripheral increment offset size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINCOS {
    #[doc = "0: The offset size for the peripheral address calculation is linked to the PSIZE"]
    Psize = 0,
    #[doc = "1: The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    Fixed4 = 1,
}
impl From<PINCOS> for bool {
    #[inline(always)]
    fn from(variant: PINCOS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINCOS` reader - Peripheral increment offset size"]
pub type PINCOS_R = crate::BitReader<PINCOS>;
impl PINCOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINCOS {
        match self.bits {
            false => PINCOS::Psize,
            true => PINCOS::Fixed4,
        }
    }
    #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
    #[inline(always)]
    pub fn is_psize(&self) -> bool {
        *self == PINCOS::Psize
    }
    #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    #[inline(always)]
    pub fn is_fixed4(&self) -> bool {
        *self == PINCOS::Fixed4
    }
}
#[doc = "Field `PINCOS` writer - Peripheral increment offset size"]
pub type PINCOS_W<'a, REG> = crate::BitWriter<'a, REG, PINCOS>;
impl<'a, REG> PINCOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The offset size for the peripheral address calculation is linked to the PSIZE"]
    #[inline(always)]
    pub fn psize(self) -> &'a mut crate::W<REG> {
        self.variant(PINCOS::Psize)
    }
    #[doc = "The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)"]
    #[inline(always)]
    pub fn fixed4(self) -> &'a mut crate::W<REG> {
        self.variant(PINCOS::Fixed4)
    }
}
#[doc = "Priority level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL {
    #[doc = "0: Low"]
    Low = 0,
    #[doc = "1: Medium"]
    Medium = 1,
    #[doc = "2: High"]
    High = 2,
    #[doc = "3: Very high"]
    VeryHigh = 3,
}
impl From<PL> for u8 {
    #[inline(always)]
    fn from(variant: PL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PL {
    type Ux = u8;
}
#[doc = "Field `PL` reader - Priority level"]
pub type PL_R = crate::FieldReader<PL>;
impl PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PL {
        match self.bits {
            0 => PL::Low,
            1 => PL::Medium,
            2 => PL::High,
            3 => PL::VeryHigh,
            _ => unreachable!(),
        }
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PL::Low
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PL::Medium
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PL::High
    }
    #[doc = "Very high"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PL::VeryHigh
    }
}
#[doc = "Field `PL` writer - Priority level"]
pub type PL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PL>;
impl<'a, REG> PL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(PL::Low)
    }
    #[doc = "Medium"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(PL::Medium)
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(PL::High)
    }
    #[doc = "Very high"]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut crate::W<REG> {
        self.variant(PL::VeryHigh)
    }
}
#[doc = "Double buffer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBM {
    #[doc = "0: No buffer switching at the end of transfer"]
    Disabled = 0,
    #[doc = "1: Memory target switched at the end of the DMA transfer"]
    Enabled = 1,
}
impl From<DBM> for bool {
    #[inline(always)]
    fn from(variant: DBM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBM` reader - Double buffer mode"]
pub type DBM_R = crate::BitReader<DBM>;
impl DBM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DBM {
        match self.bits {
            false => DBM::Disabled,
            true => DBM::Enabled,
        }
    }
    #[doc = "No buffer switching at the end of transfer"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBM::Disabled
    }
    #[doc = "Memory target switched at the end of the DMA transfer"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBM::Enabled
    }
}
#[doc = "Field `DBM` writer - Double buffer mode"]
pub type DBM_W<'a, REG> = crate::BitWriter<'a, REG, DBM>;
impl<'a, REG> DBM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No buffer switching at the end of transfer"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBM::Disabled)
    }
    #[doc = "Memory target switched at the end of the DMA transfer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBM::Enabled)
    }
}
#[doc = "Current target (only in double buffer mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT {
    #[doc = "0: The current target memory is Memory 0"]
    Memory0 = 0,
    #[doc = "1: The current target memory is Memory 1"]
    Memory1 = 1,
}
impl From<CT> for bool {
    #[inline(always)]
    fn from(variant: CT) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CT` reader - Current target (only in double buffer mode)"]
pub type CT_R = crate::BitReader<CT>;
impl CT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CT {
        match self.bits {
            false => CT::Memory0,
            true => CT::Memory1,
        }
    }
    #[doc = "The current target memory is Memory 0"]
    #[inline(always)]
    pub fn is_memory0(&self) -> bool {
        *self == CT::Memory0
    }
    #[doc = "The current target memory is Memory 1"]
    #[inline(always)]
    pub fn is_memory1(&self) -> bool {
        *self == CT::Memory1
    }
}
#[doc = "Field `CT` writer - Current target (only in double buffer mode)"]
pub type CT_W<'a, REG> = crate::BitWriter<'a, REG, CT>;
impl<'a, REG> CT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The current target memory is Memory 0"]
    #[inline(always)]
    pub fn memory0(self) -> &'a mut crate::W<REG> {
        self.variant(CT::Memory0)
    }
    #[doc = "The current target memory is Memory 1"]
    #[inline(always)]
    pub fn memory1(self) -> &'a mut crate::W<REG> {
        self.variant(CT::Memory1)
    }
}
#[doc = "Field `ACK` reader - ACK"]
pub type ACK_R = crate::BitReader;
#[doc = "Field `ACK` writer - ACK"]
pub type ACK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Peripheral burst transfer configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PBURST {
    #[doc = "0: Single transfer"]
    Single = 0,
    #[doc = "1: Incremental burst of 4 beats"]
    Incr4 = 1,
    #[doc = "2: Incremental burst of 8 beats"]
    Incr8 = 2,
    #[doc = "3: Incremental burst of 16 beats"]
    Incr16 = 3,
}
impl From<PBURST> for u8 {
    #[inline(always)]
    fn from(variant: PBURST) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PBURST {
    type Ux = u8;
}
#[doc = "Field `PBURST` reader - Peripheral burst transfer configuration"]
pub type PBURST_R = crate::FieldReader<PBURST>;
impl PBURST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PBURST {
        match self.bits {
            0 => PBURST::Single,
            1 => PBURST::Incr4,
            2 => PBURST::Incr8,
            3 => PBURST::Incr16,
            _ => unreachable!(),
        }
    }
    #[doc = "Single transfer"]
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == PBURST::Single
    }
    #[doc = "Incremental burst of 4 beats"]
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == PBURST::Incr4
    }
    #[doc = "Incremental burst of 8 beats"]
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == PBURST::Incr8
    }
    #[doc = "Incremental burst of 16 beats"]
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == PBURST::Incr16
    }
}
#[doc = "Field `PBURST` writer - Peripheral burst transfer configuration"]
pub type PBURST_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, PBURST>;
impl<'a, REG> PBURST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Single transfer"]
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(PBURST::Single)
    }
    #[doc = "Incremental burst of 4 beats"]
    #[inline(always)]
    pub fn incr4(self) -> &'a mut crate::W<REG> {
        self.variant(PBURST::Incr4)
    }
    #[doc = "Incremental burst of 8 beats"]
    #[inline(always)]
    pub fn incr8(self) -> &'a mut crate::W<REG> {
        self.variant(PBURST::Incr8)
    }
    #[doc = "Incremental burst of 16 beats"]
    #[inline(always)]
    pub fn incr16(self) -> &'a mut crate::W<REG> {
        self.variant(PBURST::Incr16)
    }
}
#[doc = "Field `MBURST` reader - Memory burst transfer configuration"]
pub use PBURST_R as MBURST_R;
#[doc = "Field `MBURST` writer - Memory burst transfer configuration"]
pub use PBURST_W as MBURST_W;
#[doc = "Field `CHSEL` reader - Channel selection"]
pub type CHSEL_R = crate::FieldReader;
#[doc = "Field `CHSEL` writer - Channel selection"]
pub type CHSEL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ACK"]
    #[inline(always)]
    pub fn ack(&self) -> ACK_R {
        ACK_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline(always)]
    pub fn chsel(&self) -> CHSEL_R {
        CHSEL_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Stream enable / flag stream ready when read low"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<S7CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Direct mode error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmeie(&mut self) -> DMEIE_W<S7CRrs> {
        DMEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<S7CRrs> {
        TEIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Half transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<S7CRrs> {
        HTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<S7CRrs> {
        TCIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Peripheral flow controller"]
    #[inline(always)]
    #[must_use]
    pub fn pfctrl(&mut self) -> PFCTRL_W<S7CRrs> {
        PFCTRL_W::new(self, 5)
    }
    #[doc = "Bits 6:7 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<S7CRrs> {
        DIR_W::new(self, 6)
    }
    #[doc = "Bit 8 - Circular mode"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<S7CRrs> {
        CIRC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<S7CRrs> {
        PINC_W::new(self, 9)
    }
    #[doc = "Bit 10 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<S7CRrs> {
        MINC_W::new(self, 10)
    }
    #[doc = "Bits 11:12 - Peripheral data size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<S7CRrs> {
        PSIZE_W::new(self, 11)
    }
    #[doc = "Bits 13:14 - Memory data size"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<S7CRrs> {
        MSIZE_W::new(self, 13)
    }
    #[doc = "Bit 15 - Peripheral increment offset size"]
    #[inline(always)]
    #[must_use]
    pub fn pincos(&mut self) -> PINCOS_W<S7CRrs> {
        PINCOS_W::new(self, 15)
    }
    #[doc = "Bits 16:17 - Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<S7CRrs> {
        PL_W::new(self, 16)
    }
    #[doc = "Bit 18 - Double buffer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbm(&mut self) -> DBM_W<S7CRrs> {
        DBM_W::new(self, 18)
    }
    #[doc = "Bit 19 - Current target (only in double buffer mode)"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CT_W<S7CRrs> {
        CT_W::new(self, 19)
    }
    #[doc = "Bit 20 - ACK"]
    #[inline(always)]
    #[must_use]
    pub fn ack(&mut self) -> ACK_W<S7CRrs> {
        ACK_W::new(self, 20)
    }
    #[doc = "Bits 21:22 - Peripheral burst transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pburst(&mut self) -> PBURST_W<S7CRrs> {
        PBURST_W::new(self, 21)
    }
    #[doc = "Bits 23:24 - Memory burst transfer configuration"]
    #[inline(always)]
    #[must_use]
    pub fn mburst(&mut self) -> MBURST_W<S7CRrs> {
        MBURST_W::new(self, 23)
    }
    #[doc = "Bits 25:27 - Channel selection"]
    #[inline(always)]
    #[must_use]
    pub fn chsel(&mut self) -> CHSEL_W<S7CRrs> {
        CHSEL_W::new(self, 25)
    }
}
#[doc = "stream x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`s7cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`s7cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S7CRrs;
impl crate::RegisterSpec for S7CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s7cr::R`](R) reader structure"]
impl crate::Readable for S7CRrs {}
#[doc = "`write(|w| ..)` method takes [`s7cr::W`](W) writer structure"]
impl crate::Writable for S7CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets S7CR to value 0"]
impl crate::Resettable for S7CRrs {
    const RESET_VALUE: u32 = 0;
}
