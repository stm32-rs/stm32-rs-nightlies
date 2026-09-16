///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**channel enable

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
///Field `EN` reader - channel enable
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
///Field `EN` writer - channel enable
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
/**transfer complete interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    ///0: Transfer Complete interrupt disabled
    Disabled = 0,
    ///1: Transfer Complete interrupt enabled
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - transfer complete interrupt enable
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
    ///Transfer Complete interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    ///Transfer Complete interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
///Field `TCIE` writer - transfer complete interrupt enable
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer Complete interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    ///Transfer Complete interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
/**half transfer interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE {
    ///0: Half Transfer interrupt disabled
    Disabled = 0,
    ///1: Half Transfer interrupt enabled
    Enabled = 1,
}
impl From<HTIE> for bool {
    #[inline(always)]
    fn from(variant: HTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIE` reader - half transfer interrupt enable
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
    ///Half Transfer interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTIE::Disabled
    }
    ///Half Transfer interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTIE::Enabled
    }
}
///Field `HTIE` writer - half transfer interrupt enable
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG, HTIE>;
impl<'a, REG> HTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Half Transfer interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::Disabled)
    }
    ///Half Transfer interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::Enabled)
    }
}
/**transfer error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    ///0: Transfer Error interrupt disabled
    Disabled = 0,
    ///1: Transfer Error interrupt enabled
    Enabled = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - transfer error interrupt enable
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
    ///Transfer Error interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE::Disabled
    }
    ///Transfer Error interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE::Enabled
    }
}
///Field `TEIE` writer - transfer error interrupt enable
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transfer Error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Disabled)
    }
    ///Transfer Error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Enabled)
    }
}
/**data transfer direction

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    ///0: Read from peripheral
    FromPeripheral = 0,
    ///1: Read from memory
    FromMemory = 1,
}
impl From<DIR> for bool {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - data transfer direction
pub type DIR_R = crate::BitReader<DIR>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIR {
        match self.bits {
            false => DIR::FromPeripheral,
            true => DIR::FromMemory,
        }
    }
    ///Read from peripheral
    #[inline(always)]
    pub fn is_from_peripheral(&self) -> bool {
        *self == DIR::FromPeripheral
    }
    ///Read from memory
    #[inline(always)]
    pub fn is_from_memory(&self) -> bool {
        *self == DIR::FromMemory
    }
}
///Field `DIR` writer - data transfer direction
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read from peripheral
    #[inline(always)]
    pub fn from_peripheral(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::FromPeripheral)
    }
    ///Read from memory
    #[inline(always)]
    pub fn from_memory(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::FromMemory)
    }
}
/**circular mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRC {
    ///0: Circular buffer disabled
    Disabled = 0,
    ///1: Circular buffer enabled
    Enabled = 1,
}
impl From<CIRC> for bool {
    #[inline(always)]
    fn from(variant: CIRC) -> Self {
        variant as u8 != 0
    }
}
///Field `CIRC` reader - circular mode
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
    ///Circular buffer disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRC::Disabled
    }
    ///Circular buffer enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRC::Enabled
    }
}
///Field `CIRC` writer - circular mode
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG, CIRC>;
impl<'a, REG> CIRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Circular buffer disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::Disabled)
    }
    ///Circular buffer enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::Enabled)
    }
}
/**peripheral increment mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINC {
    ///0: Increment mode disabled
    Disabled = 0,
    ///1: Increment mode enabled
    Enabled = 1,
}
impl From<PINC> for bool {
    #[inline(always)]
    fn from(variant: PINC) -> Self {
        variant as u8 != 0
    }
}
///Field `PINC` reader - peripheral increment mode
pub type PINC_R = crate::BitReader<PINC>;
impl PINC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINC {
        match self.bits {
            false => PINC::Disabled,
            true => PINC::Enabled,
        }
    }
    ///Increment mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINC::Disabled
    }
    ///Increment mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINC::Enabled
    }
}
///Field `PINC` writer - peripheral increment mode
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG, PINC>;
impl<'a, REG> PINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Increment mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::Disabled)
    }
    ///Increment mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::Enabled)
    }
}
///Field `MINC` reader - memory increment mode
pub use PINC_R as MINC_R;
///Field `MINC` writer - memory increment mode
pub use PINC_W as MINC_W;
/**peripheral size

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE {
    ///0: 8-bit size
    Bits8 = 0,
    ///1: 16-bit size
    Bits16 = 1,
    ///2: 32-bit size
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
///Field `PSIZE` reader - peripheral size
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
    ///8-bit size
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PSIZE::Bits8
    }
    ///16-bit size
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PSIZE::Bits16
    }
    ///32-bit size
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PSIZE::Bits32
    }
}
///Field `PSIZE` writer - peripheral size
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PSIZE>;
impl<'a, REG> PSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8-bit size
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits8)
    }
    ///16-bit size
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits16)
    }
    ///32-bit size
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits32)
    }
}
///Field `MSIZE` reader - memory size
pub use PSIZE_R as MSIZE_R;
///Field `MSIZE` writer - memory size
pub use PSIZE_W as MSIZE_W;
/**priority level

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL {
    ///0: Low priority
    Low = 0,
    ///1: Medium priority
    Medium = 1,
    ///2: High priority
    High = 2,
    ///3: Very high priority
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
///Field `PL` reader - priority level
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
    ///Low priority
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PL::Low
    }
    ///Medium priority
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PL::Medium
    }
    ///High priority
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PL::High
    }
    ///Very high priority
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PL::VeryHigh
    }
}
///Field `PL` writer - priority level
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PL, crate::Safe>;
impl<'a, REG> PL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Low priority
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(PL::Low)
    }
    ///Medium priority
    #[inline(always)]
    pub fn medium(self) -> &'a mut crate::W<REG> {
        self.variant(PL::Medium)
    }
    ///High priority
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(PL::High)
    }
    ///Very high priority
    #[inline(always)]
    pub fn very_high(self) -> &'a mut crate::W<REG> {
        self.variant(PL::VeryHigh)
    }
}
/**memory-to-memory mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM2MEM {
    ///0: Memory to memory mode disabled
    Disabled = 0,
    ///1: Memory to memory mode enabled
    Enabled = 1,
}
impl From<MEM2MEM> for bool {
    #[inline(always)]
    fn from(variant: MEM2MEM) -> Self {
        variant as u8 != 0
    }
}
///Field `MEM2MEM` reader - memory-to-memory mode
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
    ///Memory to memory mode disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MEM2MEM::Disabled
    }
    ///Memory to memory mode enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MEM2MEM::Enabled
    }
}
///Field `MEM2MEM` writer - memory-to-memory mode
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG, MEM2MEM>;
impl<'a, REG> MEM2MEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Memory to memory mode disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::Disabled)
    }
    ///Memory to memory mode enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::Enabled)
    }
}
///Field `SECM` reader - ecure mode
pub type SECM_R = crate::BitReader;
///Field `SECM` writer - ecure mode
pub type SECM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SSEC` reader - ecurity of the DMA transfer from the source
pub type SSEC_R = crate::BitReader;
///Field `SSEC` writer - ecurity of the DMA transfer from the source
pub type SSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DSEC` reader - ecurity of the DMA transfer to the destination
pub type DSEC_R = crate::BitReader;
///Field `DSEC` writer - ecurity of the DMA transfer to the destination
pub type DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
/**rivileged mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV {
    ///0: Disabled
    Disabled = 0,
    ///1: Enabled
    Enabled = 1,
}
impl From<PRIV> for bool {
    #[inline(always)]
    fn from(variant: PRIV) -> Self {
        variant as u8 != 0
    }
}
///Field `PRIV` reader - rivileged mode
pub type PRIV_R = crate::BitReader<PRIV>;
impl PRIV_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRIV {
        match self.bits {
            false => PRIV::Disabled,
            true => PRIV::Enabled,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRIV::Disabled
    }
    ///Enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRIV::Enabled
    }
}
///Field `PRIV` writer - rivileged mode
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG, PRIV>;
impl<'a, REG> PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV::Disabled)
    }
    ///Enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV::Enabled)
    }
}
impl R {
    ///Bit 0 - channel enable
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - data transfer direction
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - circular mode
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - peripheral increment mode
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - memory increment mode
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - peripheral size
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - memory size
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - priority level
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - memory-to-memory mode
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 17 - ecure mode
    #[inline(always)]
    pub fn secm(&self) -> SECM_R {
        SECM_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - ecurity of the DMA transfer from the source
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - ecurity of the DMA transfer to the destination
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - rivileged mode
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("priv_", &self.priv_())
            .field("dsec", &self.dsec())
            .field("ssec", &self.ssec())
            .field("secm", &self.secm())
            .field("mem2mem", &self.mem2mem())
            .field("pl", &self.pl())
            .field("psize", &self.psize())
            .field("msize", &self.msize())
            .field("pinc", &self.pinc())
            .field("minc", &self.minc())
            .field("circ", &self.circ())
            .field("dir", &self.dir())
            .field("teie", &self.teie())
            .field("htie", &self.htie())
            .field("tcie", &self.tcie())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    ///Bit 0 - channel enable
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - transfer complete interrupt enable
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, CRrs> {
        TCIE_W::new(self, 1)
    }
    ///Bit 2 - half transfer interrupt enable
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<'_, CRrs> {
        HTIE_W::new(self, 2)
    }
    ///Bit 3 - transfer error interrupt enable
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, CRrs> {
        TEIE_W::new(self, 3)
    }
    ///Bit 4 - data transfer direction
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<'_, CRrs> {
        DIR_W::new(self, 4)
    }
    ///Bit 5 - circular mode
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<'_, CRrs> {
        CIRC_W::new(self, 5)
    }
    ///Bit 6 - peripheral increment mode
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<'_, CRrs> {
        PINC_W::new(self, 6)
    }
    ///Bit 7 - memory increment mode
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<'_, CRrs> {
        MINC_W::new(self, 7)
    }
    ///Bits 8:9 - peripheral size
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, CRrs> {
        PSIZE_W::new(self, 8)
    }
    ///Bits 10:11 - memory size
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<'_, CRrs> {
        MSIZE_W::new(self, 10)
    }
    ///Bits 12:13 - priority level
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<'_, CRrs> {
        PL_W::new(self, 12)
    }
    ///Bit 14 - memory-to-memory mode
    #[inline(always)]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<'_, CRrs> {
        MEM2MEM_W::new(self, 14)
    }
    ///Bit 17 - ecure mode
    #[inline(always)]
    pub fn secm(&mut self) -> SECM_W<'_, CRrs> {
        SECM_W::new(self, 17)
    }
    ///Bit 18 - ecurity of the DMA transfer from the source
    #[inline(always)]
    pub fn ssec(&mut self) -> SSEC_W<'_, CRrs> {
        SSEC_W::new(self, 18)
    }
    ///Bit 19 - ecurity of the DMA transfer to the destination
    #[inline(always)]
    pub fn dsec(&mut self) -> DSEC_W<'_, CRrs> {
        DSEC_W::new(self, 19)
    }
    ///Bit 20 - rivileged mode
    #[inline(always)]
    pub fn priv_(&mut self) -> PRIV_W<'_, CRrs> {
        PRIV_W::new(self, 20)
    }
}
/**channel x configuration register

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
