#[doc = "Register `TISEL` reader"]
pub type R = crate::R<TISELrs>;
#[doc = "Register `TISEL` writer"]
pub type W = crate::W<TISELrs>;
#[doc = "TISEL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TISEL {
    #[doc = "0: TIM1_CH1 input selected"]
    Selected = 0,
}
impl From<TISEL> for u8 {
    #[inline(always)]
    fn from(variant: TISEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TISEL {
    type Ux = u8;
}
#[doc = "Field `TISEL` reader - TISEL"]
pub type TISEL_R = crate::FieldReader<TISEL>;
impl TISEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TISEL> {
        match self.bits {
            0 => Some(TISEL::Selected),
            _ => None,
        }
    }
    #[doc = "TIM1_CH1 input selected"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == TISEL::Selected
    }
}
#[doc = "Field `TISEL` writer - TISEL"]
pub type TISEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TISEL>;
impl<'a, REG> TISEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_CH1 input selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut crate::W<REG> {
        self.variant(TISEL::Selected)
    }
}
impl R {
    #[doc = "Bits 0:3 - TISEL"]
    #[inline(always)]
    pub fn tisel(&self) -> TISEL_R {
        TISEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TISEL"]
    #[inline(always)]
    #[must_use]
    pub fn tisel(&mut self) -> TISEL_W<TISELrs> {
        TISEL_W::new(self, 0)
    }
}
#[doc = "TIM16 input selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tisel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tisel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
