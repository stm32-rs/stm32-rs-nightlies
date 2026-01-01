///Register `TIM14_TISEL` reader
pub type R = crate::R<TIM14_TISELrs>;
///Register `TIM14_TISEL` writer
pub type W = crate::W<TIM14_TISELrs>;
/**selects TI1\[0\] to TI1\[15\] input Others: Reserved

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TI1SEL {
    ///0: TIM14_CH1 input
    B0x0 = 0,
    ///1: RTC CLK
    B0x1 = 1,
    ///2: HSE/32
    B0x2 = 2,
    ///3: MCO
    B0x3 = 3,
    ///4: MCO2
    B0x4 = 4,
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
///Field `TI1SEL` reader - selects TI1\[0\] to TI1\[15\] input Others: Reserved
pub type TI1SEL_R = crate::FieldReader<TI1SEL>;
impl TI1SEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<TI1SEL> {
        match self.bits {
            0 => Some(TI1SEL::B0x0),
            1 => Some(TI1SEL::B0x1),
            2 => Some(TI1SEL::B0x2),
            3 => Some(TI1SEL::B0x3),
            4 => Some(TI1SEL::B0x4),
            _ => None,
        }
    }
    ///TIM14_CH1 input
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == TI1SEL::B0x0
    }
    ///RTC CLK
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == TI1SEL::B0x1
    }
    ///HSE/32
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == TI1SEL::B0x2
    }
    ///MCO
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == TI1SEL::B0x3
    }
    ///MCO2
    #[inline(always)]
    pub fn is_b_0x4(&self) -> bool {
        *self == TI1SEL::B0x4
    }
}
///Field `TI1SEL` writer - selects TI1\[0\] to TI1\[15\] input Others: Reserved
pub type TI1SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, TI1SEL>;
impl<'a, REG> TI1SEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///TIM14_CH1 input
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::B0x0)
    }
    ///RTC CLK
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::B0x1)
    }
    ///HSE/32
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::B0x2)
    }
    ///MCO
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::B0x3)
    }
    ///MCO2
    #[inline(always)]
    pub fn b_0x4(self) -> &'a mut crate::W<REG> {
        self.variant(TI1SEL::B0x4)
    }
}
impl R {
    ///Bits 0:3 - selects TI1\[0\] to TI1\[15\] input Others: Reserved
    #[inline(always)]
    pub fn ti1sel(&self) -> TI1SEL_R {
        TI1SEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM14_TISEL")
            .field("ti1sel", &self.ti1sel())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - selects TI1\[0\] to TI1\[15\] input Others: Reserved
    #[inline(always)]
    pub fn ti1sel(&mut self) -> TI1SEL_W<'_, TIM14_TISELrs> {
        TI1SEL_W::new(self, 0)
    }
}
/**TIM14 timer input selection register

You can [`read`](crate::Reg::read) this register and get [`tim14_tisel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim14_tisel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C051.html#TIM14:TIM14_TISEL)*/
pub struct TIM14_TISELrs;
impl crate::RegisterSpec for TIM14_TISELrs {
    type Ux = u16;
}
///`read()` method returns [`tim14_tisel::R`](R) reader structure
impl crate::Readable for TIM14_TISELrs {}
///`write(|w| ..)` method takes [`tim14_tisel::W`](W) writer structure
impl crate::Writable for TIM14_TISELrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM14_TISEL to value 0
impl crate::Resettable for TIM14_TISELrs {}
