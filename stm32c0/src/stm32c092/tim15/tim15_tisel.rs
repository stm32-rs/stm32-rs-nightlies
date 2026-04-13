///Register `TIM15_TISEL` reader
pub type R = crate::R<TIM15_TISELrs>;
///Register `TIM15_TISEL` writer
pub type W = crate::W<TIM15_TISELrs>;
/**selects TI1\[0\] to TI1\[15\] input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1SEL {
    ///0: TIM15_CH1 input
    B0x0 = 0,
    ///1: TIM2_IC1
    B0x1 = 1,
    ///2: TIM3_IC1
    B0x2 = 2,
}
impl From<TI1SEL> for u8 {
    #[inline(always)]
    fn from(variant: TI1SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI1SEL {
    type Ux = u8;
}
impl crate::IsEnum for TI1SEL {}
///Field `TI1SEL` reader - selects TI1\[0\] to TI1\[15\] input
pub type TI1SEL_R = crate::FieldReader<TI1SEL>;
impl TI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI1SEL> {
        match self.bits {
            0 => Some(TI1SEL::B0x0),
            1 => Some(TI1SEL::B0x1),
            2 => Some(TI1SEL::B0x2),
            _ => None,
        }
    }
    ///TIM15_CH1 input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI1SEL::B0x0
    }
    ///TIM2_IC1
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI1SEL::B0x1
    }
    ///TIM3_IC1
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TI1SEL::B0x2
    }
}
///Field `TI1SEL` writer - selects TI1\[0\] to TI1\[15\] input
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI1SEL>;
impl<'a, REG> TI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM15_CH1 input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::B0x0)
    }
    ///TIM2_IC1
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::B0x1)
    }
    ///TIM3_IC1
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::B0x2)
    }
}
/**selects TI2\[0\] to TI2\[15\] input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI2SEL {
    ///0: TIM15_CH2 input
    B0x0 = 0,
    ///1: TIM2_IC2
    B0x1 = 1,
    ///2: TIM3_IC2
    B0x2 = 2,
}
impl From<TI2SEL> for u8 {
    #[inline(always)]
    fn from(variant: TI2SEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TI2SEL {
    type Ux = u8;
}
impl crate::IsEnum for TI2SEL {}
///Field `TI2SEL` reader - selects TI2\[0\] to TI2\[15\] input
pub type TI2SEL_R = crate::FieldReader<TI2SEL>;
impl TI2SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI2SEL> {
        match self.bits {
            0 => Some(TI2SEL::B0x0),
            1 => Some(TI2SEL::B0x1),
            2 => Some(TI2SEL::B0x2),
            _ => None,
        }
    }
    ///TIM15_CH2 input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI2SEL::B0x0
    }
    ///TIM2_IC2
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI2SEL::B0x1
    }
    ///TIM3_IC2
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TI2SEL::B0x2
    }
}
///Field `TI2SEL` writer - selects TI2\[0\] to TI2\[15\] input
pub type TI2SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI2SEL>;
impl<'a, REG> TI2SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM15_CH2 input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL::B0x0)
    }
    ///TIM2_IC2
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL::B0x1)
    }
    ///TIM3_IC2
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TI2SEL::B0x2)
    }
}
impl R {
    ///Bits 0:3 - selects TI1\[0\] to TI1\[15\] input
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 8:11 - selects TI2\[0\] to TI2\[15\] input
    #[inline(always)]
    pub fn ti2sel(&self) -> TI2SEL_R {
        TI2SEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM15_TISEL")
            .field("ti1sel", &self.ti1sel())
            .field("ti2sel", &self.ti2sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - selects TI1\[0\] to TI1\[15\] input
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W<'_, TIM15_TISELrs> {
        TI1SEL_W::new(self, 0)
    }
    ///Bits 8:11 - selects TI2\[0\] to TI2\[15\] input
    #[inline(always)]
    pub fn ti2sel(&mut self) -> TI2SEL_W<'_, TIM15_TISELrs> {
        TI2SEL_W::new(self, 8)
    }
}
/**TIM15 input selection register

You can [`read`](crate::Reg::read) this register and get [`tim15_tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim15_tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM15:TIM15_TISEL)*/
pub struct TIM15_TISELrs;
impl crate::RegisterSpec for TIM15_TISELrs {
    type Ux = u32;
}
///`read()` method returns [`tim15_tisel::R`](R) reader structure
impl crate::Readable for TIM15_TISELrs {}
///`write(|w| ..)` method takes [`tim15_tisel::W`](W) writer structure
impl crate::Writable for TIM15_TISELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM15_TISEL to value 0
impl crate::Resettable for TIM15_TISELrs {}
