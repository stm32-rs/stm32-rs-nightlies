///Register `TIM15_SR` reader
pub type R = crate::R<TIM15_SRrs>;
///Register `TIM15_SR` writer
pub type W = crate::W<TIM15_SRrs>;
/**Update interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UIF {
    ///0: No update occurred.
    B0x0 = 0,
    ///1: Update interrupt pending.
    B0x1 = 1,
}
impl From<UIF> for bool {
    #[inline(always)]
    fn from(variant: UIF) -> Self {
        variant as u8 != 0
    }
}
///Field `UIF` reader - Update interrupt flag
pub type UIF_R = crate::BitReader<UIF>;
impl UIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> UIF {
        match self.bits {
            false => UIF::B0x0,
            true => UIF::B0x1,
        }
    }
    ///No update occurred.
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == UIF::B0x0
    }
    ///Update interrupt pending.
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == UIF::B0x1
    }
}
///Field `UIF` writer - Update interrupt flag
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG, UIF>;
impl<'a, REG> UIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No update occurred.
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UIF::B0x0)
    }
    ///Update interrupt pending.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UIF::B0x1)
    }
}
/**Capture/Compare 1 interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1IF {
    ///0: No compare match / No input capture occurred
    B0x0 = 0,
    ///1: A compare match or an input capture occurred
    B0x1 = 1,
}
impl From<CC1IF> for bool {
    #[inline(always)]
    fn from(variant: CC1IF) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1IF` reader - Capture/Compare 1 interrupt flag
pub type CC1IF_R = crate::BitReader<CC1IF>;
impl CC1IF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1IF {
        match self.bits {
            false => CC1IF::B0x0,
            true => CC1IF::B0x1,
        }
    }
    ///No compare match / No input capture occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1IF::B0x0
    }
    ///A compare match or an input capture occurred
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1IF::B0x1
    }
}
///Field `CC1IF` writer - Capture/Compare 1 interrupt flag
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG, CC1IF>;
impl<'a, REG> CC1IF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No compare match / No input capture occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IF::B0x0)
    }
    ///A compare match or an input capture occurred
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1IF::B0x1)
    }
}
///Field `CC2IF` reader - Capture/Compare 2 interrupt flag
pub type CC2IF_R = crate::BitReader;
///Field `CC2IF` writer - Capture/Compare 2 interrupt flag
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
/**COM interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMIF {
    ///0: No COM event occurred
    B0x0 = 0,
    ///1: COM interrupt pending
    B0x1 = 1,
}
impl From<COMIF> for bool {
    #[inline(always)]
    fn from(variant: COMIF) -> Self {
        variant as u8 != 0
    }
}
///Field `COMIF` reader - COM interrupt flag
pub type COMIF_R = crate::BitReader<COMIF>;
impl COMIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMIF {
        match self.bits {
            false => COMIF::B0x0,
            true => COMIF::B0x1,
        }
    }
    ///No COM event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COMIF::B0x0
    }
    ///COM interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COMIF::B0x1
    }
}
///Field `COMIF` writer - COM interrupt flag
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG, COMIF>;
impl<'a, REG> COMIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No COM event occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMIF::B0x0)
    }
    ///COM interrupt pending
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMIF::B0x1)
    }
}
/**Trigger interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIF {
    ///0: No trigger event occurred
    B0x0 = 0,
    ///1: Trigger interrupt pending
    B0x1 = 1,
}
impl From<TIF> for bool {
    #[inline(always)]
    fn from(variant: TIF) -> Self {
        variant as u8 != 0
    }
}
///Field `TIF` reader - Trigger interrupt flag
pub type TIF_R = crate::BitReader<TIF>;
impl TIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIF {
        match self.bits {
            false => TIF::B0x0,
            true => TIF::B0x1,
        }
    }
    ///No trigger event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TIF::B0x0
    }
    ///Trigger interrupt pending
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TIF::B0x1
    }
}
///Field `TIF` writer - Trigger interrupt flag
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG, TIF>;
impl<'a, REG> TIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No trigger event occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TIF::B0x0)
    }
    ///Trigger interrupt pending
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TIF::B0x1)
    }
}
/**Break interrupt flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIF {
    ///0: No break event occurred
    B0x0 = 0,
    ///1: An active level has been detected on the break input
    B0x1 = 1,
}
impl From<BIF> for bool {
    #[inline(always)]
    fn from(variant: BIF) -> Self {
        variant as u8 != 0
    }
}
///Field `BIF` reader - Break interrupt flag
pub type BIF_R = crate::BitReader<BIF>;
impl BIF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BIF {
        match self.bits {
            false => BIF::B0x0,
            true => BIF::B0x1,
        }
    }
    ///No break event occurred
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == BIF::B0x0
    }
    ///An active level has been detected on the break input
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == BIF::B0x1
    }
}
///Field `BIF` writer - Break interrupt flag
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG, BIF>;
impl<'a, REG> BIF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No break event occurred
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BIF::B0x0)
    }
    ///An active level has been detected on the break input
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BIF::B0x1)
    }
}
/**Capture/Compare 1 overcapture flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1OF {
    ///0: No overcapture has been detected
    B0x0 = 0,
    ///1: The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    B0x1 = 1,
}
impl From<CC1OF> for bool {
    #[inline(always)]
    fn from(variant: CC1OF) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1OF` reader - Capture/Compare 1 overcapture flag
pub type CC1OF_R = crate::BitReader<CC1OF>;
impl CC1OF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC1OF {
        match self.bits {
            false => CC1OF::B0x0,
            true => CC1OF::B0x1,
        }
    }
    ///No overcapture has been detected
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC1OF::B0x0
    }
    ///The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC1OF::B0x1
    }
}
///Field `CC1OF` writer - Capture/Compare 1 overcapture flag
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG, CC1OF>;
impl<'a, REG> CC1OF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No overcapture has been detected
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OF::B0x0)
    }
    ///The counter value has been captured in TIMx_CCR1 register while CC1IF flag was already set
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1OF::B0x1)
    }
}
///Field `CC2OF` reader - Capture/Compare 2 overcapture flag
pub type CC2OF_R = crate::BitReader;
///Field `CC2OF` writer - Capture/Compare 2 overcapture flag
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc2if", &self.cc2if())
            .field("comif", &self.comif())
            .field("tif", &self.tif())
            .field("bif", &self.bif())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<'_, TIM15_SRrs> {
        UIF_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, TIM15_SRrs> {
        CC1IF_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W<'_, TIM15_SRrs> {
        CC2IF_W::new(self, 2)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W<'_, TIM15_SRrs> {
        COMIF_W::new(self, 5)
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<'_, TIM15_SRrs> {
        TIF_W::new(self, 6)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W<'_, TIM15_SRrs> {
        BIF_W::new(self, 7)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<'_, TIM15_SRrs> {
        CC1OF_W::new(self, 9)
    }
    ///Bit 10 - Capture/Compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W<'_, TIM15_SRrs> {
        CC2OF_W::new(self, 10)
    }
}
/**TIM15 status register

You can [`read`](crate::Reg::read) this register and get [`tim15_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM15:TIM15_SR)*/
pub struct TIM15_SRrs;
impl crate::RegisterSpec for TIM15_SRrs {
    type Ux = u16;
}
///`read()` method returns [`tim15_sr::R`](R) reader structure
impl crate::Readable for TIM15_SRrs {}
///`write(|w| ..)` method takes [`tim15_sr::W`](W) writer structure
impl crate::Writable for TIM15_SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_SR to value 0
impl crate::Resettable for TIM15_SRrs {}
