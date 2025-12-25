///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Channel enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Channel disabled
    Disabled = 0,
    ///1: Channel enabled
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Channel enable This bit is set and cleared by software.
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::Disabled,
            true => EN::Enabled,
        }
    }
    ///Channel disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///Channel enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - Channel enable This bit is set and cleared by software.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Channel disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///Channel enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
/**Transfer complete interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    ///0: TC interrupt disabled
    Disabled = 0,
    ///1: TC interrupt enabled
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - Transfer complete interrupt enable This bit is set and cleared by software.
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::Disabled,
            true => TCIE::Enabled,
        }
    }
    ///TC interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    ///TC interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
///Field `TCIE` writer - Transfer complete interrupt enable This bit is set and cleared by software.
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TC interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    ///TC interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
/**Half transfer interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE {
    ///0: HT interrupt disabled
    Disabled = 0,
    ///1: HT interrupt enabled
    Enabled = 1,
}
impl From<HTIE> for bool {
    #[inline(always)]
    fn from(variant: HTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIE` reader - Half transfer interrupt enable This bit is set and cleared by software.
pub type HTIE_R = crate::BitReader<HTIE>;
impl HTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTIE {
        match self.bits {
            false => HTIE::Disabled,
            true => HTIE::Enabled,
        }
    }
    ///HT interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTIE::Disabled
    }
    ///HT interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTIE::Enabled
    }
}
///Field `HTIE` writer - Half transfer interrupt enable This bit is set and cleared by software.
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG, HTIE>;
impl<'a, REG> HTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///HT interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::Disabled)
    }
    ///HT interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::Enabled)
    }
}
/**Transfer error interrupt enable This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    ///0: TE interrupt disabled
    Disabled = 0,
    ///1: TE interrupt enabled
    Enabled = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software.
pub type TEIE_R = crate::BitReader<TEIE>;
impl TEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIE {
        match self.bits {
            false => TEIE::Disabled,
            true => TEIE::Enabled,
        }
    }
    ///TE interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE::Disabled
    }
    ///TE interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE::Enabled
    }
}
///Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software.
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TE interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Disabled)
    }
    ///TE interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Enabled)
    }
}
/**Data transfer direction This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    ///0: Peripheral-to-memory
    PeripheralToMemory = 0,
    ///1: Memory-to-peripheral
    MemoryToPeripheral = 1,
}
impl From<DIR> for bool {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Data transfer direction This bit is set and cleared by software.
pub type DIR_R = crate::BitReader<DIR>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIR {
        match self.bits {
            false => DIR::PeripheralToMemory,
            true => DIR::MemoryToPeripheral,
        }
    }
    ///Peripheral-to-memory
    #[inline(always)]
    pub fn is_peripheral_to_memory(&self) -> bool {
        *self == DIR::PeripheralToMemory
    }
    ///Memory-to-peripheral
    #[inline(always)]
    pub fn is_memory_to_peripheral(&self) -> bool {
        *self == DIR::MemoryToPeripheral
    }
}
///Field `DIR` writer - Data transfer direction This bit is set and cleared by software.
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Peripheral-to-memory
    #[inline(always)]
    pub fn peripheral_to_memory(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::PeripheralToMemory)
    }
    ///Memory-to-peripheral
    #[inline(always)]
    pub fn memory_to_peripheral(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::MemoryToPeripheral)
    }
}
/**Circular mode This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRC {
    ///0: Circular mode disabled
    Disabled = 0,
    ///1: Circular mode enabled
    Enabled = 1,
}
impl From<CIRC> for bool {
    #[inline(always)]
    fn from(variant: CIRC) -> Self {
        variant as u8 != 0
    }
}
///Field `CIRC` reader - Circular mode This bit is set and cleared by software.
pub type CIRC_R = crate::BitReader<CIRC>;
impl CIRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CIRC {
        match self.bits {
            false => CIRC::Disabled,
            true => CIRC::Enabled,
        }
    }
    ///Circular mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRC::Disabled
    }
    ///Circular mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRC::Enabled
    }
}
///Field `CIRC` writer - Circular mode This bit is set and cleared by software.
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG, CIRC>;
impl<'a, REG> CIRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Circular mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::Disabled)
    }
    ///Circular mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::Enabled)
    }
}
/**Peripheral increment mode This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINC {
    ///0: Address pointer is fixed
    Fixed = 0,
    ///1: Address pointer is incremented after each data transfer
    Incremented = 1,
}
impl From<PINC> for bool {
    #[inline(always)]
    fn from(variant: PINC) -> Self {
        variant as u8 != 0
    }
}
///Field `PINC` reader - Peripheral increment mode This bit is set and cleared by software.
pub type PINC_R = crate::BitReader<PINC>;
impl PINC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINC {
        match self.bits {
            false => PINC::Fixed,
            true => PINC::Incremented,
        }
    }
    ///Address pointer is fixed
    #[inline(always)]
    pub fn is_fixed(&self) -> bool {
        *self == PINC::Fixed
    }
    ///Address pointer is incremented after each data transfer
    #[inline(always)]
    pub fn is_incremented(&self) -> bool {
        *self == PINC::Incremented
    }
}
///Field `PINC` writer - Peripheral increment mode This bit is set and cleared by software.
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG, PINC>;
impl<'a, REG> PINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Address pointer is fixed
    #[inline(always)]
    pub fn fixed(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::Fixed)
    }
    ///Address pointer is incremented after each data transfer
    #[inline(always)]
    pub fn incremented(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::Incremented)
    }
}
///Field `MINC` reader - Memory increment mode This bit is set and cleared by software.
pub use PINC_R as MINC_R;
///Field `MINC` writer - Memory increment mode This bit is set and cleared by software.
pub use PINC_W as MINC_W;
/**Peripheral size These bits are set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE {
    ///0: Byte (8-bit)
    Bits8 = 0,
    ///1: Half-word (16-bit)
    Bits16 = 1,
    ///2: Word (32-bit)
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
impl crate::IsEnum for PSIZE {}
///Field `PSIZE` reader - Peripheral size These bits are set and cleared by software.
pub type PSIZE_R = crate::FieldReader<PSIZE>;
impl PSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PSIZE> {
        match self.bits {
            0 => Some(PSIZE::Bits8),
            1 => Some(PSIZE::Bits16),
            2 => Some(PSIZE::Bits32),
            _ => None,
        }
    }
    ///Byte (8-bit)
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PSIZE::Bits8
    }
    ///Half-word (16-bit)
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PSIZE::Bits16
    }
    ///Word (32-bit)
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PSIZE::Bits32
    }
}
///Field `PSIZE` writer - Peripheral size These bits are set and cleared by software.
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PSIZE>;
impl<'a, REG> PSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Byte (8-bit)
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits8)
    }
    ///Half-word (16-bit)
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits16)
    }
    ///Word (32-bit)
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits32)
    }
}
///Field `MSIZE` reader - Memory size These bits are set and cleared by software.
pub use PSIZE_R as MSIZE_R;
///Field `MSIZE` writer - Memory size These bits are set and cleared by software.
pub use PSIZE_W as MSIZE_W;
/**Channel priority level These bits are set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL {
    ///0: Low
    Low = 0,
    ///1: Medium
    Medium = 1,
    ///2: High
    High = 2,
    ///3: Very high
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
impl crate::IsEnum for PL {}
///Field `PL` reader - Channel priority level These bits are set and cleared by software.
pub type PL_R = crate::FieldReader<PL>;
impl PL_R {
    ///Get enumerated values variant
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
    ///Low
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PL::Low
    }
    ///Medium
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PL::Medium
    }
    ///High
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PL::High
    }
    ///Very high
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PL::VeryHigh
    }
}
///Field `PL` writer - Channel priority level These bits are set and cleared by software.
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PL, crate::Safe>;
impl<'a, REG> PL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Low
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(PL::Low)
    }
    ///Medium
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(PL::Medium)
    }
    ///High
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(PL::High)
    }
    ///Very high
    #[inline(always)]
    pub fn very_high(self) -> &'a mut crate::W<REG> {
        self.variant(PL::VeryHigh)
    }
}
/**Memory to memory mode This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM2MEM {
    ///0: Memory-to-memory mode disabled
    Disabled = 0,
    ///1: Memory-to-memory mode enabled
    Enabled = 1,
}
impl From<MEM2MEM> for bool {
    #[inline(always)]
    fn from(variant: MEM2MEM) -> Self {
        variant as u8 != 0
    }
}
///Field `MEM2MEM` reader - Memory to memory mode This bit is set and cleared by software.
pub type MEM2MEM_R = crate::BitReader<MEM2MEM>;
impl MEM2MEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MEM2MEM {
        match self.bits {
            false => MEM2MEM::Disabled,
            true => MEM2MEM::Enabled,
        }
    }
    ///Memory-to-memory mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MEM2MEM::Disabled
    }
    ///Memory-to-memory mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MEM2MEM::Enabled
    }
}
///Field `MEM2MEM` writer - Memory to memory mode This bit is set and cleared by software.
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG, MEM2MEM>;
impl<'a, REG> MEM2MEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory-to-memory mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::Disabled)
    }
    ///Memory-to-memory mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::Enabled)
    }
}
/**double-buffer mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBM {
    ///0: No buffer switching at the end of transfer
    Disabled = 0,
    ///1: Memory target switched at the end of the DMA transfer
    Enabled = 1,
}
impl From<DBM> for bool {
    #[inline(always)]
    fn from(variant: DBM) -> Self {
        variant as u8 != 0
    }
}
///Field `DBM` reader - double-buffer mode
pub type DBM_R = crate::BitReader<DBM>;
impl DBM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DBM {
        match self.bits {
            false => DBM::Disabled,
            true => DBM::Enabled,
        }
    }
    ///No buffer switching at the end of transfer
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBM::Disabled
    }
    ///Memory target switched at the end of the DMA transfer
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBM::Enabled
    }
}
///Field `DBM` writer - double-buffer mode
pub type DBM_W<'a, REG> = crate::BitWriter<'a, REG, DBM>;
impl<'a, REG> DBM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No buffer switching at the end of transfer
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBM::Disabled)
    }
    ///Memory target switched at the end of the DMA transfer
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DBM::Enabled)
    }
}
/**current target memory of DMA transfer in double-buffer mode.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CT {
    ///0: The current target memory is Memory 0
    Memory0 = 0,
    ///1: The current target memory is Memory 1
    Memory1 = 1,
}
impl From<CT> for bool {
    #[inline(always)]
    fn from(variant: CT) -> Self {
        variant as u8 != 0
    }
}
///Field `CT` reader - current target memory of DMA transfer in double-buffer mode.
pub type CT_R = crate::BitReader<CT>;
impl CT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CT {
        match self.bits {
            false => CT::Memory0,
            true => CT::Memory1,
        }
    }
    ///The current target memory is Memory 0
    #[inline(always)]
    pub fn is_memory0(&self) -> bool {
        *self == CT::Memory0
    }
    ///The current target memory is Memory 1
    #[inline(always)]
    pub fn is_memory1(&self) -> bool {
        *self == CT::Memory1
    }
}
///Field `CT` writer - current target memory of DMA transfer in double-buffer mode.
pub type CT_W<'a, REG> = crate::BitWriter<'a, REG, CT>;
impl<'a, REG> CT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The current target memory is Memory 0
    #[inline(always)]
    pub fn memory0(self) -> &'a mut crate::W<REG> {
        self.variant(CT::Memory0)
    }
    ///The current target memory is Memory 1
    #[inline(always)]
    pub fn memory1(self) -> &'a mut crate::W<REG> {
        self.variant(CT::Memory1)
    }
}
impl R {
    ///Bit 0 - Channel enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Half transfer interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data transfer direction This bit is set and cleared by software.
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Circular mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Peripheral increment mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Memory increment mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Peripheral size These bits are set and cleared by software.
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Memory size These bits are set and cleared by software.
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Channel priority level These bits are set and cleared by software.
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Memory to memory mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - double-buffer mode
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - current target memory of DMA transfer in double-buffer mode.
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("en", &self.en())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dir", &self.dir())
            .field("circ", &self.circ())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pl", &self.pl())
            .field("mem2mem", &self.mem2mem())
            .field("dbm", &self.dbm())
            .field("ct", &self.ct())
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Transfer complete interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 1)
    }
    ///Bit 2 - Half transfer interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<'_, CRrs> {
        HTIE_W::new(self, 2)
    }
    ///Bit 3 - Transfer error interrupt enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CRrs> {
        TEIE_W::new(self, 3)
    }
    ///Bit 4 - Data transfer direction This bit is set and cleared by software.
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<'_, CRrs> {
        DIR_W::new(self, 4)
    }
    ///Bit 5 - Circular mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<'_, CRrs> {
        CIRC_W::new(self, 5)
    }
    ///Bit 6 - Peripheral increment mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<'_, CRrs> {
        PINC_W::new(self, 6)
    }
    ///Bit 7 - Memory increment mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<'_, CRrs> {
        MINC_W::new(self, 7)
    }
    ///Bits 8:9 - Peripheral size These bits are set and cleared by software.
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, CRrs> {
        PSIZE_W::new(self, 8)
    }
    ///Bits 10:11 - Memory size These bits are set and cleared by software.
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<'_, CRrs> {
        MSIZE_W::new(self, 10)
    }
    ///Bits 12:13 - Channel priority level These bits are set and cleared by software.
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<'_, CRrs> {
        PL_W::new(self, 12)
    }
    ///Bit 14 - Memory to memory mode This bit is set and cleared by software.
    #[inline(always)]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<'_, CRrs> {
        MEM2MEM_W::new(self, 14)
    }
    ///Bit 15 - double-buffer mode
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W<'_, CRrs> {
        DBM_W::new(self, 15)
    }
    ///Bit 16 - current target memory of DMA transfer in double-buffer mode.
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<'_, CRrs> {
        CT_W::new(self, 16)
    }
}
/**DMA channel x configuration register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
