#[doc = "Register `APB1HRSTR` reader"]
pub type R = crate::R<APB1HRSTRrs>;
#[doc = "Register `APB1HRSTR` writer"]
pub type W = crate::W<APB1HRSTRrs>;
#[doc = "Clock Recovery System reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRSRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<CRSRST> for bool {
    #[inline(always)]
    fn from(variant: CRSRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CRSRST` reader - Clock Recovery System reset"]
pub type CRSRST_R = crate::BitReader<CRSRST>;
impl CRSRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CRSRST> {
        match self.bits {
            true => Some(CRSRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == CRSRST::Reset
    }
}
#[doc = "Field `CRSRST` writer - Clock Recovery System reset"]
pub type CRSRST_W<'a, REG> = crate::BitWriter<'a, REG, CRSRST>;
impl<'a, REG> CRSRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(CRSRST::Reset)
    }
}
#[doc = "Field `SWPRST` reader - SWPMI block reset"]
pub use CRSRST_R as SWPRST_R;
#[doc = "Field `OPAMPRST` reader - OPAMP block reset"]
pub use CRSRST_R as OPAMPRST_R;
#[doc = "Field `MDIOSRST` reader - MDIOS block reset"]
pub use CRSRST_R as MDIOSRST_R;
#[doc = "Field `FDCANRST` reader - FDCAN block reset"]
pub use CRSRST_R as FDCANRST_R;
#[doc = "Field `SWPRST` writer - SWPMI block reset"]
pub use CRSRST_W as SWPRST_W;
#[doc = "Field `OPAMPRST` writer - OPAMP block reset"]
pub use CRSRST_W as OPAMPRST_W;
#[doc = "Field `MDIOSRST` writer - MDIOS block reset"]
pub use CRSRST_W as MDIOSRST_W;
#[doc = "Field `FDCANRST` writer - FDCAN block reset"]
pub use CRSRST_W as FDCANRST_W;
impl R {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    pub fn crsrst(&self) -> CRSRST_R {
        CRSRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    pub fn swprst(&self) -> SWPRST_R {
        SWPRST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    pub fn opamprst(&self) -> OPAMPRST_R {
        OPAMPRST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    pub fn mdiosrst(&self) -> MDIOSRST_R {
        MDIOSRST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FDCANRST_R {
        FDCANRST_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    #[must_use]
    pub fn crsrst(&mut self) -> CRSRST_W<APB1HRSTRrs> {
        CRSRST_W::new(self, 1)
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    #[must_use]
    pub fn swprst(&mut self) -> SWPRST_W<APB1HRSTRrs> {
        SWPRST_W::new(self, 2)
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    #[must_use]
    pub fn opamprst(&mut self) -> OPAMPRST_W<APB1HRSTRrs> {
        OPAMPRST_W::new(self, 4)
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    #[must_use]
    pub fn mdiosrst(&mut self) -> MDIOSRST_W<APB1HRSTRrs> {
        MDIOSRST_W::new(self, 5)
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    #[must_use]
    pub fn fdcanrst(&mut self) -> FDCANRST_W<APB1HRSTRrs> {
        FDCANRST_W::new(self, 8)
    }
}
#[doc = "RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb1hrstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
