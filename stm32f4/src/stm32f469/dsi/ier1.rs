///Register `IER1` reader
pub type R = crate::R<IER1rs>;
///Register `IER1` writer
pub type W = crate::W<IER1rs>;
/**Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission .

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOHSTXIE {
    ///0: Interrupt on timeout high-speed transmission disabled
    B0x0 = 0,
    ///1: Interrupt on timeout high-speed transmission enabled
    B0x1 = 1,
}
impl From<TOHSTXIE> for bool {
    #[inline(always)]
    fn from(variant: TOHSTXIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TOHSTXIE` reader - Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission .
pub type TOHSTXIE_R = crate::BitReader<TOHSTXIE>;
impl TOHSTXIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOHSTXIE {
        match self.bits {
            false => TOHSTXIE::B0x0,
            true => TOHSTXIE::B0x1,
        }
    }
    ///Interrupt on timeout high-speed transmission disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOHSTXIE::B0x0
    }
    ///Interrupt on timeout high-speed transmission enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOHSTXIE::B0x1
    }
}
///Field `TOHSTXIE` writer - Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission .
pub type TOHSTXIE_W<'a, REG> = crate::BitWriter<'a, REG, TOHSTXIE>;
impl<'a, REG> TOHSTXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on timeout high-speed transmission disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOHSTXIE::B0x0)
    }
    ///Interrupt on timeout high-speed transmission enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOHSTXIE::B0x1)
    }
}
/**Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOLPRXIE {
    ///0: Interrupt on timeout low-power reception disabled
    B0x0 = 0,
    ///1: Interrupt on timeout low-power reception enabled
    B0x1 = 1,
}
impl From<TOLPRXIE> for bool {
    #[inline(always)]
    fn from(variant: TOLPRXIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TOLPRXIE` reader - Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception.
pub type TOLPRXIE_R = crate::BitReader<TOLPRXIE>;
impl TOLPRXIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOLPRXIE {
        match self.bits {
            false => TOLPRXIE::B0x0,
            true => TOLPRXIE::B0x1,
        }
    }
    ///Interrupt on timeout low-power reception disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOLPRXIE::B0x0
    }
    ///Interrupt on timeout low-power reception enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOLPRXIE::B0x1
    }
}
///Field `TOLPRXIE` writer - Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception.
pub type TOLPRXIE_W<'a, REG> = crate::BitWriter<'a, REG, TOLPRXIE>;
impl<'a, REG> TOLPRXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on timeout low-power reception disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOLPRXIE::B0x0)
    }
    ///Interrupt on timeout low-power reception enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOLPRXIE::B0x1)
    }
}
/**ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCSEIE {
    ///0: Interrupt on ECC single-bit error disabled
    B0x0 = 0,
    ///1: Interrupt on ECC single-bit error enabled
    B0x1 = 1,
}
impl From<ECCSEIE> for bool {
    #[inline(always)]
    fn from(variant: ECCSEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCSEIE` reader - ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error.
pub type ECCSEIE_R = crate::BitReader<ECCSEIE>;
impl ECCSEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCSEIE {
        match self.bits {
            false => ECCSEIE::B0x0,
            true => ECCSEIE::B0x1,
        }
    }
    ///Interrupt on ECC single-bit error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ECCSEIE::B0x0
    }
    ///Interrupt on ECC single-bit error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ECCSEIE::B0x1
    }
}
///Field `ECCSEIE` writer - ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error.
pub type ECCSEIE_W<'a, REG> = crate::BitWriter<'a, REG, ECCSEIE>;
impl<'a, REG> ECCSEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on ECC single-bit error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCSEIE::B0x0)
    }
    ///Interrupt on ECC single-bit error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCSEIE::B0x1)
    }
}
/**ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ECCMEIE {
    ///0: Interrupt on ECC multi-bit error disabled
    B0x0 = 0,
    ///1: Interrupt on ECC multi-bit error enabled
    B0x1 = 1,
}
impl From<ECCMEIE> for bool {
    #[inline(always)]
    fn from(variant: ECCMEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `ECCMEIE` reader - ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error.
pub type ECCMEIE_R = crate::BitReader<ECCMEIE>;
impl ECCMEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ECCMEIE {
        match self.bits {
            false => ECCMEIE::B0x0,
            true => ECCMEIE::B0x1,
        }
    }
    ///Interrupt on ECC multi-bit error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ECCMEIE::B0x0
    }
    ///Interrupt on ECC multi-bit error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ECCMEIE::B0x1
    }
}
///Field `ECCMEIE` writer - ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error.
pub type ECCMEIE_W<'a, REG> = crate::BitWriter<'a, REG, ECCMEIE>;
impl<'a, REG> ECCMEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on ECC multi-bit error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMEIE::B0x0)
    }
    ///Interrupt on ECC multi-bit error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ECCMEIE::B0x1)
    }
}
/**CRC error interrupt enable This bit enables the interrupt generation on CRC error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEIE {
    ///0: Interrupt on CRC error disabled
    B0x0 = 0,
    ///1: Interrupt on CRC error enabled
    B0x1 = 1,
}
impl From<CRCEIE> for bool {
    #[inline(always)]
    fn from(variant: CRCEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `CRCEIE` reader - CRC error interrupt enable This bit enables the interrupt generation on CRC error.
pub type CRCEIE_R = crate::BitReader<CRCEIE>;
impl CRCEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CRCEIE {
        match self.bits {
            false => CRCEIE::B0x0,
            true => CRCEIE::B0x1,
        }
    }
    ///Interrupt on CRC error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CRCEIE::B0x0
    }
    ///Interrupt on CRC error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CRCEIE::B0x1
    }
}
///Field `CRCEIE` writer - CRC error interrupt enable This bit enables the interrupt generation on CRC error.
pub type CRCEIE_W<'a, REG> = crate::BitWriter<'a, REG, CRCEIE>;
impl<'a, REG> CRCEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on CRC error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEIE::B0x0)
    }
    ///Interrupt on CRC error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCEIE::B0x1)
    }
}
/**Packet size error interrupt enable This bit enables the interrupt generation on packet size error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PSEIE {
    ///0: Interrupt on packet size error disabled
    B0x0 = 0,
    ///1: Interrupt on packet size error enabled
    B0x1 = 1,
}
impl From<PSEIE> for bool {
    #[inline(always)]
    fn from(variant: PSEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `PSEIE` reader - Packet size error interrupt enable This bit enables the interrupt generation on packet size error.
pub type PSEIE_R = crate::BitReader<PSEIE>;
impl PSEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PSEIE {
        match self.bits {
            false => PSEIE::B0x0,
            true => PSEIE::B0x1,
        }
    }
    ///Interrupt on packet size error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PSEIE::B0x0
    }
    ///Interrupt on packet size error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PSEIE::B0x1
    }
}
///Field `PSEIE` writer - Packet size error interrupt enable This bit enables the interrupt generation on packet size error.
pub type PSEIE_W<'a, REG> = crate::BitWriter<'a, REG, PSEIE>;
impl<'a, REG> PSEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on packet size error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PSEIE::B0x0)
    }
    ///Interrupt on packet size error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PSEIE::B0x1)
    }
}
/**EoTp error interrupt enable This bit enables the interrupt generation on EoTp error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOTPEIE {
    ///0: Interrupt on EoTp error disabled
    B0x0 = 0,
    ///1: Interrupt on EoTp error enabled
    B0x1 = 1,
}
impl From<EOTPEIE> for bool {
    #[inline(always)]
    fn from(variant: EOTPEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `EOTPEIE` reader - EoTp error interrupt enable This bit enables the interrupt generation on EoTp error.
pub type EOTPEIE_R = crate::BitReader<EOTPEIE>;
impl EOTPEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOTPEIE {
        match self.bits {
            false => EOTPEIE::B0x0,
            true => EOTPEIE::B0x1,
        }
    }
    ///Interrupt on EoTp error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EOTPEIE::B0x0
    }
    ///Interrupt on EoTp error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EOTPEIE::B0x1
    }
}
///Field `EOTPEIE` writer - EoTp error interrupt enable This bit enables the interrupt generation on EoTp error.
pub type EOTPEIE_W<'a, REG> = crate::BitWriter<'a, REG, EOTPEIE>;
impl<'a, REG> EOTPEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on EoTp error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EOTPEIE::B0x0)
    }
    ///Interrupt on EoTp error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EOTPEIE::B0x1)
    }
}
/**LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPWREIE {
    ///0: Interrupt on LTDC payload write error disabled
    B0x0 = 0,
    ///1: Interrupt on LTDC payload write error enabled
    B0x1 = 1,
}
impl From<LPWREIE> for bool {
    #[inline(always)]
    fn from(variant: LPWREIE) -> Self {
        variant as u8 != 0
    }
}
///Field `LPWREIE` reader - LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error.
pub type LPWREIE_R = crate::BitReader<LPWREIE>;
impl LPWREIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LPWREIE {
        match self.bits {
            false => LPWREIE::B0x0,
            true => LPWREIE::B0x1,
        }
    }
    ///Interrupt on LTDC payload write error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == LPWREIE::B0x0
    }
    ///Interrupt on LTDC payload write error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == LPWREIE::B0x1
    }
}
///Field `LPWREIE` writer - LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error.
pub type LPWREIE_W<'a, REG> = crate::BitWriter<'a, REG, LPWREIE>;
impl<'a, REG> LPWREIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on LTDC payload write error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(LPWREIE::B0x0)
    }
    ///Interrupt on LTDC payload write error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(LPWREIE::B0x1)
    }
}
/**Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCWREIE {
    ///0: Interrupt on generic command write error disabled
    B0x0 = 0,
    ///1: Interrupt on generic command write error enabled
    B0x1 = 1,
}
impl From<GCWREIE> for bool {
    #[inline(always)]
    fn from(variant: GCWREIE) -> Self {
        variant as u8 != 0
    }
}
///Field `GCWREIE` reader - Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error.
pub type GCWREIE_R = crate::BitReader<GCWREIE>;
impl GCWREIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GCWREIE {
        match self.bits {
            false => GCWREIE::B0x0,
            true => GCWREIE::B0x1,
        }
    }
    ///Interrupt on generic command write error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GCWREIE::B0x0
    }
    ///Interrupt on generic command write error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GCWREIE::B0x1
    }
}
///Field `GCWREIE` writer - Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error.
pub type GCWREIE_W<'a, REG> = crate::BitWriter<'a, REG, GCWREIE>;
impl<'a, REG> GCWREIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on generic command write error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GCWREIE::B0x0)
    }
    ///Interrupt on generic command write error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GCWREIE::B0x1)
    }
}
/**Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPWREIE {
    ///0: Interrupt on generic payload write error disabled
    B0x0 = 0,
    ///1: Interrupt on generic payload write error enabled
    B0x1 = 1,
}
impl From<GPWREIE> for bool {
    #[inline(always)]
    fn from(variant: GPWREIE) -> Self {
        variant as u8 != 0
    }
}
///Field `GPWREIE` reader - Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error.
pub type GPWREIE_R = crate::BitReader<GPWREIE>;
impl GPWREIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPWREIE {
        match self.bits {
            false => GPWREIE::B0x0,
            true => GPWREIE::B0x1,
        }
    }
    ///Interrupt on generic payload write error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPWREIE::B0x0
    }
    ///Interrupt on generic payload write error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPWREIE::B0x1
    }
}
///Field `GPWREIE` writer - Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error.
pub type GPWREIE_W<'a, REG> = crate::BitWriter<'a, REG, GPWREIE>;
impl<'a, REG> GPWREIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on generic payload write error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPWREIE::B0x0)
    }
    ///Interrupt on generic payload write error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPWREIE::B0x1)
    }
}
/**Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPTXEIE {
    ///0: Interrupt on generic payload transmit error disabled
    B0x0 = 0,
    ///1: Interrupt on generic payload transmit error enabled
    B0x1 = 1,
}
impl From<GPTXEIE> for bool {
    #[inline(always)]
    fn from(variant: GPTXEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `GPTXEIE` reader - Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error.
pub type GPTXEIE_R = crate::BitReader<GPTXEIE>;
impl GPTXEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPTXEIE {
        match self.bits {
            false => GPTXEIE::B0x0,
            true => GPTXEIE::B0x1,
        }
    }
    ///Interrupt on generic payload transmit error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPTXEIE::B0x0
    }
    ///Interrupt on generic payload transmit error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPTXEIE::B0x1
    }
}
///Field `GPTXEIE` writer - Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error.
pub type GPTXEIE_W<'a, REG> = crate::BitWriter<'a, REG, GPTXEIE>;
impl<'a, REG> GPTXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on generic payload transmit error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPTXEIE::B0x0)
    }
    ///Interrupt on generic payload transmit error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPTXEIE::B0x1)
    }
}
/**Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPRDEIE {
    ///0: Interrupt on generic payload read error disabled
    B0x0 = 0,
    ///1: Interrupt on generic payload read error enabled
    B0x1 = 1,
}
impl From<GPRDEIE> for bool {
    #[inline(always)]
    fn from(variant: GPRDEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `GPRDEIE` reader - Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error.
pub type GPRDEIE_R = crate::BitReader<GPRDEIE>;
impl GPRDEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPRDEIE {
        match self.bits {
            false => GPRDEIE::B0x0,
            true => GPRDEIE::B0x1,
        }
    }
    ///Interrupt on generic payload read error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPRDEIE::B0x0
    }
    ///Interrupt on generic payload read error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPRDEIE::B0x1
    }
}
///Field `GPRDEIE` writer - Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error.
pub type GPRDEIE_W<'a, REG> = crate::BitWriter<'a, REG, GPRDEIE>;
impl<'a, REG> GPRDEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on generic payload read error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPRDEIE::B0x0)
    }
    ///Interrupt on generic payload read error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPRDEIE::B0x1)
    }
}
/**Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPRXEIE {
    ///0: Interrupt on generic payload receive error disabled
    B0x0 = 0,
    ///1: Interrupt on generic payload receive error enabled
    B0x1 = 1,
}
impl From<GPRXEIE> for bool {
    #[inline(always)]
    fn from(variant: GPRXEIE) -> Self {
        variant as u8 != 0
    }
}
///Field `GPRXEIE` reader - Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error.
pub type GPRXEIE_R = crate::BitReader<GPRXEIE>;
impl GPRXEIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> GPRXEIE {
        match self.bits {
            false => GPRXEIE::B0x0,
            true => GPRXEIE::B0x1,
        }
    }
    ///Interrupt on generic payload receive error disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == GPRXEIE::B0x0
    }
    ///Interrupt on generic payload receive error enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == GPRXEIE::B0x1
    }
}
///Field `GPRXEIE` writer - Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error.
pub type GPRXEIE_W<'a, REG> = crate::BitWriter<'a, REG, GPRXEIE>;
impl<'a, REG> GPRXEIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt on generic payload receive error disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(GPRXEIE::B0x0)
    }
    ///Interrupt on generic payload receive error enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(GPRXEIE::B0x1)
    }
}
impl R {
    ///Bit 0 - Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission .
    #[inline(always)]
    pub fn tohstxie(&self) -> TOHSTXIE_R {
        TOHSTXIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception.
    #[inline(always)]
    pub fn tolprxie(&self) -> TOLPRXIE_R {
        TOLPRXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error.
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error.
    #[inline(always)]
    pub fn eccmeie(&self) -> ECCMEIE_R {
        ECCMEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CRC error interrupt enable This bit enables the interrupt generation on CRC error.
    #[inline(always)]
    pub fn crceie(&self) -> CRCEIE_R {
        CRCEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Packet size error interrupt enable This bit enables the interrupt generation on packet size error.
    #[inline(always)]
    pub fn pseie(&self) -> PSEIE_R {
        PSEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - EoTp error interrupt enable This bit enables the interrupt generation on EoTp error.
    #[inline(always)]
    pub fn eotpeie(&self) -> EOTPEIE_R {
        EOTPEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error.
    #[inline(always)]
    pub fn lpwreie(&self) -> LPWREIE_R {
        LPWREIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error.
    #[inline(always)]
    pub fn gcwreie(&self) -> GCWREIE_R {
        GCWREIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error.
    #[inline(always)]
    pub fn gpwreie(&self) -> GPWREIE_R {
        GPWREIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error.
    #[inline(always)]
    pub fn gptxeie(&self) -> GPTXEIE_R {
        GPTXEIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error.
    #[inline(always)]
    pub fn gprdeie(&self) -> GPRDEIE_R {
        GPRDEIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error.
    #[inline(always)]
    pub fn gprxeie(&self) -> GPRXEIE_R {
        GPRXEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER1")
            .field("tohstxie", &self.tohstxie())
            .field("tolprxie", &self.tolprxie())
            .field("eccseie", &self.eccseie())
            .field("eccmeie", &self.eccmeie())
            .field("crceie", &self.crceie())
            .field("pseie", &self.pseie())
            .field("eotpeie", &self.eotpeie())
            .field("lpwreie", &self.lpwreie())
            .field("gcwreie", &self.gcwreie())
            .field("gpwreie", &self.gpwreie())
            .field("gptxeie", &self.gptxeie())
            .field("gprdeie", &self.gprdeie())
            .field("gprxeie", &self.gprxeie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Timeout high-speed transmission interrupt enable This bit enables the interrupt generation on timeout high-speed transmission .
    #[inline(always)]
    pub fn tohstxie(&mut self) -> TOHSTXIE_W<IER1rs> {
        TOHSTXIE_W::new(self, 0)
    }
    ///Bit 1 - Timeout low-power reception interrupt enable This bit enables the interrupt generation on timeout low-power reception.
    #[inline(always)]
    pub fn tolprxie(&mut self) -> TOLPRXIE_W<IER1rs> {
        TOLPRXIE_W::new(self, 1)
    }
    ///Bit 2 - ECC single-bit error interrupt enable This bit enables the interrupt generation on ECC single-bit error.
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W<IER1rs> {
        ECCSEIE_W::new(self, 2)
    }
    ///Bit 3 - ECC multi-bit error interrupt enable This bit enables the interrupt generation on ECC multi-bit error.
    #[inline(always)]
    pub fn eccmeie(&mut self) -> ECCMEIE_W<IER1rs> {
        ECCMEIE_W::new(self, 3)
    }
    ///Bit 4 - CRC error interrupt enable This bit enables the interrupt generation on CRC error.
    #[inline(always)]
    pub fn crceie(&mut self) -> CRCEIE_W<IER1rs> {
        CRCEIE_W::new(self, 4)
    }
    ///Bit 5 - Packet size error interrupt enable This bit enables the interrupt generation on packet size error.
    #[inline(always)]
    pub fn pseie(&mut self) -> PSEIE_W<IER1rs> {
        PSEIE_W::new(self, 5)
    }
    ///Bit 6 - EoTp error interrupt enable This bit enables the interrupt generation on EoTp error.
    #[inline(always)]
    pub fn eotpeie(&mut self) -> EOTPEIE_W<IER1rs> {
        EOTPEIE_W::new(self, 6)
    }
    ///Bit 7 - LTDC payload write error interrupt enable This bit enables the interrupt generation on LTDC payload write error.
    #[inline(always)]
    pub fn lpwreie(&mut self) -> LPWREIE_W<IER1rs> {
        LPWREIE_W::new(self, 7)
    }
    ///Bit 8 - Generic command write error interrupt enable This bit enables the interrupt generation on generic command write error.
    #[inline(always)]
    pub fn gcwreie(&mut self) -> GCWREIE_W<IER1rs> {
        GCWREIE_W::new(self, 8)
    }
    ///Bit 9 - Generic payload write error interrupt enable This bit enables the interrupt generation on generic payload write error.
    #[inline(always)]
    pub fn gpwreie(&mut self) -> GPWREIE_W<IER1rs> {
        GPWREIE_W::new(self, 9)
    }
    ///Bit 10 - Generic payload transmit error interrupt enable This bit enables the interrupt generation on generic payload transmit error.
    #[inline(always)]
    pub fn gptxeie(&mut self) -> GPTXEIE_W<IER1rs> {
        GPTXEIE_W::new(self, 10)
    }
    ///Bit 11 - Generic payload read error interrupt enable This bit enables the interrupt generation on generic payload read error.
    #[inline(always)]
    pub fn gprdeie(&mut self) -> GPRDEIE_W<IER1rs> {
        GPRDEIE_W::new(self, 11)
    }
    ///Bit 12 - Generic payload receive error interrupt enable This bit enables the interrupt generation on generic payload receive error.
    #[inline(always)]
    pub fn gprxeie(&mut self) -> GPRXEIE_W<IER1rs> {
        GPRXEIE_W::new(self, 12)
    }
}
/**DSI Host interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:IER1)*/
pub struct IER1rs;
impl crate::RegisterSpec for IER1rs {
    type Ux = u32;
}
///`read()` method returns [`ier1::R`](R) reader structure
impl crate::Readable for IER1rs {}
///`write(|w| ..)` method takes [`ier1::W`](W) writer structure
impl crate::Writable for IER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER1 to value 0
impl crate::Resettable for IER1rs {}
