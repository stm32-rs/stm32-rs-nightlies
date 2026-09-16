///Register `FDCAN_IR` reader
pub type R = crate::R<FDCAN_IRrs>;
///Register `FDCAN_IR` writer
pub type W = crate::W<FDCAN_IRrs>;
/**Rx FIFO 0 new message

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0N {
    ///0: No new message written to Rx FIFO 0
    B0x0 = 0,
    ///1: New message written to Rx FIFO 0
    B0x1 = 1,
}
impl From<RF0N> for bool {
    #[inline(always)]
    fn from(variant: RF0N) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0N` reader - Rx FIFO 0 new message
pub type RF0N_R = crate::BitReader<RF0N>;
impl RF0N_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF0N {
        match self.bits {
            false => RF0N::B0x0,
            true => RF0N::B0x1,
        }
    }
    ///No new message written to Rx FIFO 0
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0N::B0x0
    }
    ///New message written to Rx FIFO 0
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0N::B0x1
    }
}
///Field `RF0N` writer - Rx FIFO 0 new message
pub type RF0N_W<'a, REG> = crate::BitWriter<'a, REG, RF0N>;
impl<'a, REG> RF0N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No new message written to Rx FIFO 0
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0N::B0x0)
    }
    ///New message written to Rx FIFO 0
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0N::B0x1)
    }
}
/**Rx FIFO 0 full

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0F {
    ///0: Rx FIFO 0 not full
    B0x0 = 0,
    ///1: Rx FIFO 0 full
    B0x1 = 1,
}
impl From<RF0F> for bool {
    #[inline(always)]
    fn from(variant: RF0F) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0F` reader - Rx FIFO 0 full
pub type RF0F_R = crate::BitReader<RF0F>;
impl RF0F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF0F {
        match self.bits {
            false => RF0F::B0x0,
            true => RF0F::B0x1,
        }
    }
    ///Rx FIFO 0 not full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0F::B0x0
    }
    ///Rx FIFO 0 full
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0F::B0x1
    }
}
///Field `RF0F` writer - Rx FIFO 0 full
pub type RF0F_W<'a, REG> = crate::BitWriter<'a, REG, RF0F>;
impl<'a, REG> RF0F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx FIFO 0 not full
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0F::B0x0)
    }
    ///Rx FIFO 0 full
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0F::B0x1)
    }
}
/**Rx FIFO 0 message lost

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF0L {
    ///0: No Rx FIFO 0 message lost
    B0x0 = 0,
    ///1: Rx FIFO 0 message lost
    B0x1 = 1,
}
impl From<RF0L> for bool {
    #[inline(always)]
    fn from(variant: RF0L) -> Self {
        variant as u8 != 0
    }
}
///Field `RF0L` reader - Rx FIFO 0 message lost
pub type RF0L_R = crate::BitReader<RF0L>;
impl RF0L_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF0L {
        match self.bits {
            false => RF0L::B0x0,
            true => RF0L::B0x1,
        }
    }
    ///No Rx FIFO 0 message lost
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF0L::B0x0
    }
    ///Rx FIFO 0 message lost
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF0L::B0x1
    }
}
///Field `RF0L` writer - Rx FIFO 0 message lost
pub type RF0L_W<'a, REG> = crate::BitWriter<'a, REG, RF0L>;
impl<'a, REG> RF0L_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Rx FIFO 0 message lost
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF0L::B0x0)
    }
    ///Rx FIFO 0 message lost
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF0L::B0x1)
    }
}
/**Rx FIFO 1 new message

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1N {
    ///0: No new message written to Rx FIFO 1
    B0x0 = 0,
    ///1: New message written to Rx FIFO 1
    B0x1 = 1,
}
impl From<RF1N> for bool {
    #[inline(always)]
    fn from(variant: RF1N) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1N` reader - Rx FIFO 1 new message
pub type RF1N_R = crate::BitReader<RF1N>;
impl RF1N_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF1N {
        match self.bits {
            false => RF1N::B0x0,
            true => RF1N::B0x1,
        }
    }
    ///No new message written to Rx FIFO 1
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1N::B0x0
    }
    ///New message written to Rx FIFO 1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1N::B0x1
    }
}
///Field `RF1N` writer - Rx FIFO 1 new message
pub type RF1N_W<'a, REG> = crate::BitWriter<'a, REG, RF1N>;
impl<'a, REG> RF1N_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No new message written to Rx FIFO 1
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1N::B0x0)
    }
    ///New message written to Rx FIFO 1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1N::B0x1)
    }
}
/**Rx FIFO 1 full

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1F {
    ///0: Rx FIFO 1 not full
    B0x0 = 0,
    ///1: Rx FIFO 1 full
    B0x1 = 1,
}
impl From<RF1F> for bool {
    #[inline(always)]
    fn from(variant: RF1F) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1F` reader - Rx FIFO 1 full
pub type RF1F_R = crate::BitReader<RF1F>;
impl RF1F_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF1F {
        match self.bits {
            false => RF1F::B0x0,
            true => RF1F::B0x1,
        }
    }
    ///Rx FIFO 1 not full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1F::B0x0
    }
    ///Rx FIFO 1 full
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1F::B0x1
    }
}
///Field `RF1F` writer - Rx FIFO 1 full
pub type RF1F_W<'a, REG> = crate::BitWriter<'a, REG, RF1F>;
impl<'a, REG> RF1F_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Rx FIFO 1 not full
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1F::B0x0)
    }
    ///Rx FIFO 1 full
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1F::B0x1)
    }
}
/**Rx FIFO 1 message lost

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF1L {
    ///0: No Rx FIFO 1 message lost
    B0x0 = 0,
    ///1: Rx FIFO 1 message lost
    B0x1 = 1,
}
impl From<RF1L> for bool {
    #[inline(always)]
    fn from(variant: RF1L) -> Self {
        variant as u8 != 0
    }
}
///Field `RF1L` reader - Rx FIFO 1 message lost
pub type RF1L_R = crate::BitReader<RF1L>;
impl RF1L_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RF1L {
        match self.bits {
            false => RF1L::B0x0,
            true => RF1L::B0x1,
        }
    }
    ///No Rx FIFO 1 message lost
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == RF1L::B0x0
    }
    ///Rx FIFO 1 message lost
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == RF1L::B0x1
    }
}
///Field `RF1L` writer - Rx FIFO 1 message lost
pub type RF1L_W<'a, REG> = crate::BitWriter<'a, REG, RF1L>;
impl<'a, REG> RF1L_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Rx FIFO 1 message lost
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(RF1L::B0x0)
    }
    ///Rx FIFO 1 message lost
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(RF1L::B0x1)
    }
}
/**High-priority message

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HPM {
    ///0: No high-priority message received
    B0x0 = 0,
    ///1: High-priority message received
    B0x1 = 1,
}
impl From<HPM> for bool {
    #[inline(always)]
    fn from(variant: HPM) -> Self {
        variant as u8 != 0
    }
}
///Field `HPM` reader - High-priority message
pub type HPM_R = crate::BitReader<HPM>;
impl HPM_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> HPM {
        match self.bits {
            false => HPM::B0x0,
            true => HPM::B0x1,
        }
    }
    ///No high-priority message received
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == HPM::B0x0
    }
    ///High-priority message received
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == HPM::B0x1
    }
}
///Field `HPM` writer - High-priority message
pub type HPM_W<'a, REG> = crate::BitWriter<'a, REG, HPM>;
impl<'a, REG> HPM_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No high-priority message received
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(HPM::B0x0)
    }
    ///High-priority message received
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(HPM::B0x1)
    }
}
/**Transmission completed

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC {
    ///0: No transmission completed
    B0x0 = 0,
    ///1: Transmission completed
    B0x1 = 1,
}
impl From<TC> for bool {
    #[inline(always)]
    fn from(variant: TC) -> Self {
        variant as u8 != 0
    }
}
///Field `TC` reader - Transmission completed
pub type TC_R = crate::BitReader<TC>;
impl TC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TC {
        match self.bits {
            false => TC::B0x0,
            true => TC::B0x1,
        }
    }
    ///No transmission completed
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TC::B0x0
    }
    ///Transmission completed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TC::B0x1
    }
}
///Field `TC` writer - Transmission completed
pub type TC_W<'a, REG> = crate::BitWriter<'a, REG, TC>;
impl<'a, REG> TC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No transmission completed
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TC::B0x0)
    }
    ///Transmission completed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TC::B0x1)
    }
}
/**Transmission cancellation finished

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCF {
    ///0: No transmission cancellation finished
    B0x0 = 0,
    ///1: Transmission cancellation finished
    B0x1 = 1,
}
impl From<TCF> for bool {
    #[inline(always)]
    fn from(variant: TCF) -> Self {
        variant as u8 != 0
    }
}
///Field `TCF` reader - Transmission cancellation finished
pub type TCF_R = crate::BitReader<TCF>;
impl TCF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TCF {
        match self.bits {
            false => TCF::B0x0,
            true => TCF::B0x1,
        }
    }
    ///No transmission cancellation finished
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TCF::B0x0
    }
    ///Transmission cancellation finished
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TCF::B0x1
    }
}
///Field `TCF` writer - Transmission cancellation finished
pub type TCF_W<'a, REG> = crate::BitWriter<'a, REG, TCF>;
impl<'a, REG> TCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No transmission cancellation finished
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TCF::B0x0)
    }
    ///Transmission cancellation finished
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TCF::B0x1)
    }
}
/**Tx FIFO empty

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TFE {
    ///0: Tx FIFO non-empty
    B0x0 = 0,
    ///1: Tx FIFO empty
    B0x1 = 1,
}
impl From<TFE> for bool {
    #[inline(always)]
    fn from(variant: TFE) -> Self {
        variant as u8 != 0
    }
}
///Field `TFE` reader - Tx FIFO empty
pub type TFE_R = crate::BitReader<TFE>;
impl TFE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TFE {
        match self.bits {
            false => TFE::B0x0,
            true => TFE::B0x1,
        }
    }
    ///Tx FIFO non-empty
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TFE::B0x0
    }
    ///Tx FIFO empty
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TFE::B0x1
    }
}
///Field `TFE` writer - Tx FIFO empty
pub type TFE_W<'a, REG> = crate::BitWriter<'a, REG, TFE>;
impl<'a, REG> TFE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx FIFO non-empty
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TFE::B0x0)
    }
    ///Tx FIFO empty
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TFE::B0x1)
    }
}
/**Tx event FIFO new entry

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFN {
    ///0: Tx event FIFO unchanged
    B0x0 = 0,
    ///1: Tx handler wrote Tx event FIFO element.
    B0x1 = 1,
}
impl From<TEFN> for bool {
    #[inline(always)]
    fn from(variant: TEFN) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFN` reader - Tx event FIFO new entry
pub type TEFN_R = crate::BitReader<TEFN>;
impl TEFN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEFN {
        match self.bits {
            false => TEFN::B0x0,
            true => TEFN::B0x1,
        }
    }
    ///Tx event FIFO unchanged
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFN::B0x0
    }
    ///Tx handler wrote Tx event FIFO element.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFN::B0x1
    }
}
///Field `TEFN` writer - Tx event FIFO new entry
pub type TEFN_W<'a, REG> = crate::BitWriter<'a, REG, TEFN>;
impl<'a, REG> TEFN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx event FIFO unchanged
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFN::B0x0)
    }
    ///Tx handler wrote Tx event FIFO element.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFN::B0x1)
    }
}
/**Tx event FIFO full

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFF {
    ///0: Tx event FIFO Not full
    B0x0 = 0,
    ///1: Tx event FIFO full
    B0x1 = 1,
}
impl From<TEFF> for bool {
    #[inline(always)]
    fn from(variant: TEFF) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFF` reader - Tx event FIFO full
pub type TEFF_R = crate::BitReader<TEFF>;
impl TEFF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEFF {
        match self.bits {
            false => TEFF::B0x0,
            true => TEFF::B0x1,
        }
    }
    ///Tx event FIFO Not full
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFF::B0x0
    }
    ///Tx event FIFO full
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFF::B0x1
    }
}
///Field `TEFF` writer - Tx event FIFO full
pub type TEFF_W<'a, REG> = crate::BitWriter<'a, REG, TEFF>;
impl<'a, REG> TEFF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Tx event FIFO Not full
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFF::B0x0)
    }
    ///Tx event FIFO full
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFF::B0x1)
    }
}
/**Tx event FIFO element lost

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEFL {
    ///0: No Tx event FIFO element lost
    B0x0 = 0,
    ///1: Tx event FIFO element lost
    B0x1 = 1,
}
impl From<TEFL> for bool {
    #[inline(always)]
    fn from(variant: TEFL) -> Self {
        variant as u8 != 0
    }
}
///Field `TEFL` reader - Tx event FIFO element lost
pub type TEFL_R = crate::BitReader<TEFL>;
impl TEFL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TEFL {
        match self.bits {
            false => TEFL::B0x0,
            true => TEFL::B0x1,
        }
    }
    ///No Tx event FIFO element lost
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TEFL::B0x0
    }
    ///Tx event FIFO element lost
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TEFL::B0x1
    }
}
///Field `TEFL` writer - Tx event FIFO element lost
pub type TEFL_W<'a, REG> = crate::BitWriter<'a, REG, TEFL>;
impl<'a, REG> TEFL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No Tx event FIFO element lost
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TEFL::B0x0)
    }
    ///Tx event FIFO element lost
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TEFL::B0x1)
    }
}
/**Timestamp wraparound

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSW {
    ///0: No timestamp counter wrap-around
    B0x0 = 0,
    ///1: Timestamp counter wrapped around
    B0x1 = 1,
}
impl From<TSW> for bool {
    #[inline(always)]
    fn from(variant: TSW) -> Self {
        variant as u8 != 0
    }
}
///Field `TSW` reader - Timestamp wraparound
pub type TSW_R = crate::BitReader<TSW>;
impl TSW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TSW {
        match self.bits {
            false => TSW::B0x0,
            true => TSW::B0x1,
        }
    }
    ///No timestamp counter wrap-around
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TSW::B0x0
    }
    ///Timestamp counter wrapped around
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TSW::B0x1
    }
}
///Field `TSW` writer - Timestamp wraparound
pub type TSW_W<'a, REG> = crate::BitWriter<'a, REG, TSW>;
impl<'a, REG> TSW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No timestamp counter wrap-around
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TSW::B0x0)
    }
    ///Timestamp counter wrapped around
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TSW::B0x1)
    }
}
/**Message RAM access failure

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MRAF {
    ///0: No message RAM access failure occurred
    B0x0 = 0,
    ///1: Message RAM access failure occurred
    B0x1 = 1,
}
impl From<MRAF> for bool {
    #[inline(always)]
    fn from(variant: MRAF) -> Self {
        variant as u8 != 0
    }
}
///Field `MRAF` reader - Message RAM access failure
pub type MRAF_R = crate::BitReader<MRAF>;
impl MRAF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MRAF {
        match self.bits {
            false => MRAF::B0x0,
            true => MRAF::B0x1,
        }
    }
    ///No message RAM access failure occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == MRAF::B0x0
    }
    ///Message RAM access failure occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == MRAF::B0x1
    }
}
///Field `MRAF` writer - Message RAM access failure
pub type MRAF_W<'a, REG> = crate::BitWriter<'a, REG, MRAF>;
impl<'a, REG> MRAF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No message RAM access failure occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MRAF::B0x0)
    }
    ///Message RAM access failure occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MRAF::B0x1)
    }
}
/**Timeout occurred

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOO {
    ///0: No timeout
    B0x0 = 0,
    ///1: Timeout reached
    B0x1 = 1,
}
impl From<TOO> for bool {
    #[inline(always)]
    fn from(variant: TOO) -> Self {
        variant as u8 != 0
    }
}
///Field `TOO` reader - Timeout occurred
pub type TOO_R = crate::BitReader<TOO>;
impl TOO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TOO {
        match self.bits {
            false => TOO::B0x0,
            true => TOO::B0x1,
        }
    }
    ///No timeout
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TOO::B0x0
    }
    ///Timeout reached
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TOO::B0x1
    }
}
///Field `TOO` writer - Timeout occurred
pub type TOO_W<'a, REG> = crate::BitWriter<'a, REG, TOO>;
impl<'a, REG> TOO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No timeout
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TOO::B0x0)
    }
    ///Timeout reached
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TOO::B0x1)
    }
}
/**Error logging overflow

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ELO {
    ///0: CAN error logging counter did not overflow.
    B0x0 = 0,
    ///1: Overflow of CAN error logging counter occurred.
    B0x1 = 1,
}
impl From<ELO> for bool {
    #[inline(always)]
    fn from(variant: ELO) -> Self {
        variant as u8 != 0
    }
}
///Field `ELO` reader - Error logging overflow
pub type ELO_R = crate::BitReader<ELO>;
impl ELO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ELO {
        match self.bits {
            false => ELO::B0x0,
            true => ELO::B0x1,
        }
    }
    ///CAN error logging counter did not overflow.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ELO::B0x0
    }
    ///Overflow of CAN error logging counter occurred.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ELO::B0x1
    }
}
///Field `ELO` writer - Error logging overflow
pub type ELO_W<'a, REG> = crate::BitWriter<'a, REG, ELO>;
impl<'a, REG> ELO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CAN error logging counter did not overflow.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ELO::B0x0)
    }
    ///Overflow of CAN error logging counter occurred.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ELO::B0x1)
    }
}
/**Error passive

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EP {
    ///0: Error-passive status unchanged
    B0x0 = 0,
    ///1: Error-passive status changed
    B0x1 = 1,
}
impl From<EP> for bool {
    #[inline(always)]
    fn from(variant: EP) -> Self {
        variant as u8 != 0
    }
}
///Field `EP` reader - Error passive
pub type EP_R = crate::BitReader<EP>;
impl EP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EP {
        match self.bits {
            false => EP::B0x0,
            true => EP::B0x1,
        }
    }
    ///Error-passive status unchanged
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EP::B0x0
    }
    ///Error-passive status changed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EP::B0x1
    }
}
///Field `EP` writer - Error passive
pub type EP_W<'a, REG> = crate::BitWriter<'a, REG, EP>;
impl<'a, REG> EP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error-passive status unchanged
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EP::B0x0)
    }
    ///Error-passive status changed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EP::B0x1)
    }
}
/**Warning status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EW {
    ///0: Error-warning status unchanged
    B0x0 = 0,
    ///1: Error-warning status changed
    B0x1 = 1,
}
impl From<EW> for bool {
    #[inline(always)]
    fn from(variant: EW) -> Self {
        variant as u8 != 0
    }
}
///Field `EW` reader - Warning status
pub type EW_R = crate::BitReader<EW>;
impl EW_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EW {
        match self.bits {
            false => EW::B0x0,
            true => EW::B0x1,
        }
    }
    ///Error-warning status unchanged
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == EW::B0x0
    }
    ///Error-warning status changed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == EW::B0x1
    }
}
///Field `EW` writer - Warning status
pub type EW_W<'a, REG> = crate::BitWriter<'a, REG, EW>;
impl<'a, REG> EW_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Error-warning status unchanged
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(EW::B0x0)
    }
    ///Error-warning status changed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(EW::B0x1)
    }
}
/**Bus-off status

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BO {
    ///0: Bus-off status unchanged
    B0x0 = 0,
    ///1: Bus-off status changed
    B0x1 = 1,
}
impl From<BO> for bool {
    #[inline(always)]
    fn from(variant: BO) -> Self {
        variant as u8 != 0
    }
}
///Field `BO` reader - Bus-off status
pub type BO_R = crate::BitReader<BO>;
impl BO_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BO {
        match self.bits {
            false => BO::B0x0,
            true => BO::B0x1,
        }
    }
    ///Bus-off status unchanged
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BO::B0x0
    }
    ///Bus-off status changed
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BO::B0x1
    }
}
///Field `BO` writer - Bus-off status
pub type BO_W<'a, REG> = crate::BitWriter<'a, REG, BO>;
impl<'a, REG> BO_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Bus-off status unchanged
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BO::B0x0)
    }
    ///Bus-off status changed
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BO::B0x1)
    }
}
/**Watchdog interrupt

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDI {
    ///0: No message RAM watchdog event occurred
    B0x0 = 0,
    ///1: Message RAM watchdog event due to missing READY
    B0x1 = 1,
}
impl From<WDI> for bool {
    #[inline(always)]
    fn from(variant: WDI) -> Self {
        variant as u8 != 0
    }
}
///Field `WDI` reader - Watchdog interrupt
pub type WDI_R = crate::BitReader<WDI>;
impl WDI_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> WDI {
        match self.bits {
            false => WDI::B0x0,
            true => WDI::B0x1,
        }
    }
    ///No message RAM watchdog event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == WDI::B0x0
    }
    ///Message RAM watchdog event due to missing READY
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == WDI::B0x1
    }
}
///Field `WDI` writer - Watchdog interrupt
pub type WDI_W<'a, REG> = crate::BitWriter<'a, REG, WDI>;
impl<'a, REG> WDI_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No message RAM watchdog event occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(WDI::B0x0)
    }
    ///Message RAM watchdog event due to missing READY
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(WDI::B0x1)
    }
}
/**Protocol error in arbitration phase (nominal bit time is used)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEA {
    ///0: No protocol error in arbitration phase
    B0x0 = 0,
    ///1: Protocol error in arbitration phase detected (LEC\[2:0\] different from 0 and 7 in FDCAN_PSR)
    B0x1 = 1,
}
impl From<PEA> for bool {
    #[inline(always)]
    fn from(variant: PEA) -> Self {
        variant as u8 != 0
    }
}
///Field `PEA` reader - Protocol error in arbitration phase (nominal bit time is used)
pub type PEA_R = crate::BitReader<PEA>;
impl PEA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PEA {
        match self.bits {
            false => PEA::B0x0,
            true => PEA::B0x1,
        }
    }
    ///No protocol error in arbitration phase
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PEA::B0x0
    }
    ///Protocol error in arbitration phase detected (LEC\[2:0\] different from 0 and 7 in FDCAN_PSR)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PEA::B0x1
    }
}
///Field `PEA` writer - Protocol error in arbitration phase (nominal bit time is used)
pub type PEA_W<'a, REG> = crate::BitWriter<'a, REG, PEA>;
impl<'a, REG> PEA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No protocol error in arbitration phase
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PEA::B0x0)
    }
    ///Protocol error in arbitration phase detected (LEC\[2:0\] different from 0 and 7 in FDCAN_PSR)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PEA::B0x1)
    }
}
/**Protocol error in data phase (data bit time is used)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PED {
    ///0: No protocol error in data phase
    B0x0 = 0,
    ///1: Protocol error in data phase detected (DLEC\[2:0\] different from 0 and 7 in FDCAN_PSR)
    B0x1 = 1,
}
impl From<PED> for bool {
    #[inline(always)]
    fn from(variant: PED) -> Self {
        variant as u8 != 0
    }
}
///Field `PED` reader - Protocol error in data phase (data bit time is used)
pub type PED_R = crate::BitReader<PED>;
impl PED_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PED {
        match self.bits {
            false => PED::B0x0,
            true => PED::B0x1,
        }
    }
    ///No protocol error in data phase
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == PED::B0x0
    }
    ///Protocol error in data phase detected (DLEC\[2:0\] different from 0 and 7 in FDCAN_PSR)
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == PED::B0x1
    }
}
///Field `PED` writer - Protocol error in data phase (data bit time is used)
pub type PED_W<'a, REG> = crate::BitWriter<'a, REG, PED>;
impl<'a, REG> PED_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No protocol error in data phase
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(PED::B0x0)
    }
    ///Protocol error in data phase detected (DLEC\[2:0\] different from 0 and 7 in FDCAN_PSR)
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(PED::B0x1)
    }
}
/**Access to reserved address

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARA {
    ///0: No access to reserved address occurred
    B0x0 = 0,
    ///1: Access to reserved address occurred
    B0x1 = 1,
}
impl From<ARA> for bool {
    #[inline(always)]
    fn from(variant: ARA) -> Self {
        variant as u8 != 0
    }
}
///Field `ARA` reader - Access to reserved address
pub type ARA_R = crate::BitReader<ARA>;
impl ARA_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ARA {
        match self.bits {
            false => ARA::B0x0,
            true => ARA::B0x1,
        }
    }
    ///No access to reserved address occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == ARA::B0x0
    }
    ///Access to reserved address occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == ARA::B0x1
    }
}
///Field `ARA` writer - Access to reserved address
pub type ARA_W<'a, REG> = crate::BitWriter<'a, REG, ARA>;
impl<'a, REG> ARA_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No access to reserved address occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ARA::B0x0)
    }
    ///Access to reserved address occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ARA::B0x1)
    }
}
impl R {
    ///Bit 0 - Rx FIFO 0 new message
    #[inline(always)]
    pub fn rf0n(&self) -> RF0N_R {
        RF0N_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rx FIFO 0 full
    #[inline(always)]
    pub fn rf0f(&self) -> RF0F_R {
        RF0F_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rx FIFO 0 message lost
    #[inline(always)]
    pub fn rf0l(&self) -> RF0L_R {
        RF0L_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rx FIFO 1 new message
    #[inline(always)]
    pub fn rf1n(&self) -> RF1N_R {
        RF1N_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rx FIFO 1 full
    #[inline(always)]
    pub fn rf1f(&self) -> RF1F_R {
        RF1F_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rx FIFO 1 message lost
    #[inline(always)]
    pub fn rf1l(&self) -> RF1L_R {
        RF1L_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - High-priority message
    #[inline(always)]
    pub fn hpm(&self) -> HPM_R {
        HPM_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Transmission completed
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Transmission cancellation finished
    #[inline(always)]
    pub fn tcf(&self) -> TCF_R {
        TCF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Tx FIFO empty
    #[inline(always)]
    pub fn tfe(&self) -> TFE_R {
        TFE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Tx event FIFO new entry
    #[inline(always)]
    pub fn tefn(&self) -> TEFN_R {
        TEFN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Tx event FIFO full
    #[inline(always)]
    pub fn teff(&self) -> TEFF_R {
        TEFF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Tx event FIFO element lost
    #[inline(always)]
    pub fn tefl(&self) -> TEFL_R {
        TEFL_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Timestamp wraparound
    #[inline(always)]
    pub fn tsw(&self) -> TSW_R {
        TSW_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Message RAM access failure
    #[inline(always)]
    pub fn mraf(&self) -> MRAF_R {
        MRAF_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Timeout occurred
    #[inline(always)]
    pub fn too(&self) -> TOO_R {
        TOO_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Error logging overflow
    #[inline(always)]
    pub fn elo(&self) -> ELO_R {
        ELO_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Error passive
    #[inline(always)]
    pub fn ep(&self) -> EP_R {
        EP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Warning status
    #[inline(always)]
    pub fn ew(&self) -> EW_R {
        EW_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Bus-off status
    #[inline(always)]
    pub fn bo(&self) -> BO_R {
        BO_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Watchdog interrupt
    #[inline(always)]
    pub fn wdi(&self) -> WDI_R {
        WDI_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Protocol error in arbitration phase (nominal bit time is used)
    #[inline(always)]
    pub fn pea(&self) -> PEA_R {
        PEA_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Protocol error in data phase (data bit time is used)
    #[inline(always)]
    pub fn ped(&self) -> PED_R {
        PED_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Access to reserved address
    #[inline(always)]
    pub fn ara(&self) -> ARA_R {
        ARA_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_IR")
            .field("rf0n", &self.rf0n())
            .field("rf0f", &self.rf0f())
            .field("rf0l", &self.rf0l())
            .field("rf1n", &self.rf1n())
            .field("rf1f", &self.rf1f())
            .field("rf1l", &self.rf1l())
            .field("hpm", &self.hpm())
            .field("tc", &self.tc())
            .field("tcf", &self.tcf())
            .field("tfe", &self.tfe())
            .field("tefn", &self.tefn())
            .field("teff", &self.teff())
            .field("tefl", &self.tefl())
            .field("tsw", &self.tsw())
            .field("mraf", &self.mraf())
            .field("too", &self.too())
            .field("elo", &self.elo())
            .field("ep", &self.ep())
            .field("ew", &self.ew())
            .field("bo", &self.bo())
            .field("wdi", &self.wdi())
            .field("pea", &self.pea())
            .field("ped", &self.ped())
            .field("ara", &self.ara())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rx FIFO 0 new message
    #[inline(always)]
    pub fn rf0n(&mut self) -> RF0N_W<'_, FDCAN_IRrs> {
        RF0N_W::new(self, 0)
    }
    ///Bit 1 - Rx FIFO 0 full
    #[inline(always)]
    pub fn rf0f(&mut self) -> RF0F_W<'_, FDCAN_IRrs> {
        RF0F_W::new(self, 1)
    }
    ///Bit 2 - Rx FIFO 0 message lost
    #[inline(always)]
    pub fn rf0l(&mut self) -> RF0L_W<'_, FDCAN_IRrs> {
        RF0L_W::new(self, 2)
    }
    ///Bit 3 - Rx FIFO 1 new message
    #[inline(always)]
    pub fn rf1n(&mut self) -> RF1N_W<'_, FDCAN_IRrs> {
        RF1N_W::new(self, 3)
    }
    ///Bit 4 - Rx FIFO 1 full
    #[inline(always)]
    pub fn rf1f(&mut self) -> RF1F_W<'_, FDCAN_IRrs> {
        RF1F_W::new(self, 4)
    }
    ///Bit 5 - Rx FIFO 1 message lost
    #[inline(always)]
    pub fn rf1l(&mut self) -> RF1L_W<'_, FDCAN_IRrs> {
        RF1L_W::new(self, 5)
    }
    ///Bit 6 - High-priority message
    #[inline(always)]
    pub fn hpm(&mut self) -> HPM_W<'_, FDCAN_IRrs> {
        HPM_W::new(self, 6)
    }
    ///Bit 7 - Transmission completed
    #[inline(always)]
    pub fn tc(&mut self) -> TC_W<'_, FDCAN_IRrs> {
        TC_W::new(self, 7)
    }
    ///Bit 8 - Transmission cancellation finished
    #[inline(always)]
    pub fn tcf(&mut self) -> TCF_W<'_, FDCAN_IRrs> {
        TCF_W::new(self, 8)
    }
    ///Bit 9 - Tx FIFO empty
    #[inline(always)]
    pub fn tfe(&mut self) -> TFE_W<'_, FDCAN_IRrs> {
        TFE_W::new(self, 9)
    }
    ///Bit 10 - Tx event FIFO new entry
    #[inline(always)]
    pub fn tefn(&mut self) -> TEFN_W<'_, FDCAN_IRrs> {
        TEFN_W::new(self, 10)
    }
    ///Bit 11 - Tx event FIFO full
    #[inline(always)]
    pub fn teff(&mut self) -> TEFF_W<'_, FDCAN_IRrs> {
        TEFF_W::new(self, 11)
    }
    ///Bit 12 - Tx event FIFO element lost
    #[inline(always)]
    pub fn tefl(&mut self) -> TEFL_W<'_, FDCAN_IRrs> {
        TEFL_W::new(self, 12)
    }
    ///Bit 13 - Timestamp wraparound
    #[inline(always)]
    pub fn tsw(&mut self) -> TSW_W<'_, FDCAN_IRrs> {
        TSW_W::new(self, 13)
    }
    ///Bit 14 - Message RAM access failure
    #[inline(always)]
    pub fn mraf(&mut self) -> MRAF_W<'_, FDCAN_IRrs> {
        MRAF_W::new(self, 14)
    }
    ///Bit 15 - Timeout occurred
    #[inline(always)]
    pub fn too(&mut self) -> TOO_W<'_, FDCAN_IRrs> {
        TOO_W::new(self, 15)
    }
    ///Bit 16 - Error logging overflow
    #[inline(always)]
    pub fn elo(&mut self) -> ELO_W<'_, FDCAN_IRrs> {
        ELO_W::new(self, 16)
    }
    ///Bit 17 - Error passive
    #[inline(always)]
    pub fn ep(&mut self) -> EP_W<'_, FDCAN_IRrs> {
        EP_W::new(self, 17)
    }
    ///Bit 18 - Warning status
    #[inline(always)]
    pub fn ew(&mut self) -> EW_W<'_, FDCAN_IRrs> {
        EW_W::new(self, 18)
    }
    ///Bit 19 - Bus-off status
    #[inline(always)]
    pub fn bo(&mut self) -> BO_W<'_, FDCAN_IRrs> {
        BO_W::new(self, 19)
    }
    ///Bit 20 - Watchdog interrupt
    #[inline(always)]
    pub fn wdi(&mut self) -> WDI_W<'_, FDCAN_IRrs> {
        WDI_W::new(self, 20)
    }
    ///Bit 21 - Protocol error in arbitration phase (nominal bit time is used)
    #[inline(always)]
    pub fn pea(&mut self) -> PEA_W<'_, FDCAN_IRrs> {
        PEA_W::new(self, 21)
    }
    ///Bit 22 - Protocol error in data phase (data bit time is used)
    #[inline(always)]
    pub fn ped(&mut self) -> PED_W<'_, FDCAN_IRrs> {
        PED_W::new(self, 22)
    }
    ///Bit 23 - Access to reserved address
    #[inline(always)]
    pub fn ara(&mut self) -> ARA_W<'_, FDCAN_IRrs> {
        ARA_W::new(self, 23)
    }
}
/**FDCAN interrupt register

You can [`read`](crate::Reg::read) this register and get [`fdcan_ir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#FDCAN:FDCAN_IR)*/
pub struct FDCAN_IRrs;
impl crate::RegisterSpec for FDCAN_IRrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_ir::R`](R) reader structure
impl crate::Readable for FDCAN_IRrs {}
///`write(|w| ..)` method takes [`fdcan_ir::W`](W) writer structure
impl crate::Writable for FDCAN_IRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FDCAN_IR to value 0
impl crate::Resettable for FDCAN_IRrs {}
