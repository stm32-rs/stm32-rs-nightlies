#[doc = "Register `JSQR` reader"]
pub type R = crate::R<JSQRrs>;
#[doc = "Register `JSQR` writer"]
pub type W = crate::W<JSQRrs>;
#[doc = "Field `JL` reader - Injected channel sequence length"]
pub type JL_R = crate::FieldReader;
#[doc = "Field `JL` writer - Injected channel sequence length"]
pub type JL_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2>;
#[doc = "External Trigger Selection for injected group\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL {
    #[doc = "0: Timer 1 TRGO event"]
    Tim1Trgo = 0,
    #[doc = "1: Timer 1 CC4 event"]
    Tim1Cc4 = 1,
    #[doc = "2: Timer 2 TRGO event"]
    Tim2Trgo = 2,
    #[doc = "3: Timer 2 CC1 event"]
    Tim2Cc1 = 3,
    #[doc = "4: Timer 3 CC4 event"]
    Tim3Cc4 = 4,
    #[doc = "6: EXTI line 15"]
    Exti15 = 6,
    #[doc = "8: Timer 1 TRGO2 event"]
    Tim1Trgo2 = 8,
    #[doc = "9: HRTIM_ADCTRG2 event"]
    HrtimAdctrg2 = 9,
    #[doc = "10: HRTIM_ADCTRG4 event"]
    HrtimAdctrg4 = 10,
    #[doc = "11: Timer 3 CC3 event"]
    Tim3Cc3 = 11,
    #[doc = "12: Timer 3 TRGO event"]
    Tim3Trgo = 12,
    #[doc = "13: Timer 3 CC1 event"]
    Tim3Cc1 = 13,
    #[doc = "14: Timer 6 TRGO event"]
    Tim6Trgo = 14,
    #[doc = "15: Timer 15 TRGO event"]
    Tim15Trgo = 15,
}
impl From<JEXTSEL> for u8 {
    #[inline(always)]
    fn from(variant: JEXTSEL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTSEL {
    type Ux = u8;
}
#[doc = "Field `JEXTSEL` reader - External Trigger Selection for injected group"]
pub type JEXTSEL_R = crate::FieldReader<JEXTSEL>;
impl JEXTSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<JEXTSEL> {
        match self.bits {
            0 => Some(JEXTSEL::Tim1Trgo),
            1 => Some(JEXTSEL::Tim1Cc4),
            2 => Some(JEXTSEL::Tim2Trgo),
            3 => Some(JEXTSEL::Tim2Cc1),
            4 => Some(JEXTSEL::Tim3Cc4),
            6 => Some(JEXTSEL::Exti15),
            8 => Some(JEXTSEL::Tim1Trgo2),
            9 => Some(JEXTSEL::HrtimAdctrg2),
            10 => Some(JEXTSEL::HrtimAdctrg4),
            11 => Some(JEXTSEL::Tim3Cc3),
            12 => Some(JEXTSEL::Tim3Trgo),
            13 => Some(JEXTSEL::Tim3Cc1),
            14 => Some(JEXTSEL::Tim6Trgo),
            15 => Some(JEXTSEL::Tim15Trgo),
            _ => None,
        }
    }
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == JEXTSEL::Tim1Trgo
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline(always)]
    pub fn is_tim1_cc4(&self) -> bool {
        *self == JEXTSEL::Tim1Cc4
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == JEXTSEL::Tim2Trgo
    }
    #[doc = "Timer 2 CC1 event"]
    #[inline(always)]
    pub fn is_tim2_cc1(&self) -> bool {
        *self == JEXTSEL::Tim2Cc1
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == JEXTSEL::Tim3Cc4
    }
    #[doc = "EXTI line 15"]
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL::Exti15
    }
    #[doc = "Timer 1 TRGO2 event"]
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == JEXTSEL::Tim1Trgo2
    }
    #[doc = "HRTIM_ADCTRG2 event"]
    #[inline(always)]
    pub fn is_hrtim_adctrg2(&self) -> bool {
        *self == JEXTSEL::HrtimAdctrg2
    }
    #[doc = "HRTIM_ADCTRG4 event"]
    #[inline(always)]
    pub fn is_hrtim_adctrg4(&self) -> bool {
        *self == JEXTSEL::HrtimAdctrg4
    }
    #[doc = "Timer 3 CC3 event"]
    #[inline(always)]
    pub fn is_tim3_cc3(&self) -> bool {
        *self == JEXTSEL::Tim3Cc3
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == JEXTSEL::Tim3Trgo
    }
    #[doc = "Timer 3 CC1 event"]
    #[inline(always)]
    pub fn is_tim3_cc1(&self) -> bool {
        *self == JEXTSEL::Tim3Cc1
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == JEXTSEL::Tim6Trgo
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == JEXTSEL::Tim15Trgo
    }
}
#[doc = "Field `JEXTSEL` writer - External Trigger Selection for injected group"]
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, JEXTSEL>;
impl<'a, REG> JEXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer 1 TRGO event"]
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Trgo)
    }
    #[doc = "Timer 1 CC4 event"]
    #[inline(always)]
    pub fn tim1_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Cc4)
    }
    #[doc = "Timer 2 TRGO event"]
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2Trgo)
    }
    #[doc = "Timer 2 CC1 event"]
    #[inline(always)]
    pub fn tim2_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2Cc1)
    }
    #[doc = "Timer 3 CC4 event"]
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Cc4)
    }
    #[doc = "EXTI line 15"]
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Exti15)
    }
    #[doc = "Timer 1 TRGO2 event"]
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Trgo2)
    }
    #[doc = "HRTIM_ADCTRG2 event"]
    #[inline(always)]
    pub fn hrtim_adctrg2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::HrtimAdctrg2)
    }
    #[doc = "HRTIM_ADCTRG4 event"]
    #[inline(always)]
    pub fn hrtim_adctrg4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::HrtimAdctrg4)
    }
    #[doc = "Timer 3 CC3 event"]
    #[inline(always)]
    pub fn tim3_cc3(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Cc3)
    }
    #[doc = "Timer 3 TRGO event"]
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Trgo)
    }
    #[doc = "Timer 3 CC1 event"]
    #[inline(always)]
    pub fn tim3_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Cc1)
    }
    #[doc = "Timer 6 TRGO event"]
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim6Trgo)
    }
    #[doc = "Timer 15 TRGO event"]
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim15Trgo)
    }
}
#[doc = "External Trigger Enable and Polarity Selection for injected channels\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN {
    #[doc = "0: Trigger detection disabled"]
    Disabled = 0,
    #[doc = "1: Trigger detection on the rising edge"]
    RisingEdge = 1,
    #[doc = "2: Trigger detection on the falling edge"]
    FallingEdge = 2,
    #[doc = "3: Trigger detection on both the rising and falling edges"]
    BothEdges = 3,
}
impl From<JEXTEN> for u8 {
    #[inline(always)]
    fn from(variant: JEXTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for JEXTEN {
    type Ux = u8;
}
#[doc = "Field `JEXTEN` reader - External Trigger Enable and Polarity Selection for injected channels"]
pub type JEXTEN_R = crate::FieldReader<JEXTEN>;
impl JEXTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> JEXTEN {
        match self.bits {
            0 => JEXTEN::Disabled,
            1 => JEXTEN::RisingEdge,
            2 => JEXTEN::FallingEdge,
            3 => JEXTEN::BothEdges,
            _ => unreachable!(),
        }
    }
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN::Disabled
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN::RisingEdge
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN::FallingEdge
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN::BothEdges
    }
}
#[doc = "Field `JEXTEN` writer - External Trigger Enable and Polarity Selection for injected channels"]
pub type JEXTEN_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, JEXTEN>;
impl<'a, REG> JEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger detection disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::Disabled)
    }
    #[doc = "Trigger detection on the rising edge"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::RisingEdge)
    }
    #[doc = "Trigger detection on the falling edge"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::FallingEdge)
    }
    #[doc = "Trigger detection on both the rising and falling edges"]
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::BothEdges)
    }
}
#[doc = "Field `JSQ1` reader - 1st conversion in the injected sequence"]
pub type JSQ1_R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - 1st conversion in the injected sequence"]
pub type JSQ1_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ2` reader - 2nd conversion in the injected sequence"]
pub type JSQ2_R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - 2nd conversion in the injected sequence"]
pub type JSQ2_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ3` reader - 3rd conversion in the injected sequence"]
pub type JSQ3_R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - 3rd conversion in the injected sequence"]
pub type JSQ3_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ4` reader - 4th conversion in the injected sequence"]
pub type JSQ4_R = crate::FieldReader;
#[doc = "Field `JSQ4` writer - 4th conversion in the injected sequence"]
pub type JSQ4_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - Injected channel sequence length"]
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5 - External Trigger Selection for injected group"]
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7 - External Trigger Enable and Polarity Selection for injected channels"]
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - 1st conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ1_R {
        JSQ1_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 14:18 - 2nd conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ2_R {
        JSQ2_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - 3rd conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ3_R {
        JSQ3_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - 4th conversion in the injected sequence"]
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ4_R {
        JSQ4_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Injected channel sequence length"]
    #[inline(always)]
    #[must_use]
    pub fn jl(&mut self) -> JL_W<JSQRrs> {
        JL_W::new(self, 0)
    }
    #[doc = "Bits 2:5 - External Trigger Selection for injected group"]
    #[inline(always)]
    #[must_use]
    pub fn jextsel(&mut self) -> JEXTSEL_W<JSQRrs> {
        JEXTSEL_W::new(self, 2)
    }
    #[doc = "Bits 6:7 - External Trigger Enable and Polarity Selection for injected channels"]
    #[inline(always)]
    #[must_use]
    pub fn jexten(&mut self) -> JEXTEN_W<JSQRrs> {
        JEXTEN_W::new(self, 6)
    }
    #[doc = "Bits 8:12 - 1st conversion in the injected sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq1(&mut self) -> JSQ1_W<JSQRrs> {
        JSQ1_W::new(self, 8)
    }
    #[doc = "Bits 14:18 - 2nd conversion in the injected sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq2(&mut self) -> JSQ2_W<JSQRrs> {
        JSQ2_W::new(self, 14)
    }
    #[doc = "Bits 20:24 - 3rd conversion in the injected sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq3(&mut self) -> JSQ3_W<JSQRrs> {
        JSQ3_W::new(self, 20)
    }
    #[doc = "Bits 26:30 - 4th conversion in the injected sequence"]
    #[inline(always)]
    #[must_use]
    pub fn jsq4(&mut self) -> JSQ4_W<JSQRrs> {
        JSQ4_W::new(self, 26)
    }
}
#[doc = "ADC injected sequence register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`jsqr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JSQRrs;
impl crate::RegisterSpec for JSQRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jsqr::R`](R) reader structure"]
impl crate::Readable for JSQRrs {}
#[doc = "`write(|w| ..)` method takes [`jsqr::W`](W) writer structure"]
impl crate::Writable for JSQRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JSQRrs {
    const RESET_VALUE: u32 = 0;
}
