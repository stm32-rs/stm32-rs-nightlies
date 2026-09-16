///Register `ICR` writer
pub type W = crate::W<ICRrs>;
/**compare match Clear Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPMCFW {
    ///1: Compare match Clear Flag
    Clear = 1,
}
impl From<CMPMCFW> for bool {
    #[inline(always)]
    fn from(variant: CMPMCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPMCF` writer - compare match Clear Flag
pub type CMPMCF_W<'a, REG> = crate::BitWriter<'a, REG, CMPMCFW>;
impl<'a, REG> CMPMCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Compare match Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMPMCFW::Clear)
    }
}
/**Autoreload match Clear Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARRMCFW {
    ///1: Autoreload match Clear Flag
    Clear = 1,
}
impl From<ARRMCFW> for bool {
    #[inline(always)]
    fn from(variant: ARRMCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `ARRMCF` writer - Autoreload match Clear Flag
pub type ARRMCF_W<'a, REG> = crate::BitWriter<'a, REG, ARRMCFW>;
impl<'a, REG> ARRMCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Autoreload match Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ARRMCFW::Clear)
    }
}
/**External trigger valid edge Clear Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTTRIGCFW {
    ///1: External trigger valid edge Clear Flag
    Clear = 1,
}
impl From<EXTTRIGCFW> for bool {
    #[inline(always)]
    fn from(variant: EXTTRIGCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `EXTTRIGCF` writer - External trigger valid edge Clear Flag
pub type EXTTRIGCF_W<'a, REG> = crate::BitWriter<'a, REG, EXTTRIGCFW>;
impl<'a, REG> EXTTRIGCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///External trigger valid edge Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EXTTRIGCFW::Clear)
    }
}
/**Compare register update OK Clear Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPOKCFW {
    ///1: Compare register update OK Clear Flag
    Clear = 1,
}
impl From<CMPOKCFW> for bool {
    #[inline(always)]
    fn from(variant: CMPOKCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `CMPOKCF` writer - Compare register update OK Clear Flag
pub type CMPOKCF_W<'a, REG> = crate::BitWriter<'a, REG, CMPOKCFW>;
impl<'a, REG> CMPOKCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Compare register update OK Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(CMPOKCFW::Clear)
    }
}
/**Autoreload register update OK Clear Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARROKCFW {
    ///1: Autoreload register update OK Clear Flag
    Clear = 1,
}
impl From<ARROKCFW> for bool {
    #[inline(always)]
    fn from(variant: ARROKCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `ARROKCF` writer - Autoreload register update OK Clear Flag
pub type ARROKCF_W<'a, REG> = crate::BitWriter<'a, REG, ARROKCFW>;
impl<'a, REG> ARROKCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Autoreload register update OK Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ARROKCFW::Clear)
    }
}
/**Direction change to UP Clear Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPCFW {
    ///1: Direction change to up Clear Flag
    Clear = 1,
}
impl From<UPCFW> for bool {
    #[inline(always)]
    fn from(variant: UPCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `UPCF` writer - Direction change to UP Clear Flag
pub type UPCF_W<'a, REG> = crate::BitWriter<'a, REG, UPCFW>;
impl<'a, REG> UPCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Direction change to up Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(UPCFW::Clear)
    }
}
/**Direction change to down Clear Flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOWNCFW {
    ///1: Direction change to down Clear Flag
    Clear = 1,
}
impl From<DOWNCFW> for bool {
    #[inline(always)]
    fn from(variant: DOWNCFW) -> Self {
        variant as u8 != 0
    }
}
///Field `DOWNCF` writer - Direction change to down Clear Flag
pub type DOWNCF_W<'a, REG> = crate::BitWriter<'a, REG, DOWNCFW>;
impl<'a, REG> DOWNCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Direction change to down Clear Flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(DOWNCFW::Clear)
    }
}
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - compare match Clear Flag
    #[inline(always)]
    pub fn cmpmcf(&mut self) -> CMPMCF_W<'_, ICRrs> {
        CMPMCF_W::new(self, 0)
    }
    ///Bit 1 - Autoreload match Clear Flag
    #[inline(always)]
    pub fn arrmcf(&mut self) -> ARRMCF_W<'_, ICRrs> {
        ARRMCF_W::new(self, 1)
    }
    ///Bit 2 - External trigger valid edge Clear Flag
    #[inline(always)]
    pub fn exttrigcf(&mut self) -> EXTTRIGCF_W<'_, ICRrs> {
        EXTTRIGCF_W::new(self, 2)
    }
    ///Bit 3 - Compare register update OK Clear Flag
    #[inline(always)]
    pub fn cmpokcf(&mut self) -> CMPOKCF_W<'_, ICRrs> {
        CMPOKCF_W::new(self, 3)
    }
    ///Bit 4 - Autoreload register update OK Clear Flag
    #[inline(always)]
    pub fn arrokcf(&mut self) -> ARROKCF_W<'_, ICRrs> {
        ARROKCF_W::new(self, 4)
    }
    ///Bit 5 - Direction change to UP Clear Flag
    #[inline(always)]
    pub fn upcf(&mut self) -> UPCF_W<'_, ICRrs> {
        UPCF_W::new(self, 5)
    }
    ///Bit 6 - Direction change to down Clear Flag
    #[inline(always)]
    pub fn downcf(&mut self) -> DOWNCF_W<'_, ICRrs> {
        DOWNCF_W::new(self, 6)
    }
}
/**Interrupt Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G4A1.html#LPTIM1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
