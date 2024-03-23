#[doc = "Register `APB1HRSTR` reader"]
pub type R = crate::R<APB1HRSTRrs>;
#[doc = "Register `APB1HRSTR` writer"]
pub type W = crate::W<APB1HRSTRrs>;
#[doc = "DTS block reset Set and reset by software.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTSRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<DTSRST> for bool {
    #[inline(always)]
    fn from(variant: DTSRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTSRST` reader - DTS block reset Set and reset by software."]
pub type DTSRST_R = crate::BitReader<DTSRST>;
impl DTSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DTSRST> {
        match self.bits {
            true => Some(DTSRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == DTSRST::Reset
    }
}
#[doc = "Field `DTSRST` writer - DTS block reset Set and reset by software."]
pub type DTSRST_W<'a, REG> = crate::BitWriter<'a, REG, DTSRST>;
impl<'a, REG> DTSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(DTSRST::Reset)
    }
}
#[doc = "Field `LPTIM2RST` reader - LPTIM2 block reset Set and reset by software."]
pub use DTSRST_R as LPTIM2RST_R;
#[doc = "Field `FDCANRST` reader - FDCAN block reset"]
pub use DTSRST_R as FDCANRST_R;
#[doc = "Field `LPTIM2RST` writer - LPTIM2 block reset Set and reset by software."]
pub use DTSRST_W as LPTIM2RST_W;
#[doc = "Field `FDCANRST` writer - FDCAN block reset"]
pub use DTSRST_W as FDCANRST_W;
impl R {
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
    #[doc = "Bit 9 - FDCAN block reset"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
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
    #[doc = "Bit 9 - FDCAN block reset"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<APB1HRSTRrs> {
        FDCANRST_W::new(self, 9)
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
