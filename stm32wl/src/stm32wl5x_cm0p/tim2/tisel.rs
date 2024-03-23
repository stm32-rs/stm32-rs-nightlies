#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TISELrs>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TISELrs>;
#[doc = "TI1SEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1SEL {
    #[doc = "0: TIM1_CHx input selected"]
    Selected = 0,
}
impl From<TI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: TI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI1SEL {
    type Ux = u8;
}
#[doc = "Field `TI1SEL` reader - TI1SEL"]
pub type TI1SEL_R = crate::FieldReader<TI1SEL>;
impl TI1SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI1SEL> {
        match self.bits {
            0 => Some(TI1SEL::Selected),
            _ => None,
        }
    }
    #[doc = "TIM1_CHx input selected"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == TI1SEL::Selected
    }
}
#[doc = "Field `TI1SEL` writer - TI1SEL"]
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI1SEL>;
impl<'a, REG> TI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CHx input selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::Selected)
    }
}
#[doc = "Field `TI2SEL` reader - TI2SEL"]
pub use TI1SEL_R as TI2SEL_R;
#[doc = "Field `TI2SEL` writer - TI2SEL"]
pub use TI1SEL_W as TI2SEL_W;
impl R {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - TI2SEL"]
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TI1SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti1sel(&mut self) -> TI1SEL_W<TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - TI2SEL"]
    #[inline(always)]
    #[must_use]
    pub fn ti2sel(&mut self) -> TI2SEL_W<TISELrs> {
        TI2SEL_W::new(self, 8)
    }
}
#[doc = "TIM2 timer input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TISELrs;
impl crate::RegisterSpec for TISELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tisel::R`](R) reader structure"]
impl crate::Readable for TISELrs {}
#[doc = "`write(|w| ..)` method takes [`tisel::W`](W) writer structure"]
impl crate::Writable for TISELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TISEL to value 0"]
impl crate::Resettable for TISELrs {
    const RESET_VALUE: u32 = 0;
}
