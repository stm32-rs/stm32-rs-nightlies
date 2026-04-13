///Register `TIM15_DIER` reader
pub type R = crate::R<TIM15_DIERrs>;
///Register `TIM15_DIER` writer
pub type W = crate::W<TIM15_DIERrs>;
/**Update interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIE {
    ///0: Update interrupt disabled
    B0x0 = 0,
    ///1: Update interrupt enabled
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
    ///Update interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UIE::B0x0
    }
    ///Update interrupt enabled
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
    ///Update interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UIE::B0x0)
    }
    ///Update interrupt enabled
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
    ///0: CC1 interrupt disabled
    B0x0 = 0,
    ///1: CC1 interrupt enabled
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
    ///CC1 interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1IE::B0x0
    }
    ///CC1 interrupt enabled
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
    ///CC1 interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IE::B0x0)
    }
    ///CC1 interrupt enabled
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
    ///0: CC2 interrupt disabled
    B0x0 = 0,
    ///1: CC2 interrupt enabled
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
    ///CC2 interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC2IE::B0x0
    }
    ///CC2 interrupt enabled
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
    ///CC2 interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC2IE::B0x0)
    }
    ///CC2 interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC2IE::B0x1)
    }
}
/**COM interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIE {
    ///0: COM interrupt disabled
    B0x0 = 0,
    ///1: COM interrupt enabled
    B0x1 = 1,
}
impl From<COMIE> for bool {
    #[inline(always)]
    fn from(variant: COMIE) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIE` reader - COM interrupt enable
pub type COMIE_R = crate::BitReader<COMIE>;
impl COMIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMIE {
        match self.bits {
            false => COMIE::B0x0,
            true => COMIE::B0x1,
        }
    }
    ///COM interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COMIE::B0x0
    }
    ///COM interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COMIE::B0x1
    }
}
///Field `COMIE` writer - COM interrupt enable
pub type COMIE_W<'a, REG> = crate::BitWriter<'a, REG, COMIE>;
impl<'a, REG> COMIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///COM interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMIE::B0x0)
    }
    ///COM interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMIE::B0x1)
    }
}
/**Trigger interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE {
    ///0: Trigger interrupt disabled
    B0x0 = 0,
    ///1: Trigger interrupt enabled
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
    ///Trigger interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIE::B0x0
    }
    ///Trigger interrupt enabled
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
    ///Trigger interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIE::B0x0)
    }
    ///Trigger interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIE::B0x1)
    }
}
/**Break interrupt enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIE {
    ///0: Break interrupt disabled
    B0x0 = 0,
    ///1: Break interrupt enabled
    B0x1 = 1,
}
impl From<BIE> for bool {
    #[inline(always)]
    fn from(variant: BIE) -> Self {
        variant as u8 != 0
    }
}
///Field `BIE` reader - Break interrupt enable
pub type BIE_R = crate::BitReader<BIE>;
impl BIE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIE {
        match self.bits {
            false => BIE::B0x0,
            true => BIE::B0x1,
        }
    }
    ///Break interrupt disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BIE::B0x0
    }
    ///Break interrupt enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BIE::B0x1
    }
}
///Field `BIE` writer - Break interrupt enable
pub type BIE_W<'a, REG> = crate::BitWriter<'a, REG, BIE>;
impl<'a, REG> BIE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Break interrupt disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BIE::B0x0)
    }
    ///Break interrupt enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BIE::B0x1)
    }
}
/**Update DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDE {
    ///0: Update DMA request disabled
    B0x0 = 0,
    ///1: Update DMA request enabled
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
    ///Update DMA request disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UDE::B0x0
    }
    ///Update DMA request enabled
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
    ///Update DMA request disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UDE::B0x0)
    }
    ///Update DMA request enabled
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
    ///0: CC1 DMA request disabled
    B0x0 = 0,
    ///1: CC1 DMA request enabled
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
    ///CC1 DMA request disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1DE::B0x0
    }
    ///CC1 DMA request enabled
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
    ///CC1 DMA request disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::B0x0)
    }
    ///CC1 DMA request enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1DE::B0x1)
    }
}
/**COM DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMDE {
    ///0: COM DMA request disabled
    B0x0 = 0,
    ///1: COM DMA request enabled
    B0x1 = 1,
}
impl From<COMDE> for bool {
    #[inline(always)]
    fn from(variant: COMDE) -> Self {
        variant as u8 != 0
    }
}
///Field `COMDE` reader - COM DMA request enable
pub type COMDE_R = crate::BitReader<COMDE>;
impl COMDE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMDE {
        match self.bits {
            false => COMDE::B0x0,
            true => COMDE::B0x1,
        }
    }
    ///COM DMA request disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COMDE::B0x0
    }
    ///COM DMA request enabled
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COMDE::B0x1
    }
}
///Field `COMDE` writer - COM DMA request enable
pub type COMDE_W<'a, REG> = crate::BitWriter<'a, REG, COMDE>;
impl<'a, REG> COMDE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///COM DMA request disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMDE::B0x0)
    }
    ///COM DMA request enabled
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMDE::B0x1)
    }
}
/**Trigger DMA request enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDE {
    ///0: Trigger DMA request disabled
    B0x0 = 0,
    ///1: Trigger DMA request enabled
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
    ///Trigger DMA request disabled
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TDE::B0x0
    }
    ///Trigger DMA request enabled
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
    ///Trigger DMA request disabled
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TDE::B0x0)
    }
    ///Trigger DMA request enabled
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
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&self) -> BIE_R {
        BIE_R::new(((self.bits >> 7) & 1) != 0)
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
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&self) -> COMDE_R {
        COMDE_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&self) -> TDE_R {
        TDE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .field("cc2ie", &self.cc2ie())
            .field("comie", &self.comie())
            .field("tie", &self.tie())
            .field("bie", &self.bie())
            .field("ude", &self.ude())
            .field("cc1de", &self.cc1de())
            .field("comde", &self.comde())
            .field("tde", &self.tde())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<'_, TIM15_DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, TIM15_DIERrs> {
        CC1IE_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt enable
    #[inline(always)]
    pub fn cc2ie(&mut self) -> CC2IE_W<'_, TIM15_DIERrs> {
        CC2IE_W::new(self, 2)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W<'_, TIM15_DIERrs> {
        COMIE_W::new(self, 5)
    }
    ///Bit 6 - Trigger interrupt enable
    #[inline(always)]
    pub fn tie(&mut self) -> TIE_W<'_, TIM15_DIERrs> {
        TIE_W::new(self, 6)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<'_, TIM15_DIERrs> {
        BIE_W::new(self, 7)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<'_, TIM15_DIERrs> {
        UDE_W::new(self, 8)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<'_, TIM15_DIERrs> {
        CC1DE_W::new(self, 9)
    }
    ///Bit 13 - COM DMA request enable
    #[inline(always)]
    pub fn comde(&mut self) -> COMDE_W<'_, TIM15_DIERrs> {
        COMDE_W::new(self, 13)
    }
    ///Bit 14 - Trigger DMA request enable
    #[inline(always)]
    pub fn tde(&mut self) -> TDE_W<'_, TIM15_DIERrs> {
        TDE_W::new(self, 14)
    }
}
/**TIM15 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim15_dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM15:TIM15_DIER)*/
pub struct TIM15_DIERrs;
impl crate::RegisterSpec for TIM15_DIERrs {
    type Ux = u16;
}
///`read()` method returns [`tim15_dier::R`](R) reader structure
impl crate::Readable for TIM15_DIERrs {}
///`write(|w| ..)` method takes [`tim15_dier::W`](W) writer structure
impl crate::Writable for TIM15_DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_DIER to value 0
impl crate::Resettable for TIM15_DIERrs {}
