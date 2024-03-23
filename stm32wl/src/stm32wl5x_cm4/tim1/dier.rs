#[doc = "Register `DIER` reader"]
pub type R = crate::R<DIERrs>;
#[doc = "Register `DIER` writer"]
pub type W = crate::W<DIERrs>;
#[doc = "Update interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIE {
    #[doc = "0: Update interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Update interrupt enabled"]
    Enabled = 1,
}
impl From<UIE> for bool {
    #[inline(always)]
    fn from(variant: UIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UIE` reader - Update interrupt enable"]
pub type UIE_R = crate::BitReader<UIE>;
impl UIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UIE {
        match self.bits {
            false => UIE::Disabled,
            true => UIE::Enabled,
        }
    }
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UIE::Disabled
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UIE::Enabled
    }
}
#[doc = "Field `UIE` writer - Update interrupt enable"]
pub type UIE_W<'a, REG> = crate::BitWriter<'a, REG, UIE>;
impl<'a, REG> UIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::Disabled)
    }
    #[doc = "Update interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::Enabled)
    }
}
#[doc = "Capture/Compare %s interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IE {
    #[doc = "0: CCx interrupt disabled"]
    Disabled = 0,
    #[doc = "1: CCx interrupt enabled"]
    Enabled = 1,
}
impl From<CC1IE> for bool {
    #[inline(always)]
    fn from(variant: CC1IE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCIE(1-4)` reader - Capture/Compare %s interrupt enable"]
pub type CCIE_R = crate::BitReader<CC1IE>;
impl CCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1IE {
        match self.bits {
            false => CC1IE::Disabled,
            true => CC1IE::Enabled,
        }
    }
    #[doc = "CCx interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1IE::Disabled
    }
    #[doc = "CCx interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1IE::Enabled
    }
}
#[doc = "Field `CCIE(1-4)` writer - Capture/Compare %s interrupt enable"]
pub type CCIE_W<'a, REG> = crate::BitWriter<'a, REG, CC1IE>;
impl<'a, REG> CCIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::Disabled)
    }
    #[doc = "CCx interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::Enabled)
    }
}
#[doc = "COM interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIE {
    #[doc = "0: COM interrupt disabled"]
    Disabled = 0,
    #[doc = "1: COM interrupt enabled"]
    Enabled = 1,
}
impl From<COMIE> for bool {
    #[inline(always)]
    fn from(variant: COMIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMIE` reader - COM interrupt enable"]
pub type COMIE_R = crate::BitReader<COMIE>;
impl COMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMIE {
        match self.bits {
            false => COMIE::Disabled,
            true => COMIE::Enabled,
        }
    }
    #[doc = "COM interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMIE::Disabled
    }
    #[doc = "COM interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMIE::Enabled
    }
}
#[doc = "Field `COMIE` writer - COM interrupt enable"]
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG, COMIE>;
impl<'a, REG> COMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COM interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMIE::Disabled)
    }
    #[doc = "COM interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMIE::Enabled)
    }
}
#[doc = "Trigger interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE {
    #[doc = "0: Trigger interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Trigger interrupt enabled"]
    Enabled = 1,
}
impl From<TIE> for bool {
    #[inline(always)]
    fn from(variant: TIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIE` reader - Trigger interrupt enable"]
pub type TIE_R = crate::BitReader<TIE>;
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TIE {
        match self.bits {
            false => TIE::Disabled,
            true => TIE::Enabled,
        }
    }
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIE::Disabled
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIE::Enabled
    }
}
#[doc = "Field `TIE` writer - Trigger interrupt enable"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG, TIE>;
impl<'a, REG> TIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIE::Disabled)
    }
    #[doc = "Trigger interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TIE::Enabled)
    }
}
#[doc = "Break interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIE {
    #[doc = "0: Break interrupt disabled"]
    Disabled = 0,
    #[doc = "1: Break interrupt enabled"]
    Enabled = 1,
}
impl From<BIE> for bool {
    #[inline(always)]
    fn from(variant: BIE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIE` reader - Break interrupt enable"]
pub type BIE_R = crate::BitReader<BIE>;
impl BIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BIE {
        match self.bits {
            false => BIE::Disabled,
            true => BIE::Enabled,
        }
    }
    #[doc = "Break interrupt disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == BIE::Disabled
    }
    #[doc = "Break interrupt enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == BIE::Enabled
    }
}
#[doc = "Field `BIE` writer - Break interrupt enable"]
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG, BIE>;
impl<'a, REG> BIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Break interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(BIE::Disabled)
    }
    #[doc = "Break interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(BIE::Enabled)
    }
}
#[doc = "Update DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDE {
    #[doc = "0: Update DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Update DMA request enabled"]
    Enabled = 1,
}
impl From<UDE> for bool {
    #[inline(always)]
    fn from(variant: UDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UDE` reader - Update DMA request enable"]
pub type UDE_R = crate::BitReader<UDE>;
impl UDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UDE {
        match self.bits {
            false => UDE::Disabled,
            true => UDE::Enabled,
        }
    }
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDE::Disabled
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDE::Enabled
    }
}
#[doc = "Field `UDE` writer - Update DMA request enable"]
pub type UDE_W<'a, REG> = crate::BitWriter<'a, REG, UDE>;
impl<'a, REG> UDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Update DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::Disabled)
    }
    #[doc = "Update DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::Enabled)
    }
}
#[doc = "Capture/Compare %s DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1DE {
    #[doc = "0: CCx DMA request disabled"]
    Disabled = 0,
    #[doc = "1: CCx DMA request enabled"]
    Enabled = 1,
}
impl From<CC1DE> for bool {
    #[inline(always)]
    fn from(variant: CC1DE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCDE(1-4)` reader - Capture/Compare %s DMA request enable"]
pub type CCDE_R = crate::BitReader<CC1DE>;
impl CCDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CC1DE {
        match self.bits {
            false => CC1DE::Disabled,
            true => CC1DE::Enabled,
        }
    }
    #[doc = "CCx DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CC1DE::Disabled
    }
    #[doc = "CCx DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CC1DE::Enabled
    }
}
#[doc = "Field `CCDE(1-4)` writer - Capture/Compare %s DMA request enable"]
pub type CCDE_W<'a, REG> = crate::BitWriter<'a, REG, CC1DE>;
impl<'a, REG> CCDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CCx DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::Disabled)
    }
    #[doc = "CCx DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::Enabled)
    }
}
#[doc = "COM DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMDE {
    #[doc = "0: COM DMA request disabled"]
    Disabled = 0,
    #[doc = "1: COM DMA request enabled"]
    Enabled = 1,
}
impl From<COMDE> for bool {
    #[inline(always)]
    fn from(variant: COMDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COMDE` reader - COM DMA request enable"]
pub type COMDE_R = crate::BitReader<COMDE>;
impl COMDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COMDE {
        match self.bits {
            false => COMDE::Disabled,
            true => COMDE::Enabled,
        }
    }
    #[doc = "COM DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == COMDE::Disabled
    }
    #[doc = "COM DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == COMDE::Enabled
    }
}
#[doc = "Field `COMDE` writer - COM DMA request enable"]
pub type COMDE_W<'a, REG> = crate::BitWriter<'a, REG, COMDE>;
impl<'a, REG> COMDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "COM DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMDE::Disabled)
    }
    #[doc = "COM DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(COMDE::Enabled)
    }
}
#[doc = "Trigger DMA request enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE {
    #[doc = "0: Trigger DMA request disabled"]
    Disabled = 0,
    #[doc = "1: Trigger DMA request enabled"]
    Enabled = 1,
}
impl From<TDE> for bool {
    #[inline(always)]
    fn from(variant: TDE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDE` reader - Trigger DMA request enable"]
pub type TDE_R = crate::BitReader<TDE>;
impl TDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TDE {
        match self.bits {
            false => TDE::Disabled,
            true => TDE::Enabled,
        }
    }
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDE::Disabled
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDE::Enabled
    }
}
#[doc = "Field `TDE` writer - Trigger DMA request enable"]
pub type TDE_W<'a, REG> = crate::BitWriter<'a, REG, TDE>;
impl<'a, REG> TDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger DMA request disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TDE::Disabled)
    }
    #[doc = "Trigger DMA request enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TDE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Capture/Compare (1-4) interrupt enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IE` field"]
    #[inline(always)]
    pub fn ccie(&self, n: u8) -> CCIE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIE_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-4) interrupt enable"]
    #[inline(always)]
    pub fn ccie_iter(&self) -> impl Iterator<Item = CCIE_R> + '_ {
        (0..4).map(move |n| CCIE_R::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    pub fn cc1ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    pub fn cc2ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    pub fn cc3ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    pub fn cc4ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    pub fn ude(&self) -> UDE_R {
        UDE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Capture/Compare (1-4) DMA request enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1DE` field"]
    #[inline(always)]
    pub fn ccde(&self, n: u8) -> CCDE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCDE_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Capture/Compare (1-4) DMA request enable"]
    #[inline(always)]
    pub fn ccde_iter(&self) -> impl Iterator<Item = CCDE_R> + '_ {
        (0..4).map(move |n| CCDE_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    pub fn cc1de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    pub fn cc2de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    pub fn cc3de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    pub fn cc4de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    #[doc = "Capture/Compare (1-4) interrupt enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1IE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccie(&mut self, n: u8) -> CCIE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIE_W::new(self, n + 1)
    }
    #[doc = "Bit 1 - Capture/Compare 1 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Capture/Compare 2 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Capture/Compare 3 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc3ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Capture/Compare 4 interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc4ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - COM interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn comie(&mut self) -> COMIE_W<DIERrs> {
        COMIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Trigger interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<DIERrs> {
        TIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Break interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn bie(&mut self) -> BIE_W<DIERrs> {
        BIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Update DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn ude(&mut self) -> UDE_W<DIERrs> {
        UDE_W::new(self, 8)
    }
    #[doc = "Capture/Compare (1-4) DMA request enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CC1DE` field"]
    #[inline(always)]
    #[must_use]
    pub fn ccde(&mut self, n: u8) -> CCDE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCDE_W::new(self, n + 9)
    }
    #[doc = "Bit 9 - Capture/Compare 1 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc1de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Capture/Compare 2 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc2de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Capture/Compare 3 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc3de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Capture/Compare 4 DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn cc4de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 12)
    }
    #[doc = "Bit 13 - COM DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn comde(&mut self) -> COMDE_W<DIERrs> {
        COMDE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Trigger DMA request enable"]
    #[inline(always)]
    #[must_use]
    pub fn tde(&mut self) -> TDE_W<DIERrs> {
        TDE_W::new(self, 14)
    }
}
#[doc = "DMA/interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dier::R`](R) reader structure"]
impl crate::Readable for DIERrs {}
#[doc = "`write(|w| ..)` method takes [`dier::W`](W) writer structure"]
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIER to value 0"]
impl crate::Resettable for DIERrs {
    const RESET_VALUE: u32 = 0;
}
