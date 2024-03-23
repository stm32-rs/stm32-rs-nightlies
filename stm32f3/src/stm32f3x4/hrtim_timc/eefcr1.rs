#[doc = "Register `EEFCR1` reader"]
pub type R = crate::R<EEFCR1rs>;
#[doc = "Register `EEFCR1` writer"]
pub type W = crate::W<EEFCR1rs>;
#[doc = "External Event 1 latch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EE1LTCH {
    #[doc = "0: Event is ignored if it happens during a blank, or passed through during a window"]
    Disabled = 0,
    #[doc = "1: Event is latched and delayed till the end of the blanking or windowing period"]
    Enabled = 1,
}
impl From<EE1LTCH> for bool {
    #[inline(always)]
    fn from(variant: EE1LTCH) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EE1LTCH` reader - External Event 1 latch"]
pub type EE1LTCH_R = crate::BitReader<EE1LTCH>;
impl EE1LTCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EE1LTCH {
        match self.bits {
            false => EE1LTCH::Disabled,
            true => EE1LTCH::Enabled,
        }
    }
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE1LTCH::Disabled
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EE1LTCH::Enabled
    }
}
#[doc = "Field `EE1LTCH` writer - External Event 1 latch"]
pub type EE1LTCH_W<'a, REG> = crate::BitWriter<'a, REG, EE1LTCH>;
impl<'a, REG> EE1LTCH_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Event is ignored if it happens during a blank, or passed through during a window"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE1LTCH::Disabled)
    }
    #[doc = "Event is latched and delayed till the end of the blanking or windowing period"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE1LTCH::Enabled)
    }
}
#[doc = "External Event 1 filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EE1FLTR {
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
impl From<EE1FLTR> for u8 {
    #[inline(always)]
    fn from(variant: EE1FLTR) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for EE1FLTR {
    type Ux = u8;
}
#[doc = "Field `EE1FLTR` reader - External Event 1 filter"]
pub type EE1FLTR_R = crate::FieldReader<EE1FLTR>;
impl EE1FLTR_R {
    #[doc = "Get enumerated values variant"]
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
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EE1FLTR::Disabled
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare1(&self) -> bool {
        *self == EE1FLTR::BlankResetToCompare1
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare2(&self) -> bool {
        *self == EE1FLTR::BlankResetToCompare2
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare3(&self) -> bool {
        *self == EE1FLTR::BlankResetToCompare3
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn is_blank_reset_to_compare4(&self) -> bool {
        *self == EE1FLTR::BlankResetToCompare4
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn is_blank_timfltr1(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr1
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn is_blank_timfltr2(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr2
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn is_blank_timfltr3(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr3
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn is_blank_timfltr4(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr4
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn is_blank_timfltr5(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr5
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn is_blank_timfltr6(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr6
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn is_blank_timfltr7(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr7
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn is_blank_timfltr8(&self) -> bool {
        *self == EE1FLTR::BlankTimfltr8
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn is_window_reset_to_compare2(&self) -> bool {
        *self == EE1FLTR::WindowResetToCompare2
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn is_window_reset_to_compare3(&self) -> bool {
        *self == EE1FLTR::WindowResetToCompare3
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn is_window_timwin(&self) -> bool {
        *self == EE1FLTR::WindowTimwin
    }
}
#[doc = "Field `EE1FLTR` writer - External Event 1 filter"]
pub type EE1FLTR_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, EE1FLTR>;
impl<'a, REG> EE1FLTR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No filtering"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::Disabled)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 1"]
    #[inline(always)]
    pub fn blank_reset_to_compare1(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankResetToCompare1)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 2"]
    #[inline(always)]
    pub fn blank_reset_to_compare2(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankResetToCompare2)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 3"]
    #[inline(always)]
    pub fn blank_reset_to_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankResetToCompare3)
    }
    #[doc = "Blanking from counter reset/roll-over to Compare 4"]
    #[inline(always)]
    pub fn blank_reset_to_compare4(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankResetToCompare4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR1 source"]
    #[inline(always)]
    pub fn blank_timfltr1(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr1)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR2 source"]
    #[inline(always)]
    pub fn blank_timfltr2(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr2)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR3 source"]
    #[inline(always)]
    pub fn blank_timfltr3(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr3)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR4 source"]
    #[inline(always)]
    pub fn blank_timfltr4(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr4)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR5 source"]
    #[inline(always)]
    pub fn blank_timfltr5(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr5)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR6 source"]
    #[inline(always)]
    pub fn blank_timfltr6(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr6)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR7 source"]
    #[inline(always)]
    pub fn blank_timfltr7(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr7)
    }
    #[doc = "Blanking from another timing unit: TIMFLTR8 source"]
    #[inline(always)]
    pub fn blank_timfltr8(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::BlankTimfltr8)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 2"]
    #[inline(always)]
    pub fn window_reset_to_compare2(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::WindowResetToCompare2)
    }
    #[doc = "Windowing from counter reset/roll-over to compare 3"]
    #[inline(always)]
    pub fn window_reset_to_compare3(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::WindowResetToCompare3)
    }
    #[doc = "Windowing from another timing unit: TIMWIN source"]
    #[inline(always)]
    pub fn window_timwin(self) -> &'a mut crate::W<REG> {
        self.variant(EE1FLTR::WindowTimwin)
    }
}
#[doc = "Field `EE2FLTR` reader - External Event 2 filter"]
pub use EE1FLTR_R as EE2FLTR_R;
#[doc = "Field `EE3FLTR` reader - External Event 3 filter"]
pub use EE1FLTR_R as EE3FLTR_R;
#[doc = "Field `EE4FLTR` reader - External Event 4 filter"]
pub use EE1FLTR_R as EE4FLTR_R;
#[doc = "Field `EE5FLTR` reader - External Event 5 filter"]
pub use EE1FLTR_R as EE5FLTR_R;
#[doc = "Field `EE2FLTR` writer - External Event 2 filter"]
pub use EE1FLTR_W as EE2FLTR_W;
#[doc = "Field `EE3FLTR` writer - External Event 3 filter"]
pub use EE1FLTR_W as EE3FLTR_W;
#[doc = "Field `EE4FLTR` writer - External Event 4 filter"]
pub use EE1FLTR_W as EE4FLTR_W;
#[doc = "Field `EE5FLTR` writer - External Event 5 filter"]
pub use EE1FLTR_W as EE5FLTR_W;
#[doc = "Field `EE2LTCH` reader - External Event 2 latch"]
pub use EE1LTCH_R as EE2LTCH_R;
#[doc = "Field `EE3LTCH` reader - External Event 3 latch"]
pub use EE1LTCH_R as EE3LTCH_R;
#[doc = "Field `EE4LTCH` reader - External Event 4 latch"]
pub use EE1LTCH_R as EE4LTCH_R;
#[doc = "Field `EE5LTCH` reader - External Event 5 latch"]
pub use EE1LTCH_R as EE5LTCH_R;
#[doc = "Field `EE2LTCH` writer - External Event 2 latch"]
pub use EE1LTCH_W as EE2LTCH_W;
#[doc = "Field `EE3LTCH` writer - External Event 3 latch"]
pub use EE1LTCH_W as EE3LTCH_W;
#[doc = "Field `EE4LTCH` writer - External Event 4 latch"]
pub use EE1LTCH_W as EE4LTCH_W;
#[doc = "Field `EE5LTCH` writer - External Event 5 latch"]
pub use EE1LTCH_W as EE5LTCH_W;
impl R {
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    pub fn ee1ltch(&self) -> EE1LTCH_R {
        EE1LTCH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    pub fn ee1fltr(&self) -> EE1FLTR_R {
        EE1FLTR_R::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    pub fn ee2ltch(&self) -> EE2LTCH_R {
        EE2LTCH_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    pub fn ee2fltr(&self) -> EE2FLTR_R {
        EE2FLTR_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    pub fn ee3ltch(&self) -> EE3LTCH_R {
        EE3LTCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    pub fn ee3fltr(&self) -> EE3FLTR_R {
        EE3FLTR_R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    pub fn ee4ltch(&self) -> EE4LTCH_R {
        EE4LTCH_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    pub fn ee4fltr(&self) -> EE4FLTR_R {
        EE4FLTR_R::new(((self.bits >> 19) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    pub fn ee5ltch(&self) -> EE5LTCH_R {
        EE5LTCH_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    pub fn ee5fltr(&self) -> EE5FLTR_R {
        EE5FLTR_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Event 1 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee1ltch(&mut self) -> EE1LTCH_W<EEFCR1rs> {
        EE1LTCH_W::new(self, 0)
    }
    #[doc = "Bits 1:4 - External Event 1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee1fltr(&mut self) -> EE1FLTR_W<EEFCR1rs> {
        EE1FLTR_W::new(self, 1)
    }
    #[doc = "Bit 6 - External Event 2 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee2ltch(&mut self) -> EE2LTCH_W<EEFCR1rs> {
        EE2LTCH_W::new(self, 6)
    }
    #[doc = "Bits 7:10 - External Event 2 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee2fltr(&mut self) -> EE2FLTR_W<EEFCR1rs> {
        EE2FLTR_W::new(self, 7)
    }
    #[doc = "Bit 12 - External Event 3 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee3ltch(&mut self) -> EE3LTCH_W<EEFCR1rs> {
        EE3LTCH_W::new(self, 12)
    }
    #[doc = "Bits 13:16 - External Event 3 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee3fltr(&mut self) -> EE3FLTR_W<EEFCR1rs> {
        EE3FLTR_W::new(self, 13)
    }
    #[doc = "Bit 18 - External Event 4 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee4ltch(&mut self) -> EE4LTCH_W<EEFCR1rs> {
        EE4LTCH_W::new(self, 18)
    }
    #[doc = "Bits 19:22 - External Event 4 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee4fltr(&mut self) -> EE4FLTR_W<EEFCR1rs> {
        EE4FLTR_W::new(self, 19)
    }
    #[doc = "Bit 24 - External Event 5 latch"]
    #[inline(always)]
    #[must_use]
    pub fn ee5ltch(&mut self) -> EE5LTCH_W<EEFCR1rs> {
        EE5LTCH_W::new(self, 24)
    }
    #[doc = "Bits 25:28 - External Event 5 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ee5fltr(&mut self) -> EE5FLTR_W<EEFCR1rs> {
        EE5FLTR_W::new(self, 25)
    }
}
#[doc = "Timerx External Event Filtering Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eefcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eefcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EEFCR1rs;
impl crate::RegisterSpec for EEFCR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eefcr1::R`](R) reader structure"]
impl crate::Readable for EEFCR1rs {}
#[doc = "`write(|w| ..)` method takes [`eefcr1::W`](W) writer structure"]
impl crate::Writable for EEFCR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEFCR1 to value 0"]
impl crate::Resettable for EEFCR1rs {
    const RESET_VALUE: u32 = 0;
}
