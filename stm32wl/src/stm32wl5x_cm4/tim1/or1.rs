#[doc = "Register `OR1` reader"]
pub type R = crate::R<OR1rs>;
#[doc = "Register `OR1` writer"]
pub type W = crate::W<OR1rs>;
#[doc = "TIM1_ETR_ADC1 remapping capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIM1_ETR_ADC1_RMP {
    #[doc = "0: TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)"]
    Select = 0,
    #[doc = "1: TIM1_ETR is connected to ADC AWD1"]
    AdcAwd1 = 1,
    #[doc = "2: TIM1_ETR is connected to ADC AWD2"]
    AdcAwd2 = 2,
    #[doc = "3: TIM1_ETR is connected to ADC AWD3"]
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
#[doc = "Field `TIM1_ETR_ADC1_RMP` reader - TIM1_ETR_ADC1 remapping capability"]
pub type TIM1_ETR_ADC1_RMP_R = crate::FieldReader<TIM1_ETR_ADC1_RMP>;
impl TIM1_ETR_ADC1_RMP_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)"]
    #[inline(always)]
    pub fn is_select(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP::Select
    }
    #[doc = "TIM1_ETR is connected to ADC AWD1"]
    #[inline(always)]
    pub fn is_adc_awd1(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP::AdcAwd1
    }
    #[doc = "TIM1_ETR is connected to ADC AWD2"]
    #[inline(always)]
    pub fn is_adc_awd2(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP::AdcAwd2
    }
    #[doc = "TIM1_ETR is connected to ADC AWD3"]
    #[inline(always)]
    pub fn is_adc_awd3(&self) -> bool {
        *self == TIM1_ETR_ADC1_RMP::AdcAwd3
    }
}
#[doc = "Field `TIM1_ETR_ADC1_RMP` writer - TIM1_ETR_ADC1 remapping capability"]
pub type TIM1_ETR_ADC1_RMP_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, TIM1_ETR_ADC1_RMP>;
impl<'a, REG> TIM1_ETR_ADC1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "TIM1_ETR is not connected to ADC AWDx (must be selected when the ETR comes from the ETR input pin)"]
    #[inline(always)]
    pub fn select(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ETR_ADC1_RMP::Select)
    }
    #[doc = "TIM1_ETR is connected to ADC AWD1"]
    #[inline(always)]
    pub fn adc_awd1(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ETR_ADC1_RMP::AdcAwd1)
    }
    #[doc = "TIM1_ETR is connected to ADC AWD2"]
    #[inline(always)]
    pub fn adc_awd2(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ETR_ADC1_RMP::AdcAwd2)
    }
    #[doc = "TIM1_ETR is connected to ADC AWD3"]
    #[inline(always)]
    pub fn adc_awd3(self) -> &'a mut crate::W<REG> {
        self.variant(TIM1_ETR_ADC1_RMP::AdcAwd3)
    }
}
#[doc = "Input Capture 1 remap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1_RMP {
    #[doc = "0: TIM1 input capture 1 is connected to I/O"]
    Io = 0,
    #[doc = "1: TIM1 input capture 1 is connected to COMP1 output"]
    Comp1 = 1,
}
impl From<TI1_RMP> for bool {
    #[inline(always)]
    fn from(variant: TI1_RMP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI1_RMP` reader - Input Capture 1 remap"]
pub type TI1_RMP_R = crate::BitReader<TI1_RMP>;
impl TI1_RMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TI1_RMP {
        match self.bits {
            false => TI1_RMP::Io,
            true => TI1_RMP::Comp1,
        }
    }
    #[doc = "TIM1 input capture 1 is connected to I/O"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == TI1_RMP::Io
    }
    #[doc = "TIM1 input capture 1 is connected to COMP1 output"]
    #[inline(always)]
    pub fn is_comp1(&self) -> bool {
        *self == TI1_RMP::Comp1
    }
}
#[doc = "Field `TI1_RMP` writer - Input Capture 1 remap"]
pub type TI1_RMP_W<'a, REG> = crate::BitWriter<'a, REG, TI1_RMP>;
impl<'a, REG> TI1_RMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TIM1 input capture 1 is connected to I/O"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Io)
    }
    #[doc = "TIM1 input capture 1 is connected to COMP1 output"]
    #[inline(always)]
    pub fn comp1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1_RMP::Comp1)
    }
}
impl R {
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    pub fn tim1_etr_adc1_rmp(&self) -> TIM1_ETR_ADC1_RMP_R {
        TIM1_ETR_ADC1_RMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    pub fn ti1_rmp(&self) -> TI1_RMP_R {
        TI1_RMP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - TIM1_ETR_ADC1 remapping capability"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_etr_adc1_rmp(&mut self) -> TIM1_ETR_ADC1_RMP_W<OR1rs> {
        TIM1_ETR_ADC1_RMP_W::new(self, 0)
    }
    #[doc = "Bit 4 - Input Capture 1 remap"]
    #[inline(always)]
    #[must_use]
    pub fn ti1_rmp(&mut self) -> TI1_RMP_W<OR1rs> {
        TI1_RMP_W::new(self, 4)
    }
}
#[doc = "option register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`or1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`or1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OR1rs;
impl crate::RegisterSpec for OR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`or1::R`](R) reader structure"]
impl crate::Readable for OR1rs {}
#[doc = "`write(|w| ..)` method takes [`or1::W`](W) writer structure"]
impl crate::Writable for OR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OR1 to value 0"]
impl crate::Resettable for OR1rs {
    const RESET_VALUE: u32 = 0;
}
