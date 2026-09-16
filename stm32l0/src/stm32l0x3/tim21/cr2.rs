///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
/**Master mode selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS {
    ///0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)
    Reset = 0,
    ///1: Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)
    Enable = 1,
    ///2: Update - The update event is selected as trigger output (TRGO)
    Update = 2,
    ///3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred
    ComparePulse = 3,
    ///4: OC1REF signal is used as trigger output (TRGO)
    Oc1ref = 4,
    ///5: OC2REF signal is used as trigger output (TRGO)
    Oc2ref = 5,
}
impl From<MMS> for u8 {
    #[inline(always)]
    fn from(variant: MMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS {
    type Ux = u8;
}
impl crate::IsEnum for MMS {}
///Field `MMS` reader - Master mode selection
pub type MMS_R = crate::FieldReader<MMS>;
impl MMS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<MMS> {
        match self.bits {
            0 => Some(MMS::Reset),
            1 => Some(MMS::Enable),
            2 => Some(MMS::Update),
            3 => Some(MMS::ComparePulse),
            4 => Some(MMS::Oc1ref),
            5 => Some(MMS::Oc2ref),
            _ => None,
        }
    }
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS::Reset
    }
    ///Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS::Enable
    }
    ///Update - The update event is selected as trigger output (TRGO)
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS::Update
    }
    ///Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS::ComparePulse
    }
    ///OC1REF signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn is_oc1ref(&self) -> bool {
        *self == MMS::Oc1ref
    }
    ///OC2REF signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn is_oc2ref(&self) -> bool {
        *self == MMS::Oc2ref
    }
}
///Field `MMS` writer - Master mode selection
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MMS>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Reset)
    }
    ///Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Enable)
    }
    ///Update - The update event is selected as trigger output (TRGO)
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Update)
    }
    ///Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::ComparePulse)
    }
    ///OC1REF signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn oc1ref(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Oc1ref)
    }
    ///OC2REF signal is used as trigger output (TRGO)
    #[inline(always)]
    pub fn oc2ref(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Oc2ref)
    }
}
impl R {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2").field("mms", &self.mms()).finish()
    }
}
impl W {
    ///Bits 4:6 - Master mode selection
    #[inline(always)]
    pub fn mms(&mut self) -> MMS_W<'_, CR2rs> {
        MMS_W::new(self, 4)
    }
}
/**control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x3.html#TIM21:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u16;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
