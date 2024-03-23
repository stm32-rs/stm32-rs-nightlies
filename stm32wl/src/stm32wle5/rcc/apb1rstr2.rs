#[doc = "Register `APB1RSTR2` reader"]
pub type R = crate::R<APB1RSTR2rs>;
#[doc = "Register `APB1RSTR2` writer"]
pub type W = crate::W<APB1RSTR2rs>;
#[doc = "Low-power UART 1 reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPUART1RST {
    #[doc = "0: No effect"]
    NoReset = 0,
    #[doc = "1: Reset peripheral"]
    Reset = 1,
}
impl From<LPUART1RST> for bool {
    #[inline(always)]
    fn from(variant: LPUART1RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LPUART1RST` reader - Low-power UART 1 reset"]
pub type LPUART1RST_R = crate::BitReader<LPUART1RST>;
impl LPUART1RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LPUART1RST {
        match self.bits {
            false => LPUART1RST::NoReset,
            true => LPUART1RST::Reset,
        }
    }
    #[doc = "No effect"]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == LPUART1RST::NoReset
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LPUART1RST::Reset
    }
}
#[doc = "Field `LPUART1RST` writer - Low-power UART 1 reset"]
pub type LPUART1RST_W<'a, REG> = crate::BitWriter<'a, REG, LPUART1RST>;
impl<'a, REG> LPUART1RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::NoReset)
    }
    #[doc = "Reset peripheral"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LPUART1RST::Reset)
    }
}
#[doc = "Field `LPTIM2RST` reader - Low-power timer 2 reset"]
pub use LPUART1RST_R as LPTIM2RST_R;
#[doc = "Field `LPTIM3RST` reader - Low-power timer 3 reset"]
pub use LPUART1RST_R as LPTIM3RST_R;
#[doc = "Field `LPTIM2RST` writer - Low-power timer 2 reset"]
pub use LPUART1RST_W as LPTIM2RST_W;
#[doc = "Field `LPTIM3RST` writer - Low-power timer 3 reset"]
pub use LPUART1RST_W as LPTIM3RST_W;
impl R {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> LPUART1RST_R {
        LPUART1RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-power timer 3 reset"]
    #[inline(always)]
    pub fn lptim3rst(&self) -> LPTIM3RST_R {
        LPTIM3RST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low-power UART 1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lpuart1rst(&mut self) -> LPUART1RST_W<APB1RSTR2rs> {
        LPUART1RST_W::new(self, 0)
    }
    #[doc = "Bit 5 - Low-power timer 2 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<APB1RSTR2rs> {
        LPTIM2RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Low-power timer 3 reset"]
    #[inline(always)]
    #[must_use]
    pub fn lptim3rst(&mut self) -> LPTIM3RST_W<APB1RSTR2rs> {
        LPTIM3RST_W::new(self, 6)
    }
}
#[doc = "APB1 peripheral reset register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1rstr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1rstr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1RSTR2rs;
impl crate::RegisterSpec for APB1RSTR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1rstr2::R`](R) reader structure"]
impl crate::Readable for APB1RSTR2rs {}
#[doc = "`write(|w| ..)` method takes [`apb1rstr2::W`](W) writer structure"]
impl crate::Writable for APB1RSTR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1RSTR2 to value 0"]
impl crate::Resettable for APB1RSTR2rs {
    const RESET_VALUE: u32 = 0;
}
