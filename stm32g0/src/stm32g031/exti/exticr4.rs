#[doc = "Register `EXTICR4` reader"]
pub type R = crate::R<EXTICR4rs>;
#[doc = "Register `EXTICR4` writer"]
pub type W = crate::W<EXTICR4rs>;
#[doc = "GPIO port selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EXTI0_7 {
    #[doc = "0: GPIO port A selected"]
    Pa = 0,
    #[doc = "1: GPIO port B selected"]
    Pb = 1,
    #[doc = "2: GPIO port C selected"]
    Pc = 2,
    #[doc = "3: GPIO port D selected"]
    Pd = 3,
    #[doc = "5: GPIO port F selected"]
    Pf = 5,
}
impl From<EXTI0_7> for u8 {
    #[inline(always)]
    fn from(variant: EXTI0_7) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EXTI0_7 {
    type Ux = u8;
}
#[doc = "Field `EXTI0_7` reader - GPIO port selection"]
pub type EXTI0_7_R = crate::FieldReader<EXTI0_7>;
impl EXTI0_7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<EXTI0_7> {
        match self.bits {
            0 => Some(EXTI0_7::Pa),
            1 => Some(EXTI0_7::Pb),
            2 => Some(EXTI0_7::Pc),
            3 => Some(EXTI0_7::Pd),
            5 => Some(EXTI0_7::Pf),
            _ => None,
        }
    }
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn is_pa(&self) -> bool {
        *self == EXTI0_7::Pa
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn is_pb(&self) -> bool {
        *self == EXTI0_7::Pb
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn is_pc(&self) -> bool {
        *self == EXTI0_7::Pc
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn is_pd(&self) -> bool {
        *self == EXTI0_7::Pd
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn is_pf(&self) -> bool {
        *self == EXTI0_7::Pf
    }
}
#[doc = "Field `EXTI0_7` writer - GPIO port selection"]
pub type EXTI0_7_W<'a, REG> = crate::FieldWriter<'a, REG, 8, EXTI0_7>;
impl<'a, REG> EXTI0_7_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "GPIO port A selected"]
    #[inline(always)]
    pub fn pa(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pa)
    }
    #[doc = "GPIO port B selected"]
    #[inline(always)]
    pub fn pb(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pb)
    }
    #[doc = "GPIO port C selected"]
    #[inline(always)]
    pub fn pc(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pc)
    }
    #[doc = "GPIO port D selected"]
    #[inline(always)]
    pub fn pd(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pd)
    }
    #[doc = "GPIO port F selected"]
    #[inline(always)]
    pub fn pf(self) -> &'a mut crate::W<REG> {
        self.variant(EXTI0_7::Pf)
    }
}
#[doc = "Field `EXTI8_15` reader - GPIO port selection"]
pub use EXTI0_7_R as EXTI8_15_R;
#[doc = "Field `EXTI16_23` reader - GPIO port selection"]
pub use EXTI0_7_R as EXTI16_23_R;
#[doc = "Field `EXTI24_31` reader - GPIO port selection"]
pub use EXTI0_7_R as EXTI24_31_R;
#[doc = "Field `EXTI8_15` writer - GPIO port selection"]
pub use EXTI0_7_W as EXTI8_15_W;
#[doc = "Field `EXTI16_23` writer - GPIO port selection"]
pub use EXTI0_7_W as EXTI16_23_W;
#[doc = "Field `EXTI24_31` writer - GPIO port selection"]
pub use EXTI0_7_W as EXTI24_31_W;
impl R {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    pub fn exti0_7(&self) -> EXTI0_7_R {
        EXTI0_7_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    pub fn exti8_15(&self) -> EXTI8_15_R {
        EXTI8_15_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    pub fn exti16_23(&self) -> EXTI16_23_R {
        EXTI16_23_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    pub fn exti24_31(&self) -> EXTI24_31_R {
        EXTI24_31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti0_7(&mut self) -> EXTI0_7_W<EXTICR4rs> {
        EXTI0_7_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti8_15(&mut self) -> EXTI8_15_W<EXTICR4rs> {
        EXTI8_15_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti16_23(&mut self) -> EXTI16_23_W<EXTICR4rs> {
        EXTI16_23_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO port selection"]
    #[inline(always)]
    #[must_use]
    pub fn exti24_31(&mut self) -> EXTI24_31_W<EXTICR4rs> {
        EXTI24_31_W::new(self, 24)
    }
}
#[doc = "EXTI external interrupt selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exticr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTICR4rs;
impl crate::RegisterSpec for EXTICR4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`exticr4::R`](R) reader structure"]
impl crate::Readable for EXTICR4rs {}
#[doc = "`write(|w| ..)` method takes [`exticr4::W`](W) writer structure"]
impl crate::Writable for EXTICR4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTICR4 to value 0"]
impl crate::Resettable for EXTICR4rs {
    const RESET_VALUE: u32 = 0;
}
