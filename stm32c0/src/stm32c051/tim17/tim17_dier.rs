///Register `TIM17_DIER` reader
pub type R = crate::R<TIM17_DIERrs>;
///Register `TIM17_DIER` writer
pub type W = crate::W<TIM17_DIERrs>;
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
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&self) -> COMIE_R {
        COMIE_R::new(((self.bits >> 5) & 1) != 0)
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM17_DIER")
            .field("uie", &self.uie())
            .field("cc1ie", &self.cc1ie())
            .field("comie", &self.comie())
            .field("bie", &self.bie())
            .field("ude", &self.ude())
            .field("cc1de", &self.cc1de())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt enable
    #[inline(always)]
    pub fn uie(&mut self) -> UIE_W<'_, TIM17_DIERrs> {
        UIE_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt enable
    #[inline(always)]
    pub fn cc1ie(&mut self) -> CC1IE_W<'_, TIM17_DIERrs> {
        CC1IE_W::new(self, 1)
    }
    ///Bit 5 - COM interrupt enable
    #[inline(always)]
    pub fn comie(&mut self) -> COMIE_W<'_, TIM17_DIERrs> {
        COMIE_W::new(self, 5)
    }
    ///Bit 7 - Break interrupt enable
    #[inline(always)]
    pub fn bie(&mut self) -> BIE_W<'_, TIM17_DIERrs> {
        BIE_W::new(self, 7)
    }
    ///Bit 8 - Update DMA request enable
    #[inline(always)]
    pub fn ude(&mut self) -> UDE_W<'_, TIM17_DIERrs> {
        UDE_W::new(self, 8)
    }
    ///Bit 9 - Capture/Compare 1 DMA request enable
    #[inline(always)]
    pub fn cc1de(&mut self) -> CC1DE_W<'_, TIM17_DIERrs> {
        CC1DE_W::new(self, 9)
    }
}
/**TIM17 DMA/interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`tim17_dier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim17_dier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM17:TIM17_DIER)*/
pub struct TIM17_DIERrs;
impl crate::RegisterSpec for TIM17_DIERrs {
    type Ux = u16;
}
///`read()` method returns [`tim17_dier::R`](R) reader structure
impl crate::Readable for TIM17_DIERrs {}
///`write(|w| ..)` method takes [`tim17_dier::W`](W) writer structure
impl crate::Writable for TIM17_DIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM17_DIER to value 0
impl crate::Resettable for TIM17_DIERrs {}
