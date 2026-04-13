///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
/**External trigger remap

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ETR_RMP {
    ///0: TIM2 ETR is connected to GPIO: Refer to Alternate Function mapping
    Gpio = 0,
    ///1: LSE internal clock is connected to TIM2_ETR input
    Tim2Etr = 1,
}
impl From<ETR_RMP> for bool {
    #[inline(always)]
    fn from(variant: ETR_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `ETR_RMP` reader - External trigger remap
pub type ETR_RMP_R = crate::BitReader<ETR_RMP>;
impl ETR_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ETR_RMP {
        match self.bits {
            false => ETR_RMP::Gpio,
            true => ETR_RMP::Tim2Etr,
        }
    }
    ///TIM2 ETR is connected to GPIO: Refer to Alternate Function mapping
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == ETR_RMP::Gpio
    }
    ///LSE internal clock is connected to TIM2_ETR input
    #[inline(always)]
    pub fn is_tim2_etr(&self) -> bool {
        *self == ETR_RMP::Tim2Etr
    }
}
///Field `ETR_RMP` writer - External trigger remap
pub type ETR_RMP_W<'a, REG> = crate::BitWriter<'a, REG, ETR_RMP>;
impl<'a, REG> ETR_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIM2 ETR is connected to GPIO: Refer to Alternate Function mapping
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Gpio)
    }
    ///LSE internal clock is connected to TIM2_ETR input
    #[inline(always)]
    pub fn tim2_etr(self) -> &'a mut crate::W<REG> {
        self.variant(ETR_RMP::Tim2Etr)
    }
}
/**Input capture 4 remap

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI4_RMP {
    ///0: TIM2 TI4 is connected to GPIO: Refer to Alternate Function mapping
    Gpio = 0,
    ///1: TIM2 TI4 is connected to COMP1_OUT
    Comp1 = 1,
    ///2: TIM2 TI4 is connected to COMP2_OUT
    Comp2 = 2,
    ///3: TIM2 TI4 is connected to a logical OR between COMP1_OUT and COMP2_OUT
    Comp12 = 3,
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
///Field `TI4_RMP` reader - Input capture 4 remap
pub type TI4_RMP_R = crate::FieldReader<TI4_RMP>;
impl TI4_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TI4_RMP {
        match self.bits {
            0 => TI4_RMP::Gpio,
            1 => TI4_RMP::Comp1,
            2 => TI4_RMP::Comp2,
            3 => TI4_RMP::Comp12,
            _ => unreachable!(),
        }
    }
    ///TIM2 TI4 is connected to GPIO: Refer to Alternate Function mapping
    #[inline(always)]
    pub fn is_gpio(&self) -> bool {
        *self == TI4_RMP::Gpio
    }
    ///TIM2 TI4 is connected to COMP1_OUT
    #[inline(always)]
    pub fn is_comp_1(&self) -> bool {
        *self == TI4_RMP::Comp1
    }
    ///TIM2 TI4 is connected to COMP2_OUT
    #[inline(always)]
    pub fn is_comp_2(&self) -> bool {
        *self == TI4_RMP::Comp2
    }
    ///TIM2 TI4 is connected to a logical OR between COMP1_OUT and COMP2_OUT
    #[inline(always)]
    pub fn is_comp_12(&self) -> bool {
        *self == TI4_RMP::Comp12
    }
}
///Field `TI4_RMP` writer - Input capture 4 remap
pub type TI4_RMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2, TI4_RMP, crate::Safe>;
impl<'a, REG> TI4_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM2 TI4 is connected to GPIO: Refer to Alternate Function mapping
    #[inline(always)]
    pub fn gpio(self) -> &'a mut crate::W<REG> {
        self.variant(TI4_RMP::Gpio)
    }
    ///TIM2 TI4 is connected to COMP1_OUT
    #[inline(always)]
    pub fn comp_1(self) -> &'a mut crate::W<REG> {
        self.variant(TI4_RMP::Comp1)
    }
    ///TIM2 TI4 is connected to COMP2_OUT
    #[inline(always)]
    pub fn comp_2(self) -> &'a mut crate::W<REG> {
        self.variant(TI4_RMP::Comp2)
    }
    ///TIM2 TI4 is connected to a logical OR between COMP1_OUT and COMP2_OUT
    #[inline(always)]
    pub fn comp_12(self) -> &'a mut crate::W<REG> {
        self.variant(TI4_RMP::Comp12)
    }
}
impl R {
    ///Bit 1 - External trigger remap
    #[inline(always)]
    pub fn etr_rmp(&self) -> ETR_RMP_R {
        ETR_RMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Input capture 4 remap
    #[inline(always)]
    pub fn ti4_rmp(&self) -> TI4_RMP_R {
        TI4_RMP_R::new(((self.bits >> 2) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("ti4_rmp", &self.ti4_rmp())
            .field("etr_rmp", &self.etr_rmp())
            .finish()
    }
}
impl W {
    ///Bit 1 - External trigger remap
    #[inline(always)]
    pub fn etr_rmp(&mut self) -> ETR_RMP_W<'_, OR1rs> {
        ETR_RMP_W::new(self, 1)
    }
    ///Bits 2:3 - Input capture 4 remap
    #[inline(always)]
    pub fn ti4_rmp(&mut self) -> TI4_RMP_W<'_, OR1rs> {
        TI4_RMP_W::new(self, 2)
    }
}
/**TIM2 option register

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#TIM2:OR1)*/
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
///`read()` method returns [`or1::R`](R) reader structure
impl crate::Readable for OR1rs {}
///`write(|w| ..)` method takes [`or1::W`](W) writer structure
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OR1 to value 0
impl crate::Resettable for OR1rs {}
