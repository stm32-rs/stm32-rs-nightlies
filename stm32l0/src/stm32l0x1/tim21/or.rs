#[doc = "Register `OR` reader"]
pub type R = crate::R<ORrs>;
#[doc = "Register `OR` writer"]
pub type W = crate::W<ORrs>;
#[doc = "Timer21 ETR remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETR_RMP {
    #[doc = "0: TIM2x ETR input connected to GPIO"]
    Gpio = 0,
    #[doc = "1: TIM2x ETR input connected to COMP2_OUT"]
    Comp2Out = 1,
    #[doc = "2: TIM2x ETR input connected to COMP1_OUT"]
    Comp1Out = 2,
    #[doc = "3: TIM2x ETR input connected to LSE clock"]
    Lse = 3,
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
#[doc = "Field `ETR_RMP` reader - Timer21 ETR remap"]
pub type ETR_RMP_R = crate::FieldReader<ETR_RMP>;
impl ETR_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ETR_RMP {
        match self.bits {
            0 => ETR_RMP::Gpio,
            1 => ETR_RMP::Comp2Out,
            2 => ETR_RMP::Comp1Out,
            3 => ETR_RMP::Lse,
            _ => unreachable!(),
        }
    }
    #[doc = "TIM2x ETR input connected to GPIO"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == ETR_RMP::Gpio
    }
    #[doc = "TIM2x ETR input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == ETR_RMP::Comp2Out
    }
    #[doc = "TIM2x ETR input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == ETR_RMP::Comp1Out
    }
    #[doc = "TIM2x ETR input connected to LSE clock"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == ETR_RMP::Lse
    }
}
#[doc = "Field `ETR_RMP` writer - Timer21 ETR remap"]
pub type ETR_RMP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ETR_RMP>;
impl<'a, REG> ETR_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM2x ETR input connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Gpio)
    }
    #[doc = "TIM2x ETR input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Comp2Out)
    }
    #[doc = "TIM2x ETR input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Comp1Out)
    }
    #[doc = "TIM2x ETR input connected to LSE clock"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Lse)
    }
}
#[doc = "Timer21 TI1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1_RMP {
    #[doc = "0: TIM2x TI1 input connected to GPIO"]
    Gpio = 0,
    #[doc = "1: TIM2x TI1 input connected to COMP2_OUT"]
    Comp2Out = 1,
    #[doc = "2: TIM2x TI1 input connected to COMP1_OUT"]
    Comp1Out = 2,
}
impl From<TI1_RMP> for u8 {
    #[inline(always)]
    fn from(variant: TI1_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI1_RMP {
    type Ux = u8;
}
#[doc = "Field `TI1_RMP` reader - Timer21 TI1"]
pub type TI1_RMP_R = crate::FieldReader<TI1_RMP>;
impl TI1_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI1_RMP> {
        match self.bits {
            0 => Some(TI1_RMP::Gpio),
            1 => Some(TI1_RMP::Comp2Out),
            2 => Some(TI1_RMP::Comp1Out),
            _ => None,
        }
    }
    #[doc = "TIM2x TI1 input connected to GPIO"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI1_RMP::Gpio
    }
    #[doc = "TIM2x TI1 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI1_RMP::Comp2Out
    }
    #[doc = "TIM2x TI1 input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TI1_RMP::Comp1Out
    }
}
#[doc = "Field `TI1_RMP` writer - Timer21 TI1"]
pub type TI1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, TI1_RMP>;
impl<'a, REG> TI1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM2x TI1 input connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Gpio)
    }
    #[doc = "TIM2x TI1 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Comp2Out)
    }
    #[doc = "TIM2x TI1 input connected to COMP1_OUT"]
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Comp1Out)
    }
}
#[doc = "Timer21 TI2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI2_RMP {
    #[doc = "0: TIM2x TI2 input connected to GPIO"]
    Gpio = 0,
    #[doc = "1: TIM2x TI2 input connected to COMP2_OUT"]
    Comp2Out = 1,
}
impl From<TI2_RMP> for bool {
    #[inline(always)]
    fn from(variant: TI2_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI2_RMP` reader - Timer21 TI2"]
pub type TI2_RMP_R = crate::BitReader<TI2_RMP>;
impl TI2_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI2_RMP {
        match self.bits {
            false => TI2_RMP::Gpio,
            true => TI2_RMP::Comp2Out,
        }
    }
    #[doc = "TIM2x TI2 input connected to GPIO"]
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI2_RMP::Gpio
    }
    #[doc = "TIM2x TI2 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI2_RMP::Comp2Out
    }
}
#[doc = "Field `TI2_RMP` writer - Timer21 TI2"]
pub type TI2_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TI2_RMP>;
impl<'a, REG> TI2_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM2x TI2 input connected to GPIO"]
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(TI2_RMP::Gpio)
    }
    #[doc = "TIM2x TI2 input connected to COMP2_OUT"]
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(TI2_RMP::Comp2Out)
    }
}
impl R {
    #[doc = "Bits 0:1 - Timer21 ETR remap"]
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - Timer21 TI1"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bit 5 - Timer21 TI2"]
    #[inline(always)]
    pub fn ti2_rmp(&self) -> TI2_RMP_R {
        TI2_RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer21 ETR remap"]
    #[inline(always)]
    #[must_use]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<ORrs> {
        ETR_RMP_W::new(self, 0)
    }
    #[doc = "Bits 2:4 - Timer21 TI1"]
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<ORrs> {
        TI1_RMP_W::new(self, 2)
    }
    #[doc = "Bit 5 - Timer21 TI2"]
    #[inline(always)]
    #[must_use]
    pub fn ti2_rmp(&mut self) -> TI2_RMP_W<ORrs> {
        TI2_RMP_W::new(self, 5)
    }
}
#[doc = "TIM21 option register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
