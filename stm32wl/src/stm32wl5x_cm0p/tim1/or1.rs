///Register `OR1` reader
pub type R = crate::R<OR1rs>;
///Register `OR1` writer
pub type W = crate::W<OR1rs>;
/**TIM1_ETR_ADC1 remapping capability

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIM1_ETR_ADC1_RMP {
    ///0: TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)
    Select = 0,
    ///1: TIM1_ETR is connected to ADC AWD1
    AdcAwd1 = 1,
    ///2: TIM1_ETR is connected to ADC AWD2
    AdcAwd2 = 2,
    ///3: TIM1_ETR is connected to ADC AWD3
    AdcAwd3 = 3,
}
impl From<TIM1_ETR_ADC1_RMP> for u8 {
    #[inline(always)]
    fn from(variant: TIM1_ETR_ADC1_RMP) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TIM1_ETR_ADC1_RMP {
    type Ux = u8;
}
impl crate::IsEnum for TIM1_ETR_ADC1_RMP {}
///Field `TIM1_ETR_ADC1_RMP` reader - TIM1_ETR_ADC1 remapping capability
pub type TIM1_ETR_ADC1_RMP_R = crate::FieldReader<TIM1_ETR_ADC1_RMP>;
impl TIM1_ETR_ADC1_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TIM1_ETR_ADC1_RMP {
        match self.bits {
            0 => TIM1_ETR_ADC1_RMP::Select,
            1 => TIM1_ETR_ADC1_RMP::AdcAwd1,
            2 => TIM1_ETR_ADC1_RMP::AdcAwd2,
            3 => TIM1_ETR_ADC1_RMP::AdcAwd3,
            _ => unreachable!(),
        }
    }
    ///TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)
    #[inline(always)]
    pub fn is_select(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP::Select
    }
    ///TIM1_ETR is connected to ADC AWD1
    #[inline(always)]
    pub fn is_adc_awd1(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP::AdcAwd1
    }
    ///TIM1_ETR is connected to ADC AWD2
    #[inline(always)]
    pub fn is_adc_awd2(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP::AdcAwd2
    }
    ///TIM1_ETR is connected to ADC AWD3
    #[inline(always)]
    pub fn is_adc_awd3(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP::AdcAwd3
    }
}
///Field `TIM1_ETR_ADC1_RMP` writer - TIM1_ETR_ADC1 remapping capability
pub type TIM1_ETR_ADC1_RMP_W<'a, REG> =
    crate::FieldWriter<'a, REG, 2, TIM1_ETR_ADC1_RMP, crate::Safe>;
impl<'a, REG> TIM1_ETR_ADC1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)
    #[inline(always)]
    pub fn select(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ETR_ADC1_RMP::Select)
    }
    ///TIM1_ETR is connected to ADC AWD1
    #[inline(always)]
    pub fn adc_awd1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ETR_ADC1_RMP::AdcAwd1)
    }
    ///TIM1_ETR is connected to ADC AWD2
    #[inline(always)]
    pub fn adc_awd2(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ETR_ADC1_RMP::AdcAwd2)
    }
    ///TIM1_ETR is connected to ADC AWD3
    #[inline(always)]
    pub fn adc_awd3(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ETR_ADC1_RMP::AdcAwd3)
    }
}
/**Input Capture 1 remap

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1_RMP {
    ///0: TIM1 input capture 1 is connected to I/O
    Io = 0,
    ///1: TIM1 input capture 1 is connected to COMP1 output
    Comp1 = 1,
}
impl From<TI1_RMP> for bool {
    #[inline(always)]
    fn from(variant: TI1_RMP) -> Self {
        variant as u8 != 0
    }
}
///Field `TI1_RMP` reader - Input Capture 1 remap
pub type TI1_RMP_R = crate::BitReader<TI1_RMP>;
impl TI1_RMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> TI1_RMP {
        match self.bits {
            false => TI1_RMP::Io,
            true => TI1_RMP::Comp1,
        }
    }
    ///TIM1 input capture 1 is connected to I/O
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == TI1_RMP::Io
    }
    ///TIM1 input capture 1 is connected to COMP1 output
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == TI1_RMP::Comp1
    }
}
///Field `TI1_RMP` writer - Input Capture 1 remap
pub type TI1_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TI1_RMP>;
impl<'a, REG> TI1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///TIM1 input capture 1 is connected to I/O
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Io)
    }
    ///TIM1 input capture 1 is connected to COMP1 output
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Comp1)
    }
}
impl R {
    ///Bits 0:1 - TIM1_ETR_ADC1 remapping capability
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&self) -> TIM1_ETR_ADC1_RMP_R {
        TIM1_ETR_ADC1_RMP_R::new((self.bits & 3) as u8)
    }
    ///Bit 4 - Input Capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OR1")
            .field("ti1_rmp", &self.ti1_rmp())
            .field("tim1_etr_adc1_rmp", &self.tim1_etr_adc1_rmp())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - TIM1_ETR_ADC1 remapping capability
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&mut self) -> TIM1_ETR_ADC1_RMP_W<'_, OR1rs> {
        TIM1_ETR_ADC1_RMP_W::new(self, 0)
    }
    ///Bit 4 - Input Capture 1 remap
    #[inline(always)]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<'_, OR1rs> {
        TI1_RMP_W::new(self, 4)
    }
}
/**option register 1

You can [`read`](crate::Reg::read) this register and get [`or1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#TIM1:OR1)*/
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
