///Register `STMODR` reader
pub type R = crate::R<STMODRrs>;
///Register `STMODR` writer
pub type W = crate::W<STMODRrs>;
/**DAC Channel 1 Sawtooth Reset trigger selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STRSTTRIGSEL1 {
    ///0: Software trigger
    Swtrig = 0,
    ///1: Timer 8 (DAC1
    Tim1or8trgo = 1,
    ///2: Timer 7 TRGO event
    Tim7trgo = 2,
    ///3: Timer 15 TRGO event
    Tim15trgo = 3,
    ///4: Timer 2 TRGO event
    Tim2trgo = 4,
    ///5: Timer 4 TRGO event
    Tim4trgo = 5,
    ///6: EXTI line 9
    Exti9 = 6,
    ///7: Timer 6 TRGO event
    Tim6trgo = 7,
    ///8: Timer 3 TRGO event
    Tim3trgo = 8,
    ///9: hrtim_dac_reset_trg1
    HrtimDacReset1 = 9,
    ///10: hrtim_dac_reset_trg2
    HrtimDacReset2 = 10,
    ///11: hrtim_dac_reset_trg3
    HrtimDacReset3 = 11,
    ///12: hrtim_dac_reset_trg4
    HrtimDacReset4 = 12,
    ///13: hrtim_dac_reset_trg5
    HrtimDacReset5 = 13,
    ///14: hrtim_dac_reset_trg6
    HrtimDacReset6 = 14,
    ///15: hrtim_dac_trg1 (DAC1
    HrtimDacX = 15,
}
impl From<STRSTTRIGSEL1> for u8 {
    #[inline(always)]
    fn from(variant: STRSTTRIGSEL1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STRSTTRIGSEL1 {
    type Ux = u8;
}
impl crate::IsEnum for STRSTTRIGSEL1 {}
///Field `STRSTTRIGSEL(1-2)` reader - DAC Channel 1 Sawtooth Reset trigger selection
pub type STRSTTRIGSEL_R = crate::FieldReader<STRSTTRIGSEL1>;
impl STRSTTRIGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STRSTTRIGSEL1 {
        match self.bits {
            0 => STRSTTRIGSEL1::Swtrig,
            1 => STRSTTRIGSEL1::Tim1or8trgo,
            2 => STRSTTRIGSEL1::Tim7trgo,
            3 => STRSTTRIGSEL1::Tim15trgo,
            4 => STRSTTRIGSEL1::Tim2trgo,
            5 => STRSTTRIGSEL1::Tim4trgo,
            6 => STRSTTRIGSEL1::Exti9,
            7 => STRSTTRIGSEL1::Tim6trgo,
            8 => STRSTTRIGSEL1::Tim3trgo,
            9 => STRSTTRIGSEL1::HrtimDacReset1,
            10 => STRSTTRIGSEL1::HrtimDacReset2,
            11 => STRSTTRIGSEL1::HrtimDacReset3,
            12 => STRSTTRIGSEL1::HrtimDacReset4,
            13 => STRSTTRIGSEL1::HrtimDacReset5,
            14 => STRSTTRIGSEL1::HrtimDacReset6,
            15 => STRSTTRIGSEL1::HrtimDacX,
            _ => unreachable!(),
        }
    }
    ///Software trigger
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        *self == STRSTTRIGSEL1::Swtrig
    }
    ///Timer 8 (DAC1
    #[inline(always)]
    pub fn is_tim1or8trgo(&self) -> bool {
        *self == STRSTTRIGSEL1::Tim1or8trgo
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn is_tim7trgo(&self) -> bool {
        *self == STRSTTRIGSEL1::Tim7trgo
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn is_tim15trgo(&self) -> bool {
        *self == STRSTTRIGSEL1::Tim15trgo
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == STRSTTRIGSEL1::Tim2trgo
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == STRSTTRIGSEL1::Tim4trgo
    }
    ///EXTI line 9
    #[inline(always)]
    pub fn is_exti9(&self) -> bool {
        *self == STRSTTRIGSEL1::Exti9
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn is_tim6trgo(&self) -> bool {
        *self == STRSTTRIGSEL1::Tim6trgo
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn is_tim3trgo(&self) -> bool {
        *self == STRSTTRIGSEL1::Tim3trgo
    }
    ///hrtim_dac_reset_trg1
    #[inline(always)]
    pub fn is_hrtim_dac_reset1(&self) -> bool {
        *self == STRSTTRIGSEL1::HrtimDacReset1
    }
    ///hrtim_dac_reset_trg2
    #[inline(always)]
    pub fn is_hrtim_dac_reset2(&self) -> bool {
        *self == STRSTTRIGSEL1::HrtimDacReset2
    }
    ///hrtim_dac_reset_trg3
    #[inline(always)]
    pub fn is_hrtim_dac_reset3(&self) -> bool {
        *self == STRSTTRIGSEL1::HrtimDacReset3
    }
    ///hrtim_dac_reset_trg4
    #[inline(always)]
    pub fn is_hrtim_dac_reset4(&self) -> bool {
        *self == STRSTTRIGSEL1::HrtimDacReset4
    }
    ///hrtim_dac_reset_trg5
    #[inline(always)]
    pub fn is_hrtim_dac_reset5(&self) -> bool {
        *self == STRSTTRIGSEL1::HrtimDacReset5
    }
    ///hrtim_dac_reset_trg6
    #[inline(always)]
    pub fn is_hrtim_dac_reset6(&self) -> bool {
        *self == STRSTTRIGSEL1::HrtimDacReset6
    }
    ///hrtim_dac_trg1 (DAC1
    #[inline(always)]
    pub fn is_hrtim_dac_x(&self) -> bool {
        *self == STRSTTRIGSEL1::HrtimDacX
    }
}
///Field `STRSTTRIGSEL(1-2)` writer - DAC Channel 1 Sawtooth Reset trigger selection
pub type STRSTTRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, STRSTTRIGSEL1, crate::Safe>;
impl<'a, REG> STRSTTRIGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Software trigger
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::Swtrig)
    }
    ///Timer 8 (DAC1
    #[inline(always)]
    pub fn tim1or8trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::Tim1or8trgo)
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn tim7trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::Tim7trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::Tim15trgo)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::Tim2trgo)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::Tim4trgo)
    }
    ///EXTI line 9
    #[inline(always)]
    pub fn exti9(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::Exti9)
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::Tim6trgo)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::Tim3trgo)
    }
    ///hrtim_dac_reset_trg1
    #[inline(always)]
    pub fn hrtim_dac_reset1(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::HrtimDacReset1)
    }
    ///hrtim_dac_reset_trg2
    #[inline(always)]
    pub fn hrtim_dac_reset2(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::HrtimDacReset2)
    }
    ///hrtim_dac_reset_trg3
    #[inline(always)]
    pub fn hrtim_dac_reset3(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::HrtimDacReset3)
    }
    ///hrtim_dac_reset_trg4
    #[inline(always)]
    pub fn hrtim_dac_reset4(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::HrtimDacReset4)
    }
    ///hrtim_dac_reset_trg5
    #[inline(always)]
    pub fn hrtim_dac_reset5(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::HrtimDacReset5)
    }
    ///hrtim_dac_reset_trg6
    #[inline(always)]
    pub fn hrtim_dac_reset6(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::HrtimDacReset6)
    }
    ///hrtim_dac_trg1 (DAC1
    #[inline(always)]
    pub fn hrtim_dac_x(self) -> &'a mut crate::W<REG> {
        self.variant(STRSTTRIGSEL1::HrtimDacX)
    }
}
/**DAC Channel %s Sawtooth Increment trigger selection

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum STINCTRIGSEL1 {
    ///0: Software sawtooth increment trigger
    Swtrig = 0,
    ///1: Timer 8 (DAC1
    Tim1or8trgo = 1,
    ///2: Timer 7 TRGO event
    Tim7trgo = 2,
    ///3: Timer 15 TRGO event
    Tim15trgo = 3,
    ///4: Timer 2 TRGO event
    Tim2trgo = 4,
    ///5: Timer 4 TRGO event
    Tim4trgo = 5,
    ///6: EXTI line 10
    Exti10 = 6,
    ///7: Timer 6 TRGO event
    Tim6trgo = 7,
    ///8: Timer 3 TRGO event
    Tim3trgo = 8,
    ///9: hrtim_dac_step_trg1
    HrtimDacStep1 = 9,
    ///10: hrtim_dac_step_trg2
    HrtimDacStep2 = 10,
    ///11: hrtim_dac_step_trg3
    HrtimDacStep3 = 11,
    ///12: hrtim_dac_step_trg4
    HrtimDacStep4 = 12,
    ///13: hrtim_dac_step_trg5
    HrtimDacStep5 = 13,
    ///14: hrtim_dac_step_trg6
    HrtimDacStep6 = 14,
}
impl From<STINCTRIGSEL1> for u8 {
    #[inline(always)]
    fn from(variant: STINCTRIGSEL1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for STINCTRIGSEL1 {
    type Ux = u8;
}
impl crate::IsEnum for STINCTRIGSEL1 {}
///Field `STINCTRIGSEL(1-2)` reader - DAC Channel %s Sawtooth Increment trigger selection
pub type STINCTRIGSEL_R = crate::FieldReader<STINCTRIGSEL1>;
impl STINCTRIGSEL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<STINCTRIGSEL1> {
        match self.bits {
            0 => Some(STINCTRIGSEL1::Swtrig),
            1 => Some(STINCTRIGSEL1::Tim1or8trgo),
            2 => Some(STINCTRIGSEL1::Tim7trgo),
            3 => Some(STINCTRIGSEL1::Tim15trgo),
            4 => Some(STINCTRIGSEL1::Tim2trgo),
            5 => Some(STINCTRIGSEL1::Tim4trgo),
            6 => Some(STINCTRIGSEL1::Exti10),
            7 => Some(STINCTRIGSEL1::Tim6trgo),
            8 => Some(STINCTRIGSEL1::Tim3trgo),
            9 => Some(STINCTRIGSEL1::HrtimDacStep1),
            10 => Some(STINCTRIGSEL1::HrtimDacStep2),
            11 => Some(STINCTRIGSEL1::HrtimDacStep3),
            12 => Some(STINCTRIGSEL1::HrtimDacStep4),
            13 => Some(STINCTRIGSEL1::HrtimDacStep5),
            14 => Some(STINCTRIGSEL1::HrtimDacStep6),
            _ => None,
        }
    }
    ///Software sawtooth increment trigger
    #[inline(always)]
    pub fn is_swtrig(&self) -> bool {
        *self == STINCTRIGSEL1::Swtrig
    }
    ///Timer 8 (DAC1
    #[inline(always)]
    pub fn is_tim1or8trgo(&self) -> bool {
        *self == STINCTRIGSEL1::Tim1or8trgo
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn is_tim7trgo(&self) -> bool {
        *self == STINCTRIGSEL1::Tim7trgo
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn is_tim15trgo(&self) -> bool {
        *self == STINCTRIGSEL1::Tim15trgo
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn is_tim2trgo(&self) -> bool {
        *self == STINCTRIGSEL1::Tim2trgo
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn is_tim4trgo(&self) -> bool {
        *self == STINCTRIGSEL1::Tim4trgo
    }
    ///EXTI line 10
    #[inline(always)]
    pub fn is_exti10(&self) -> bool {
        *self == STINCTRIGSEL1::Exti10
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn is_tim6trgo(&self) -> bool {
        *self == STINCTRIGSEL1::Tim6trgo
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn is_tim3trgo(&self) -> bool {
        *self == STINCTRIGSEL1::Tim3trgo
    }
    ///hrtim_dac_step_trg1
    #[inline(always)]
    pub fn is_hrtim_dac_step1(&self) -> bool {
        *self == STINCTRIGSEL1::HrtimDacStep1
    }
    ///hrtim_dac_step_trg2
    #[inline(always)]
    pub fn is_hrtim_dac_step2(&self) -> bool {
        *self == STINCTRIGSEL1::HrtimDacStep2
    }
    ///hrtim_dac_step_trg3
    #[inline(always)]
    pub fn is_hrtim_dac_step3(&self) -> bool {
        *self == STINCTRIGSEL1::HrtimDacStep3
    }
    ///hrtim_dac_step_trg4
    #[inline(always)]
    pub fn is_hrtim_dac_step4(&self) -> bool {
        *self == STINCTRIGSEL1::HrtimDacStep4
    }
    ///hrtim_dac_step_trg5
    #[inline(always)]
    pub fn is_hrtim_dac_step5(&self) -> bool {
        *self == STINCTRIGSEL1::HrtimDacStep5
    }
    ///hrtim_dac_step_trg6
    #[inline(always)]
    pub fn is_hrtim_dac_step6(&self) -> bool {
        *self == STINCTRIGSEL1::HrtimDacStep6
    }
}
///Field `STINCTRIGSEL(1-2)` writer - DAC Channel %s Sawtooth Increment trigger selection
pub type STINCTRIGSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4, STINCTRIGSEL1>;
impl<'a, REG> STINCTRIGSEL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Software sawtooth increment trigger
    #[inline(always)]
    pub fn swtrig(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::Swtrig)
    }
    ///Timer 8 (DAC1
    #[inline(always)]
    pub fn tim1or8trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::Tim1or8trgo)
    }
    ///Timer 7 TRGO event
    #[inline(always)]
    pub fn tim7trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::Tim7trgo)
    }
    ///Timer 15 TRGO event
    #[inline(always)]
    pub fn tim15trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::Tim15trgo)
    }
    ///Timer 2 TRGO event
    #[inline(always)]
    pub fn tim2trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::Tim2trgo)
    }
    ///Timer 4 TRGO event
    #[inline(always)]
    pub fn tim4trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::Tim4trgo)
    }
    ///EXTI line 10
    #[inline(always)]
    pub fn exti10(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::Exti10)
    }
    ///Timer 6 TRGO event
    #[inline(always)]
    pub fn tim6trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::Tim6trgo)
    }
    ///Timer 3 TRGO event
    #[inline(always)]
    pub fn tim3trgo(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::Tim3trgo)
    }
    ///hrtim_dac_step_trg1
    #[inline(always)]
    pub fn hrtim_dac_step1(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::HrtimDacStep1)
    }
    ///hrtim_dac_step_trg2
    #[inline(always)]
    pub fn hrtim_dac_step2(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::HrtimDacStep2)
    }
    ///hrtim_dac_step_trg3
    #[inline(always)]
    pub fn hrtim_dac_step3(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::HrtimDacStep3)
    }
    ///hrtim_dac_step_trg4
    #[inline(always)]
    pub fn hrtim_dac_step4(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::HrtimDacStep4)
    }
    ///hrtim_dac_step_trg5
    #[inline(always)]
    pub fn hrtim_dac_step5(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::HrtimDacStep5)
    }
    ///hrtim_dac_step_trg6
    #[inline(always)]
    pub fn hrtim_dac_step6(self) -> &'a mut crate::W<REG> {
        self.variant(STINCTRIGSEL1::HrtimDacStep6)
    }
}
impl R {
    ///DAC Channel 1 Sawtooth Reset trigger selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `STRSTTRIGSEL1` field.</div>
    #[inline(always)]
    pub fn strsttrigsel(&self, n: u8) -> STRSTTRIGSEL_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        STRSTTRIGSEL_R::new(((self.bits >> (n * 16)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///DAC Channel 1 Sawtooth Reset trigger selection
    #[inline(always)]
    pub fn strsttrigsel_iter(&self) -> impl Iterator<Item = STRSTTRIGSEL_R> + '_ {
        (0..2).map(move |n| STRSTTRIGSEL_R::new(((self.bits >> (n * 16)) & 0x0f) as u8))
    }
    ///Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection
    #[inline(always)]
    pub fn strsttrigsel1(&self) -> STRSTTRIGSEL_R {
        STRSTTRIGSEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection
    #[inline(always)]
    pub fn strsttrigsel2(&self) -> STRSTTRIGSEL_R {
        STRSTTRIGSEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///DAC Channel (1-2) Sawtooth Increment trigger selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `STINCTRIGSEL1` field.</div>
    #[inline(always)]
    pub fn stinctrigsel(&self, n: u8) -> STINCTRIGSEL_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        STINCTRIGSEL_R::new(((self.bits >> (n * 16 + 8)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///DAC Channel (1-2) Sawtooth Increment trigger selection
    #[inline(always)]
    pub fn stinctrigsel_iter(&self) -> impl Iterator<Item = STINCTRIGSEL_R> + '_ {
        (0..2).map(move |n| STINCTRIGSEL_R::new(((self.bits >> (n * 16 + 8)) & 0x0f) as u8))
    }
    ///Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection
    #[inline(always)]
    pub fn stinctrigsel1(&self) -> STINCTRIGSEL_R {
        STINCTRIGSEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection
    #[inline(always)]
    pub fn stinctrigsel2(&self) -> STINCTRIGSEL_R {
        STINCTRIGSEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STMODR")
            .field("strsttrigsel1", &self.strsttrigsel1())
            .field("strsttrigsel2", &self.strsttrigsel2())
            .field("stinctrigsel1", &self.stinctrigsel1())
            .field("stinctrigsel2", &self.stinctrigsel2())
            .finish()
    }
}
impl W {
    ///DAC Channel 1 Sawtooth Reset trigger selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `STRSTTRIGSEL1` field.</div>
    #[inline(always)]
    pub fn strsttrigsel(&mut self, n: u8) -> STRSTTRIGSEL_W<'_, STMODRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        STRSTTRIGSEL_W::new(self, n * 16)
    }
    ///Bits 0:3 - DAC Channel 1 Sawtooth Reset trigger selection
    #[inline(always)]
    pub fn strsttrigsel1(&mut self) -> STRSTTRIGSEL_W<'_, STMODRrs> {
        STRSTTRIGSEL_W::new(self, 0)
    }
    ///Bits 16:19 - DAC Channel 1 Sawtooth Reset trigger selection
    #[inline(always)]
    pub fn strsttrigsel2(&mut self) -> STRSTTRIGSEL_W<'_, STMODRrs> {
        STRSTTRIGSEL_W::new(self, 16)
    }
    ///DAC Channel (1-2) Sawtooth Increment trigger selection
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `STINCTRIGSEL1` field.</div>
    #[inline(always)]
    pub fn stinctrigsel(&mut self, n: u8) -> STINCTRIGSEL_W<'_, STMODRrs> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        STINCTRIGSEL_W::new(self, n * 16 + 8)
    }
    ///Bits 8:11 - DAC Channel 1 Sawtooth Increment trigger selection
    #[inline(always)]
    pub fn stinctrigsel1(&mut self) -> STINCTRIGSEL_W<'_, STMODRrs> {
        STINCTRIGSEL_W::new(self, 8)
    }
    ///Bits 24:27 - DAC Channel 2 Sawtooth Increment trigger selection
    #[inline(always)]
    pub fn stinctrigsel2(&mut self) -> STINCTRIGSEL_W<'_, STMODRrs> {
        STINCTRIGSEL_W::new(self, 24)
    }
}
/**Sawtooth Mode register

You can [`read`](crate::Reg::read) this register and get [`stmodr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stmodr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G474.html#DAC1:STMODR)*/
pub struct STMODRrs;
impl crate::RegisterSpec for STMODRrs {
    type Ux = u32;
}
///`read()` method returns [`stmodr::R`](R) reader structure
impl crate::Readable for STMODRrs {}
///`write(|w| ..)` method takes [`stmodr::W`](W) writer structure
impl crate::Writable for STMODRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets STMODR to value 0
impl crate::Resettable for STMODRrs {}
