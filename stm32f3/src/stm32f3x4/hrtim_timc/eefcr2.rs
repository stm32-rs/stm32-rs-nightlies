#[doc = "Register `EEFCR2` reader"]
pub type R = crate::R<EEFCR2rs>;
#[doc = "Register `EEFCR2` writer"]
pub type W = crate::W<EEFCR2rs>;
#[doc = "External Event 6 latch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE6LTCH {
    #[doc = "0: Event is ignored if it happens during a blank, or passed through during a window"]
    Disabled = 0,
    #[doc = "1: Event is latched and delayed till the end of the blanking or windowing period"]
    Enabled = 1,
}
impl From<EE6LTCH> for bool {
    #[inline(always)]
    fn from(variant: EE6LTCH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EE6LTCH` reader - External Event 6 latch"]
pub type EE6LTCH_R = crate::BitReader<EE6LTCH>;
impl EE6LTCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EE6LTCH {
        match self.bits {
            false => EE6LTCH::Disabled,
            true => EE6LTCH::Enabled,
        }
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6LTCH::Disabled
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EE6LTCH::Enabled
    }
}
#[doc = "Field `EE6LTCH` writer - External Event 6 latch"]
pub type EE6LTCH_W<'a, REG> = crate::BitWriter<'a, REG, EE6LTCH>;
impl<'a, REG> EE6LTCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE6LTCH::Disabled)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE6LTCH::Enabled)
    }
}
#[doc = "External Event 6 filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE6FLTR {
    #[doc = "0: No filtering"]
    Disabled = 0,
    #[doc = "1: Blanking from counter reset/roll-over to Compare 1"]
    BlankResetToCompare1 = 1,
    #[doc = "2: Blanking from counter reset/roll-over to Compare 2"]
    BlankResetToCompare2 = 2,
    #[doc = "3: Blanking from counter reset/roll-over to Compare 3"]
    BlankResetToCompare3 = 3,
    #[doc = "4: Blanking from counter reset/roll-over to Compare 4"]
    BlankResetToCompare4 = 4,
    #[doc = "5: Blanking from another timing unit: TIMFLTR1 source"]
    BlankTimfltr1 = 5,
    #[doc = "6: Blanking from another timing unit: TIMFLTR2 source"]
    BlankTimfltr2 = 6,
    #[doc = "7: Blanking from another timing unit: TIMFLTR3 source"]
    BlankTimfltr3 = 7,
    #[doc = "8: Blanking from another timing unit: TIMFLTR4 source"]
    BlankTimfltr4 = 8,
    #[doc = "9: Blanking from another timing unit: TIMFLTR5 source"]
    BlankTimfltr5 = 9,
    #[doc = "10: Blanking from another timing unit: TIMFLTR6 source"]
    BlankTimfltr6 = 10,
    #[doc = "11: Blanking from another timing unit: TIMFLTR7 source"]
    BlankTimfltr7 = 11,
    #[doc = "12: Blanking from another timing unit: TIMFLTR8 source"]
    BlankTimfltr8 = 12,
    #[doc = "13: Windowing from counter reset/roll-over to compare 2"]
    WindowResetToCompare2 = 13,
    #[doc = "14: Windowing from counter reset/roll-over to compare 3"]
    WindowResetToCompare3 = 14,
    #[doc = "15: Windowing from another timing unit: TIMWIN source"]
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
#[doc = "Field `EE6FLTR` reader - External Event 6 filter"]
pub type EE6FLTR_R = crate::FieldReader<EE6FLTR>;
impl EE6FLTR_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE6FLTR::Disabled
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare1(&self) -> bool {
        *self == EE6FLTR::BlankResetToCompare1
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare2(&self) -> bool {
        *self == EE6FLTR::BlankResetToCompare2
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare3(&self) -> bool {
        *self == EE6FLTR::BlankResetToCompare3
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare4(&self) -> bool {
        *self == EE6FLTR::BlankResetToCompare4
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn is_blank_timfltr1(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr1
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn is_blank_timfltr2(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr2
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn is_blank_timfltr3(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr3
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn is_blank_timfltr4(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr4
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn is_blank_timfltr5(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr5
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn is_blank_timfltr6(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr6
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn is_blank_timfltr7(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr7
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn is_blank_timfltr8(&self) -> bool {
        *self == EE6FLTR::BlankTimfltr8
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn is_window_reset_to_compare2(&self) -> bool {
        *self == EE6FLTR::WindowResetToCompare2
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn is_window_reset_to_compare3(&self) -> bool {
        *self == EE6FLTR::WindowResetToCompare3
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn is_window_timwin(&self) -> bool {
        *self == EE6FLTR::WindowTimwin
    }
}
#[doc = "Field `EE6FLTR` writer - External Event 6 filter"]
pub type EE6FLTR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, EE6FLTR>;
impl<'a, REG> EE6FLTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::Disabled)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankResetToCompare1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankResetToCompare2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankResetToCompare3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankResetToCompare4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::BlankTimfltr8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::WindowResetToCompare2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::WindowResetToCompare3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut crate::W<REG> {
        self.variant(EE6FLTR::WindowTimwin)
    }
}
#[doc = "Field `EE7FLTR` reader - External Event 7 filter"]
pub use EE6FLTR_R as EE7FLTR_R;
#[doc = "Field `EE8FLTR` reader - External Event 8 filter"]
pub use EE6FLTR_R as EE8FLTR_R;
#[doc = "Field `EE9FLTR` reader - External Event 9 filter"]
pub use EE6FLTR_R as EE9FLTR_R;
#[doc = "Field `EE10FLTR` reader - External Event 10 filter"]
pub use EE6FLTR_R as EE10FLTR_R;
#[doc = "Field `EE7FLTR` writer - External Event 7 filter"]
pub use EE6FLTR_W as EE7FLTR_W;
#[doc = "Field `EE8FLTR` writer - External Event 8 filter"]
pub use EE6FLTR_W as EE8FLTR_W;
#[doc = "Field `EE9FLTR` writer - External Event 9 filter"]
pub use EE6FLTR_W as EE9FLTR_W;
#[doc = "Field `EE10FLTR` writer - External Event 10 filter"]
pub use EE6FLTR_W as EE10FLTR_W;
#[doc = "Field `EE7LTCH` reader - External Event 7 latch"]
pub use EE6LTCH_R as EE7LTCH_R;
#[doc = "Field `EE8LTCH` reader - External Event 8 latch"]
pub use EE6LTCH_R as EE8LTCH_R;
#[doc = "Field `EE9LTCH` reader - External Event 9 latch"]
pub use EE6LTCH_R as EE9LTCH_R;
#[doc = "Field `EE10LTCH` reader - External Event 10 latch"]
pub use EE6LTCH_R as EE10LTCH_R;
#[doc = "Field `EE7LTCH` writer - External Event 7 latch"]
pub use EE6LTCH_W as EE7LTCH_W;
#[doc = "Field `EE8LTCH` writer - External Event 8 latch"]
pub use EE6LTCH_W as EE8LTCH_W;
#[doc = "Field `EE9LTCH` writer - External Event 9 latch"]
pub use EE6LTCH_W as EE9LTCH_W;
#[doc = "Field `EE10LTCH` writer - External Event 10 latch"]
pub use EE6LTCH_W as EE10LTCH_W;
impl R {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    pub fn ee6ltch(&self) -> EE6LTCH_R {
        EE6LTCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    pub fn ee6fltr(&self) -> EE6FLTR_R {
        EE6FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    pub fn ee7ltch(&self) -> EE7LTCH_R {
        EE7LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    pub fn ee7fltr(&self) -> EE7FLTR_R {
        EE7FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    pub fn ee8ltch(&self) -> EE8LTCH_R {
        EE8LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    pub fn ee8fltr(&self) -> EE8FLTR_R {
        EE8FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    pub fn ee9ltch(&self) -> EE9LTCH_R {
        EE9LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    pub fn ee9fltr(&self) -> EE9FLTR_R {
        EE9FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    pub fn ee10ltch(&self) -> EE10LTCH_R {
        EE10LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    pub fn ee10fltr(&self) -> EE10FLTR_R {
        EE10FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event 6 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee6ltch(&mut self) -> EE6LTCH_W<EEFCR2rs> {
        EE6LTCH_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - External Event 6 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee6fltr(&mut self) -> EE6FLTR_W<EEFCR2rs> {
        EE6FLTR_W::new(self, 1)
    }
    #[doc = "Bit 6 - External Event 7 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee7ltch(&mut self) -> EE7LTCH_W<EEFCR2rs> {
        EE7LTCH_W::new(self, 6)
    }
    #[doc = "Bits 7:10 - External Event 7 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee7fltr(&mut self) -> EE7FLTR_W<EEFCR2rs> {
        EE7FLTR_W::new(self, 7)
    }
    #[doc = "Bit 12 - External Event 8 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee8ltch(&mut self) -> EE8LTCH_W<EEFCR2rs> {
        EE8LTCH_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - External Event 8 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee8fltr(&mut self) -> EE8FLTR_W<EEFCR2rs> {
        EE8FLTR_W::new(self, 13)
    }
    #[doc = "Bit 18 - External Event 9 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee9ltch(&mut self) -> EE9LTCH_W<EEFCR2rs> {
        EE9LTCH_W::new(self, 18)
    }
    #[doc = "Bits 19:22 - External Event 9 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee9fltr(&mut self) -> EE9FLTR_W<EEFCR2rs> {
        EE9FLTR_W::new(self, 19)
    }
    #[doc = "Bit 24 - External Event 10 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee10ltch(&mut self) -> EE10LTCH_W<EEFCR2rs> {
        EE10LTCH_W::new(self, 24)
    }
    #[doc = "Bits 25:28 - External Event 10 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee10fltr(&mut self) -> EE10FLTR_W<EEFCR2rs> {
        EE10FLTR_W::new(self, 25)
    }
}
#[doc = "Timerx External Event Filtering Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEFCR2rs;
impl crate::RegisterSpec for EEFCR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefcr2::R`](R) reader structure"]
impl crate::Readable for EEFCR2rs {}
#[doc = "`write(|w| ..)` method takes [`eefcr2::W`](W) writer structure"]
impl crate::Writable for EEFCR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEFCR2 to value 0"]
impl crate::Resettable for EEFCR2rs {
    const RESET_VALUE: u32 = 0;
}
