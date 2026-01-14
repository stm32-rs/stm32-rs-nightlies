///Register `FDCAN_IE` reader
pub type R = crate::R<FDCAN_IErs>;
///Register `FDCAN_IE` writer
pub type W = crate::W<FDCAN_IErs>;
/**Rx FIFO 0 new message interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0NE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<RF0NE> for bool {
    #[inline(always)]
    fn from(variant: RF0NE) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0NE` reader - Rx FIFO 0 new message interrupt enable
pub type RF0NE_R = crate::BitReader<RF0NE>;
impl RF0NE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF0NE {
        match self.bits {
            false => RF0NE::B0x0,
            true => RF0NE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0NE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0NE::B0x1
    }
}
///Field `RF0NE` writer - Rx FIFO 0 new message interrupt enable
pub type RF0NE_W<'a, REG> = crate::BitWriter<'a, REG, RF0NE>;
impl<'a, REG> RF0NE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0NE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0NE::B0x1)
    }
}
/**Rx FIFO 0 full interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0FE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<RF0FE> for bool {
    #[inline(always)]
    fn from(variant: RF0FE) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0FE` reader - Rx FIFO 0 full interrupt enable
pub type RF0FE_R = crate::BitReader<RF0FE>;
impl RF0FE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF0FE {
        match self.bits {
            false => RF0FE::B0x0,
            true => RF0FE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0FE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0FE::B0x1
    }
}
///Field `RF0FE` writer - Rx FIFO 0 full interrupt enable
pub type RF0FE_W<'a, REG> = crate::BitWriter<'a, REG, RF0FE>;
impl<'a, REG> RF0FE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0FE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0FE::B0x1)
    }
}
/**Rx FIFO 0 message lost interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0LE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<RF0LE> for bool {
    #[inline(always)]
    fn from(variant: RF0LE) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0LE` reader - Rx FIFO 0 message lost interrupt enable
pub type RF0LE_R = crate::BitReader<RF0LE>;
impl RF0LE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF0LE {
        match self.bits {
            false => RF0LE::B0x0,
            true => RF0LE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0LE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0LE::B0x1
    }
}
///Field `RF0LE` writer - Rx FIFO 0 message lost interrupt enable
pub type RF0LE_W<'a, REG> = crate::BitWriter<'a, REG, RF0LE>;
impl<'a, REG> RF0LE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0LE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0LE::B0x1)
    }
}
/**Rx FIFO 1 new message interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1NE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<RF1NE> for bool {
    #[inline(always)]
    fn from(variant: RF1NE) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1NE` reader - Rx FIFO 1 new message interrupt enable
pub type RF1NE_R = crate::BitReader<RF1NE>;
impl RF1NE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF1NE {
        match self.bits {
            false => RF1NE::B0x0,
            true => RF1NE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1NE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1NE::B0x1
    }
}
///Field `RF1NE` writer - Rx FIFO 1 new message interrupt enable
pub type RF1NE_W<'a, REG> = crate::BitWriter<'a, REG, RF1NE>;
impl<'a, REG> RF1NE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1NE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1NE::B0x1)
    }
}
/**Rx FIFO 1 full interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1FE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<RF1FE> for bool {
    #[inline(always)]
    fn from(variant: RF1FE) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1FE` reader - Rx FIFO 1 full interrupt enable
pub type RF1FE_R = crate::BitReader<RF1FE>;
impl RF1FE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF1FE {
        match self.bits {
            false => RF1FE::B0x0,
            true => RF1FE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1FE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1FE::B0x1
    }
}
///Field `RF1FE` writer - Rx FIFO 1 full interrupt enable
pub type RF1FE_W<'a, REG> = crate::BitWriter<'a, REG, RF1FE>;
impl<'a, REG> RF1FE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1FE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1FE::B0x1)
    }
}
/**Rx FIFO 1 message lost interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1LE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<RF1LE> for bool {
    #[inline(always)]
    fn from(variant: RF1LE) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1LE` reader - Rx FIFO 1 message lost interrupt enable
pub type RF1LE_R = crate::BitReader<RF1LE>;
impl RF1LE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF1LE {
        match self.bits {
            false => RF1LE::B0x0,
            true => RF1LE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1LE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1LE::B0x1
    }
}
///Field `RF1LE` writer - Rx FIFO 1 message lost interrupt enable
pub type RF1LE_W<'a, REG> = crate::BitWriter<'a, REG, RF1LE>;
impl<'a, REG> RF1LE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1LE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1LE::B0x1)
    }
}
/**High-priority message interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPME {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<HPME> for bool {
    #[inline(always)]
    fn from(variant: HPME) -> Self {
        variant as u8 != 0
    }
}
///Field `HPME` reader - High-priority message interrupt enable
pub type HPME_R = crate::BitReader<HPME>;
impl HPME_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPME {
        match self.bits {
            false => HPME::B0x0,
            true => HPME::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HPME::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HPME::B0x1
    }
}
///Field `HPME` writer - High-priority message interrupt enable
pub type HPME_W<'a, REG> = crate::BitWriter<'a, REG, HPME>;
impl<'a, REG> HPME_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HPME::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HPME::B0x1)
    }
}
/**Transmission completed interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<TCE> for bool {
    #[inline(always)]
    fn from(variant: TCE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCE` reader - Transmission completed interrupt enable
pub type TCE_R = crate::BitReader<TCE>;
impl TCE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCE {
        match self.bits {
            false => TCE::B0x0,
            true => TCE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCE::B0x1
    }
}
///Field `TCE` writer - Transmission completed interrupt enable
pub type TCE_W<'a, REG> = crate::BitWriter<'a, REG, TCE>;
impl<'a, REG> TCE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCE::B0x1)
    }
}
/**Transmission cancellation finished interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCFE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<TCFE> for bool {
    #[inline(always)]
    fn from(variant: TCFE) -> Self {
        variant as u8 != 0
    }
}
///Field `TCFE` reader - Transmission cancellation finished interrupt enable
pub type TCFE_R = crate::BitReader<TCFE>;
impl TCFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCFE {
        match self.bits {
            false => TCFE::B0x0,
            true => TCFE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCFE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCFE::B0x1
    }
}
///Field `TCFE` writer - Transmission cancellation finished interrupt enable
pub type TCFE_W<'a, REG> = crate::BitWriter<'a, REG, TCFE>;
impl<'a, REG> TCFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCFE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCFE::B0x1)
    }
}
/**Tx FIFO empty interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFEE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<TFEE> for bool {
    #[inline(always)]
    fn from(variant: TFEE) -> Self {
        variant as u8 != 0
    }
}
///Field `TFEE` reader - Tx FIFO empty interrupt enable
pub type TFEE_R = crate::BitReader<TFEE>;
impl TFEE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFEE {
        match self.bits {
            false => TFEE::B0x0,
            true => TFEE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TFEE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TFEE::B0x1
    }
}
///Field `TFEE` writer - Tx FIFO empty interrupt enable
pub type TFEE_W<'a, REG> = crate::BitWriter<'a, REG, TFEE>;
impl<'a, REG> TFEE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TFEE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TFEE::B0x1)
    }
}
/**Tx event FIFO new entry interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFNE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<TEFNE> for bool {
    #[inline(always)]
    fn from(variant: TEFNE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFNE` reader - Tx event FIFO new entry interrupt enable
pub type TEFNE_R = crate::BitReader<TEFNE>;
impl TEFNE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEFNE {
        match self.bits {
            false => TEFNE::B0x0,
            true => TEFNE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFNE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFNE::B0x1
    }
}
///Field `TEFNE` writer - Tx event FIFO new entry interrupt enable
pub type TEFNE_W<'a, REG> = crate::BitWriter<'a, REG, TEFNE>;
impl<'a, REG> TEFNE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFNE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFNE::B0x1)
    }
}
/**Tx event FIFO full interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFFE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<TEFFE> for bool {
    #[inline(always)]
    fn from(variant: TEFFE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFFE` reader - Tx event FIFO full interrupt enable
pub type TEFFE_R = crate::BitReader<TEFFE>;
impl TEFFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEFFE {
        match self.bits {
            false => TEFFE::B0x0,
            true => TEFFE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFFE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFFE::B0x1
    }
}
///Field `TEFFE` writer - Tx event FIFO full interrupt enable
pub type TEFFE_W<'a, REG> = crate::BitWriter<'a, REG, TEFFE>;
impl<'a, REG> TEFFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFFE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFFE::B0x1)
    }
}
/**Tx event FIFO element lost interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFLE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<TEFLE> for bool {
    #[inline(always)]
    fn from(variant: TEFLE) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFLE` reader - Tx event FIFO element lost interrupt enable
pub type TEFLE_R = crate::BitReader<TEFLE>;
impl TEFLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEFLE {
        match self.bits {
            false => TEFLE::B0x0,
            true => TEFLE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFLE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFLE::B0x1
    }
}
///Field `TEFLE` writer - Tx event FIFO element lost interrupt enable
pub type TEFLE_W<'a, REG> = crate::BitWriter<'a, REG, TEFLE>;
impl<'a, REG> TEFLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFLE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFLE::B0x1)
    }
}
/**Timestamp wraparound interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSWE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<TSWE> for bool {
    #[inline(always)]
    fn from(variant: TSWE) -> Self {
        variant as u8 != 0
    }
}
///Field `TSWE` reader - Timestamp wraparound interrupt enable
pub type TSWE_R = crate::BitReader<TSWE>;
impl TSWE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSWE {
        match self.bits {
            false => TSWE::B0x0,
            true => TSWE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSWE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSWE::B0x1
    }
}
///Field `TSWE` writer - Timestamp wraparound interrupt enable
pub type TSWE_W<'a, REG> = crate::BitWriter<'a, REG, TSWE>;
impl<'a, REG> TSWE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSWE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSWE::B0x1)
    }
}
/**Message RAM access failure interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRAFE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<MRAFE> for bool {
    #[inline(always)]
    fn from(variant: MRAFE) -> Self {
        variant as u8 != 0
    }
}
///Field `MRAFE` reader - Message RAM access failure interrupt enable
pub type MRAFE_R = crate::BitReader<MRAFE>;
impl MRAFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MRAFE {
        match self.bits {
            false => MRAFE::B0x0,
            true => MRAFE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MRAFE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MRAFE::B0x1
    }
}
///Field `MRAFE` writer - Message RAM access failure interrupt enable
pub type MRAFE_W<'a, REG> = crate::BitWriter<'a, REG, MRAFE>;
impl<'a, REG> MRAFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MRAFE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MRAFE::B0x1)
    }
}
/**Timeout occurred interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOOE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<TOOE> for bool {
    #[inline(always)]
    fn from(variant: TOOE) -> Self {
        variant as u8 != 0
    }
}
///Field `TOOE` reader - Timeout occurred interrupt enable
pub type TOOE_R = crate::BitReader<TOOE>;
impl TOOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOOE {
        match self.bits {
            false => TOOE::B0x0,
            true => TOOE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOOE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOOE::B0x1
    }
}
///Field `TOOE` writer - Timeout occurred interrupt enable
pub type TOOE_W<'a, REG> = crate::BitWriter<'a, REG, TOOE>;
impl<'a, REG> TOOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOOE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOOE::B0x1)
    }
}
/**Error logging overflow interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELOE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<ELOE> for bool {
    #[inline(always)]
    fn from(variant: ELOE) -> Self {
        variant as u8 != 0
    }
}
///Field `ELOE` reader - Error logging overflow interrupt enable
pub type ELOE_R = crate::BitReader<ELOE>;
impl ELOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELOE {
        match self.bits {
            false => ELOE::B0x0,
            true => ELOE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ELOE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ELOE::B0x1
    }
}
///Field `ELOE` writer - Error logging overflow interrupt enable
pub type ELOE_W<'a, REG> = crate::BitWriter<'a, REG, ELOE>;
impl<'a, REG> ELOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ELOE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ELOE::B0x1)
    }
}
/**Error passive interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<EPE> for bool {
    #[inline(always)]
    fn from(variant: EPE) -> Self {
        variant as u8 != 0
    }
}
///Field `EPE` reader - Error passive interrupt enable
pub type EPE_R = crate::BitReader<EPE>;
impl EPE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EPE {
        match self.bits {
            false => EPE::B0x0,
            true => EPE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EPE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EPE::B0x1
    }
}
///Field `EPE` writer - Error passive interrupt enable
pub type EPE_W<'a, REG> = crate::BitWriter<'a, REG, EPE>;
impl<'a, REG> EPE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EPE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EPE::B0x1)
    }
}
/**Warning status interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EWE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<EWE> for bool {
    #[inline(always)]
    fn from(variant: EWE) -> Self {
        variant as u8 != 0
    }
}
///Field `EWE` reader - Warning status interrupt enable
pub type EWE_R = crate::BitReader<EWE>;
impl EWE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EWE {
        match self.bits {
            false => EWE::B0x0,
            true => EWE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EWE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EWE::B0x1
    }
}
///Field `EWE` writer - Warning status interrupt enable
pub type EWE_W<'a, REG> = crate::BitWriter<'a, REG, EWE>;
impl<'a, REG> EWE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EWE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EWE::B0x1)
    }
}
/**Bus-off status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BOE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<BOE> for bool {
    #[inline(always)]
    fn from(variant: BOE) -> Self {
        variant as u8 != 0
    }
}
///Field `BOE` reader - Bus-off status
pub type BOE_R = crate::BitReader<BOE>;
impl BOE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BOE {
        match self.bits {
            false => BOE::B0x0,
            true => BOE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BOE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BOE::B0x1
    }
}
///Field `BOE` writer - Bus-off status
pub type BOE_W<'a, REG> = crate::BitWriter<'a, REG, BOE>;
impl<'a, REG> BOE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BOE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BOE::B0x1)
    }
}
/**Watchdog interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDIE {
    ///0: Interrupt disabled
    B0x0 = 0,
    ///1: Interrupt enabled
    B0x1 = 1,
}
impl From<WDIE> for bool {
    #[inline(always)]
    fn from(variant: WDIE) -> Self {
        variant as u8 != 0
    }
}
///Field `WDIE` reader - Watchdog interrupt enable
pub type WDIE_R = crate::BitReader<WDIE>;
impl WDIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDIE {
        match self.bits {
            false => WDIE::B0x0,
            true => WDIE::B0x1,
        }
    }
    ///Interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDIE::B0x0
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDIE::B0x1
    }
}
///Field `WDIE` writer - Watchdog interrupt enable
pub type WDIE_W<'a, REG> = crate::BitWriter<'a, REG, WDIE>;
impl<'a, REG> WDIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDIE::B0x0)
    }
    ///Interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDIE::B0x1)
    }
}
///Field `PEAE` reader - Protocol error in arbitration phase enable
pub type PEAE_R = crate::BitReader;
///Field `PEAE` writer - Protocol error in arbitration phase enable
pub type PEAE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PEDE` reader - Protocol error in data phase enable
pub type PEDE_R = crate::BitReader;
///Field `PEDE` writer - Protocol error in data phase enable
pub type PEDE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARAE` reader - Access to reserved address enable
pub type ARAE_R = crate::BitReader;
///Field `ARAE` writer - Access to reserved address enable
pub type ARAE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Rx FIFO 0 new message interrupt enable
    #[inline(always)]
    pub fn rf0ne(&self) -> RF0NE_R {
        RF0NE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 full interrupt enable
    #[inline(always)]
    pub fn rf0fe(&self) -> RF0FE_R {
        RF0FE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 message lost interrupt enable
    #[inline(always)]
    pub fn rf0le(&self) -> RF0LE_R {
        RF0LE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 1 new message interrupt enable
    #[inline(always)]
    pub fn rf1ne(&self) -> RF1NE_R {
        RF1NE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 full interrupt enable
    #[inline(always)]
    pub fn rf1fe(&self) -> RF1FE_R {
        RF1FE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 message lost interrupt enable
    #[inline(always)]
    pub fn rf1le(&self) -> RF1LE_R {
        RF1LE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - High-priority message interrupt enable
    #[inline(always)]
    pub fn hpme(&self) -> HPME_R {
        HPME_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmission completed interrupt enable
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmission cancellation finished interrupt enable
    #[inline(always)]
    pub fn tcfe(&self) -> TCFE_R {
        TCFE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tx FIFO empty interrupt enable
    #[inline(always)]
    pub fn tfee(&self) -> TFEE_R {
        TFEE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx event FIFO new entry interrupt enable
    #[inline(always)]
    pub fn tefne(&self) -> TEFNE_R {
        TEFNE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx event FIFO full interrupt enable
    #[inline(always)]
    pub fn teffe(&self) -> TEFFE_R {
        TEFFE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx event FIFO element lost interrupt enable
    #[inline(always)]
    pub fn tefle(&self) -> TEFLE_R {
        TEFLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Timestamp wraparound interrupt enable
    #[inline(always)]
    pub fn tswe(&self) -> TSWE_R {
        TSWE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Message RAM access failure interrupt enable
    #[inline(always)]
    pub fn mrafe(&self) -> MRAFE_R {
        MRAFE_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timeout occurred interrupt enable
    #[inline(always)]
    pub fn tooe(&self) -> TOOE_R {
        TOOE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Error logging overflow interrupt enable
    #[inline(always)]
    pub fn eloe(&self) -> ELOE_R {
        ELOE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Error passive interrupt enable
    #[inline(always)]
    pub fn epe(&self) -> EPE_R {
        EPE_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Warning status interrupt enable
    #[inline(always)]
    pub fn ewe(&self) -> EWE_R {
        EWE_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bus-off status
    #[inline(always)]
    pub fn boe(&self) -> BOE_R {
        BOE_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Watchdog interrupt enable
    #[inline(always)]
    pub fn wdie(&self) -> WDIE_R {
        WDIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Protocol error in arbitration phase enable
    #[inline(always)]
    pub fn peae(&self) -> PEAE_R {
        PEAE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Protocol error in data phase enable
    #[inline(always)]
    pub fn pede(&self) -> PEDE_R {
        PEDE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Access to reserved address enable
    #[inline(always)]
    pub fn arae(&self) -> ARAE_R {
        ARAE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_IE")
            .field("rf0ne", &self.rf0ne())
            .field("rf0fe", &self.rf0fe())
            .field("rf0le", &self.rf0le())
            .field("rf1ne", &self.rf1ne())
            .field("rf1fe", &self.rf1fe())
            .field("rf1le", &self.rf1le())
            .field("hpme", &self.hpme())
            .field("tce", &self.tce())
            .field("tcfe", &self.tcfe())
            .field("tfee", &self.tfee())
            .field("tefne", &self.tefne())
            .field("teffe", &self.teffe())
            .field("tefle", &self.tefle())
            .field("tswe", &self.tswe())
            .field("mrafe", &self.mrafe())
            .field("tooe", &self.tooe())
            .field("eloe", &self.eloe())
            .field("epe", &self.epe())
            .field("ewe", &self.ewe())
            .field("boe", &self.boe())
            .field("wdie", &self.wdie())
            .field("peae", &self.peae())
            .field("pede", &self.pede())
            .field("arae", &self.arae())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 new message interrupt enable
    #[inline(always)]
    pub fn rf0ne(&mut self) -> RF0NE_W<'_, FDCAN_IErs> {
        RF0NE_W::new(self, 0)
    }
    ///Bit 1 - Rx FIFO 0 full interrupt enable
    #[inline(always)]
    pub fn rf0fe(&mut self) -> RF0FE_W<'_, FDCAN_IErs> {
        RF0FE_W::new(self, 1)
    }
    ///Bit 2 - Rx FIFO 0 message lost interrupt enable
    #[inline(always)]
    pub fn rf0le(&mut self) -> RF0LE_W<'_, FDCAN_IErs> {
        RF0LE_W::new(self, 2)
    }
    ///Bit 3 - Rx FIFO 1 new message interrupt enable
    #[inline(always)]
    pub fn rf1ne(&mut self) -> RF1NE_W<'_, FDCAN_IErs> {
        RF1NE_W::new(self, 3)
    }
    ///Bit 4 - Rx FIFO 1 full interrupt enable
    #[inline(always)]
    pub fn rf1fe(&mut self) -> RF1FE_W<'_, FDCAN_IErs> {
        RF1FE_W::new(self, 4)
    }
    ///Bit 5 - Rx FIFO 1 message lost interrupt enable
    #[inline(always)]
    pub fn rf1le(&mut self) -> RF1LE_W<'_, FDCAN_IErs> {
        RF1LE_W::new(self, 5)
    }
    ///Bit 6 - High-priority message interrupt enable
    #[inline(always)]
    pub fn hpme(&mut self) -> HPME_W<'_, FDCAN_IErs> {
        HPME_W::new(self, 6)
    }
    ///Bit 7 - Transmission completed interrupt enable
    #[inline(always)]
    pub fn tce(&mut self) -> TCE_W<'_, FDCAN_IErs> {
        TCE_W::new(self, 7)
    }
    ///Bit 8 - Transmission cancellation finished interrupt enable
    #[inline(always)]
    pub fn tcfe(&mut self) -> TCFE_W<'_, FDCAN_IErs> {
        TCFE_W::new(self, 8)
    }
    ///Bit 9 - Tx FIFO empty interrupt enable
    #[inline(always)]
    pub fn tfee(&mut self) -> TFEE_W<'_, FDCAN_IErs> {
        TFEE_W::new(self, 9)
    }
    ///Bit 10 - Tx event FIFO new entry interrupt enable
    #[inline(always)]
    pub fn tefne(&mut self) -> TEFNE_W<'_, FDCAN_IErs> {
        TEFNE_W::new(self, 10)
    }
    ///Bit 11 - Tx event FIFO full interrupt enable
    #[inline(always)]
    pub fn teffe(&mut self) -> TEFFE_W<'_, FDCAN_IErs> {
        TEFFE_W::new(self, 11)
    }
    ///Bit 12 - Tx event FIFO element lost interrupt enable
    #[inline(always)]
    pub fn tefle(&mut self) -> TEFLE_W<'_, FDCAN_IErs> {
        TEFLE_W::new(self, 12)
    }
    ///Bit 13 - Timestamp wraparound interrupt enable
    #[inline(always)]
    pub fn tswe(&mut self) -> TSWE_W<'_, FDCAN_IErs> {
        TSWE_W::new(self, 13)
    }
    ///Bit 14 - Message RAM access failure interrupt enable
    #[inline(always)]
    pub fn mrafe(&mut self) -> MRAFE_W<'_, FDCAN_IErs> {
        MRAFE_W::new(self, 14)
    }
    ///Bit 15 - Timeout occurred interrupt enable
    #[inline(always)]
    pub fn tooe(&mut self) -> TOOE_W<'_, FDCAN_IErs> {
        TOOE_W::new(self, 15)
    }
    ///Bit 16 - Error logging overflow interrupt enable
    #[inline(always)]
    pub fn eloe(&mut self) -> ELOE_W<'_, FDCAN_IErs> {
        ELOE_W::new(self, 16)
    }
    ///Bit 17 - Error passive interrupt enable
    #[inline(always)]
    pub fn epe(&mut self) -> EPE_W<'_, FDCAN_IErs> {
        EPE_W::new(self, 17)
    }
    ///Bit 18 - Warning status interrupt enable
    #[inline(always)]
    pub fn ewe(&mut self) -> EWE_W<'_, FDCAN_IErs> {
        EWE_W::new(self, 18)
    }
    ///Bit 19 - Bus-off status
    #[inline(always)]
    pub fn boe(&mut self) -> BOE_W<'_, FDCAN_IErs> {
        BOE_W::new(self, 19)
    }
    ///Bit 20 - Watchdog interrupt enable
    #[inline(always)]
    pub fn wdie(&mut self) -> WDIE_W<'_, FDCAN_IErs> {
        WDIE_W::new(self, 20)
    }
    ///Bit 21 - Protocol error in arbitration phase enable
    #[inline(always)]
    pub fn peae(&mut self) -> PEAE_W<'_, FDCAN_IErs> {
        PEAE_W::new(self, 21)
    }
    ///Bit 22 - Protocol error in data phase enable
    #[inline(always)]
    pub fn pede(&mut self) -> PEDE_W<'_, FDCAN_IErs> {
        PEDE_W::new(self, 22)
    }
    ///Bit 23 - Access to reserved address enable
    #[inline(always)]
    pub fn arae(&mut self) -> ARAE_W<'_, FDCAN_IErs> {
        ARAE_W::new(self, 23)
    }
}
/**FDCAN interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_IE)*/
pub struct FDCAN_IErs;
impl crate::RegisterSpec for FDCAN_IErs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ie::R`](R) reader structure
impl crate::Readable for FDCAN_IErs {}
///`write(|w| ..)` method takes [`fdcan_ie::W`](W) writer structure
impl crate::Writable for FDCAN_IErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_IE to value 0
impl crate::Resettable for FDCAN_IErs {}
