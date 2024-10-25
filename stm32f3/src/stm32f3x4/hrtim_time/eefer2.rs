///Register `EEFER2` reader
pub type R = crate::R<EEFER2rs>;
///Register `EEFER2` writer
pub type W = crate::W<EEFER2rs>;
/**External Event 6 latch

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE6LTCH {
    ///0: Event is ignored if it happens during a blank, or passed through during a window
    Disabled = 0,
    ///1: Event is latched and delayed till the end of the blanking or windowing period
    Enabled = 1,
}
impl From<EE6LTCH> for bool {
    #[inline(always)]
    fn from(variant: EE6LTCH) -> Self {
        variant as u8 != 0
    }
}
///Field `EE6LTCH` reader - External Event 6 latch
pub type EE6LTCH_R = crate::BitReader<EE6LTCH>;
impl EE6LTCH_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE6LTCH {
        match self.bits {
            false => EE6LTCH::Disabled,
            true => EE6LTCH::Enabled,
        }
    }
    ///Event is ignored if it happens during a blank, or passed through during a window
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6LTCH::Disabled
    }
    ///Event is latched and delayed till the end of the blanking or windowing period
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EE6LTCH::Enabled
    }
}
///Field `EE6LTCH` writer - External Event 6 latch
pub type EE6LTCH_W<'a, REG> = crate::BitWriter<'a, REG, EE6LTCH>;
impl<'a, REG> EE6LTCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Event is ignored if it happens during a blank, or passed through during a window
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE6LTCH::Disabled)
    }
    ///Event is latched and delayed till the end of the blanking or windowing period
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE6LTCH::Enabled)
    }
}
/**External Event 6 filter

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE6FLTR {
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
impl From<EE6FLTR> for u8 {
    #[inline(always)]
    fn from(variant: EE6FLTR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EE6FLTR {
    type Ux = u8;
}
impl crate::IsEnum for EE6FLTR {}
///Field `EE6FLTR` reader - External Event 6 filter
pub type EE6FLTR_R = crate::FieldReader<EE6FLTR>;
impl EE6FLTR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EE6FLTR {
        match self.bits {
            0 => EE6FLTR::Disabled,
            1 => EE6FLTR::BlankResetToCompare1,
            2 => EE6FLTR::BlankResetToCompare2,
            3 => EE6FLTR::BlankResetToCompare3,
            4 => EE6FLTR::BlankResetToCompare4,
            5 => EE6FLTR::BlankTimfltr1,
            6 => EE6FLTR::BlankTimfltr2,
            7 => EE6FLTR::BlankTimfltr3,
            8 => EE6FLTR::BlankTimfltr4,
            9 => EE6FLTR::BlankTimfltr5,
            10 => EE6FLTR::BlankTimfltr6,
            11 => EE6FLTR::BlankTimfltr7,
            12 => EE6FLTR::BlankTimfltr8,
            13 => EE6FLTR::WindowResetToCompare2,
            14 => EE6FLTR::WindowResetToCompare3,
            15 => EE6FLTR::WindowTimwin,
            _ => unreachable!(),
        }
    }
    ///No filtering
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6FLTR::Disabled
    }
    ///Blanking from counter reset/roll-over to Compare 1
    #[inline(always)]
    pub fn is_blank_reset_to_compare1(&self) -> bool {
        *self == EE6FLTR::BlankResetToCompare1
    }
    ///Blanking from counter reset/roll-over to Compare 2
    #[inline(always)]
    pub fn is_blank_reset_to_compare2(&self) -> bool {
        *self == EE6FLTR::BlankResetToCompare2
    }
    ///Blanking from counter reset/roll-over to Compare 3
    #[inline(always)]
    pub fn is_blank_reset_to_compare3(&self) -> bool {
        *self == EE6FLTR::BlankResetToCompare3
    }
    ///Blanking from counter reset/roll-over to Compare 4
    #[inline(always)]
    pub fn is_blank_reset_to_compare4(&self) -> bool {
        *self == EE6FLTR::BlankResetToCompare4
    }
    ///Blanking from another timing unit: TIMFLTR1 source
    #[inline(always)]
    pub fn is_blank_timfltr1(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr1
    }
    ///Blanking from another timing unit: TIMFLTR2 source
    #[inline(always)]
    pub fn is_blank_timfltr2(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr2
    }
    ///Blanking from another timing unit: TIMFLTR3 source
    #[inline(always)]
    pub fn is_blank_timfltr3(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr3
    }
    ///Blanking from another timing unit: TIMFLTR4 source
    #[inline(always)]
    pub fn is_blank_timfltr4(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr4
    }
    ///Blanking from another timing unit: TIMFLTR5 source
    #[inline(always)]
    pub fn is_blank_timfltr5(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr5
    }
    ///Blanking from another timing unit: TIMFLTR6 source
    #[inline(always)]
    pub fn is_blank_timfltr6(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr6
    }
    ///Blanking from another timing unit: TIMFLTR7 source
    #[inline(always)]
    pub fn is_blank_timfltr7(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr7
    }
    ///Blanking from another timing unit: TIMFLTR8 source
    #[inline(always)]
    pub fn is_blank_timfltr8(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr8
    }
    ///Windowing from counter reset/roll-over to compare 2
    #[inline(always)]
    pub fn is_window_reset_to_compare2(&self) -> bool {
        *self == EE6FLTR::WindowResetToCompare2
    }
    ///Windowing from counter reset/roll-over to compare 3
    #[inline(always)]
    pub fn is_window_reset_to_compare3(&self) -> bool {
        *self == EE6FLTR::WindowResetToCompare3
    }
    ///Windowing from another timing unit: TIMWIN source
    #[inline(always)]
    pub fn is_window_timwin(&self) -> bool {
        *self == EE6FLTR::WindowTimwin
    }
}
///Field `EE6FLTR` writer - External Event 6 filter
pub type EE6FLTR_W<'a, REG> = crate::FieldWriter<'a, REG, 4, EE6FLTR, crate::Safe>;
impl<'a, REG> EE6FLTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///No filtering
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::Disabled)
    }
    ///Blanking from counter reset/roll-over to Compare 1
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankResetToCompare1)
    }
    ///Blanking from counter reset/roll-over to Compare 2
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankResetToCompare2)
    }
    ///Blanking from counter reset/roll-over to Compare 3
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankResetToCompare3)
    }
    ///Blanking from counter reset/roll-over to Compare 4
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankResetToCompare4)
    }
    ///Blanking from another timing unit: TIMFLTR1 source
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr1)
    }
    ///Blanking from another timing unit: TIMFLTR2 source
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr2)
    }
    ///Blanking from another timing unit: TIMFLTR3 source
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr3)
    }
    ///Blanking from another timing unit: TIMFLTR4 source
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr4)
    }
    ///Blanking from another timing unit: TIMFLTR5 source
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr5)
    }
    ///Blanking from another timing unit: TIMFLTR6 source
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr6)
    }
    ///Blanking from another timing unit: TIMFLTR7 source
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr7)
    }
    ///Blanking from another timing unit: TIMFLTR8 source
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr8)
    }
    ///Windowing from counter reset/roll-over to compare 2
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::WindowResetToCompare2)
    }
    ///Windowing from counter reset/roll-over to compare 3
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::WindowResetToCompare3)
    }
    ///Windowing from another timing unit: TIMWIN source
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::WindowTimwin)
    }
}
///Field `EE7FLTR` reader - External Event 7 filter
pub use EE6FLTR_R as EE7FLTR_R;
///Field `EE8FLTR` reader - External Event 8 filter
pub use EE6FLTR_R as EE8FLTR_R;
///Field `EE9FLTR` reader - External Event 9 filter
pub use EE6FLTR_R as EE9FLTR_R;
///Field `EE10FLTR` reader - External Event 10 filter
pub use EE6FLTR_R as EE10FLTR_R;
///Field `EE7FLTR` writer - External Event 7 filter
pub use EE6FLTR_W as EE7FLTR_W;
///Field `EE8FLTR` writer - External Event 8 filter
pub use EE6FLTR_W as EE8FLTR_W;
///Field `EE9FLTR` writer - External Event 9 filter
pub use EE6FLTR_W as EE9FLTR_W;
///Field `EE10FLTR` writer - External Event 10 filter
pub use EE6FLTR_W as EE10FLTR_W;
///Field `EE7LTCH` reader - External Event 7 latch
pub use EE6LTCH_R as EE7LTCH_R;
///Field `EE8LTCH` reader - External Event 8 latch
pub use EE6LTCH_R as EE8LTCH_R;
///Field `EE9LTCH` reader - External Event 9 latch
pub use EE6LTCH_R as EE9LTCH_R;
///Field `EE10LTCH` reader - External Event 10 latch
pub use EE6LTCH_R as EE10LTCH_R;
///Field `EE7LTCH` writer - External Event 7 latch
pub use EE6LTCH_W as EE7LTCH_W;
///Field `EE8LTCH` writer - External Event 8 latch
pub use EE6LTCH_W as EE8LTCH_W;
///Field `EE9LTCH` writer - External Event 9 latch
pub use EE6LTCH_W as EE9LTCH_W;
///Field `EE10LTCH` writer - External Event 10 latch
pub use EE6LTCH_W as EE10LTCH_W;
impl R {
    ///Bit 0 - External Event 6 latch
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:4 - External Event 6 filter
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    ///Bit 6 - External Event 7 latch
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bits 7:10 - External Event 7 filter
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    ///Bit 12 - External Event 8 latch
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 13:16 - External Event 8 filter
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    ///Bit 18 - External Event 9 latch
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bits 19:22 - External Event 9 filter
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    ///Bit 24 - External Event 10 latch
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bits 25:28 - External Event 10 filter
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EEFER2")
            .field("ee6fltr", &self.ee6fltr())
            .field("ee10fltr", &self.ee10fltr())
            .field("ee6ltch", &self.ee6ltch())
            .field("ee10ltch", &self.ee10ltch())
            .field("ee9fltr", &self.ee9fltr())
            .field("ee9ltch", &self.ee9ltch())
            .field("ee8fltr", &self.ee8fltr())
            .field("ee8ltch", &self.ee8ltch())
            .field("ee7fltr", &self.ee7fltr())
            .field("ee7ltch", &self.ee7ltch())
            .finish()
    }
}
impl W {
    ///Bit 0 - External Event 6 latch
    #[inline(always)]
    #[must_use]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W<EEFER2rs> {
        EE6LTCH_W::new(self, 0)
    }
    ///Bits 1:4 - External Event 6 filter
    #[inline(always)]
    #[must_use]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W<EEFER2rs> {
        EE6FLTR_W::new(self, 1)
    }
    ///Bit 6 - External Event 7 latch
    #[inline(always)]
    #[must_use]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W<EEFER2rs> {
        EE7LTCH_W::new(self, 6)
    }
    ///Bits 7:10 - External Event 7 filter
    #[inline(always)]
    #[must_use]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W<EEFER2rs> {
        EE7FLTR_W::new(self, 7)
    }
    ///Bit 12 - External Event 8 latch
    #[inline(always)]
    #[must_use]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W<EEFER2rs> {
        EE8LTCH_W::new(self, 12)
    }
    ///Bits 13:16 - External Event 8 filter
    #[inline(always)]
    #[must_use]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W<EEFER2rs> {
        EE8FLTR_W::new(self, 13)
    }
    ///Bit 18 - External Event 9 latch
    #[inline(always)]
    #[must_use]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W<EEFER2rs> {
        EE9LTCH_W::new(self, 18)
    }
    ///Bits 19:22 - External Event 9 filter
    #[inline(always)]
    #[must_use]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W<EEFER2rs> {
        EE9FLTR_W::new(self, 19)
    }
    ///Bit 24 - External Event 10 latch
    #[inline(always)]
    #[must_use]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W<EEFER2rs> {
        EE10LTCH_W::new(self, 24)
    }
    ///Bits 25:28 - External Event 10 filter
    #[inline(always)]
    #[must_use]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W<EEFER2rs> {
        EE10FLTR_W::new(self, 25)
    }
}
/**Timerx External Event Filtering Register 2

You can [`read`](crate::Reg::read) this register and get [`eefer2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eefer2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#HRTIM_TIME:EEFER2)*/
pub struct EEFER2rs;
impl crate::RegisterSpec for EEFER2rs {
    type Ux = u32;
}
///`read()` method returns [`eefer2::R`](R) reader structure
impl crate::Readable for EEFER2rs {}
///`write(|w| ..)` method takes [`eefer2::W`](W) writer structure
impl crate::Writable for EEFER2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EEFER2 to value 0
impl crate::Resettable for EEFER2rs {
    const RESET_VALUE: u32 = 0;
}
