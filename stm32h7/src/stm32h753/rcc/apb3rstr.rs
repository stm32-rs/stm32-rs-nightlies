#[doc = "Register `APB3RSTR` reader"]
pub type R = crate::R<APB3RSTRrs>;
#[doc = "Register `APB3RSTR` writer"]
pub type W = crate::W<APB3RSTRrs>;
#[doc = "LTDC block reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LTDCRST {
    #[doc = "1: Reset the selected module"]
    Reset = 1,
}
impl From<LTDCRST> for bool {
    #[inline(always)]
    fn from(variant: LTDCRST) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LTDCRST` reader - LTDC block reset"]
pub type LTDCRST_R = crate::BitReader<LTDCRST>;
impl LTDCRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LTDCRST> {
        match self.bits {
            true => Some(LTDCRST::Reset),
            _ => None,
        }
    }
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == LTDCRST::Reset
    }
}
#[doc = "Field `LTDCRST` writer - LTDC block reset"]
pub type LTDCRST_W<'a, REG> = crate::BitWriter<'a, REG, LTDCRST>;
impl<'a, REG> LTDCRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Reset the selected module"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(LTDCRST::Reset)
    }
}
impl R {
    #[doc = "Bit 3 - LTDC block reset"]
    #[inline(always)]
    pub fn ltdcrst(&self) -> LTDCRST_R {
        LTDCRST_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - LTDC block reset"]
    #[inline(always)]
    #[must_use]
    pub fn ltdcrst(&mut self) -> LTDCRST_W<APB3RSTRrs> {
        LTDCRST_W::new(self, 3)
    }
}
#[doc = "RCC APB3 Peripheral Reset Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb3rstr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb3rstr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB3RSTRrs;
impl crate::RegisterSpec for APB3RSTRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb3rstr::R`](R) reader structure"]
impl crate::Readable for APB3RSTRrs {}
#[doc = "`write(|w| ..)` method takes [`apb3rstr::W`](W) writer structure"]
impl crate::Writable for APB3RSTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB3RSTR to value 0"]
impl crate::Resettable for APB3RSTRrs {
    const RESET_VALUE: u32 = 0;
}
