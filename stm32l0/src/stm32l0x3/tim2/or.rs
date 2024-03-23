#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Timer2 ETR remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETR_RMP {
    #[doc = "3: TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set"]
    Hsi = 3,
    #[doc = "5: TIM2 ETR input is connected to LSE"]
    Lse = 5,
    #[doc = "6: TIM2 ETR input is connected to COMP2_OUT"]
    Comp2Out = 6,
    #[doc = "7: TIM2 ETR input is connected to COMP1_OUT"]
    Comp1Out = 7,
}
impl From<ETR_RMP> for u8 {
    #[inline(always)]
    fn from(variant: ETR_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ETR_RMP {
    type Ux = u8;
}
#[doc = "Field `ETR_RMP` reader - Timer2 ETR remap"]
pub type ETR_RMP_R = crate::FieldReader<ETR_RMP>;
impl ETR_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ETR_RMP> {
        match self.bits {
            3 => Some(ETR_RMP::Hsi),
            5 => Some(ETR_RMP::Lse),
            6 => Some(ETR_RMP::Comp2Out),
            7 => Some(ETR_RMP::Comp1Out),
            _ => None,
        }
    }
    #[doc = "TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == ETR_RMP::Hsi
    }
    #[doc = "TIM2 ETR input is connected to LSE"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == ETR_RMP::Lse
    }
    #[doc = "TIM2 ETR input is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == ETR_RMP::Comp2Out
    }
    #[doc = "TIM2 ETR input is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == ETR_RMP::Comp1Out
    }
}
#[doc = "Field `ETR_RMP` writer - Timer2 ETR remap"]
pub type ETR_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ETR_RMP>;
impl<'a, REG> ETR_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Hsi)
    }
    #[doc = "TIM2 ETR input is connected to LSE"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Lse)
    }
    #[doc = "TIM2 ETR input is connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Comp2Out)
    }
    #[doc = "TIM2 ETR input is connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Comp1Out)
    }
}
#[doc = "Internal trigger\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI4_RMP {
    #[doc = "1: TIM2 TI4 input connected to COMP2_OUT"]
    Comp2Out = 1,
    #[doc = "2: TIM2 TI4 input connected to COMP1_OUT"]
    Comp1Out = 2,
}
impl From<TI4_RMP> for u8 {
    #[inline(always)]
    fn from(variant: TI4_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI4_RMP {
    type Ux = u8;
}
#[doc = "Field `TI4_RMP` reader - Internal trigger"]
pub type TI4_RMP_R = crate::FieldReader<TI4_RMP>;
impl TI4_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI4_RMP> {
        match self.bits {
            1 => Some(TI4_RMP::Comp2Out),
            2 => Some(TI4_RMP::Comp1Out),
            _ => None,
        }
    }
    #[doc = "TIM2 TI4 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI4_RMP::Comp2Out
    }
    #[doc = "TIM2 TI4 input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TI4_RMP::Comp1Out
    }
}
#[doc = "Field `TI4_RMP` writer - Internal trigger"]
pub type TI4_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TI4_RMP>;
impl<'a, REG> TI4_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM2 TI4 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(TI4_RMP::Comp2Out)
    }
    #[doc = "TIM2 TI4 input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(TI4_RMP::Comp1Out)
    }
}
impl R {
    #[doc = "Bits 0:2 - Timer2 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Internal trigger"]
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Timer2 ETR remap"]
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<ORrs> {
        ETR_RMP_W::new(self, 0)
    }
    #[doc = "Bits 3:4 - Internal trigger"]
    #[inline(always)]
    #[must_use]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<ORrs> {
        TI4_RMP_W::new(self, 3)
    }
}
#[doc = "TIM2 option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or::R`](R) reader structure"]
impl crate::Readable for ORrs {}
#[doc = "`write(|w| ..)` method takes [`or::W`](W) writer structure"]
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR to value 0"]
impl crate::Resettable for ORrs {
    const RESET_VALUE: u32 = 0;
}
