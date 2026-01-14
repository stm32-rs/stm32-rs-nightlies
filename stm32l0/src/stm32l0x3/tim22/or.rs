///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
/**Timer22 ETR remap

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETR_RMP {
    ///0: TIM2x ETR input connected to GPIO
    Gpio = 0,
    ///1: TIM2x ETR input connected to COMP2_OUT
    Comp2Out = 1,
    ///2: TIM2x ETR input connected to COMP1_OUT
    Comp1Out = 2,
    ///3: TIM2x ETR input connected to LSE clock
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
impl crate::IsEnum for ETR_RMP {}
///Field `ETR_RMP` reader - Timer22 ETR remap
pub type ETR_RMP_R = crate::FieldReader<ETR_RMP>;
impl ETR_RMP_R {
    ///Get enumerated values variant
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
    ///TIM2x ETR input connected to GPIO
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == ETR_RMP::Gpio
    }
    ///TIM2x ETR input connected to COMP2_OUT
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == ETR_RMP::Comp2Out
    }
    ///TIM2x ETR input connected to COMP1_OUT
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == ETR_RMP::Comp1Out
    }
    ///TIM2x ETR input connected to LSE clock
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == ETR_RMP::Lse
    }
}
///Field `ETR_RMP` writer - Timer22 ETR remap
pub type ETR_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ETR_RMP, crate::Safe>;
impl<'a, REG> ETR_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM2x ETR input connected to GPIO
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Gpio)
    }
    ///TIM2x ETR input connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Comp2Out)
    }
    ///TIM2x ETR input connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Comp1Out)
    }
    ///TIM2x ETR input connected to LSE clock
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Lse)
    }
}
/**Timer22 TI1

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1_RMP {
    ///0: TIM2x TI1 input connected to GPIO
    Gpio = 0,
    ///1: TIM2x TI1 input connected to COMP2_OUT
    Comp2Out = 1,
    ///2: TIM2x TI1 input connected to COMP1_OUT
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
impl crate::IsEnum for TI1_RMP {}
///Field `TI1_RMP` reader - Timer22 TI1
pub type TI1_RMP_R = crate::FieldReader<TI1_RMP>;
impl TI1_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI1_RMP> {
        match self.bits {
            0 => Some(TI1_RMP::Gpio),
            1 => Some(TI1_RMP::Comp2Out),
            2 => Some(TI1_RMP::Comp1Out),
            _ => None,
        }
    }
    ///TIM2x TI1 input connected to GPIO
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI1_RMP::Gpio
    }
    ///TIM2x TI1 input connected to COMP2_OUT
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI1_RMP::Comp2Out
    }
    ///TIM2x TI1 input connected to COMP1_OUT
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TI1_RMP::Comp1Out
    }
}
///Field `TI1_RMP` writer - Timer22 TI1
pub type TI1_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TI1_RMP>;
impl<'a, REG> TI1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM2x TI1 input connected to GPIO
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Gpio)
    }
    ///TIM2x TI1 input connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Comp2Out)
    }
    ///TIM2x TI1 input connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Comp1Out)
    }
}
impl R {
    ///Bits 0:1 - Timer22 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Timer22 TI1
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("etr_rmp", &self.etr_rmp())
            .field("ti1_rmp", &self.ti1_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Timer22 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<'_, ORrs> {
        ETR_RMP_W::new(self, 0)
    }
    ///Bits 2:3 - Timer22 TI1
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<'_, ORrs> {
        TI1_RMP_W::new(self, 2)
    }
}
/**TIM22 option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#TIM22:OR)*/
pub struct ORrs;
impl crate::RegisterSpec for ORrs {
    type Ux = u16;
}
///`read()` method returns [`or::R`](R) reader structure
impl crate::Readable for ORrs {}
///`write(|w| ..)` method takes [`or::W`](W) writer structure
impl crate::Writable for ORrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR to value 0
impl crate::Resettable for ORrs {}
