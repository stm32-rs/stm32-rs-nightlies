#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Master mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS {
    #[doc = "0: Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)"]
    Reset = 0,
    #[doc = "1: Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)"]
    Enable = 1,
    #[doc = "2: Update - The update event is selected as trigger output (TRGO)"]
    Update = 2,
    #[doc = "3: Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred"]
    ComparePulse = 3,
    #[doc = "4: OC1REF signal is used as trigger output (TRGO)"]
    Oc1ref = 4,
    #[doc = "5: OC2REF signal is used as trigger output (TRGO)"]
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
#[doc = "Field `MMS` reader - Master mode selection"]
pub type MMS_R = crate::FieldReader<MMS>;
impl MMS_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS::Reset
    }
    #[doc = "Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS::Enable
    }
    #[doc = "Update - The update event is selected as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS::Update
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS::ComparePulse
    }
    #[doc = "OC1REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_oc1ref(&self) -> bool {
        *self == MMS::Oc1ref
    }
    #[doc = "OC2REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn is_oc2ref(&self) -> bool {
        *self == MMS::Oc2ref
    }
}
#[doc = "Field `MMS` writer - Master mode selection"]
pub type MMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, MMS>;
impl<'a, REG> MMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset - the UG bit from the TIMx_EGR register is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Reset)
    }
    #[doc = "Enable - the Counter enable signal, CNT_EN, is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Enable)
    }
    #[doc = "Update - The update event is selected as trigger output (TRGO)"]
    #[inline(always)]
    pub fn update(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Update)
    }
    #[doc = "Compare Pulse - The trigger output send a positive pulse when the CC1IF flag is to be set (even if it was already high), as soon as a capture or a compare match occurred"]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::ComparePulse)
    }
    #[doc = "OC1REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn oc1ref(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Oc1ref)
    }
    #[doc = "OC2REF signal is used as trigger output (TRGO)"]
    #[inline(always)]
    pub fn oc2ref(self) -> &'a mut crate::W<REG> {
        self.variant(MMS::Oc2ref)
    }
}
impl R {
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<CR2rs> {
        MMS_W::new(self, 4)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
