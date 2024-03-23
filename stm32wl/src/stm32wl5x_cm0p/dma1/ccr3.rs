#[doc = "Register `CCR3` reader"]
pub type R = crate::R<CCR3rs>;
#[doc = "Register `CCR3` writer"]
pub type W = crate::W<CCR3rs>;
#[doc = "channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EN` reader - channel enable"]
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
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN::Enabled
    }
}
#[doc = "Field `EN` writer - channel enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EN::Enabled)
    }
}
#[doc = "transfer complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCIE` reader - transfer complete interrupt enable"]
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
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE::Enabled
    }
}
#[doc = "Field `TCIE` writer - transfer complete interrupt enable"]
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::Enabled)
    }
}
#[doc = "half transfer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<HTIE> for bool {
    #[inline(always)]
    fn from(variant: HTIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HTIE` reader - half transfer interrupt enable"]
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
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTIE::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTIE::Enabled
    }
}
#[doc = "Field `HTIE` writer - half transfer interrupt enable"]
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG, HTIE>;
impl<'a, REG> HTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::Enabled)
    }
}
#[doc = "transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEIE` reader - transfer error interrupt enable"]
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
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE::Enabled
    }
}
#[doc = "Field `TEIE` writer - transfer error interrupt enable"]
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::Enabled)
    }
}
#[doc = "data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    #[doc = "0: Read from peripheral"]
    Peripheral = 0,
    #[doc = "1: Read from memory"]
    Memory = 1,
}
impl From<DIR> for bool {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIR` reader - data transfer direction"]
pub type DIR_R = crate::BitReader<DIR>;
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DIR {
        match self.bits {
            false => DIR::Peripheral,
            true => DIR::Memory,
        }
    }
    #[doc = "Read from peripheral"]
    #[inline(always)]
    pub fn is_peripheral(&self) -> bool {
        *self == DIR::Peripheral
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn is_memory(&self) -> bool {
        *self == DIR::Memory
    }
}
#[doc = "Field `DIR` writer - data transfer direction"]
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Read from peripheral"]
    #[inline(always)]
    pub fn peripheral(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::Peripheral)
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn memory(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::Memory)
    }
}
#[doc = "circular mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRC {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<CIRC> for bool {
    #[inline(always)]
    fn from(variant: CIRC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CIRC` reader - circular mode"]
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
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRC::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRC::Enabled
    }
}
#[doc = "Field `CIRC` writer - circular mode"]
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG, CIRC>;
impl<'a, REG> CIRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::Enabled)
    }
}
#[doc = "peripheral increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINC {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<PINC> for bool {
    #[inline(always)]
    fn from(variant: PINC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PINC` reader - peripheral increment mode"]
pub type PINC_R = crate::BitReader<PINC>;
impl PINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PINC {
        match self.bits {
            false => PINC::Disabled,
            true => PINC::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINC::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINC::Enabled
    }
}
#[doc = "Field `PINC` writer - peripheral increment mode"]
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG, PINC>;
impl<'a, REG> PINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::Enabled)
    }
}
#[doc = "memory increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINC {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<MINC> for bool {
    #[inline(always)]
    fn from(variant: MINC) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MINC` reader - memory increment mode"]
pub type MINC_R = crate::BitReader<MINC>;
impl MINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MINC {
        match self.bits {
            false => MINC::Disabled,
            true => MINC::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MINC::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MINC::Enabled
    }
}
#[doc = "Field `MINC` writer - memory increment mode"]
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG, MINC>;
impl<'a, REG> MINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MINC::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MINC::Enabled)
    }
}
#[doc = "peripheral size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE {
    #[doc = "0: 8 bits"]
    Bits8 = 0,
    #[doc = "1: 16 bits"]
    Bits16 = 1,
    #[doc = "2: 32 bits"]
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
#[doc = "Field `PSIZE` reader - peripheral size"]
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
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PSIZE::Bits8
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PSIZE::Bits16
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PSIZE::Bits32
    }
}
#[doc = "Field `PSIZE` writer - peripheral size"]
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PSIZE>;
impl<'a, REG> PSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits8)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits16)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::Bits32)
    }
}
#[doc = "memory size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIZE {
    #[doc = "0: 8 bits"]
    Bits8 = 0,
    #[doc = "1: 16 bits"]
    Bits16 = 1,
    #[doc = "2: 32 bits"]
    Bits32 = 2,
}
impl From<MSIZE> for u8 {
    #[inline(always)]
    fn from(variant: MSIZE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MSIZE {
    type Ux = u8;
}
#[doc = "Field `MSIZE` reader - memory size"]
pub type MSIZE_R = crate::FieldReader<MSIZE>;
impl MSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSIZE> {
        match self.bits {
            0 => Some(MSIZE::Bits8),
            1 => Some(MSIZE::Bits16),
            2 => Some(MSIZE::Bits32),
            _ => None,
        }
    }
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == MSIZE::Bits8
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == MSIZE::Bits16
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == MSIZE::Bits32
    }
}
#[doc = "Field `MSIZE` writer - memory size"]
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MSIZE>;
impl<'a, REG> MSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bits"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut crate::W<REG> {
        self.variant(MSIZE::Bits8)
    }
    #[doc = "16 bits"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut crate::W<REG> {
        self.variant(MSIZE::Bits16)
    }
    #[doc = "32 bits"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut crate::W<REG> {
        self.variant(MSIZE::Bits32)
    }
}
#[doc = "priority level\n\nValue on reset: 0"]
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
#[doc = "Field `PL` reader - priority level"]
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
#[doc = "Field `PL` writer - priority level"]
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
#[doc = "memory-to-memory mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM2MEM {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<MEM2MEM> for bool {
    #[inline(always)]
    fn from(variant: MEM2MEM) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MEM2MEM` reader - memory-to-memory mode"]
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
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MEM2MEM::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MEM2MEM::Enabled
    }
}
#[doc = "Field `MEM2MEM` writer - memory-to-memory mode"]
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG, MEM2MEM>;
impl<'a, REG> MEM2MEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::Enabled)
    }
}
#[doc = "Field `SECM` reader - ecure mode"]
pub type SECM_R = crate::BitReader;
#[doc = "Field `SECM` writer - ecure mode"]
pub type SECM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSEC` reader - ecurity of the DMA transfer from the source"]
pub type SSEC_R = crate::BitReader;
#[doc = "Field `SSEC` writer - ecurity of the DMA transfer from the source"]
pub type SSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSEC` reader - ecurity of the DMA transfer to the destination"]
pub type DSEC_R = crate::BitReader;
#[doc = "Field `DSEC` writer - ecurity of the DMA transfer to the destination"]
pub type DSEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "rivileged mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRIV {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<PRIV> for bool {
    #[inline(always)]
    fn from(variant: PRIV) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRIV` reader - rivileged mode"]
pub type PRIV_R = crate::BitReader<PRIV>;
impl PRIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRIV {
        match self.bits {
            false => PRIV::Disabled,
            true => PRIV::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRIV::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRIV::Enabled
    }
}
#[doc = "Field `PRIV` writer - rivileged mode"]
pub type PRIV_W<'a, REG> = crate::BitWriter<'a, REG, PRIV>;
impl<'a, REG> PRIV_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRIV::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - half transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - peripheral size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - memory size"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - memory-to-memory mode"]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - ecure mode"]
    #[inline(always)]
    pub fn secm(&self) -> SECM_R {
        SECM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - ecurity of the DMA transfer from the source"]
    #[inline(always)]
    pub fn ssec(&self) -> SSEC_R {
        SSEC_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ecurity of the DMA transfer to the destination"]
    #[inline(always)]
    pub fn dsec(&self) -> DSEC_R {
        DSEC_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - rivileged mode"]
    #[inline(always)]
    pub fn priv_(&self) -> PRIV_R {
        PRIV_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CCR3rs> {
        EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<CCR3rs> {
        TCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - half transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<CCR3rs> {
        HTIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<CCR3rs> {
        TEIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<CCR3rs> {
        DIR_W::new(self, 4)
    }
    #[doc = "Bit 5 - circular mode"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<CCR3rs> {
        CIRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<CCR3rs> {
        PINC_W::new(self, 6)
    }
    #[doc = "Bit 7 - memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<CCR3rs> {
        MINC_W::new(self, 7)
    }
    #[doc = "Bits 8:9 - peripheral size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<CCR3rs> {
        PSIZE_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - memory size"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<CCR3rs> {
        MSIZE_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - priority level"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<CCR3rs> {
        PL_W::new(self, 12)
    }
    #[doc = "Bit 14 - memory-to-memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<CCR3rs> {
        MEM2MEM_W::new(self, 14)
    }
    #[doc = "Bit 17 - ecure mode"]
    #[inline(always)]
    #[must_use]
    pub fn secm(&mut self) -> SECM_W<CCR3rs> {
        SECM_W::new(self, 17)
    }
    #[doc = "Bit 18 - ecurity of the DMA transfer from the source"]
    #[inline(always)]
    #[must_use]
    pub fn ssec(&mut self) -> SSEC_W<CCR3rs> {
        SSEC_W::new(self, 18)
    }
    #[doc = "Bit 19 - ecurity of the DMA transfer to the destination"]
    #[inline(always)]
    #[must_use]
    pub fn dsec(&mut self) -> DSEC_W<CCR3rs> {
        DSEC_W::new(self, 19)
    }
    #[doc = "Bit 20 - rivileged mode"]
    #[inline(always)]
    #[must_use]
    pub fn priv_(&mut self) -> PRIV_W<CCR3rs> {
        PRIV_W::new(self, 20)
    }
}
#[doc = "channel x configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ccr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ccr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CCR3rs;
impl crate::RegisterSpec for CCR3rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr3::R`](R) reader structure"]
impl crate::Readable for CCR3rs {}
#[doc = "`write(|w| ..)` method takes [`ccr3::W`](W) writer structure"]
impl crate::Writable for CCR3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CCR3 to value 0"]
impl crate::Resettable for CCR3rs {
    const RESET_VALUE: u32 = 0;
}
