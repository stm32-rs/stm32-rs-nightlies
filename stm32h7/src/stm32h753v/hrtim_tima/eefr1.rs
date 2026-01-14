///Register `EEFR1` reader
pub type R = crate::R<EEFR1rs>;
///Register `EEFR1` writer
pub type W = crate::W<EEFR1rs>;
/**External Event %s latch

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE1LTCH {
    ///0: Event is ignored if it happens during a blank, or passed through during a window
    Disabled = 0,
    ///1: Event is latched and delayed till the end of the blanking or windowing period
    Enabled = 1,
}
impl From<EE1LTCH> for bool {
    #[inline(always)]
    fn from(variant: EE1LTCH) -> Self {
        variant as u8 != 0
    }
}
///Field `EELTCH(1-5)` reader - External Event %s latch
pub type EELTCH_R = crate::BitReader<EE1LTCH>;
impl EELTCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE1LTCH {
        match self.bits {
            false => EE1LTCH::Disabled,
            true => EE1LTCH::Enabled,
        }
    }
    ///Event is ignored if it happens during a blank, or passed through during a window
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE1LTCH::Disabled
    }
    ///Event is latched and delayed till the end of the blanking or windowing period
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EE1LTCH::Enabled
    }
}
///Field `EELTCH(1-5)` writer - External Event %s latch
pub type EELTCH_W<'a, REG> = crate::BitWriter<'a, REG, EE1LTCH>;
impl<'a, REG> EELTCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event is ignored if it happens during a blank, or passed through during a window
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE1LTCH::Disabled)
    }
    ///Event is latched and delayed till the end of the blanking or windowing period
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE1LTCH::Enabled)
    }
}
/**External Event %s filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE1FLTR {
    ///0: No filtering
    Disabled = 0,
    ///1: Blanking from counter reset/roll-over to Compare 1
    BlankResetToCompare1 = 1,
    ///2: Blanking from counter reset/roll-over to Compare 2
    BlankResetToCompare2 = 2,
    ///3: Blanking from counter reset/roll-over to Compare 3
    BlankResetToCompare3 = 3,
    ///4: Blanking from counter reset/roll-over to Compare 4
    BlankResetToCompare4 = 4,
    ///5: Blanking from another timing unit: TIMFLTR1 source
    BlankTimfltr1 = 5,
    ///6: Blanking from another timing unit: TIMFLTR2 source
    BlankTimfltr2 = 6,
    ///7: Blanking from another timing unit: TIMFLTR3 source
    BlankTimfltr3 = 7,
    ///8: Blanking from another timing unit: TIMFLTR4 source
    BlankTimfltr4 = 8,
    ///9: Blanking from another timing unit: TIMFLTR5 source
    BlankTimfltr5 = 9,
    ///10: Blanking from another timing unit: TIMFLTR6 source
    BlankTimfltr6 = 10,
    ///11: Blanking from another timing unit: TIMFLTR7 source
    BlankTimfltr7 = 11,
    ///12: Blanking from another timing unit: TIMFLTR8 source
    BlankTimfltr8 = 12,
    ///13: Windowing from counter reset/roll-over to compare 2
    WindowResetToCompare2 = 13,
    ///14: Windowing from counter reset/roll-over to compare 3
    WindowResetToCompare3 = 14,
    ///15: Windowing from another timing unit: TIMWIN source
    WindowTimwin = 15,
}
impl From<EE1FLTR> for u8 {
    #[inline(always)]
    fn from(variant: EE1FLTR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EE1FLTR {
    type Ux = u8;
}
impl crate::IsEnum for EE1FLTR {}
///Field `EEFLTR(1-5)` reader - External Event %s filter
pub type EEFLTR_R = crate::FieldReader<EE1FLTR>;
impl EEFLTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE1FLTR {
        match self.bits {
            0 => EE1FLTR::Disabled,
            1 => EE1FLTR::BlankResetToCompare1,
            2 => EE1FLTR::BlankResetToCompare2,
            3 => EE1FLTR::BlankResetToCompare3,
            4 => EE1FLTR::BlankResetToCompare4,
            5 => EE1FLTR::BlankTimfltr1,
            6 => EE1FLTR::BlankTimfltr2,
            7 => EE1FLTR::BlankTimfltr3,
            8 => EE1FLTR::BlankTimfltr4,
            9 => EE1FLTR::BlankTimfltr5,
            10 => EE1FLTR::BlankTimfltr6,
            11 => EE1FLTR::BlankTimfltr7,
            12 => EE1FLTR::BlankTimfltr8,
            13 => EE1FLTR::WindowResetToCompare2,
            14 => EE1FLTR::WindowResetToCompare3,
            15 => EE1FLTR::WindowTimwin,
            _ => unreachable!(),
        }
    }
    ///No filtering
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE1FLTR::Disabled
    }
    ///Blanking from counter reset/roll-over to Compare 1
    #[inline(always)]
    pub fn is_blank_reset_to_compare1(&self) -> bool {
        *self == EE1FLTR::BlankResetToCompare1
    }
    ///Blanking from counter reset/roll-over to Compare 2
    #[inline(always)]
    pub fn is_blank_reset_to_compare2(&self) -> bool {
        *self == EE1FLTR::BlankResetToCompare2
    }
    ///Blanking from counter reset/roll-over to Compare 3
    #[inline(always)]
    pub fn is_blank_reset_to_compare3(&self) -> bool {
        *self == EE1FLTR::BlankResetToCompare3
    }
    ///Blanking from counter reset/roll-over to Compare 4
    #[inline(always)]
    pub fn is_blank_reset_to_compare4(&self) -> bool {
        *self == EE1FLTR::BlankResetToCompare4
    }
    ///Blanking from another timing unit: TIMFLTR1 source
    #[inline(always)]
    pub fn is_blank_timfltr1(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr1
    }
    ///Blanking from another timing unit: TIMFLTR2 source
    #[inline(always)]
    pub fn is_blank_timfltr2(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr2
    }
    ///Blanking from another timing unit: TIMFLTR3 source
    #[inline(always)]
    pub fn is_blank_timfltr3(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr3
    }
    ///Blanking from another timing unit: TIMFLTR4 source
    #[inline(always)]
    pub fn is_blank_timfltr4(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr4
    }
    ///Blanking from another timing unit: TIMFLTR5 source
    #[inline(always)]
    pub fn is_blank_timfltr5(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr5
    }
    ///Blanking from another timing unit: TIMFLTR6 source
    #[inline(always)]
    pub fn is_blank_timfltr6(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr6
    }
    ///Blanking from another timing unit: TIMFLTR7 source
    #[inline(always)]
    pub fn is_blank_timfltr7(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr7
    }
    ///Blanking from another timing unit: TIMFLTR8 source
    #[inline(always)]
    pub fn is_blank_timfltr8(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr8
    }
    ///Windowing from counter reset/roll-over to compare 2
    #[inline(always)]
    pub fn is_window_reset_to_compare2(&self) -> bool {
        *self == EE1FLTR::WindowResetToCompare2
    }
    ///Windowing from counter reset/roll-over to compare 3
    #[inline(always)]
    pub fn is_window_reset_to_compare3(&self) -> bool {
        *self == EE1FLTR::WindowResetToCompare3
    }
    ///Windowing from another timing unit: TIMWIN source
    #[inline(always)]
    pub fn is_window_timwin(&self) -> bool {
        *self == EE1FLTR::WindowTimwin
    }
}
///Field `EEFLTR(1-5)` writer - External Event %s filter
pub type EEFLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EE1FLTR, crate::Safe>;
impl<'a, REG> EEFLTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filtering
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::Disabled)
    }
    ///Blanking from counter reset/roll-over to Compare 1
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankResetToCompare1)
    }
    ///Blanking from counter reset/roll-over to Compare 2
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankResetToCompare2)
    }
    ///Blanking from counter reset/roll-over to Compare 3
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankResetToCompare3)
    }
    ///Blanking from counter reset/roll-over to Compare 4
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankResetToCompare4)
    }
    ///Blanking from another timing unit: TIMFLTR1 source
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr1)
    }
    ///Blanking from another timing unit: TIMFLTR2 source
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr2)
    }
    ///Blanking from another timing unit: TIMFLTR3 source
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr3)
    }
    ///Blanking from another timing unit: TIMFLTR4 source
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr4)
    }
    ///Blanking from another timing unit: TIMFLTR5 source
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr5)
    }
    ///Blanking from another timing unit: TIMFLTR6 source
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr6)
    }
    ///Blanking from another timing unit: TIMFLTR7 source
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr7)
    }
    ///Blanking from another timing unit: TIMFLTR8 source
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr8)
    }
    ///Windowing from counter reset/roll-over to compare 2
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::WindowResetToCompare2)
    }
    ///Windowing from counter reset/roll-over to compare 3
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::WindowResetToCompare3)
    }
    ///Windowing from another timing unit: TIMWIN source
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::WindowTimwin)
    }
}
impl R {
    ///External Event (1-5) latch
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1LTCH` field.</div>
    #[inline(always)]
    pub fn eeltch(&self, n: u8) -> EELTCH_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EELTCH_R::new(((self.bits >> (n * 6)) & 1) != 0)
    }
    ///Iterator for array of:
    ///External Event (1-5) latch
    #[inline(always)]
    pub fn eeltch_iter(&self) -> impl Iterator<Item = EELTCH_R> + '_ {
        (0..5).map(move |n| EELTCH_R::new(((self.bits >> (n * 6)) & 1) != 0))
    }
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    pub fn ee1ltch(&self) -> EELTCH_R {
        EELTCH_R::new((self.bits & 1) != 0)
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    pub fn ee2ltch(&self) -> EELTCH_R {
        EELTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    pub fn ee3ltch(&self) -> EELTCH_R {
        EELTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    pub fn ee4ltch(&self) -> EELTCH_R {
        EELTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    pub fn ee5ltch(&self) -> EELTCH_R {
        EELTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///External Event (1-5) filter
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1FLTR` field.</div>
    #[inline(always)]
    pub fn eefltr(&self, n: u8) -> EEFLTR_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEFLTR_R::new(((self.bits >> (n * 6 + 1)) & 0x0f) as u8)
    }
    ///Iterator for array of:
    ///External Event (1-5) filter
    #[inline(always)]
    pub fn eefltr_iter(&self) -> impl Iterator<Item = EEFLTR_R> + '_ {
        (0..5).map(move |n| EEFLTR_R::new(((self.bits >> (n * 6 + 1)) & 0x0f) as u8))
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    pub fn ee1fltr(&self) -> EEFLTR_R {
        EEFLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    pub fn ee2fltr(&self) -> EEFLTR_R {
        EEFLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    pub fn ee3fltr(&self) -> EEFLTR_R {
        EEFLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    pub fn ee4fltr(&self) -> EEFLTR_R {
        EEFLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    pub fn ee5fltr(&self) -> EEFLTR_R {
        EEFLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EEFR1")
            .field("ee1fltr", &self.ee1fltr())
            .field("ee2fltr", &self.ee2fltr())
            .field("ee3fltr", &self.ee3fltr())
            .field("ee4fltr", &self.ee4fltr())
            .field("ee5fltr", &self.ee5fltr())
            .field("ee1ltch", &self.ee1ltch())
            .field("ee2ltch", &self.ee2ltch())
            .field("ee3ltch", &self.ee3ltch())
            .field("ee4ltch", &self.ee4ltch())
            .field("ee5ltch", &self.ee5ltch())
            .finish()
    }
}
impl W {
    ///External Event (1-5) latch
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1LTCH` field.</div>
    #[inline(always)]
    pub fn eeltch(&mut self, n: u8) -> EELTCH_W<'_, EEFR1rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EELTCH_W::new(self, n * 6)
    }
    ///Bit 0 - External Event 1 latch
    #[inline(always)]
    pub fn ee1ltch(&mut self) -> EELTCH_W<'_, EEFR1rs> {
        EELTCH_W::new(self, 0)
    }
    ///Bit 6 - External Event 2 latch
    #[inline(always)]
    pub fn ee2ltch(&mut self) -> EELTCH_W<'_, EEFR1rs> {
        EELTCH_W::new(self, 6)
    }
    ///Bit 12 - External Event 3 latch
    #[inline(always)]
    pub fn ee3ltch(&mut self) -> EELTCH_W<'_, EEFR1rs> {
        EELTCH_W::new(self, 12)
    }
    ///Bit 18 - External Event 4 latch
    #[inline(always)]
    pub fn ee4ltch(&mut self) -> EELTCH_W<'_, EEFR1rs> {
        EELTCH_W::new(self, 18)
    }
    ///Bit 24 - External Event 5 latch
    #[inline(always)]
    pub fn ee5ltch(&mut self) -> EELTCH_W<'_, EEFR1rs> {
        EELTCH_W::new(self, 24)
    }
    ///External Event (1-5) filter
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `EE1FLTR` field.</div>
    #[inline(always)]
    pub fn eefltr(&mut self, n: u8) -> EEFLTR_W<'_, EEFR1rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        EEFLTR_W::new(self, n * 6 + 1)
    }
    ///Bits 1:4 - External Event 1 filter
    #[inline(always)]
    pub fn ee1fltr(&mut self) -> EEFLTR_W<'_, EEFR1rs> {
        EEFLTR_W::new(self, 1)
    }
    ///Bits 7:10 - External Event 2 filter
    #[inline(always)]
    pub fn ee2fltr(&mut self) -> EEFLTR_W<'_, EEFR1rs> {
        EEFLTR_W::new(self, 7)
    }
    ///Bits 13:16 - External Event 3 filter
    #[inline(always)]
    pub fn ee3fltr(&mut self) -> EEFLTR_W<'_, EEFR1rs> {
        EEFLTR_W::new(self, 13)
    }
    ///Bits 19:22 - External Event 4 filter
    #[inline(always)]
    pub fn ee4fltr(&mut self) -> EEFLTR_W<'_, EEFR1rs> {
        EEFLTR_W::new(self, 19)
    }
    ///Bits 25:28 - External Event 5 filter
    #[inline(always)]
    pub fn ee5fltr(&mut self) -> EEFLTR_W<'_, EEFR1rs> {
        EEFLTR_W::new(self, 25)
    }
}
/**Timerx External Event Filtering Register 1

You can [`read`](crate::Reg::read) this register and get [`eefr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#HRTIM_TIMA:EEFR1)*/
pub struct EEFR1rs;
impl crate::RegisterSpec for EEFR1rs {
    type Ux = u32;
}
///`read()` method returns [`eefr1::R`](R) reader structure
impl crate::Readable for EEFR1rs {}
///`write(|w| ..)` method takes [`eefr1::W`](W) writer structure
impl crate::Writable for EEFR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EEFR1 to value 0
impl crate::Resettable for EEFR1rs {}
