///Register `TIM2_CCMR2` reader
pub type R = crate::R<TIM2_CCMR2rs>;
///Register `TIM2_CCMR2` writer
pub type W = crate::W<TIM2_CCMR2rs>;
/**Capture/Compare 3 selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S {
    ///0: CC3 channel is configured as output
    B0x0 = 0,
    ///1: CC3 channel is configured as input, IC3 is mapped on TI3
    B0x1 = 1,
    ///2: CC3 channel is configured as input, IC3 is mapped on TI4
    B0x2 = 2,
    ///3: CC3 channel is configured as input, IC3 is mapped on TRC.
    B0x3 = 3,
}
impl From<CC3S> for u8 {
    #[inline(always)]
    fn from(variant: CC3S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC3S {
    type Ux = u8;
}
impl crate::IsEnum for CC3S {}
///Field `CC3S` reader - Capture/Compare 3 selection
pub type CC3S_R = crate::FieldReader<CC3S>;
impl CC3S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC3S {
        match self.bits {
            0 => CC3S::B0x0,
            1 => CC3S::B0x1,
            2 => CC3S::B0x2,
            3 => CC3S::B0x3,
            _ => unreachable!(),
        }
    }
    ///CC3 channel is configured as output
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC3S::B0x0
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI3
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC3S::B0x1
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI4
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC3S::B0x2
    }
    ///CC3 channel is configured as input, IC3 is mapped on TRC.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC3S::B0x3
    }
}
///Field `CC3S` writer - Capture/Compare 3 selection
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC3S, crate::Safe>;
impl<'a, REG> CC3S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC3 channel is configured as output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::B0x0)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI3
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::B0x1)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI4
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::B0x2)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TRC.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::B0x3)
    }
}
///Field `IC3PSC` reader - Input capture 3 prescaler
pub type IC3PSC_R = crate::FieldReader;
///Field `IC3PSC` writer - Input capture 3 prescaler
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC3F` reader - Input capture 3 filter
pub type IC3F_R = crate::FieldReader;
///Field `IC3F` writer - Input capture 3 filter
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
/**Capture/Compare 4 selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC4S {
    ///0: CC4 channel is configured as output
    B0x0 = 0,
    ///1: CC4 channel is configured as input, IC4 is mapped on TI4
    B0x1 = 1,
    ///2: CC4 channel is configured as input, IC4 is mapped on TI3
    B0x2 = 2,
    ///3: CC4 channel is configured as input, IC4 is mapped on TRC.
    B0x3 = 3,
}
impl From<CC4S> for u8 {
    #[inline(always)]
    fn from(variant: CC4S) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC4S {
    type Ux = u8;
}
impl crate::IsEnum for CC4S {}
///Field `CC4S` reader - Capture/Compare 4 selection
pub type CC4S_R = crate::FieldReader<CC4S>;
impl CC4S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CC4S {
        match self.bits {
            0 => CC4S::B0x0,
            1 => CC4S::B0x1,
            2 => CC4S::B0x2,
            3 => CC4S::B0x3,
            _ => unreachable!(),
        }
    }
    ///CC4 channel is configured as output
    #[inline(always)]
    pub fn is_b_0x0(&self) -> bool {
        *self == CC4S::B0x0
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI4
    #[inline(always)]
    pub fn is_b_0x1(&self) -> bool {
        *self == CC4S::B0x1
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI3
    #[inline(always)]
    pub fn is_b_0x2(&self) -> bool {
        *self == CC4S::B0x2
    }
    ///CC4 channel is configured as input, IC4 is mapped on TRC.
    #[inline(always)]
    pub fn is_b_0x3(&self) -> bool {
        *self == CC4S::B0x3
    }
}
///Field `CC4S` writer - Capture/Compare 4 selection
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC4S, crate::Safe>;
impl<'a, REG> CC4S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC4 channel is configured as output
    #[inline(always)]
    pub fn b_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::B0x0)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI4
    #[inline(always)]
    pub fn b_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::B0x1)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI3
    #[inline(always)]
    pub fn b_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::B0x2)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TRC.
    #[inline(always)]
    pub fn b_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::B0x3)
    }
}
///Field `IC4PSC` reader - Input capture 4 prescaler
pub type IC4PSC_R = crate::FieldReader;
///Field `IC4PSC` writer - Input capture 4 prescaler
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `IC4F` reader - Input capture 4 filter
pub type IC4F_R = crate::FieldReader;
///Field `IC4F` writer - Input capture 4 filter
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&self) -> CC3S_R {
        CC3S_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&self) -> IC3PSC_R {
        IC3PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&self) -> IC3F_R {
        IC3F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&self) -> CC4S_R {
        CC4S_R::new(((self.bits >> 8) & 3) as u8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&self) -> IC4PSC_R {
        IC4PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&self) -> IC4F_R {
        IC4F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM2_CCMR2")
            .field("cc3s", &self.cc3s())
            .field("ic3psc", &self.ic3psc())
            .field("ic3f", &self.ic3f())
            .field("cc4s", &self.cc4s())
            .field("ic4psc", &self.ic4psc())
            .field("ic4f", &self.ic4f())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Capture/Compare 3 selection
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<'_, TIM2_CCMR2rs> {
        CC3S_W::new(self, 0)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W<'_, TIM2_CCMR2rs> {
        IC3PSC_W::new(self, 2)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W<'_, TIM2_CCMR2rs> {
        IC3F_W::new(self, 4)
    }
    ///Bits 8:9 - Capture/Compare 4 selection
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<'_, TIM2_CCMR2rs> {
        CC4S_W::new(self, 8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W<'_, TIM2_CCMR2rs> {
        IC4PSC_W::new(self, 10)
    }
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W<'_, TIM2_CCMR2rs> {
        IC4F_W::new(self, 12)
    }
}
/**TIM2 capture/compare mode register 2

You can [`read`](crate::Reg::read) this register and get [`tim2_ccmr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim2_ccmr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#TIM2:TIM2_CCMR2)*/
pub struct TIM2_CCMR2rs;
impl crate::RegisterSpec for TIM2_CCMR2rs {
    type Ux = u32;
}
///`read()` method returns [`tim2_ccmr2::R`](R) reader structure
impl crate::Readable for TIM2_CCMR2rs {}
///`write(|w| ..)` method takes [`tim2_ccmr2::W`](W) writer structure
impl crate::Writable for TIM2_CCMR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIM2_CCMR2 to value 0
impl crate::Resettable for TIM2_CCMR2rs {}
