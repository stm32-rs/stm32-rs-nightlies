///Register `TIM15_EGR` reader
pub type R = crate::R<TIM15_EGRrs>;
///Register `TIM15_EGR` writer
pub type W = crate::W<TIM15_EGRrs>;
/**Update generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UG {
    ///0: No action
    B0x0 = 0,
    ///1: Reinitialize the counter and generates an update of the registers.
    B0x1 = 1,
}
impl From<UG> for bool {
    #[inline(always)]
    fn from(variant: UG) -> Self {
        variant as u8 != 0
    }
}
///Field `UG` writer - Update generation
pub type UG_W<'a, REG> = crate::BitWriter<'a, REG, UG>;
impl<'a, REG> UG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(UG::B0x0)
    }
    ///Reinitialize the counter and generates an update of the registers.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(UG::B0x1)
    }
}
/**Capture/Compare 1 generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CC1G {
    ///0: No action
    B0x0 = 0,
    ///1: A capture/compare event is generated on channel 1:
    B0x1 = 1,
}
impl From<CC1G> for bool {
    #[inline(always)]
    fn from(variant: CC1G) -> Self {
        variant as u8 != 0
    }
}
///Field `CC1G` writer - Capture/Compare 1 generation
pub type CC1G_W<'a, REG> = crate::BitWriter<'a, REG, CC1G>;
impl<'a, REG> CC1G_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC1G::B0x0)
    }
    ///A capture/compare event is generated on channel 1:
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC1G::B0x1)
    }
}
///Field `CC2G` writer - Capture/Compare 2 generation
pub type CC2G_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Capture/Compare control update generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COMG {
    ///0: No action
    B0x0 = 0,
    ///1: When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits
    B0x1 = 1,
}
impl From<COMG> for bool {
    #[inline(always)]
    fn from(variant: COMG) -> Self {
        variant as u8 != 0
    }
}
///Field `COMG` reader - Capture/Compare control update generation
pub type COMG_R = crate::BitReader<COMG>;
impl COMG_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> COMG {
        match self.bits {
            false => COMG::B0x0,
            true => COMG::B0x1,
        }
    }
    ///No action
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == COMG::B0x0
    }
    ///When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == COMG::B0x1
    }
}
///Field `COMG` writer - Capture/Compare control update generation
pub type COMG_W<'a, REG> = crate::BitWriter<'a, REG, COMG>;
impl<'a, REG> COMG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(COMG::B0x0)
    }
    ///When the CCPC bit is set, it is possible to update the CCxE, CCxNE and OCxM bits
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(COMG::B0x1)
    }
}
/**Trigger generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TG {
    ///0: No action
    B0x0 = 0,
    ///1: The TIF flag is set in TIMx_SR register.
    B0x1 = 1,
}
impl From<TG> for bool {
    #[inline(always)]
    fn from(variant: TG) -> Self {
        variant as u8 != 0
    }
}
///Field `TG` writer - Trigger generation
pub type TG_W<'a, REG> = crate::BitWriter<'a, REG, TG>;
impl<'a, REG> TG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TG::B0x0)
    }
    ///The TIF flag is set in TIMx_SR register.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TG::B0x1)
    }
}
/**Break generation

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BG {
    ///0: No action
    B0x0 = 0,
    ///1: A break event is generated.
    B0x1 = 1,
}
impl From<BG> for bool {
    #[inline(always)]
    fn from(variant: BG) -> Self {
        variant as u8 != 0
    }
}
///Field `BG` writer - Break generation
pub type BG_W<'a, REG> = crate::BitWriter<'a, REG, BG>;
impl<'a, REG> BG_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No action
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(BG::B0x0)
    }
    ///A break event is generated.
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(BG::B0x1)
    }
}
impl R {
    ///Bit 5 - Capture/Compare control update generation
    #[inline(always)]
    pub fn comg(&self) -> COMG_R {
        COMG_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_EGR")
            .field("comg", &self.comg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update generation
    #[inline(always)]
    pub fn ug(&mut self) -> UG_W<'_, TIM15_EGRrs> {
        UG_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 generation
    #[inline(always)]
    pub fn cc1g(&mut self) -> CC1G_W<'_, TIM15_EGRrs> {
        CC1G_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 2 generation
    #[inline(always)]
    pub fn cc2g(&mut self) -> CC2G_W<'_, TIM15_EGRrs> {
        CC2G_W::new(self, 2)
    }
    ///Bit 5 - Capture/Compare control update generation
    #[inline(always)]
    pub fn comg(&mut self) -> COMG_W<'_, TIM15_EGRrs> {
        COMG_W::new(self, 5)
    }
    ///Bit 6 - Trigger generation
    #[inline(always)]
    pub fn tg(&mut self) -> TG_W<'_, TIM15_EGRrs> {
        TG_W::new(self, 6)
    }
    ///Bit 7 - Break generation
    #[inline(always)]
    pub fn bg(&mut self) -> BG_W<'_, TIM15_EGRrs> {
        BG_W::new(self, 7)
    }
}
/**TIM15 event generation register

You can [`read`](crate::Reg::read) this register and get [`tim15_egr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_egr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C091.html#TIM15:TIM15_EGR)*/
pub struct TIM15_EGRrs;
impl crate::RegisterSpec for TIM15_EGRrs {
    type Ux = u16;
}
///`read()` method returns [`tim15_egr::R`](R) reader structure
impl crate::Readable for TIM15_EGRrs {}
///`write(|w| ..)` method takes [`tim15_egr::W`](W) writer structure
impl crate::Writable for TIM15_EGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_EGR to value 0
impl crate::Resettable for TIM15_EGRrs {}
