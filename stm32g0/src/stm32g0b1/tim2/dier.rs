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
/**Capture/Compare %s interrupt enable

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
///Field `CCIE(1-4)` reader - Capture/Compare %s interrupt enable
pub type CCIE_R = crate::BitReader<CC1IE>;
impl CCIE_R {
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
///Field `CCIE(1-4)` writer - Capture/Compare %s interrupt enable
pub type CCIE_W<'a, REG> = crate::BitWriter<'a, REG, CC1IE>;
impl<'a, REG> CCIE_W<'a, REG>
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
/**Capture/Compare %s DMA request enable

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
///Field `CCDE(1-4)` reader - Capture/Compare %s DMA request enable
pub type CCDE_R = crate::BitReader<CC1DE>;
impl CCDE_R {
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
///Field `CCDE(1-4)` writer - Capture/Compare %s DMA request enable
pub type CCDE_W<'a, REG> = crate::BitWriter<'a, REG, CC1DE>;
impl<'a, REG> CCDE_W<'a, REG>
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
impl R {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&self) -> UIE_R {
        UIE_R::new((self.bits & 1) != 0)
    }
    ///Capture/Compare (1-4) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IE` field.</div>
    #[inline(always)]
    pub fn ccie(&self, n: u8) -> CCIE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIE_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-4) interrupt enable
    #[inline(always)]
    pub fn ccie_iter(&self) -> impl Iterator<Item = CCIE_R> + '_ {
        (0..4).map(move |n| CCIE_R::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&self) -> CCIE_R {
        CCIE_R::new(((self.bits >> 4) & 1) != 0)
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
    ///Capture/Compare (1-4) DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1DE` field.</div>
    #[inline(always)]
    pub fn ccde(&self, n: u8) -> CCDE_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCDE_R::new(((self.bits >> (n + 9)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Capture/Compare (1-4) DMA request enable
    #[inline(always)]
    pub fn ccde_iter(&self) -> impl Iterator<Item = CCDE_R> + '_ {
        (0..4).map(move |n| CCDE_R::new(((self.bits >> (n + 9)) & 1) != 0))
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&self) -> CCDE_R {
        CCDE_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
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
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Capture/Compare (1-4) interrupt enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1IE` field.</div>
    #[inline(always)]
    pub fn ccie(&mut self, n: u8) -> CCIE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCIE_W::new(self, n + 1)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 3 interrupt enable
    #[inline(always)]
    pub fn cc3ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 4 interrupt enable
    #[inline(always)]
    pub fn cc4ie(&mut self) -> CCIE_W<DIERrs> {
        CCIE_W::new(self, 4)
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
    ///Capture/Compare (1-4) DMA request enable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `CC1DE` field.</div>
    #[inline(always)]
    pub fn ccde(&mut self, n: u8) -> CCDE_W<DIERrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CCDE_W::new(self, n + 9)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 9)
    }
    ///Bit 10 - Capture/Compare 2 DMA request enable
    #[inline(always)]
    pub fn cc2de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 DMA request enable
    #[inline(always)]
    pub fn cc3de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 DMA request enable
    #[inline(always)]
    pub fn cc4de(&mut self) -> CCDE_W<DIERrs> {
        CCDE_W::new(self, 12)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<DIERrs> {
        TDE_W::new(self, 14)
    }
}
/**DMA/Interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G0B1.html#TIM2:DIER)*/
pub struct DIERrs;
impl crate::RegisterSpec for DIERrs {
    type Ux = u32;
}
///`read()` method returns [`dier::R`](R) reader structure
impl crate::Readable for DIERrs {}
///`write(|w| ..)` method takes [`dier::W`](W) writer structure
impl crate::Writable for DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DIER to value 0
impl crate::Resettable for DIERrs {}
