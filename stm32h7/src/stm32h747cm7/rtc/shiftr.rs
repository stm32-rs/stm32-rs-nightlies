#[doc = "Register `SHIFTR` writer"]
pub type W = crate::W<SHIFTRrs>;
#[doc = "Field `SUBFS` writer - Subtract a fraction of a second These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR). The value which is written to SUBFS is added to the synchronous prescaler counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by: Delay (seconds) = SUBFS / (PREDIV_S + 1) A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by: Advance (seconds) = (1 - (SUBFS / (PREDIV_S + 1))). Note: Writing to SUBFS causes RSF to be cleared. Software can then wait until RSF=1 to be sure that the shadow registers have been updated with the shifted time."]
pub type SUBFS_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 15, u16>;
#[doc = "Add one second This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD1SW {
    #[doc = "1: Add one second to the clock/calendar"]
    Add1 = 1,
}
impl From<ADD1SW> for bool {
    #[inline(always)]
    fn from(variant: ADD1SW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADD1S` writer - Add one second This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation."]
pub type ADD1S_W<'a, REG> = crate::BitWriter<'a, REG, ADD1SW>;
impl<'a, REG> ADD1S_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Add one second to the clock/calendar"]
    #[inline(always)]
    pub fn add1(self) -> &'a mut crate::W<REG> {
        self.variant(ADD1SW::Add1)
    }
}
impl W {
    #[doc = "Bits 0:14 - Subtract a fraction of a second These bits are write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR). The value which is written to SUBFS is added to the synchronous prescaler counter. Since this counter counts down, this operation effectively subtracts from (delays) the clock by: Delay (seconds) = SUBFS / (PREDIV_S + 1) A fraction of a second can effectively be added to the clock (advancing the clock) when the ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by: Advance (seconds) = (1 - (SUBFS / (PREDIV_S + 1))). Note: Writing to SUBFS causes RSF to be cleared. Software can then wait until RSF=1 to be sure that the shadow registers have been updated with the shifted time."]
    #[inline(always)]
    #[must_use]
    pub fn subfs(&mut self) -> SUBFS_W<SHIFTRrs> {
        SUBFS_W::new(self, 0)
    }
    #[doc = "Bit 31 - Add one second This bit is write only and is always read as zero. Writing to this bit has no effect when a shift operation is pending (when SHPF=1, in RTC_ISR). This function is intended to be used with SUBFS (see description below) in order to effectively add a fraction of a second to the clock in an atomic operation."]
    #[inline(always)]
    #[must_use]
    pub fn add1s(&mut self) -> ADD1S_W<SHIFTRrs> {
        ADD1S_W::new(self, 31)
    }
}
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHIFTRrs;
impl crate::RegisterSpec for SHIFTRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`shiftr::W`](W) writer structure"]
impl crate::Writable for SHIFTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHIFTR to value 0"]
impl crate::Resettable for SHIFTRrs {
    const RESET_VALUE: u32 = 0;
}
