///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**Stream enable / flag stream ready when read low

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Stream disabled
    Disabled = 0,
    ///1: Stream enabled
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Stream enable / flag stream ready when read low
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
    ///Stream disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    ///Stream enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
///Field `EN` writer - Stream enable / flag stream ready when read low
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Stream disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    ///Stream enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
/**Direct mode error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMEIE {
    ///0: DME interrupt disabled
    Disabled = 0,
    ///1: DME interrupt enabled
    Enabled = 1,
}
impl From<DMEIE> for bool {
    #[inline(always)]
    fn from(variant: DMEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `DMEIE` reader - Direct mode error interrupt enable
pub type DMEIE_R = crate::BitReader<DMEIE>;
impl DMEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMEIE {
        match self.bits {
            false => DMEIE::Disabled,
            true => DMEIE::Enabled,
        }
    }
    ///DME interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMEIE::Disabled
    }
    ///DME interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMEIE::Enabled
    }
}
///Field `DMEIE` writer - Direct mode error interrupt enable
pub type DMEIE_W<'a, REG> = crate::BitWriter<'a, REG, DMEIE>;
impl<'a, REG> DMEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///DME interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMEIE::Disabled)
    }
    ///DME interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DMEIE::Enabled)
    }
}
/**Transfer error interrupt enable

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
///Field `TEIE` reader - Transfer error interrupt enable
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
///Field `TEIE` writer - Transfer error interrupt enable
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
/**Half transfer interrupt enable

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
///Field `HTIE` reader - Half transfer interrupt enable
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
///Field `HTIE` writer - Half transfer interrupt enable
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
/**Transfer complete interrupt enable

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
///Field `TCIE` reader - Transfer complete interrupt enable
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
///Field `TCIE` writer - Transfer complete interrupt enable
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
/**Peripheral flow controller

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PFCTRL {
    ///0: The DMA is the flow controller
    Dma = 0,
    ///1: The peripheral is the flow controller
    Peripheral = 1,
}
impl From<PFCTRL> for bool {
    #[inline(always)]
    fn from(variant: PFCTRL) -> Self {
        variant as u8 != 0
    }
}
///Field `PFCTRL` reader - Peripheral flow controller
pub type PFCTRL_R = crate::BitReader<PFCTRL>;
impl PFCTRL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PFCTRL {
        match self.bits {
            false => PFCTRL::Dma,
            true => PFCTRL::Peripheral,
        }
    }
    ///The DMA is the flow controller
    #[inline(always)]
    pub fn is_dma(&self) -> bool {
        *self == PFCTRL::Dma
    }
    ///The peripheral is the flow controller
    #[inline(always)]
    pub fn is_peripheral(&self) -> bool {
        *self == PFCTRL::Peripheral
    }
}
///Field `PFCTRL` writer - Peripheral flow controller
pub type PFCTRL_W<'a, REG> = crate::BitWriter<'a, REG, PFCTRL>;
impl<'a, REG> PFCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The DMA is the flow controller
    #[inline(always)]
    pub fn dma(self) -> &'a mut crate::W<REG> {
        self.variant(PFCTRL::Dma)
    }
    ///The peripheral is the flow controller
    #[inline(always)]
    pub fn peripheral(self) -> &'a mut crate::W<REG> {
        self.variant(PFCTRL::Peripheral)
    }
}
/**Data transfer direction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIR {
    ///0: Peripheral-to-memory
    PeripheralToMemory = 0,
    ///1: Memory-to-peripheral
    MemoryToPeripheral = 1,
    ///2: Memory-to-memory
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
impl crate::IsEnum for DIR {}
///Field `DIR` reader - Data transfer direction
pub type DIR_R = crate::FieldReader<DIR>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<DIR> {
        match self.bits {
            0 => Some(DIR::PeripheralToMemory),
            1 => Some(DIR::MemoryToPeripheral),
            2 => Some(DIR::MemoryToMemory),
            _ => None,
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
    ///Memory-to-memory
    #[inline(always)]
    pub fn is_memory_to_memory(&self) -> bool {
        *self == DIR::MemoryToMemory
    }
}
///Field `DIR` writer - Data transfer direction
pub type DIR_W<'a, REG> = crate::FieldWriter<'a, REG, 2, DIR>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
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
    ///Memory-to-memory
    #[inline(always)]
    pub fn memory_to_memory(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::MemoryToMemory)
    }
}
/**Circular mode

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
///Field `CIRC` reader - Circular mode
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
///Field `CIRC` writer - Circular mode
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
/**Peripheral increment mode

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
///Field `PINC` reader - Peripheral increment mode
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
///Field `PINC` writer - Peripheral increment mode
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
///Field `MINC` reader - Memory increment mode
pub use PINC_R as MINC_R;
///Field `MINC` writer - Memory increment mode
pub use PINC_W as MINC_W;
/**Peripheral data size

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
///Field `PSIZE` reader - Peripheral data size
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
///Field `PSIZE` writer - Peripheral data size
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
///Field `MSIZE` reader - Memory data size
pub use PSIZE_R as MSIZE_R;
///Field `MSIZE` writer - Memory data size
pub use PSIZE_W as MSIZE_W;
/**Peripheral increment offset size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINCOS {
    ///0: The offset size for the peripheral address calculation is linked to the PSIZE
    Psize = 0,
    ///1: The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)
    Fixed4 = 1,
}
impl From<PINCOS> for bool {
    #[inline(always)]
    fn from(variant: PINCOS) -> Self {
        variant as u8 != 0
    }
}
///Field `PINCOS` reader - Peripheral increment offset size
pub type PINCOS_R = crate::BitReader<PINCOS>;
impl PINCOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINCOS {
        match self.bits {
            false => PINCOS::Psize,
            true => PINCOS::Fixed4,
        }
    }
    ///The offset size for the peripheral address calculation is linked to the PSIZE
    #[inline(always)]
    pub fn is_psize(&self) -> bool {
        *self == PINCOS::Psize
    }
    ///The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)
    #[inline(always)]
    pub fn is_fixed4(&self) -> bool {
        *self == PINCOS::Fixed4
    }
}
///Field `PINCOS` writer - Peripheral increment offset size
pub type PINCOS_W<'a, REG> = crate::BitWriter<'a, REG, PINCOS>;
impl<'a, REG> PINCOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The offset size for the peripheral address calculation is linked to the PSIZE
    #[inline(always)]
    pub fn psize(self) -> &'a mut crate::W<REG> {
        self.variant(PINCOS::Psize)
    }
    ///The offset size for the peripheral address calculation is fixed to 4 (32-bit alignment)
    #[inline(always)]
    pub fn fixed4(self) -> &'a mut crate::W<REG> {
        self.variant(PINCOS::Fixed4)
    }
}
/**Priority level

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
///Field `PL` reader - Priority level
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
///Field `PL` writer - Priority level
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
/**Double buffer mode

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
///Field `DBM` reader - Double buffer mode
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
///Field `DBM` writer - Double buffer mode
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
/**Current target (only in double buffer mode)

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
///Field `CT` reader - Current target (only in double buffer mode)
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
///Field `CT` writer - Current target (only in double buffer mode)
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
/**Enable the DMA to handle bufferable transfers

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRBUFF {
    ///0: Bufferable transfers not enabled
    Disabled = 0,
    ///1: Bufferable transfers enabled
    Enabled = 1,
}
impl From<TRBUFF> for bool {
    #[inline(always)]
    fn from(variant: TRBUFF) -> Self {
        variant as u8 != 0
    }
}
///Field `TRBUFF` reader - Enable the DMA to handle bufferable transfers
pub type TRBUFF_R = crate::BitReader<TRBUFF>;
impl TRBUFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TRBUFF {
        match self.bits {
            false => TRBUFF::Disabled,
            true => TRBUFF::Enabled,
        }
    }
    ///Bufferable transfers not enabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRBUFF::Disabled
    }
    ///Bufferable transfers enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TRBUFF::Enabled
    }
}
///Field `TRBUFF` writer - Enable the DMA to handle bufferable transfers
pub type TRBUFF_W<'a, REG> = crate::BitWriter<'a, REG, TRBUFF>;
impl<'a, REG> TRBUFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bufferable transfers not enabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRBUFF::Disabled)
    }
    ///Bufferable transfers enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TRBUFF::Enabled)
    }
}
/**Peripheral burst transfer configuration

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PBURST {
    ///0: Single transfer
    Single = 0,
    ///1: Incremental burst of 4 beats
    Incr4 = 1,
    ///2: Incremental burst of 8 beats
    Incr8 = 2,
    ///3: Incremental burst of 16 beats
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
impl crate::IsEnum for PBURST {}
///Field `PBURST` reader - Peripheral burst transfer configuration
pub type PBURST_R = crate::FieldReader<PBURST>;
impl PBURST_R {
    ///Get enumerated values variant
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
    ///Single transfer
    #[inline(always)]
    pub fn is_single(&self) -> bool {
        *self == PBURST::Single
    }
    ///Incremental burst of 4 beats
    #[inline(always)]
    pub fn is_incr4(&self) -> bool {
        *self == PBURST::Incr4
    }
    ///Incremental burst of 8 beats
    #[inline(always)]
    pub fn is_incr8(&self) -> bool {
        *self == PBURST::Incr8
    }
    ///Incremental burst of 16 beats
    #[inline(always)]
    pub fn is_incr16(&self) -> bool {
        *self == PBURST::Incr16
    }
}
///Field `PBURST` writer - Peripheral burst transfer configuration
pub type PBURST_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PBURST, crate::Safe>;
impl<'a, REG> PBURST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Single transfer
    #[inline(always)]
    pub fn single(self) -> &'a mut crate::W<REG> {
        self.variant(PBURST::Single)
    }
    ///Incremental burst of 4 beats
    #[inline(always)]
    pub fn incr4(self) -> &'a mut crate::W<REG> {
        self.variant(PBURST::Incr4)
    }
    ///Incremental burst of 8 beats
    #[inline(always)]
    pub fn incr8(self) -> &'a mut crate::W<REG> {
        self.variant(PBURST::Incr8)
    }
    ///Incremental burst of 16 beats
    #[inline(always)]
    pub fn incr16(self) -> &'a mut crate::W<REG> {
        self.variant(PBURST::Incr16)
    }
}
///Field `MBURST` reader - Memory burst transfer configuration
pub use PBURST_R as MBURST_R;
///Field `MBURST` writer - Memory burst transfer configuration
pub use PBURST_W as MBURST_W;
impl R {
    ///Bit 0 - Stream enable / flag stream ready when read low
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Direct mode error interrupt enable
    #[inline(always)]
    pub fn dmeie(&self) -> DMEIE_R {
        DMEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Peripheral flow controller
    #[inline(always)]
    pub fn pfctrl(&self) -> PFCTRL_R {
        PFCTRL_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Data transfer direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Circular mode
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:12 - Peripheral data size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 11) & 3) as u8)
    }
    ///Bits 13:14 - Memory data size
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 13) & 3) as u8)
    }
    ///Bit 15 - Peripheral increment offset size
    #[inline(always)]
    pub fn pincos(&self) -> PINCOS_R {
        PINCOS_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bits 16:17 - Priority level
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 18 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&self) -> DBM_R {
        DBM_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Current target (only in double buffer mode)
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Enable the DMA to handle bufferable transfers
    #[inline(always)]
    pub fn trbuff(&self) -> TRBUFF_R {
        TRBUFF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bits 21:22 - Peripheral burst transfer configuration
    #[inline(always)]
    pub fn pburst(&self) -> PBURST_R {
        PBURST_R::new(((self.bits >> 21) & 3) as u8)
    }
    ///Bits 23:24 - Memory burst transfer configuration
    #[inline(always)]
    pub fn mburst(&self) -> MBURST_R {
        MBURST_R::new(((self.bits >> 23) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("pburst", &self.pburst())
            .field("mburst", &self.mburst())
            .field("ct", &self.ct())
            .field("dbm", &self.dbm())
            .field("pl", &self.pl())
            .field("pincos", &self.pincos())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("circ", &self.circ())
            .field("dir", &self.dir())
            .field("pfctrl", &self.pfctrl())
            .field("tcie", &self.tcie())
            .field("htie", &self.htie())
            .field("teie", &self.teie())
            .field("dmeie", &self.dmeie())
            .field("en", &self.en())
            .field("trbuff", &self.trbuff())
            .finish()
    }
}
impl W {
    ///Bit 0 - Stream enable / flag stream ready when read low
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Direct mode error interrupt enable
    #[inline(always)]
    pub fn dmeie(&mut self) -> DMEIE_W<'_, CRrs> {
        DMEIE_W::new(self, 1)
    }
    ///Bit 2 - Transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CRrs> {
        TEIE_W::new(self, 2)
    }
    ///Bit 3 - Half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<'_, CRrs> {
        HTIE_W::new(self, 3)
    }
    ///Bit 4 - Transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 4)
    }
    ///Bit 5 - Peripheral flow controller
    #[inline(always)]
    pub fn pfctrl(&mut self) -> PFCTRL_W<'_, CRrs> {
        PFCTRL_W::new(self, 5)
    }
    ///Bits 6:7 - Data transfer direction
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<'_, CRrs> {
        DIR_W::new(self, 6)
    }
    ///Bit 8 - Circular mode
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<'_, CRrs> {
        CIRC_W::new(self, 8)
    }
    ///Bit 9 - Peripheral increment mode
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<'_, CRrs> {
        PINC_W::new(self, 9)
    }
    ///Bit 10 - Memory increment mode
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<'_, CRrs> {
        MINC_W::new(self, 10)
    }
    ///Bits 11:12 - Peripheral data size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, CRrs> {
        PSIZE_W::new(self, 11)
    }
    ///Bits 13:14 - Memory data size
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<'_, CRrs> {
        MSIZE_W::new(self, 13)
    }
    ///Bit 15 - Peripheral increment offset size
    #[inline(always)]
    pub fn pincos(&mut self) -> PINCOS_W<'_, CRrs> {
        PINCOS_W::new(self, 15)
    }
    ///Bits 16:17 - Priority level
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<'_, CRrs> {
        PL_W::new(self, 16)
    }
    ///Bit 18 - Double buffer mode
    #[inline(always)]
    pub fn dbm(&mut self) -> DBM_W<'_, CRrs> {
        DBM_W::new(self, 18)
    }
    ///Bit 19 - Current target (only in double buffer mode)
    #[inline(always)]
    pub fn ct(&mut self) -> CT_W<'_, CRrs> {
        CT_W::new(self, 19)
    }
    ///Bit 20 - Enable the DMA to handle bufferable transfers
    #[inline(always)]
    pub fn trbuff(&mut self) -> TRBUFF_W<'_, CRrs> {
        TRBUFF_W::new(self, 20)
    }
    ///Bits 21:22 - Peripheral burst transfer configuration
    #[inline(always)]
    pub fn pburst(&mut self) -> PBURST_W<'_, CRrs> {
        PBURST_W::new(self, 21)
    }
    ///Bits 23:24 - Memory burst transfer configuration
    #[inline(always)]
    pub fn mburst(&mut self) -> MBURST_W<'_, CRrs> {
        MBURST_W::new(self, 23)
    }
}
/**stream x configuration register

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
