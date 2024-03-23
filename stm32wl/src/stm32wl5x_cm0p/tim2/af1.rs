#[doc = "Register `AF1` reader"]
pub type R = crate::R<AF1rs>;
#[doc = "Register `AF1` writer"]
pub type W = crate::W<AF1rs>;
#[doc = "External trigger source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETRSEL {
    #[doc = "0: ETR legacy mode"]
    Legacy = 0,
    #[doc = "1: COMP1 output"]
    Comp1 = 1,
    #[doc = "2: COMP2 output"]
    Comp2 = 2,
}
impl From<ETRSEL> for u8 {
    #[inline(always)]
    fn from(variant: ETRSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETRSEL {
    type Ux = u8;
}
#[doc = "Field `ETRSEL` reader - External trigger source selection"]
pub type ETRSEL_R = crate::FieldReader<ETRSEL>;
impl ETRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETRSEL> {
        match self.bits {
            0 => Some(ETRSEL::Legacy),
            1 => Some(ETRSEL::Comp1),
            2 => Some(ETRSEL::Comp2),
            _ => None,
        }
    }
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn is_legacy(&self) -> bool {
        *self == ETRSEL::Legacy
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == ETRSEL::Comp1
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn is_comp2(&self) -> bool {
        *self == ETRSEL::Comp2
    }
}
#[doc = "Field `ETRSEL` writer - External trigger source selection"]
pub type ETRSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, ETRSEL>;
impl<'a, REG> ETRSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ETR legacy mode"]
    #[inline(always)]
    pub fn legacy(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::Legacy)
    }
    #[doc = "COMP1 output"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::Comp1)
    }
    #[doc = "COMP2 output"]
    #[inline(always)]
    pub fn comp2(self) -> &'a mut crate::W<REG> {
        self.variant(ETRSEL::Comp2)
    }
}
impl R {
    #[doc = "Bits 14:17 - External trigger source selection"]
    #[inline(always)]
    pub fn etrsel(&self) -> ETRSEL_R {
        ETRSEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 14:17 - External trigger source selection"]
    #[inline(always)]
    #[must_use]
    pub fn etrsel(&mut self) -> ETRSEL_W<AF1rs> {
        ETRSEL_W::new(self, 14)
    }
}
#[doc = "TIM2 alternate function option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF1rs;
impl crate::RegisterSpec for AF1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af1::R`](R) reader structure"]
impl crate::Readable for AF1rs {}
#[doc = "`write(|w| ..)` method takes [`af1::W`](W) writer structure"]
impl crate::Writable for AF1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF1 to value 0"]
impl crate::Resettable for AF1rs {
    const RESET_VALUE: u32 = 0;
}
