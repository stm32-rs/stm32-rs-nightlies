#[doc = "Register `APB1HRSTR` reader"]
pub type R = crate::R<APB1HRSTRrs>;
#[doc = "Register `APB1HRSTR` writer"]
pub type W = crate::W<APB1HRSTRrs>;
#[doc = "UART9 block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UART9RST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<UART9RST> for bool {
    #[inline(always)]
    fn from(variant: UART9RST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `UART9RST` reader - UART9 block reset Set and reset by software."]
pub type UART9RST_R = crate::BitReader<UART9RST>;
impl UART9RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<UART9RST> {
        match self.bits {
            true => Some(UART9RST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == UART9RST::Reset
    }
}
#[doc = "Field `UART9RST` writer - UART9 block reset Set and reset by software."]
pub type UART9RST_W<'a, REG> = crate::BitWriter<'a, REG, UART9RST>;
impl<'a, REG> UART9RST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(UART9RST::Reset)
    }
}
#[doc = "Field `UART12RST` reader - UART12 block reset Set and reset by software."]
pub use UART9RST_R as UART12RST_R;
#[doc = "Field `DTSRST` reader - DTS block reset Set and reset by software."]
pub use UART9RST_R as DTSRST_R;
#[doc = "Field `LPTIM2RST` reader - LPTIM2 block reset Set and reset by software."]
pub use UART9RST_R as LPTIM2RST_R;
#[doc = "Field `FDCAN12RST` reader - FDCAN1 and FDCAN2 blocks reset Set and reset by software."]
pub use UART9RST_R as FDCAN12RST_R;
#[doc = "Field `UCPDRST` reader - UCPD block reset Set and reset by software."]
pub use UART9RST_R as UCPDRST_R;
#[doc = "Field `UART12RST` writer - UART12 block reset Set and reset by software."]
pub use UART9RST_W as UART12RST_W;
#[doc = "Field `DTSRST` writer - DTS block reset Set and reset by software."]
pub use UART9RST_W as DTSRST_W;
#[doc = "Field `LPTIM2RST` writer - LPTIM2 block reset Set and reset by software."]
pub use UART9RST_W as LPTIM2RST_W;
#[doc = "Field `FDCAN12RST` writer - FDCAN1 and FDCAN2 blocks reset Set and reset by software."]
pub use UART9RST_W as FDCAN12RST_W;
#[doc = "Field `UCPDRST` writer - UCPD block reset Set and reset by software."]
pub use UART9RST_W as UCPDRST_W;
impl R {
    #[doc = "Bit 0 - UART9 block reset Set and reset by software."]
    #[inline(always)]
    pub fn uart9rst(&self) -> UART9RST_R {
        UART9RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UART12 block reset Set and reset by software."]
    #[inline(always)]
    pub fn uart12rst(&self) -> UART12RST_R {
        UART12RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - DTS block reset Set and reset by software."]
    #[inline(always)]
    pub fn dtsrst(&self) -> DTSRST_R {
        DTSRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - LPTIM2 block reset Set and reset by software."]
    #[inline(always)]
    pub fn lptim2rst(&self) -> LPTIM2RST_R {
        LPTIM2RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 blocks reset Set and reset by software."]
    #[inline(always)]
    pub fn fdcan12rst(&self) -> FDCAN12RST_R {
        FDCAN12RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 23 - UCPD block reset Set and reset by software."]
    #[inline(always)]
    pub fn ucpdrst(&self) -> UCPDRST_R {
        UCPDRST_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART9 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart9rst(&mut self) -> UART9RST_W<APB1HRSTRrs> {
        UART9RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - UART12 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn uart12rst(&mut self) -> UART12RST_W<APB1HRSTRrs> {
        UART12RST_W::new(self, 1)
    }
    #[doc = "Bit 3 - DTS block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn dtsrst(&mut self) -> DTSRST_W<APB1HRSTRrs> {
        DTSRST_W::new(self, 3)
    }
    #[doc = "Bit 5 - LPTIM2 block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn lptim2rst(&mut self) -> LPTIM2RST_W<APB1HRSTRrs> {
        LPTIM2RST_W::new(self, 5)
    }
    #[doc = "Bit 9 - FDCAN1 and FDCAN2 blocks reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn fdcan12rst(&mut self) -> FDCAN12RST_W<APB1HRSTRrs> {
        FDCAN12RST_W::new(self, 9)
    }
    #[doc = "Bit 23 - UCPD block reset Set and reset by software."]
    #[inline(always)]
    #[must_use]
    pub fn ucpdrst(&mut self) -> UCPDRST_W<APB1HRSTRrs> {
        UCPDRST_W::new(self, 23)
    }
}
#[doc = "RCC APB1 peripheral high reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB1HRSTRrs;
impl crate::RegisterSpec for APB1HRSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hrstr::R`](R) reader structure"]
impl crate::Readable for APB1HRSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb1hrstr::W`](W) writer structure"]
impl crate::Writable for APB1HRSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB1HRSTR to value 0"]
impl crate::Resettable for APB1HRSTRrs {
    const RESET_VALUE: u32 = 0;
}
