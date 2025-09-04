///Register `JSQR` reader
pub type R = crate::R<JSQRrs>;
///Register `JSQR` writer
pub type W = crate::W<JSQRrs>;
///Field `JL` reader - ADC group injected sequencer scan length
pub type JL_R = crate::FieldReader;
///Field `JL` writer - ADC group injected sequencer scan length
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
/**ADC group injected external trigger source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTSEL {
    ///0: Timer 1 TRGO event
    Tim1Trgo = 0,
    ///1: Timer 1 CC4 event
    Tim1Cc4 = 1,
    ///2: Timer 2 TRGO event
    Tim2Trgo = 2,
    ///3: Timer 2 CC1 event
    Tim2Cc1 = 3,
    ///4: Timer 3 CC4 event
    Tim3Cc4 = 4,
    ///5: Timer 4 TRGO event
    Tim4Trgo = 5,
    ///6: EXTI line 15
    Exti15 = 6,
    ///7: Timer 8 CC4 event
    Tim8Cc4 = 7,
    ///8: Timer 1 TRGO2 event
    Tim1Trgo2 = 8,
    ///9: Timer 8 TRGO event
    Tim8Trgo = 9,
    ///10: Timer 8 TRGO2 event
    Tim8Trgo2 = 10,
    ///11: Timer 3 CC3 event
    Tim3Cc3 = 11,
    ///12: Timer 3 TRGO event
    Tim3Trgo = 12,
    ///13: Timer 3 CC1 event
    Tim3Cc1 = 13,
    ///14: Timer 6 TRGO event
    Tim6Trgo = 14,
    ///15: Timer 15 TRGO event
    Tim15Trgo = 15,
    ///16: HRTIM1_ADCTRG2 event
    Hrtim1Adctrg2 = 16,
    ///17: HRTIM1_ADCTRG4 event
    Hrtim1Adctrg4 = 17,
    ///18: LPTIM1_OUT event
    Lptim1Out = 18,
    ///19: LPTIM2_OUT event
    Lptim2Out = 19,
    ///20: LPTIM3_OUT event
    Lptim3Out = 20,
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
impl crate::IsEnum for JEXTSEL {}
///Field `JEXTSEL` reader - ADC group injected external trigger source
pub type JEXTSEL_R = crate::FieldReader<JEXTSEL>;
impl JEXTSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<JEXTSEL> {
        match self.bits {
            0 => Some(JEXTSEL::Tim1Trgo),
            1 => Some(JEXTSEL::Tim1Cc4),
            2 => Some(JEXTSEL::Tim2Trgo),
            3 => Some(JEXTSEL::Tim2Cc1),
            4 => Some(JEXTSEL::Tim3Cc4),
            5 => Some(JEXTSEL::Tim4Trgo),
            6 => Some(JEXTSEL::Exti15),
            7 => Some(JEXTSEL::Tim8Cc4),
            8 => Some(JEXTSEL::Tim1Trgo2),
            9 => Some(JEXTSEL::Tim8Trgo),
            10 => Some(JEXTSEL::Tim8Trgo2),
            11 => Some(JEXTSEL::Tim3Cc3),
            12 => Some(JEXTSEL::Tim3Trgo),
            13 => Some(JEXTSEL::Tim3Cc1),
            14 => Some(JEXTSEL::Tim6Trgo),
            15 => Some(JEXTSEL::Tim15Trgo),
            16 => Some(JEXTSEL::Hrtim1Adctrg2),
            17 => Some(JEXTSEL::Hrtim1Adctrg4),
            18 => Some(JEXTSEL::Lptim1Out),
            19 => Some(JEXTSEL::Lptim2Out),
            20 => Some(JEXTSEL::Lptim3Out),
            _ => None,
        }
    }
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn is_tim1_trgo(&self) -> bool {
        *self == JEXTSEL::Tim1Trgo
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn is_tim1_cc4(&self) -> bool {
        *self == JEXTSEL::Tim1Cc4
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2_trgo(&self) -> bool {
        *self == JEXTSEL::Tim2Trgo
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn is_tim2_cc1(&self) -> bool {
        *self == JEXTSEL::Tim2Cc1
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn is_tim3_cc4(&self) -> bool {
        *self == JEXTSEL::Tim3Cc4
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn is_tim4_trgo(&self) -> bool {
        *self == JEXTSEL::Tim4Trgo
    }
    ///EXTI line 15
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL::Exti15
    }
    ///Timer 8 CC4 event
    #[inline(always)]
    pub fn is_tim8_cc4(&self) -> bool {
        *self == JEXTSEL::Tim8Cc4
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == JEXTSEL::Tim1Trgo2
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn is_tim8_trgo(&self) -> bool {
        *self == JEXTSEL::Tim8Trgo
    }
    ///Timer 8 TRGO2 event
    #[inline(always)]
    pub fn is_tim8_trgo2(&self) -> bool {
        *self == JEXTSEL::Tim8Trgo2
    }
    ///Timer 3 CC3 event
    #[inline(always)]
    pub fn is_tim3_cc3(&self) -> bool {
        *self == JEXTSEL::Tim3Cc3
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn is_tim3_trgo(&self) -> bool {
        *self == JEXTSEL::Tim3Trgo
    }
    ///Timer 3 CC1 event
    #[inline(always)]
    pub fn is_tim3_cc1(&self) -> bool {
        *self == JEXTSEL::Tim3Cc1
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn is_tim6_trgo(&self) -> bool {
        *self == JEXTSEL::Tim6Trgo
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn is_tim15_trgo(&self) -> bool {
        *self == JEXTSEL::Tim15Trgo
    }
    ///HRTIM1_ADCTRG2 event
    #[inline(always)]
    pub fn is_hrtim1_adctrg2(&self) -> bool {
        *self == JEXTSEL::Hrtim1Adctrg2
    }
    ///HRTIM1_ADCTRG4 event
    #[inline(always)]
    pub fn is_hrtim1_adctrg4(&self) -> bool {
        *self == JEXTSEL::Hrtim1Adctrg4
    }
    ///LPTIM1_OUT event
    #[inline(always)]
    pub fn is_lptim1_out(&self) -> bool {
        *self == JEXTSEL::Lptim1Out
    }
    ///LPTIM2_OUT event
    #[inline(always)]
    pub fn is_lptim2_out(&self) -> bool {
        *self == JEXTSEL::Lptim2Out
    }
    ///LPTIM3_OUT event
    #[inline(always)]
    pub fn is_lptim3_out(&self) -> bool {
        *self == JEXTSEL::Lptim3Out
    }
}
///Field `JEXTSEL` writer - ADC group injected external trigger source
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5, JEXTSEL>;
impl<'a, REG> JEXTSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Timer 1 TRGO event
    #[inline(always)]
    pub fn tim1_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Trgo)
    }
    ///Timer 1 CC4 event
    #[inline(always)]
    pub fn tim1_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Cc4)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2Trgo)
    }
    ///Timer 2 CC1 event
    #[inline(always)]
    pub fn tim2_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim2Cc1)
    }
    ///Timer 3 CC4 event
    #[inline(always)]
    pub fn tim3_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Cc4)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim4Trgo)
    }
    ///EXTI line 15
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Exti15)
    }
    ///Timer 8 CC4 event
    #[inline(always)]
    pub fn tim8_cc4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8Cc4)
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Trgo2)
    }
    ///Timer 8 TRGO event
    #[inline(always)]
    pub fn tim8_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8Trgo)
    }
    ///Timer 8 TRGO2 event
    #[inline(always)]
    pub fn tim8_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim8Trgo2)
    }
    ///Timer 3 CC3 event
    #[inline(always)]
    pub fn tim3_cc3(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Cc3)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Trgo)
    }
    ///Timer 3 CC1 event
    #[inline(always)]
    pub fn tim3_cc1(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim3Cc1)
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim6Trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15_trgo(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim15Trgo)
    }
    ///HRTIM1_ADCTRG2 event
    #[inline(always)]
    pub fn hrtim1_adctrg2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Hrtim1Adctrg2)
    }
    ///HRTIM1_ADCTRG4 event
    #[inline(always)]
    pub fn hrtim1_adctrg4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Hrtim1Adctrg4)
    }
    ///LPTIM1_OUT event
    #[inline(always)]
    pub fn lptim1_out(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Lptim1Out)
    }
    ///LPTIM2_OUT event
    #[inline(always)]
    pub fn lptim2_out(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Lptim2Out)
    }
    ///LPTIM3_OUT event
    #[inline(always)]
    pub fn lptim3_out(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Lptim3Out)
    }
}
/**ADC group injected external trigger polarity

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum JEXTEN {
    ///0: Trigger detection disabled
    Disabled = 0,
    ///1: Trigger detection on the rising edge
    RisingEdge = 1,
    ///2: Trigger detection on the falling edge
    FallingEdge = 2,
    ///3: Trigger detection on both the rising and falling edges
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
impl crate::IsEnum for JEXTEN {}
///Field `JEXTEN` reader - ADC group injected external trigger polarity
pub type JEXTEN_R = crate::FieldReader<JEXTEN>;
impl JEXTEN_R {
    ///Get enumerated values variant
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
    ///Trigger detection disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == JEXTEN::Disabled
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == JEXTEN::RisingEdge
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == JEXTEN::FallingEdge
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn is_both_edges(&self) -> bool {
        *self == JEXTEN::BothEdges
    }
}
///Field `JEXTEN` writer - ADC group injected external trigger polarity
pub type JEXTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, JEXTEN, crate::Safe>;
impl<'a, REG> JEXTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Trigger detection disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::Disabled)
    }
    ///Trigger detection on the rising edge
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::RisingEdge)
    }
    ///Trigger detection on the falling edge
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::FallingEdge)
    }
    ///Trigger detection on both the rising and falling edges
    #[inline(always)]
    pub fn both_edges(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTEN::BothEdges)
    }
}
///Field `JSQ(1-4)` reader - %s conversion in injected sequence
pub type JSQ_R = crate::FieldReader;
///Field `JSQ(1-4)` writer - %s conversion in injected sequence
pub type JSQ_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    ///Bits 0:1 - ADC group injected sequencer scan length
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:6 - ADC group injected external trigger source
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    ///Bits 7:8 - ADC group injected external trigger polarity
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 7) & 3) as u8)
    }
    ///(1-4) conversion in injected sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `JSQ1` field.</div>
    #[inline(always)]
    pub fn jsq(&self, n: u8) -> JSQ_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        JSQ_R::new(((self.bits >> (n * 6 + 9)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(1-4) conversion in injected sequence
    #[inline(always)]
    pub fn jsq_iter(&self) -> impl Iterator<Item = JSQ_R> + '_ {
        (0..4).map(move |n| JSQ_R::new(((self.bits >> (n * 6 + 9)) & 0x1f) as u8))
    }
    ///Bits 9:13 - 1 conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    ///Bits 15:19 - 2 conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    ///Bits 21:25 - 3 conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    ///Bits 27:31 - 4 conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("JSQR")
            .field("jsq1", &self.jsq1())
            .field("jsq2", &self.jsq2())
            .field("jsq3", &self.jsq3())
            .field("jsq4", &self.jsq4())
            .field("jexten", &self.jexten())
            .field("jextsel", &self.jextsel())
            .field("jl", &self.jl())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - ADC group injected sequencer scan length
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W<JSQRrs> {
        JL_W::new(self, 0)
    }
    ///Bits 2:6 - ADC group injected external trigger source
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<JSQRrs> {
        JEXTSEL_W::new(self, 2)
    }
    ///Bits 7:8 - ADC group injected external trigger polarity
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<JSQRrs> {
        JEXTEN_W::new(self, 7)
    }
    ///(1-4) conversion in injected sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `JSQ1` field.</div>
    #[inline(always)]
    pub fn jsq(&mut self, n: u8) -> JSQ_W<JSQRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        JSQ_W::new(self, n * 6 + 9)
    }
    ///Bits 9:13 - 1 conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 9)
    }
    ///Bits 15:19 - 2 conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 15)
    }
    ///Bits 21:25 - 3 conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 21)
    }
    ///Bits 27:31 - 4 conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 27)
    }
}
/**ADC group injected sequencer register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM7.html#ADC3:JSQR)*/
pub struct JSQRrs;
impl crate::RegisterSpec for JSQRrs {
    type Ux = u32;
}
///`read()` method returns [`jsqr::R`](R) reader structure
impl crate::Readable for JSQRrs {}
///`write(|w| ..)` method takes [`jsqr::W`](W) writer structure
impl crate::Writable for JSQRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets JSQR to value 0
impl crate::Resettable for JSQRrs {}
