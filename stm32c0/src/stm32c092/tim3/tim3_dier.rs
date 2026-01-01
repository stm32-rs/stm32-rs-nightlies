///Register `TIM3_DIER` reader
pub type R = crate::R<TIM3_DIERrs>;
///Register `TIM3_DIER` writer
pub type W = crate::W<TIM3_DIERrs>;
/**Update interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIE {
    ///0: Update interrupt disabled.
    B0x0 = 0,
    ///1: Update interrupt enabled.
    B0x1 = 1,
}
impl From<UIE> for bool {
    #[inline(always)]
    fn from(variant: UIE) -> Self {
        variant as u8 != 0
    }
}
///Field `UIE` reader - Update interrupt enable
pub type UIE_R = crate::BitReader<UIE>;
impl UIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UIE {
        match self.bits {
            false => UIE::B0x0,
            true => UIE::B0x1,
        }
    }
    ///Update interrupt disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UIE::B0x0
    }
    ///Update interrupt enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UIE::B0x1
    }
}
///Field `UIE` writer - Update interrupt enable
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG, UIE>;
impl<'a, REG> UIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::B0x0)
    }
    ///Update interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::B0x1)
    }
}
/**Capture/Compare 1 interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IE {
    ///0: CC1 interrupt disabled.
    B0x0 = 0,
    ///1: CC1 interrupt enabled.
    B0x1 = 1,
}
impl From<CC1IE> for bool {
    #[inline(always)]
    fn from(variant: CC1IE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IE` reader - Capture/Compare 1 interrupt enable
pub type CC1IE_R = crate::BitReader<CC1IE>;
impl CC1IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1IE {
        match self.bits {
            false => CC1IE::B0x0,
            true => CC1IE::B0x1,
        }
    }
    ///CC1 interrupt disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1IE::B0x0
    }
    ///CC1 interrupt enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1IE::B0x1
    }
}
///Field `CC1IE` writer - Capture/Compare 1 interrupt enable
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG, CC1IE>;
impl<'a, REG> CC1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC1 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::B0x0)
    }
    ///CC1 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::B0x1)
    }
}
/**Capture/Compare 2 interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC2IE {
    ///0: CC2 interrupt disabled.
    B0x0 = 0,
    ///1: CC2 interrupt enabled.
    B0x1 = 1,
}
impl From<CC2IE> for bool {
    #[inline(always)]
    fn from(variant: CC2IE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC2IE` reader - Capture/Compare 2 interrupt enable
pub type CC2IE_R = crate::BitReader<CC2IE>;
impl CC2IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC2IE {
        match self.bits {
            false => CC2IE::B0x0,
            true => CC2IE::B0x1,
        }
    }
    ///CC2 interrupt disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC2IE::B0x0
    }
    ///CC2 interrupt enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC2IE::B0x1
    }
}
///Field `CC2IE` writer - Capture/Compare 2 interrupt enable
pub type CC2IE_W<'a, REG> = crate::BitWriter<'a, REG, CC2IE>;
impl<'a, REG> CC2IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC2 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2IE::B0x0)
    }
    ///CC2 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2IE::B0x1)
    }
}
/**Capture/Compare 3 interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC3IE {
    ///0: CC3 interrupt disabled.
    B0x0 = 0,
    ///1: CC3 interrupt enabled.
    B0x1 = 1,
}
impl From<CC3IE> for bool {
    #[inline(always)]
    fn from(variant: CC3IE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC3IE` reader - Capture/Compare 3 interrupt enable
pub type CC3IE_R = crate::BitReader<CC3IE>;
impl CC3IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC3IE {
        match self.bits {
            false => CC3IE::B0x0,
            true => CC3IE::B0x1,
        }
    }
    ///CC3 interrupt disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC3IE::B0x0
    }
    ///CC3 interrupt enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC3IE::B0x1
    }
}
///Field `CC3IE` writer - Capture/Compare 3 interrupt enable
pub type CC3IE_W<'a, REG> = crate::BitWriter<'a, REG, CC3IE>;
impl<'a, REG> CC3IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC3 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC3IE::B0x0)
    }
    ///CC3 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC3IE::B0x1)
    }
}
/**Capture/Compare 4 interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC4IE {
    ///0: CC4 interrupt disabled.
    B0x0 = 0,
    ///1: CC4 interrupt enabled.
    B0x1 = 1,
}
impl From<CC4IE> for bool {
    #[inline(always)]
    fn from(variant: CC4IE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC4IE` reader - Capture/Compare 4 interrupt enable
pub type CC4IE_R = crate::BitReader<CC4IE>;
impl CC4IE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC4IE {
        match self.bits {
            false => CC4IE::B0x0,
            true => CC4IE::B0x1,
        }
    }
    ///CC4 interrupt disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC4IE::B0x0
    }
    ///CC4 interrupt enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC4IE::B0x1
    }
}
///Field `CC4IE` writer - Capture/Compare 4 interrupt enable
pub type CC4IE_W<'a, REG> = crate::BitWriter<'a, REG, CC4IE>;
impl<'a, REG> CC4IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC4 interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC4IE::B0x0)
    }
    ///CC4 interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC4IE::B0x1)
    }
}
/**Trigger interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE {
    ///0: Trigger interrupt disabled.
    B0x0 = 0,
    ///1: Trigger interrupt enabled.
    B0x1 = 1,
}
impl From<TIE> for bool {
    #[inline(always)]
    fn from(variant: TIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TIE` reader - Trigger interrupt enable
pub type TIE_R = crate::BitReader<TIE>;
impl TIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIE {
        match self.bits {
            false => TIE::B0x0,
            true => TIE::B0x1,
        }
    }
    ///Trigger interrupt disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIE::B0x0
    }
    ///Trigger interrupt enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIE::B0x1
    }
}
///Field `TIE` writer - Trigger interrupt enable
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG, TIE>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger interrupt disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE::B0x0)
    }
    ///Trigger interrupt enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIE::B0x1)
    }
}
/**Update DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDE {
    ///0: Update DMA request disabled.
    B0x0 = 0,
    ///1: Update DMA request enabled.
    B0x1 = 1,
}
impl From<UDE> for bool {
    #[inline(always)]
    fn from(variant: UDE) -> Self {
        variant as u8 != 0
    }
}
///Field `UDE` reader - Update DMA request enable
pub type UDE_R = crate::BitReader<UDE>;
impl UDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UDE {
        match self.bits {
            false => UDE::B0x0,
            true => UDE::B0x1,
        }
    }
    ///Update DMA request disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UDE::B0x0
    }
    ///Update DMA request enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UDE::B0x1
    }
}
///Field `UDE` writer - Update DMA request enable
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG, UDE>;
impl<'a, REG> UDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update DMA request disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::B0x0)
    }
    ///Update DMA request enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::B0x1)
    }
}
/**Capture/Compare 1 DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1DE {
    ///0: CC1 DMA request disabled.
    B0x0 = 0,
    ///1: CC1 DMA request enabled.
    B0x1 = 1,
}
impl From<CC1DE> for bool {
    #[inline(always)]
    fn from(variant: CC1DE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1DE` reader - Capture/Compare 1 DMA request enable
pub type CC1DE_R = crate::BitReader<CC1DE>;
impl CC1DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1DE {
        match self.bits {
            false => CC1DE::B0x0,
            true => CC1DE::B0x1,
        }
    }
    ///CC1 DMA request disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1DE::B0x0
    }
    ///CC1 DMA request enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1DE::B0x1
    }
}
///Field `CC1DE` writer - Capture/Compare 1 DMA request enable
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG, CC1DE>;
impl<'a, REG> CC1DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC1 DMA request disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::B0x0)
    }
    ///CC1 DMA request enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::B0x1)
    }
}
/**Capture/Compare 2 DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC2DE {
    ///0: CC2 DMA request disabled.
    B0x0 = 0,
    ///1: CC2 DMA request enabled.
    B0x1 = 1,
}
impl From<CC2DE> for bool {
    #[inline(always)]
    fn from(variant: CC2DE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC2DE` reader - Capture/Compare 2 DMA request enable
pub type CC2DE_R = crate::BitReader<CC2DE>;
impl CC2DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC2DE {
        match self.bits {
            false => CC2DE::B0x0,
            true => CC2DE::B0x1,
        }
    }
    ///CC2 DMA request disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC2DE::B0x0
    }
    ///CC2 DMA request enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC2DE::B0x1
    }
}
///Field `CC2DE` writer - Capture/Compare 2 DMA request enable
pub type CC2DE_W<'a, REG> = crate::BitWriter<'a, REG, CC2DE>;
impl<'a, REG> CC2DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC2 DMA request disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2DE::B0x0)
    }
    ///CC2 DMA request enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2DE::B0x1)
    }
}
/**Capture/Compare 3 DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC3DE {
    ///0: CC3 DMA request disabled.
    B0x0 = 0,
    ///1: CC3 DMA request enabled.
    B0x1 = 1,
}
impl From<CC3DE> for bool {
    #[inline(always)]
    fn from(variant: CC3DE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC3DE` reader - Capture/Compare 3 DMA request enable
pub type CC3DE_R = crate::BitReader<CC3DE>;
impl CC3DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC3DE {
        match self.bits {
            false => CC3DE::B0x0,
            true => CC3DE::B0x1,
        }
    }
    ///CC3 DMA request disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC3DE::B0x0
    }
    ///CC3 DMA request enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC3DE::B0x1
    }
}
///Field `CC3DE` writer - Capture/Compare 3 DMA request enable
pub type CC3DE_W<'a, REG> = crate::BitWriter<'a, REG, CC3DE>;
impl<'a, REG> CC3DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC3 DMA request disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC3DE::B0x0)
    }
    ///CC3 DMA request enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC3DE::B0x1)
    }
}
/**Capture/Compare 4 DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC4DE {
    ///0: CC4 DMA request disabled.
    B0x0 = 0,
    ///1: CC4 DMA request enabled.
    B0x1 = 1,
}
impl From<CC4DE> for bool {
    #[inline(always)]
    fn from(variant: CC4DE) -> Self {
        variant as u8 != 0
    }
}
///Field `CC4DE` reader - Capture/Compare 4 DMA request enable
pub type CC4DE_R = crate::BitReader<CC4DE>;
impl CC4DE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC4DE {
        match self.bits {
            false => CC4DE::B0x0,
            true => CC4DE::B0x1,
        }
    }
    ///CC4 DMA request disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC4DE::B0x0
    }
    ///CC4 DMA request enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC4DE::B0x1
    }
}
///Field `CC4DE` writer - Capture/Compare 4 DMA request enable
pub type CC4DE_W<'a, REG> = crate::BitWriter<'a, REG, CC4DE>;
impl<'a, REG> CC4DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CC4 DMA request disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC4DE::B0x0)
    }
    ///CC4 DMA request enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC4DE::B0x1)
    }
}
/**Trigger DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE {
    ///0: Trigger DMA request disabled.
    B0x0 = 0,
    ///1: Trigger DMA request enabled.
    B0x1 = 1,
}
impl From<TDE> for bool {
    #[inline(always)]
    fn from(variant: TDE) -> Self {
        variant as u8 != 0
    }
}
///Field `TDE` reader - Trigger DMA request enable
pub type TDE_R = crate::BitReader<TDE>;
impl TDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TDE {
        match self.bits {
            false => TDE::B0x0,
            true => TDE::B0x1,
        }
    }
    ///Trigger DMA request disabled.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TDE::B0x0
    }
    ///Trigger DMA request enabled.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TDE::B0x1
    }
}
///Field `TDE` writer - Trigger DMA request enable
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG, TDE>;
impl<'a, REG> TDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger DMA request disabled.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE::B0x0)
    }
    ///Trigger DMA request enabled.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TDE::B0x1)
    }
}
impl R {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CC1IE_R {
        CC1IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CC2IE_R {
        CC2IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&self) -> CC3IE_R {
        CC3IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&self) -> CC4IE_R {
        CC4IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CC1DE_R {
        CC1DE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&self) -> CC2DE_R {
        CC2DE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&self) -> CC3DE_R {
        CC3DE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&self) -> CC4DE_R {
        CC4DE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM3_DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .field("cc2ie", &self.cc2ie())
            .field("cc3ie", &self.cc3ie())
            .field("cc4ie", &self.cc4ie())
            .field("tie", &self.tie())
            .field("ude", &self.ude())
            .field("cc1de", &self.cc1de())
            .field("cc2de", &self.cc2de())
            .field("cc3de", &self.cc3de())
            .field("cc4de", &self.cc4de())
            .field("tde", &self.tde())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<'_, TIM3_DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, TIM3_DIERrs> {
        CC1IE_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<'_, TIM3_DIERrs> {
        CC2IE_W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W<'_, TIM3_DIERrs> {
        CC3IE_W::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W<'_, TIM3_DIERrs> {
        CC4IE_W::new(self, 4)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, TIM3_DIERrs> {
        TIE_W::new(self, 6)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<'_, TIM3_DIERrs> {
        UDE_W::new(self, 8)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<'_, TIM3_DIERrs> {
        CC1DE_W::new(self, 9)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<'_, TIM3_DIERrs> {
        CC2DE_W::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W<'_, TIM3_DIERrs> {
        CC3DE_W::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W<'_, TIM3_DIERrs> {
        CC4DE_W::new(self, 12)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<'_, TIM3_DIERrs> {
        TDE_W::new(self, 14)
    }
}
/**TIM3 DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim3_dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim3_dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM3:TIM3_DIER)*/
pub struct TIM3_DIERrs;
impl crate::RegisterSpec for TIM3_DIERrs {
    type Ux = u16;
}
///`read()` method returns [`tim3_dier::R`](R) reader structure
impl crate::Readable for TIM3_DIERrs {}
///`write(|w| ..)` method takes [`tim3_dier::W`](W) writer structure
impl crate::Writable for TIM3_DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM3_DIER to value 0
impl crate::Resettable for TIM3_DIERrs {}
