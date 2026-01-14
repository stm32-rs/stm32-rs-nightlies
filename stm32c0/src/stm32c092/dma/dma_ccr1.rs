///Register `DMA_CCR1` reader
pub type R = crate::R<DMA_CCR1rs>;
///Register `DMA_CCR1` writer
pub type W = crate::W<DMA_CCR1rs>;
/**Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN {
    ///0: Disabled
    B0x0 = 0,
    ///1: Enabled
    B0x1 = 1,
}
impl From<EN> for bool {
    #[inline(always)]
    fn from(variant: EN) -> Self {
        variant as u8 != 0
    }
}
///Field `EN` reader - Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software.
pub type EN_R = crate::BitReader<EN>;
impl EN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EN {
        match self.bits {
            false => EN::B0x0,
            true => EN::B0x1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EN::B0x0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EN::B0x1
    }
}
///Field `EN` writer - Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software.
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG, EN>;
impl<'a, REG> EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EN::B0x0)
    }
    ///Enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EN::B0x1)
    }
}
/**Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE {
    ///0: Disabled
    B0x0 = 0,
    ///1: Enabled
    B0x1 = 1,
}
impl From<TCIE> for bool {
    #[inline(always)]
    fn from(variant: TCIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCIE` reader - Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type TCIE_R = crate::BitReader<TCIE>;
impl TCIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCIE {
        match self.bits {
            false => TCIE::B0x0,
            true => TCIE::B0x1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCIE::B0x0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCIE::B0x1
    }
}
///Field `TCIE` writer - Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG, TCIE>;
impl<'a, REG> TCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::B0x0)
    }
    ///Enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCIE::B0x1)
    }
}
/**Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE {
    ///0: Disabled
    B0x0 = 0,
    ///1: Enabled
    B0x1 = 1,
}
impl From<HTIE> for bool {
    #[inline(always)]
    fn from(variant: HTIE) -> Self {
        variant as u8 != 0
    }
}
///Field `HTIE` reader - Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type HTIE_R = crate::BitReader<HTIE>;
impl HTIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HTIE {
        match self.bits {
            false => HTIE::B0x0,
            true => HTIE::B0x1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HTIE::B0x0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HTIE::B0x1
    }
}
///Field `HTIE` writer - Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type HTIE_W<'a, REG> = crate::BitWriter<'a, REG, HTIE>;
impl<'a, REG> HTIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::B0x0)
    }
    ///Enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HTIE::B0x1)
    }
}
/**Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE {
    ///0: Disabled
    B0x0 = 0,
    ///1: Enabled
    B0x1 = 1,
}
impl From<TEIE> for bool {
    #[inline(always)]
    fn from(variant: TEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEIE` reader - Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type TEIE_R = crate::BitReader<TEIE>;
impl TEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEIE {
        match self.bits {
            false => TEIE::B0x0,
            true => TEIE::B0x1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEIE::B0x0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEIE::B0x1
    }
}
///Field `TEIE` writer - Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type TEIE_W<'a, REG> = crate::BitWriter<'a, REG, TEIE>;
impl<'a, REG> TEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::B0x0)
    }
    ///Enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEIE::B0x1)
    }
}
/**Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR {
    ///0: Read from peripheral
    B0x0 = 0,
    ///1: Read from memory
    B0x1 = 1,
}
impl From<DIR> for bool {
    #[inline(always)]
    fn from(variant: DIR) -> Self {
        variant as u8 != 0
    }
}
///Field `DIR` reader - Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type DIR_R = crate::BitReader<DIR>;
impl DIR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIR {
        match self.bits {
            false => DIR::B0x0,
            true => DIR::B0x1,
        }
    }
    ///Read from peripheral
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == DIR::B0x0
    }
    ///Read from memory
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == DIR::B0x1
    }
}
///Field `DIR` writer - Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type DIR_W<'a, REG> = crate::BitWriter<'a, REG, DIR>;
impl<'a, REG> DIR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Read from peripheral
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::B0x0)
    }
    ///Read from memory
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(DIR::B0x1)
    }
}
/**Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRC {
    ///0: Disabled
    B0x0 = 0,
    ///1: Enabled
    B0x1 = 1,
}
impl From<CIRC> for bool {
    #[inline(always)]
    fn from(variant: CIRC) -> Self {
        variant as u8 != 0
    }
}
///Field `CIRC` reader - Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type CIRC_R = crate::BitReader<CIRC>;
impl CIRC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CIRC {
        match self.bits {
            false => CIRC::B0x0,
            true => CIRC::B0x1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CIRC::B0x0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CIRC::B0x1
    }
}
///Field `CIRC` writer - Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
pub type CIRC_W<'a, REG> = crate::BitWriter<'a, REG, CIRC>;
impl<'a, REG> CIRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::B0x0)
    }
    ///Enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CIRC::B0x1)
    }
}
/**Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINC {
    ///0: Disabled
    B0x0 = 0,
    ///1: Enabled
    B0x1 = 1,
}
impl From<PINC> for bool {
    #[inline(always)]
    fn from(variant: PINC) -> Self {
        variant as u8 != 0
    }
}
///Field `PINC` reader - Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type PINC_R = crate::BitReader<PINC>;
impl PINC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PINC {
        match self.bits {
            false => PINC::B0x0,
            true => PINC::B0x1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PINC::B0x0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PINC::B0x1
    }
}
///Field `PINC` writer - Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type PINC_W<'a, REG> = crate::BitWriter<'a, REG, PINC>;
impl<'a, REG> PINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::B0x0)
    }
    ///Enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PINC::B0x1)
    }
}
/**Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINC {
    ///0: Disabled
    B0x0 = 0,
    ///1: Enabled
    B0x1 = 1,
}
impl From<MINC> for bool {
    #[inline(always)]
    fn from(variant: MINC) -> Self {
        variant as u8 != 0
    }
}
///Field `MINC` reader - Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type MINC_R = crate::BitReader<MINC>;
impl MINC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MINC {
        match self.bits {
            false => MINC::B0x0,
            true => MINC::B0x1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MINC::B0x0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MINC::B0x1
    }
}
///Field `MINC` writer - Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type MINC_W<'a, REG> = crate::BitWriter<'a, REG, MINC>;
impl<'a, REG> MINC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MINC::B0x0)
    }
    ///Enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MINC::B0x1)
    }
}
/**Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE {
    ///0: 8 bits
    B0x0 = 0,
    ///1: 16 bits
    B0x1 = 1,
    ///2: 32 bits
    B0x2 = 2,
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
///Field `PSIZE` reader - Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type PSIZE_R = crate::FieldReader<PSIZE>;
impl PSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PSIZE> {
        match self.bits {
            0 => Some(PSIZE::B0x0),
            1 => Some(PSIZE::B0x1),
            2 => Some(PSIZE::B0x2),
            _ => None,
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PSIZE::B0x0
    }
    ///16 bits
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PSIZE::B0x1
    }
    ///32 bits
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PSIZE::B0x2
    }
}
///Field `PSIZE` writer - Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type PSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PSIZE>;
impl<'a, REG> PSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::B0x0)
    }
    ///16 bits
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::B0x1)
    }
    ///32 bits
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PSIZE::B0x2)
    }
}
/**Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSIZE {
    ///0: 8 bits
    B0x0 = 0,
    ///1: 16 bits
    B0x1 = 1,
    ///2: 32 bits
    B0x2 = 2,
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
impl crate::IsEnum for MSIZE {}
///Field `MSIZE` reader - Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type MSIZE_R = crate::FieldReader<MSIZE>;
impl MSIZE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MSIZE> {
        match self.bits {
            0 => Some(MSIZE::B0x0),
            1 => Some(MSIZE::B0x1),
            2 => Some(MSIZE::B0x2),
            _ => None,
        }
    }
    ///8 bits
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MSIZE::B0x0
    }
    ///16 bits
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MSIZE::B0x1
    }
    ///32 bits
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == MSIZE::B0x2
    }
}
///Field `MSIZE` writer - Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type MSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, MSIZE>;
impl<'a, REG> MSIZE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///8 bits
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MSIZE::B0x0)
    }
    ///16 bits
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MSIZE::B0x1)
    }
    ///32 bits
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MSIZE::B0x2)
    }
}
/**Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL {
    ///0: Low
    B0x0 = 0,
    ///1: Medium
    B0x1 = 1,
    ///2: High
    B0x2 = 2,
    ///3: Very high
    B0x3 = 3,
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
///Field `PL` reader - Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type PL_R = crate::FieldReader<PL>;
impl PL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PL {
        match self.bits {
            0 => PL::B0x0,
            1 => PL::B0x1,
            2 => PL::B0x2,
            3 => PL::B0x3,
            _ => unreachable!(),
        }
    }
    ///Low
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PL::B0x0
    }
    ///Medium
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PL::B0x1
    }
    ///High
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == PL::B0x2
    }
    ///Very high
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == PL::B0x3
    }
}
///Field `PL` writer - Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type PL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PL, crate::Safe>;
impl<'a, REG> PL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Low
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PL::B0x0)
    }
    ///Medium
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PL::B0x1)
    }
    ///High
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(PL::B0x2)
    }
    ///Very high
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(PL::B0x3)
    }
}
/**Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM2MEM {
    ///0: Disabled
    B0x0 = 0,
    ///1: Enabled
    B0x1 = 1,
}
impl From<MEM2MEM> for bool {
    #[inline(always)]
    fn from(variant: MEM2MEM) -> Self {
        variant as u8 != 0
    }
}
///Field `MEM2MEM` reader - Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type MEM2MEM_R = crate::BitReader<MEM2MEM>;
impl MEM2MEM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MEM2MEM {
        match self.bits {
            false => MEM2MEM::B0x0,
            true => MEM2MEM::B0x1,
        }
    }
    ///Disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MEM2MEM::B0x0
    }
    ///Enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MEM2MEM::B0x1
    }
}
///Field `MEM2MEM` writer - Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
pub type MEM2MEM_W<'a, REG> = crate::BitWriter<'a, REG, MEM2MEM>;
impl<'a, REG> MEM2MEM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::B0x0)
    }
    ///Enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MEM2MEM::B0x1)
    }
}
impl R {
    ///Bit 0 - Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software.
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:9 - Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:13 - Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bit 14 - Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CCR1")
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
            .finish()
    }
}
impl W {
    ///Bit 0 - Channel enable When a channel transfer error occurs, this bit is cleared by hardware. It can not be set again by software (channel x re-activated) until the TEIFx bit of the DMA_ISR register is cleared (by setting the CTEIFx bit of the DMA_IFCR register). Note: This bit is set and cleared by software.
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, DMA_CCR1rs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - Transfer complete interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, DMA_CCR1rs> {
        TCIE_W::new(self, 1)
    }
    ///Bit 2 - Half transfer interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn htie(&mut self) -> HTIE_W<'_, DMA_CCR1rs> {
        HTIE_W::new(self, 2)
    }
    ///Bit 3 - Transfer error interrupt enable Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn teie(&mut self) -> TEIE_W<'_, DMA_CCR1rs> {
        TEIE_W::new(self, 3)
    }
    ///Bit 4 - Data transfer direction This bit must be set only in memory-to-peripheral and peripheral-to-memory modes. Source attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Destination attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Destination attributes are defined by PSIZE and PINC, plus the DMA_CPARx register. This is still valid in a memory-to-memory mode. Source attributes are defined by MSIZE and MINC, plus the DMA_CMARx register. This is still valid in a peripheral-to-peripheral mode. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn dir(&mut self) -> DIR_W<'_, DMA_CCR1rs> {
        DIR_W::new(self, 4)
    }
    ///Bit 5 - Circular mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn circ(&mut self) -> CIRC_W<'_, DMA_CCR1rs> {
        CIRC_W::new(self, 5)
    }
    ///Bit 6 - Peripheral increment mode Defines the increment mode for each DMA transfer to the identified peripheral. n memory-to-memory mode, this bit identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn pinc(&mut self) -> PINC_W<'_, DMA_CCR1rs> {
        PINC_W::new(self, 6)
    }
    ///Bit 7 - Memory increment mode Defines the increment mode for each DMA transfer to the identified memory. In memory-to-memory mode, this bit identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bit identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn minc(&mut self) -> MINC_W<'_, DMA_CCR1rs> {
        MINC_W::new(self, 7)
    }
    ///Bits 8:9 - Peripheral size Defines the data size of each DMA transfer to the identified peripheral. In memory-to-memory mode, this bitfield identifies the memory destination if DIR = 1 and the memory source if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral destination if DIR = 1 and the peripheral source if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn psize(&mut self) -> PSIZE_W<'_, DMA_CCR1rs> {
        PSIZE_W::new(self, 8)
    }
    ///Bits 10:11 - Memory size Defines the data size of each DMA transfer to the identified memory. In memory-to-memory mode, this bitfield identifies the memory source if DIR = 1 and the memory destination if DIR = 0. In peripheral-to-peripheral mode, this bitfield identifies the peripheral source if DIR = 1 and the peripheral destination if DIR = 0. Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn msize(&mut self) -> MSIZE_W<'_, DMA_CCR1rs> {
        MSIZE_W::new(self, 10)
    }
    ///Bits 12:13 - Priority level Note: This bitfield is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn pl(&mut self) -> PL_W<'_, DMA_CCR1rs> {
        PL_W::new(self, 12)
    }
    ///Bit 14 - Memory-to-memory mode Note: This bit is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is read-only when the channel is enabled (EN = 1).
    #[inline(always)]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<'_, DMA_CCR1rs> {
        MEM2MEM_W::new(self, 14)
    }
}
/**DMA channel 1 configuration register

You can [`read`](crate::Reg::read) this register and get [`dma_ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#DMA:DMA_CCR1)*/
pub struct DMA_CCR1rs;
impl crate::RegisterSpec for DMA_CCR1rs {
    type Ux = u32;
}
///`read()` method returns [`dma_ccr1::R`](R) reader structure
impl crate::Readable for DMA_CCR1rs {}
///`write(|w| ..)` method takes [`dma_ccr1::W`](W) writer structure
impl crate::Writable for DMA_CCR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMA_CCR1 to value 0
impl crate::Resettable for DMA_CCR1rs {}
