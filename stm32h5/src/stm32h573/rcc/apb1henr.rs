#[doc = "Register `APB1HENR` reader"]
pub type R = crate::R<APB1HENRrs>;
#[doc = "Register `APB1HENR` writer"]
pub type W = crate::W<APB1HENRrs>;
#[doc = "UART9 clock enable Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART9EN {
    #[doc = "0: The selected clock is disabled"]
    Disabled = 0,
    #[doc = "1: The selected clock is enabled"]
    Enabled = 1,
}
impl From<UART9EN> for bool {
    #[inline(always)]
    fn from(variant: UART9EN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART9EN` reader - UART9 clock enable Set and reset by software."]
pub type UART9EN_R = crate::BitReader<UART9EN>;
impl UART9EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> UART9EN {
        match self.bits {
            false => UART9EN::Disabled,
            true => UART9EN::Enabled,
        }
    }
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UART9EN::Disabled
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UART9EN::Enabled
    }
}
#[doc = "Field `UART9EN` writer - UART9 clock enable Set and reset by software."]
pub type UART9EN_W<'a, REG> = crate::BitWriter<'a, REG, UART9EN>;
impl<'a, REG> UART9EN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The selected clock is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART9EN::Disabled)
    }
    #[doc = "The selected clock is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(UART9EN::Enabled)
    }
}
#[doc = "Field `UART12EN` reader - UART12 clock enable Set and reset by software."]
pub use UART9EN_R as UART12EN_R;
#[doc = "Field `DTSEN` reader - DTS clock enable Set and reset by software."]
pub use UART9EN_R as DTSEN_R;
#[doc = "Field `LPTIM2EN` reader - LPTIM2 clock enable Set and reset by software."]
pub use UART9EN_R as LPTIM2EN_R;
#[doc = "Field `FDCAN12EN` reader - FDCAN1 and FDCAN2 peripheral clock enable Set and reset by software."]
pub use UART9EN_R as FDCAN12EN_R;
#[doc = "Field `UCPDEN` reader - UCPD clock enable Set and reset by software."]
pub use UART9EN_R as UCPDEN_R;
#[doc = "Field `UART12EN` writer - UART12 clock enable Set and reset by software."]
pub use UART9EN_W as UART12EN_W;
#[doc = "Field `DTSEN` writer - DTS clock enable Set and reset by software."]
pub use UART9EN_W as DTSEN_W;
#[doc = "Field `LPTIM2EN` writer - LPTIM2 clock enable Set and reset by software."]
pub use UART9EN_W as LPTIM2EN_W;
#[doc = "Field `FDCAN12EN` writer - FDCAN1 and FDCAN2 peripheral clock enable Set and reset by software."]
pub use UART9EN_W as FDCAN12EN_W;
#[doc = "Field `UCPDEN` writer - UCPD clock enable Set and reset by software."]
pub use UART9EN_W as UCPDEN_W;
impl R {
    #[doc = "Bit 0 - UART9 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn uart9en(&self) -> UART9EN_R {
        UART9EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART12 clock enable Set and reset by software."]
    #[inline(always)]
    pub fn uart12en(&self) -> UART12EN_R {
        UART12EN_R::new(((self.bits >> 1) & 1) != 0)
    }
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
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable Set and reset by software."]
    #[inline(always)]
    pub fn fdcan12en(&self) -> FDCAN12EN_R {
        FDCAN12EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 23 - UCPD clock enable Set and reset by software."]
    #[inline(always)]
    pub fn ucpden(&self) -> UCPDEN_R {
        UCPDEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART9 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart9en(&mut self) -> UART9EN_W<APB1HENRrs> {
        UART9EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - UART12 clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart12en(&mut self) -> UART12EN_W<APB1HENRrs> {
        UART12EN_W::new(self, 1)
    }
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
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 peripheral clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcan12en(&mut self) -> FDCAN12EN_W<APB1HENRrs> {
        FDCAN12EN_W::new(self, 9)
    }
    #[doc = "Bit 23 - UCPD clock enable Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpden(&mut self) -> UCPDEN_W<APB1HENRrs> {
        UCPDEN_W::new(self, 23)
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
