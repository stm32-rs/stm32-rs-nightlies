///Register `JSQR` reader
pub type R = crate::R<JSQRrs>;
///Register `JSQR` writer
pub type W = crate::W<JSQRrs>;
///Field `JL` reader - JL
pub type JL_R = crate::FieldReader;
///Field `JL` writer - JL
pub type JL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, u8, crate::Safe>;
/**JEXTSEL

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
    ///6: EXTI line 15
    Exti15 = 6,
    ///8: Timer 1 TRGO2 event
    Tim1Trgo2 = 8,
    ///9: HRTIM_ADCTRG2 event
    HrtimAdctrg2 = 9,
    ///10: HRTIM_ADCTRG4 event
    HrtimAdctrg4 = 10,
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
///Field `JEXTSEL` reader - JEXTSEL
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
    ///EXTI line 15
    #[inline(always)]
    pub fn is_exti15(&self) -> bool {
        *self == JEXTSEL::Exti15
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn is_tim1_trgo2(&self) -> bool {
        *self == JEXTSEL::Tim1Trgo2
    }
    ///HRTIM_ADCTRG2 event
    #[inline(always)]
    pub fn is_hrtim_adctrg2(&self) -> bool {
        *self == JEXTSEL::HrtimAdctrg2
    }
    ///HRTIM_ADCTRG4 event
    #[inline(always)]
    pub fn is_hrtim_adctrg4(&self) -> bool {
        *self == JEXTSEL::HrtimAdctrg4
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
}
///Field `JEXTSEL` writer - JEXTSEL
pub type JEXTSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, JEXTSEL>;
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
    ///EXTI line 15
    #[inline(always)]
    pub fn exti15(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Exti15)
    }
    ///Timer 1 TRGO2 event
    #[inline(always)]
    pub fn tim1_trgo2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::Tim1Trgo2)
    }
    ///HRTIM_ADCTRG2 event
    #[inline(always)]
    pub fn hrtim_adctrg2(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::HrtimAdctrg2)
    }
    ///HRTIM_ADCTRG4 event
    #[inline(always)]
    pub fn hrtim_adctrg4(self) -> &'a mut crate::W<REG> {
        self.variant(JEXTSEL::HrtimAdctrg4)
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
}
/**JEXTEN

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
///Field `JEXTEN` reader - JEXTEN
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
///Field `JEXTEN` writer - JEXTEN
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
    ///Bits 0:1 - JL
    #[inline(always)]
    pub fn jl(&self) -> JL_R {
        JL_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:5 - JEXTSEL
    #[inline(always)]
    pub fn jextsel(&self) -> JEXTSEL_R {
        JEXTSEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    ///Bits 6:7 - JEXTEN
    #[inline(always)]
    pub fn jexten(&self) -> JEXTEN_R {
        JEXTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///(1-4) conversion in injected sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `JSQ1` field.</div>
    #[inline(always)]
    pub fn jsq(&self, n: u8) -> JSQ_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        JSQ_R::new(((self.bits >> (n * 6 + 8)) & 0x1f) as u8)
    }
    ///Iterator for array of:
    ///(1-4) conversion in injected sequence
    #[inline(always)]
    pub fn jsq_iter(&self) -> impl Iterator<Item = JSQ_R> + '_ {
        (0..4).map(move |n| JSQ_R::new(((self.bits >> (n * 6 + 8)) & 0x1f) as u8))
    }
    ///Bits 8:12 - 1 conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 14:18 - 2 conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    ///Bits 20:24 - 3 conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    ///Bits 26:30 - 4 conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&self) -> JSQ_R {
        JSQ_R::new(((self.bits >> 26) & 0x1f) as u8)
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
    ///Bits 0:1 - JL
    #[inline(always)]
    pub fn jl(&mut self) -> JL_W<JSQRrs> {
        JL_W::new(self, 0)
    }
    ///Bits 2:5 - JEXTSEL
    #[inline(always)]
    pub fn jextsel(&mut self) -> JEXTSEL_W<JSQRrs> {
        JEXTSEL_W::new(self, 2)
    }
    ///Bits 6:7 - JEXTEN
    #[inline(always)]
    pub fn jexten(&mut self) -> JEXTEN_W<JSQRrs> {
        JEXTEN_W::new(self, 6)
    }
    ///(1-4) conversion in injected sequence
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `JSQ1` field.</div>
    #[inline(always)]
    pub fn jsq(&mut self, n: u8) -> JSQ_W<JSQRrs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        JSQ_W::new(self, n * 6 + 8)
    }
    ///Bits 8:12 - 1 conversion in injected sequence
    #[inline(always)]
    pub fn jsq1(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 8)
    }
    ///Bits 14:18 - 2 conversion in injected sequence
    #[inline(always)]
    pub fn jsq2(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 14)
    }
    ///Bits 20:24 - 3 conversion in injected sequence
    #[inline(always)]
    pub fn jsq3(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 20)
    }
    ///Bits 26:30 - 4 conversion in injected sequence
    #[inline(always)]
    pub fn jsq4(&mut self) -> JSQ_W<JSQRrs> {
        JSQ_W::new(self, 26)
    }
}
/**injected sequence register

You can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F302.html#ADC1:JSQR)*/
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
