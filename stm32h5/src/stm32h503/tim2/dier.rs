///Register `DIER` reader
pub type R = crate::R<DIERrs>;
///Register `DIER` writer
pub type W = crate::W<DIERrs>;
/**Update interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIE {
    ///0: Update interrupt disabled
    Disabled = 0,
    ///1: Update interrupt enabled
    Enabled = 1,
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
            false => UIE::Disabled,
            true => UIE::Enabled,
        }
    }
    ///Update interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UIE::Disabled
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UIE::Enabled
    }
}
///Field `UIE` writer - Update interrupt enable
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG, UIE>;
impl<'a, REG> UIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::Disabled)
    }
    ///Update interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::Enabled)
    }
}
/**Capture/Compare 1 interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IE {
    ///0: CCx interrupt disabled
    Disabled = 0,
    ///1: CCx interrupt enabled
    Enabled = 1,
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
            false => CC1IE::Disabled,
            true => CC1IE::Enabled,
        }
    }
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1IE::Disabled
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1IE::Enabled
    }
}
///Field `CC1IE` writer - Capture/Compare 1 interrupt enable
pub type CC1IE_W<'a, REG> = crate::BitWriter<'a, REG, CC1IE>;
impl<'a, REG> CC1IE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CCx interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::Disabled)
    }
    ///CCx interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::Enabled)
    }
}
///Field `CC2IE` reader - Capture/Compare 2 interrupt enable
pub use CC1IE_R as CC2IE_R;
///Field `CC3IE` reader - Capture/Compare 3 interrupt enable
pub use CC1IE_R as CC3IE_R;
///Field `CC4IE` reader - Capture/Compare 4 interrupt enable
pub use CC1IE_R as CC4IE_R;
///Field `CC2IE` writer - Capture/Compare 2 interrupt enable
pub use CC1IE_W as CC2IE_W;
///Field `CC3IE` writer - Capture/Compare 3 interrupt enable
pub use CC1IE_W as CC3IE_W;
///Field `CC4IE` writer - Capture/Compare 4 interrupt enable
pub use CC1IE_W as CC4IE_W;
/**Trigger interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE {
    ///0: Trigger interrupt disabled
    Disabled = 0,
    ///1: Trigger interrupt enabled
    Enabled = 1,
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
            false => TIE::Disabled,
            true => TIE::Enabled,
        }
    }
    ///Trigger interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIE::Disabled
    }
    ///Trigger interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIE::Enabled
    }
}
///Field `TIE` writer - Trigger interrupt enable
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG, TIE>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIE::Disabled)
    }
    ///Trigger interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIE::Enabled)
    }
}
/**Update DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDE {
    ///0: Update DMA request disabled
    Disabled = 0,
    ///1: Update DMA request enabled
    Enabled = 1,
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
            false => UDE::Disabled,
            true => UDE::Enabled,
        }
    }
    ///Update DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDE::Disabled
    }
    ///Update DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDE::Enabled
    }
}
///Field `UDE` writer - Update DMA request enable
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG, UDE>;
impl<'a, REG> UDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Update DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::Disabled)
    }
    ///Update DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::Enabled)
    }
}
/**Capture/Compare 1 DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1DE {
    ///0: CCx DMA request disabled
    Disabled = 0,
    ///1: CCx DMA request enabled
    Enabled = 1,
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
            false => CC1DE::Disabled,
            true => CC1DE::Enabled,
        }
    }
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1DE::Disabled
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1DE::Enabled
    }
}
///Field `CC1DE` writer - Capture/Compare 1 DMA request enable
pub type CC1DE_W<'a, REG> = crate::BitWriter<'a, REG, CC1DE>;
impl<'a, REG> CC1DE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///CCx DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::Disabled)
    }
    ///CCx DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::Enabled)
    }
}
///Field `CC2DE` reader - Capture/Compare 2 DMA request enable
pub use CC1DE_R as CC2DE_R;
///Field `CC3DE` reader - Capture/Compare 3 DMA request enable
pub use CC1DE_R as CC3DE_R;
///Field `CC4DE` reader - Capture/Compare 4 DMA request enable
pub use CC1DE_R as CC4DE_R;
///Field `CC2DE` writer - Capture/Compare 2 DMA request enable
pub use CC1DE_W as CC2DE_W;
///Field `CC3DE` writer - Capture/Compare 3 DMA request enable
pub use CC1DE_W as CC3DE_W;
///Field `CC4DE` writer - Capture/Compare 4 DMA request enable
pub use CC1DE_W as CC4DE_W;
/**Trigger DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE {
    ///0: Trigger DMA request disabled
    Disabled = 0,
    ///1: Trigger DMA request enabled
    Enabled = 1,
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
            false => TDE::Disabled,
            true => TDE::Enabled,
        }
    }
    ///Trigger DMA request disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDE::Disabled
    }
    ///Trigger DMA request enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDE::Enabled
    }
}
///Field `TDE` writer - Trigger DMA request enable
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG, TDE>;
impl<'a, REG> TDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Trigger DMA request disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TDE::Disabled)
    }
    ///Trigger DMA request enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TDE::Enabled)
    }
}
/**Index interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDXIE {
    ///0: Index change interrupt disabled
    Disabled = 0,
    ///1: Index change interrupt enabled
    Enabled = 1,
}
impl From<IDXIE> for bool {
    #[inline(always)]
    fn from(variant: IDXIE) -> Self {
        variant as u8 != 0
    }
}
///Field `IDXIE` reader - Index interrupt enable
pub type IDXIE_R = crate::BitReader<IDXIE>;
impl IDXIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IDXIE {
        match self.bits {
            false => IDXIE::Disabled,
            true => IDXIE::Enabled,
        }
    }
    ///Index change interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDXIE::Disabled
    }
    ///Index change interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDXIE::Enabled
    }
}
///Field `IDXIE` writer - Index interrupt enable
pub type IDXIE_W<'a, REG> = crate::BitWriter<'a, REG, IDXIE>;
impl<'a, REG> IDXIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Index change interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDXIE::Disabled)
    }
    ///Index change interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IDXIE::Enabled)
    }
}
/**Direction change interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRIE {
    ///0: Direction change interrupt disabled
    Disabled = 0,
    ///1: Direction change interrupt enabled
    Enabled = 1,
}
impl From<DIRIE> for bool {
    #[inline(always)]
    fn from(variant: DIRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `DIRIE` reader - Direction change interrupt enable
pub type DIRIE_R = crate::BitReader<DIRIE>;
impl DIRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DIRIE {
        match self.bits {
            false => DIRIE::Disabled,
            true => DIRIE::Enabled,
        }
    }
    ///Direction change interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DIRIE::Disabled
    }
    ///Direction change interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DIRIE::Enabled
    }
}
///Field `DIRIE` writer - Direction change interrupt enable
pub type DIRIE_W<'a, REG> = crate::BitWriter<'a, REG, DIRIE>;
impl<'a, REG> DIRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Direction change interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIRIE::Disabled)
    }
    ///Direction change interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DIRIE::Enabled)
    }
}
/**Index error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IERRIE {
    ///0: Index error interrupt disabled
    Disabled = 0,
    ///1: Index error interrupt enabled
    Enabled = 1,
}
impl From<IERRIE> for bool {
    #[inline(always)]
    fn from(variant: IERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `IERRIE` reader - Index error interrupt enable
pub type IERRIE_R = crate::BitReader<IERRIE>;
impl IERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> IERRIE {
        match self.bits {
            false => IERRIE::Disabled,
            true => IERRIE::Enabled,
        }
    }
    ///Index error interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IERRIE::Disabled
    }
    ///Index error interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IERRIE::Enabled
    }
}
///Field `IERRIE` writer - Index error interrupt enable
pub type IERRIE_W<'a, REG> = crate::BitWriter<'a, REG, IERRIE>;
impl<'a, REG> IERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Index error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(IERRIE::Disabled)
    }
    ///Index error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(IERRIE::Enabled)
    }
}
/**Transition error interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TERRIE {
    ///0: Transition error interrupt disabled
    Disabled = 0,
    ///1: Transition error interrupt enabled
    Enabled = 1,
}
impl From<TERRIE> for bool {
    #[inline(always)]
    fn from(variant: TERRIE) -> Self {
        variant as u8 != 0
    }
}
///Field `TERRIE` reader - Transition error interrupt enable
pub type TERRIE_R = crate::BitReader<TERRIE>;
impl TERRIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TERRIE {
        match self.bits {
            false => TERRIE::Disabled,
            true => TERRIE::Enabled,
        }
    }
    ///Transition error interrupt disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TERRIE::Disabled
    }
    ///Transition error interrupt enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TERRIE::Enabled
    }
}
///Field `TERRIE` writer - Transition error interrupt enable
pub type TERRIE_W<'a, REG> = crate::BitWriter<'a, REG, TERRIE>;
impl<'a, REG> TERRIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Transition error interrupt disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TERRIE::Disabled)
    }
    ///Transition error interrupt enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TERRIE::Enabled)
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
    ///Bit 20 - Index interrupt enable
    #[inline(always)]
    pub fn idxie(&self) -> IDXIE_R {
        IDXIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Direction change interrupt enable
    #[inline(always)]
    pub fn dirie(&self) -> DIRIE_R {
        DIRIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Index error interrupt enable
    #[inline(always)]
    pub fn ierrie(&self) -> IERRIE_R {
        IERRIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Transition error interrupt enable
    #[inline(always)]
    pub fn terrie(&self) -> TERRIE_R {
        TERRIE_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIER")
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
            .field("idxie", &self.idxie())
            .field("dirie", &self.dirie())
            .field("ierrie", &self.ierrie())
            .field("terrie", &self.terrie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<DIERrs> {
        CC1IE_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<DIERrs> {
        CC2IE_W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CC3IE_W<DIERrs> {
        CC3IE_W::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CC4IE_W<DIERrs> {
        CC4IE_W::new(self, 4)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<DIERrs> {
        TIE_W::new(self, 6)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<DIERrs> {
        UDE_W::new(self, 8)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<DIERrs> {
        CC1DE_W::new(self, 9)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CC2DE_W<DIERrs> {
        CC2DE_W::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&mut self) -> CC3DE_W<DIERrs> {
        CC3DE_W::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&mut self) -> CC4DE_W<DIERrs> {
        CC4DE_W::new(self, 12)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<DIERrs> {
        TDE_W::new(self, 14)
    }
    ///Bit 20 - Index interrupt enable
    #[inline(always)]
    pub fn idxie(&mut self) -> IDXIE_W<DIERrs> {
        IDXIE_W::new(self, 20)
    }
    ///Bit 21 - Direction change interrupt enable
    #[inline(always)]
    pub fn dirie(&mut self) -> DIRIE_W<DIERrs> {
        DIRIE_W::new(self, 21)
    }
    ///Bit 22 - Index error interrupt enable
    #[inline(always)]
    pub fn ierrie(&mut self) -> IERRIE_W<DIERrs> {
        IERRIE_W::new(self, 22)
    }
    ///Bit 23 - Transition error interrupt enable
    #[inline(always)]
    pub fn terrie(&mut self) -> TERRIE_W<DIERrs> {
        TERRIE_W::new(self, 23)
    }
}
/**TIM2 DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#TIM2:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {
    const RESET_VALUE: u32 = 0;
}
