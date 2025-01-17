///Register `CCMR2_Input` reader
pub type R = crate::R<CCMR2_INPUTrs>;
///Register `CCMR2_Input` writer
pub type W = crate::W<CCMR2_INPUTrs>;
/**Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC3S {
    ///1: CC3 channel is configured as input, IC3 is mapped on TI3
    Ti3 = 1,
    ///2: CC3 channel is configured as input, IC3 is mapped on TI4
    Ti4 = 2,
    ///3: CC3 channel is configured as input, IC3 is mapped on TRC
    Trc = 3,
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
///Field `CC3S` reader - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
pub type CC3S_R = crate::FieldReader<CC3S>;
impl CC3S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC3S> {
        match self.bits {
            1 => Some(CC3S::Ti3),
            2 => Some(CC3S::Ti4),
            3 => Some(CC3S::Trc),
            _ => None,
        }
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI3
    #[inline(always)]
    pub fn is_ti3(&self) -> bool {
        *self == CC3S::Ti3
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI4
    #[inline(always)]
    pub fn is_ti4(&self) -> bool {
        *self == CC3S::Ti4
    }
    ///CC3 channel is configured as input, IC3 is mapped on TRC
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC3S::Trc
    }
}
///Field `CC3S` writer - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
pub type CC3S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC3S>;
impl<'a, REG> CC3S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC3 channel is configured as input, IC3 is mapped on TI3
    #[inline(always)]
    pub fn ti3(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Ti3)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TI4
    #[inline(always)]
    pub fn ti4(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Ti4)
    }
    ///CC3 channel is configured as input, IC3 is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut crate::W<REG> {
        self.variant(CC3S::Trc)
    }
}
///Field `IC3PSC` reader - Input capture 3 prescaler
pub type IC3PSC_R = crate::FieldReader;
///Field `IC3PSC` writer - Input capture 3 prescaler
pub type IC3PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
///Field `IC3F` reader - Input capture 3 filter
pub type IC3F_R = crate::FieldReader;
///Field `IC3F` writer - Input capture 3 filter
pub type IC3F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
/**Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC4S {
    ///1: CC4 channel is configured as input, IC4 is mapped on TI4
    Ti4 = 1,
    ///2: CC4 channel is configured as input, IC4 is mapped on TI3
    Ti3 = 2,
    ///3: CC4 channel is configured as input, IC4 is mapped on TRC
    Trc = 3,
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
///Field `CC4S` reader - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
pub type CC4S_R = crate::FieldReader<CC4S>;
impl CC4S_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CC4S> {
        match self.bits {
            1 => Some(CC4S::Ti4),
            2 => Some(CC4S::Ti3),
            3 => Some(CC4S::Trc),
            _ => None,
        }
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI4
    #[inline(always)]
    pub fn is_ti4(&self) -> bool {
        *self == CC4S::Ti4
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI3
    #[inline(always)]
    pub fn is_ti3(&self) -> bool {
        *self == CC4S::Ti3
    }
    ///CC4 channel is configured as input, IC4 is mapped on TRC
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC4S::Trc
    }
}
///Field `CC4S` writer - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
pub type CC4S_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CC4S>;
impl<'a, REG> CC4S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///CC4 channel is configured as input, IC4 is mapped on TI4
    #[inline(always)]
    pub fn ti4(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::Ti4)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TI3
    #[inline(always)]
    pub fn ti3(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::Ti3)
    }
    ///CC4 channel is configured as input, IC4 is mapped on TRC
    #[inline(always)]
    pub fn trc(self) -> &'a mut crate::W<REG> {
        self.variant(CC4S::Trc)
    }
}
///Field `IC4PSC` reader - Input capture 4 prescaler
pub type IC4PSC_R = crate::FieldReader;
///Field `IC4PSC` writer - Input capture 4 prescaler
pub type IC4PSC_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
///Field `IC4F` reader - Input capture 4 filter
pub type IC4F_R = crate::FieldReader;
///Field `IC4F` writer - Input capture 4 filter
pub type IC4F_W<'a, REG> = crate::FieldWriter<'a, REG, 4, u8, crate::Safe>;
impl R {
    ///Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
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
    ///Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
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
        f.debug_struct("CCMR2_Input")
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
    ///Bits 0:1 - Capture/Compare 3 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC3S bits are writable only when the channel is OFF (CC3E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc3s(&mut self) -> CC3S_W<CCMR2_INPUTrs> {
        CC3S_W::new(self, 0)
    }
    ///Bits 2:3 - Input capture 3 prescaler
    #[inline(always)]
    pub fn ic3psc(&mut self) -> IC3PSC_W<CCMR2_INPUTrs> {
        IC3PSC_W::new(self, 2)
    }
    ///Bits 4:7 - Input capture 3 filter
    #[inline(always)]
    pub fn ic3f(&mut self) -> IC3F_W<CCMR2_INPUTrs> {
        IC3F_W::new(self, 4)
    }
    ///Bits 8:9 - Capture/Compare 4 selection This bit-field defines the direction of the channel (input/output) as well as the used input. Note: CC4S bits are writable only when the channel is OFF (CC4E = 0 in TIMx_CCER).
    #[inline(always)]
    pub fn cc4s(&mut self) -> CC4S_W<CCMR2_INPUTrs> {
        CC4S_W::new(self, 8)
    }
    ///Bits 10:11 - Input capture 4 prescaler
    #[inline(always)]
    pub fn ic4psc(&mut self) -> IC4PSC_W<CCMR2_INPUTrs> {
        IC4PSC_W::new(self, 10)
    }
    ///Bits 12:15 - Input capture 4 filter
    #[inline(always)]
    pub fn ic4f(&mut self) -> IC4F_W<CCMR2_INPUTrs> {
        IC4F_W::new(self, 12)
    }
}
/**TIM2 capture/compare mode register 2 \[alternate\]

You can [`read`](crate::Reg::read) this register and get [`ccmr2_input::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccmr2_input::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#TIM2:CCMR2_Input)*/
pub struct CCMR2_INPUTrs;
impl crate::RegisterSpec for CCMR2_INPUTrs {
    type Ux = u32;
}
///`read()` method returns [`ccmr2_input::R`](R) reader structure
impl crate::Readable for CCMR2_INPUTrs {}
///`write(|w| ..)` method takes [`ccmr2_input::W`](W) writer structure
impl crate::Writable for CCMR2_INPUTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCMR2_Input to value 0
impl crate::Resettable for CCMR2_INPUTrs {
    const RESET_VALUE: u32 = 0;
}
