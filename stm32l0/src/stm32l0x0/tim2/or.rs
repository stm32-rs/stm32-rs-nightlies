///Register `OR` reader
pub type R = crate::R<ORrs>;
///Register `OR` writer
pub type W = crate::W<ORrs>;
/**Timer2 ETR remap

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ETR_RMP {
    ///3: TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set
    Hsi = 3,
    ///5: TIM2 ETR input is connected to LSE
    Lse = 5,
    ///6: TIM2 ETR input is connected to COMP2_OUT
    Comp2Out = 6,
    ///7: TIM2 ETR input is connected to COMP1_OUT
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
impl crate::IsEnum for ETR_RMP {}
///Field `ETR_RMP` reader - Timer2 ETR remap
pub type ETR_RMP_R = crate::FieldReader<ETR_RMP>;
impl ETR_RMP_R {
    ///Get enumerated values variant
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
    ///TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == ETR_RMP::Hsi
    }
    ///TIM2 ETR input is connected to LSE
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == ETR_RMP::Lse
    }
    ///TIM2 ETR input is connected to COMP2_OUT
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == ETR_RMP::Comp2Out
    }
    ///TIM2 ETR input is connected to COMP1_OUT
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == ETR_RMP::Comp1Out
    }
}
///Field `ETR_RMP` writer - Timer2 ETR remap
pub type ETR_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 3, ETR_RMP>;
impl<'a, REG> ETR_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM2 ETR input is connected to HSI16 when HSI16OUTEN bit is set
    #[inline(always)]
    pub fn hsi(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Hsi)
    }
    ///TIM2 ETR input is connected to LSE
    #[inline(always)]
    pub fn lse(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Lse)
    }
    ///TIM2 ETR input is connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Comp2Out)
    }
    ///TIM2 ETR input is connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Comp1Out)
    }
}
/**Internal trigger

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI4_RMP {
    ///1: TIM2 TI4 input connected to COMP2_OUT
    Comp2Out = 1,
    ///2: TIM2 TI4 input connected to COMP1_OUT
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
impl crate::IsEnum for TI4_RMP {}
///Field `TI4_RMP` reader - Internal trigger
pub type TI4_RMP_R = crate::FieldReader<TI4_RMP>;
impl TI4_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI4_RMP> {
        match self.bits {
            1 => Some(TI4_RMP::Comp2Out),
            2 => Some(TI4_RMP::Comp1Out),
            _ => None,
        }
    }
    ///TIM2 TI4 input connected to COMP2_OUT
    #[inline(always)]
    pub fn is_comp2_out(&self) -> bool {
        *self == TI4_RMP::Comp2Out
    }
    ///TIM2 TI4 input connected to COMP1_OUT
    #[inline(always)]
    pub fn is_comp1_out(&self) -> bool {
        *self == TI4_RMP::Comp1Out
    }
}
///Field `TI4_RMP` writer - Internal trigger
pub type TI4_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TI4_RMP>;
impl<'a, REG> TI4_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM2 TI4 input connected to COMP2_OUT
    #[inline(always)]
    pub fn comp2_out(self) -> &'a mut crate::W<REG> {
        self.variant(TI4_RMP::Comp2Out)
    }
    ///TIM2 TI4 input connected to COMP1_OUT
    #[inline(always)]
    pub fn comp1_out(self) -> &'a mut crate::W<REG> {
        self.variant(TI4_RMP::Comp1Out)
    }
}
impl R {
    ///Bits 0:2 - Timer2 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new((self.bits & 7) as u8)
    }
    ///Bits 3:4 - Internal trigger
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 3) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR")
            .field("etr_rmp", &self.etr_rmp())
            .field("ti4_rmp", &self.ti4_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Timer2 ETR remap
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<'_, ORrs> {
        ETR_RMP_W::new(self, 0)
    }
    ///Bits 3:4 - Internal trigger
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<'_, ORrs> {
        TI4_RMP_W::new(self, 3)
    }
}
/**TIM2 option register

You can [`read`](crate::Reg::read) this register and get [`or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#TIM2:OR)*/
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
