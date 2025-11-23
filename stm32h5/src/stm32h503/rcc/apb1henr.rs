///Register `APB1HENR` reader
pub type R = crate::R<APB1HENRrs>;
///Register `APB1HENR` writer
pub type W = crate::W<APB1HENRrs>;
/**DTS clock enable Set and reset by software.

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<DTSEN> for bool {
    #[inline(always)]
    fn from(variant: DTSEN) -> Self {
        variant as u8 != 0
    }
}
///Field `DTSEN` reader - DTS clock enable Set and reset by software.
pub type DTSEN_R = crate::BitReader<DTSEN>;
impl DTSEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DTSEN {
        match self.bits {
            false => DTSEN::Disabled,
            true => DTSEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTSEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTSEN::Enabled
    }
}
///Field `DTSEN` writer - DTS clock enable Set and reset by software.
pub type DTSEN_W<'a, REG> = crate::BitWriter<'a, REG, DTSEN>;
impl<'a, REG> DTSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEN::Enabled)
    }
}
///Field `LPTIM2EN` reader - LPTIM2 clock enable Set and reset by software.
pub use DTSEN_R as LPTIM2EN_R;
///Field `FDCANEN` reader - FDCAN peripheral clock enable
pub use DTSEN_R as FDCANEN_R;
///Field `LPTIM2EN` writer - LPTIM2 clock enable Set and reset by software.
pub use DTSEN_W as LPTIM2EN_W;
///Field `FDCANEN` writer - FDCAN peripheral clock enable
pub use DTSEN_W as FDCANEN_W;
impl R {
    ///Bit 3 - DTS clock enable Set and reset by software.
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 5 - LPTIM2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 9 - FDCAN peripheral clock enable
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB1HENR")
            .field("dtsen", &self.dtsen())
            .field("lptim2en", &self.lptim2en())
            .field("fdcanen", &self.fdcanen())
            .finish()
    }
}
impl W {
    ///Bit 3 - DTS clock enable Set and reset by software.
    #[inline(always)]
    pub fn dtsen(&mut self) -> DTSEN_W<'_, APB1HENRrs> {
        DTSEN_W::new(self, 3)
    }
    ///Bit 5 - LPTIM2 clock enable Set and reset by software.
    #[inline(always)]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<'_, APB1HENRrs> {
        LPTIM2EN_W::new(self, 5)
    }
    ///Bit 9 - FDCAN peripheral clock enable
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FDCANEN_W<'_, APB1HENRrs> {
        FDCANEN_W::new(self, 9)
    }
}
/**RCC APB1 peripheral clock register

You can [`read`](crate::Reg::read) this register and get [`apb1henr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#RCC:APB1HENR)*/
pub struct APB1HENRrs;
impl crate::RegisterSpec for APB1HENRrs {
    type Ux = u32;
}
///`read()` method returns [`apb1henr::R`](R) reader structure
impl crate::Readable for APB1HENRrs {}
///`write(|w| ..)` method takes [`apb1henr::W`](W) writer structure
impl crate::Writable for APB1HENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets APB1HENR to value 0
impl crate::Resettable for APB1HENRrs {}
