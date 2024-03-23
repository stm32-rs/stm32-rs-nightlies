#[doc = "Register `APB1HENR` reader"]
pub type R = crate::R<APB1HENRrs>;
#[doc = "Register `APB1HENR` writer"]
pub type W = crate::W<APB1HENRrs>;
#[doc = "DTS clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSEN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<DTSEN> for bool {
    #[inline(always)]
    fn from(variant: DTSEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTSEN` reader - DTS clock enable Set and reset by software."]
pub type DTSEN_R = crate::BitReader<DTSEN>;
impl DTSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DTSEN {
        match self.bits {
            false => DTSEN::Disabled,
            true => DTSEN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DTSEN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DTSEN::Enabled
    }
}
#[doc = "Field `DTSEN` writer - DTS clock enable Set and reset by software."]
pub type DTSEN_W<'a, REG> = crate::BitWriter<'a, REG, DTSEN>;
impl<'a, REG> DTSEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DTSEN::Enabled)
    }
}
#[doc = "Field `LPTIM2EN` reader - LPTIM2 clock enable Set and reset by software."]
pub use DTSEN_R as LPTIM2EN_R;
#[doc = "Field `FDCANEN` reader - FDCAN peripheral clock enable"]
pub use DTSEN_R as FDCANEN_R;
#[doc = "Field `LPTIM2EN` writer - LPTIM2 clock enable Set and reset by software."]
pub use DTSEN_W as LPTIM2EN_W;
#[doc = "Field `FDCANEN` writer - FDCAN peripheral clock enable"]
pub use DTSEN_W as FDCANEN_W;
impl R {
    #[doc = "Bit 3 - DTS clock enable Set and reset by software."]
    #[inline(always)]
    pub fn dtsen(&self) -> DTSEN_R {
        DTSEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn lptim2en(&self) -> LPTIM2EN_R {
        LPTIM2EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN peripheral clock enable"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FDCANEN_R {
        FDCANEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - DTS clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dtsen(&mut self) -> DTSEN_W<APB1HENRrs> {
        DTSEN_W::new(self, 3)
    }
    #[doc = "Bit 5 - LPTIM2 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2en(&mut self) -> LPTIM2EN_W<APB1HENRrs> {
        LPTIM2EN_W::new(self, 5)
    }
    #[doc = "Bit 9 - FDCAN peripheral clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanen(&mut self) -> FDCANEN_W<APB1HENRrs> {
        FDCANEN_W::new(self, 9)
    }
}
#[doc = "RCC APB1 peripheral clock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1henr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1henr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HENRrs;
impl crate::RegisterSpec for APB1HENRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1henr::R`](R) reader structure"]
impl crate::Readable for APB1HENRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1henr::W`](W) writer structure"]
impl crate::Writable for APB1HENRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1HENR to value 0"]
impl crate::Resettable for APB1HENRrs {
    const RESET_VALUE: u32 = 0;
}
