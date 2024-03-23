#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "Channel enable This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    #[doc = "0: Channel disabled"]
    Disabled = 0,
    #[doc = "1: Channel enabled"]
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - Channel enable This bit is set and cleared by software."]
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
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
#[doc = "Field `EN` writer - Channel enable This bit is set and cleared by software."]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
#[doc = "Transfer complete interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
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
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable This bit is set and cleared by software."]
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
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable This bit is set and cleared by software."]
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
#[doc = "Half transfer interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
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
#[doc = "Field `HTIE` reader - Half transfer interrupt enable This bit is set and cleared by software."]
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
#[doc = "Field `HTIE` writer - Half transfer interrupt enable This bit is set and cleared by software."]
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
#[doc = "Transfer error interrupt enable This bit is set and cleared by software.\n\nValue on reset: 0"]
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
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software."]
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
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software."]
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
#[doc = "Data transfer direction This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    #[doc = "0: Peripheral-to-memory"]
    PeripheralToMemory = 0,
    #[doc = "1: Memory-to-peripheral"]
    MemoryToPeripheral = 1,
}
impl From<DIR> for bool {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - Data transfer direction This bit is set and cleared by software."]
pub type DIR_R = crate::BitReader<DIR>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIR {
        match self.bits {
            false => DIR::PeripheralToMemory,
            true => DIR::MemoryToPeripheral,
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
}
#[doc = "Field `DIR` writer - Data transfer direction This bit is set and cleared by software."]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
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
}
#[doc = "Circular mode This bit is set and cleared by software.\n\nValue on reset: 0"]
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
#[doc = "Field `CIRC` reader - Circular mode This bit is set and cleared by software."]
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
#[doc = "Field `CIRC` writer - Circular mode This bit is set and cleared by software."]
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
#[doc = "Peripheral increment mode This bit is set and cleared by software.\n\nValue on reset: 0"]
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
#[doc = "Field `PINC` reader - Peripheral increment mode This bit is set and cleared by software."]
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
#[doc = "Field `PINC` writer - Peripheral increment mode This bit is set and cleared by software."]
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
#[doc = "Field `MINC` reader - Memory increment mode This bit is set and cleared by software."]
pub use PINC_R as MINC_R;
#[doc = "Field `MINC` writer - Memory increment mode This bit is set and cleared by software."]
pub use PINC_W as MINC_W;
#[doc = "Peripheral size These bits are set and cleared by software.\n\nValue on reset: 0"]
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
#[doc = "Field `PSIZE` reader - Peripheral size These bits are set and cleared by software."]
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
#[doc = "Field `PSIZE` writer - Peripheral size These bits are set and cleared by software."]
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
#[doc = "Field `MSIZE` reader - Memory size These bits are set and cleared by software."]
pub use PSIZE_R as MSIZE_R;
#[doc = "Field `MSIZE` writer - Memory size These bits are set and cleared by software."]
pub use PSIZE_W as MSIZE_W;
#[doc = "Channel priority level These bits are set and cleared by software.\n\nValue on reset: 0"]
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
#[doc = "Field `PL` reader - Channel priority level These bits are set and cleared by software."]
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
#[doc = "Field `PL` writer - Channel priority level These bits are set and cleared by software."]
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
#[doc = "Memory to memory mode This bit is set and cleared by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM2MEM {
    #[doc = "0: Memory-to-memory mode disabled"]
    Disabled = 0,
    #[doc = "1: Memory-to-memory mode enabled"]
    Enabled = 1,
}
impl From<MEM2MEM> for bool {
    #[inline(always)]
    fn from(variant: MEM2MEM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM2MEM` reader - Memory to memory mode This bit is set and cleared by software."]
pub type MEM2MEM_R = crate::BitReader<MEM2MEM>;
impl MEM2MEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MEM2MEM {
        match self.bits {
            false => MEM2MEM::Disabled,
            true => MEM2MEM::Enabled,
        }
    }
    #[doc = "Memory-to-memory mode disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MEM2MEM::Disabled
    }
    #[doc = "Memory-to-memory mode enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MEM2MEM::Enabled
    }
}
#[doc = "Field `MEM2MEM` writer - Memory to memory mode This bit is set and cleared by software."]
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG, MEM2MEM>;
impl<'a, REG> MEM2MEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Memory-to-memory mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::Disabled)
    }
    #[doc = "Memory-to-memory mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::Enabled)
    }
}
#[doc = "Double-buffer mode\n\nValue on reset: 0"]
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
#[doc = "Field `DBM` reader - Double-buffer mode"]
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
#[doc = "Field `DBM` writer - Double-buffer mode"]
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
#[doc = "Current target memory in double-buffer mode\n\nValue on reset: 0"]
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
#[doc = "Field `CT` reader - Current target memory in double-buffer mode"]
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
#[doc = "Field `CT` writer - Current target memory in double-buffer mode"]
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
impl R {
    #[doc = "Bit 0 - Channel enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral size These bits are set and cleared by software."]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory size These bits are set and cleared by software."]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel priority level These bits are set and cleared by software."]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Double-buffer mode"]
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Current target memory in double-buffer mode"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CRrs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CRrs> {
        TCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<CRrs> {
        HTIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<CRrs> {
        TEIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Data transfer direction This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CRrs> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Circular mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<CRrs> {
        CIRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral increment mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<CRrs> {
        PINC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Memory increment mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<CRrs> {
        MINC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - Peripheral size These bits are set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<CRrs> {
        PSIZE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Memory size These bits are set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<CRrs> {
        MSIZE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Channel priority level These bits are set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<CRrs> {
        PL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Memory to memory mode This bit is set and cleared by software."]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<CRrs> {
        MEM2MEM_W::new(self, 14)
    }
    #[doc = "Bit 15 - Double-buffer mode"]
    #[inline(always)]
    #[must_use]
    pub fn dbm(&mut self) -> DBM_W<CRrs> {
        DBM_W::new(self, 15)
    }
    #[doc = "Bit 16 - Current target memory in double-buffer mode"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CT_W<CRrs> {
        CT_W::new(self, 16)
    }
}
#[doc = "DMA channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
