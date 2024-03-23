#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1rs>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1rs>;
#[doc = "Master Update Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUDIS {
    #[doc = "0: Timer update enabled"]
    Enabled = 0,
    #[doc = "1: Timer update disabled"]
    Disabled = 1,
}
impl From<MUDIS> for bool {
    #[inline(always)]
    fn from(variant: MUDIS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MUDIS` reader - Master Update Disable"]
pub type MUDIS_R = crate::BitReader<MUDIS>;
impl MUDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MUDIS {
        match self.bits {
            false => MUDIS::Enabled,
            true => MUDIS::Disabled,
        }
    }
    #[doc = "Timer update enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUDIS::Enabled
    }
    #[doc = "Timer update disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUDIS::Disabled
    }
}
#[doc = "Field `MUDIS` writer - Master Update Disable"]
pub type MUDIS_W<'a, REG> = crate::BitWriter<'a, REG, MUDIS>;
impl<'a, REG> MUDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer update enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUDIS::Enabled)
    }
    #[doc = "Timer update disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUDIS::Disabled)
    }
}
#[doc = "Field `TAUDIS` reader - Timer A Update Disable"]
pub use MUDIS_R as TAUDIS_R;
#[doc = "Field `TBUDIS` reader - Timer B Update Disable"]
pub use MUDIS_R as TBUDIS_R;
#[doc = "Field `TCUDIS` reader - Timer C Update Disable"]
pub use MUDIS_R as TCUDIS_R;
#[doc = "Field `TDUDIS` reader - Timer D Update Disable"]
pub use MUDIS_R as TDUDIS_R;
#[doc = "Field `TEUDIS` reader - Timer E Update Disable"]
pub use MUDIS_R as TEUDIS_R;
#[doc = "Field `TAUDIS` writer - Timer A Update Disable"]
pub use MUDIS_W as TAUDIS_W;
#[doc = "Field `TBUDIS` writer - Timer B Update Disable"]
pub use MUDIS_W as TBUDIS_W;
#[doc = "Field `TCUDIS` writer - Timer C Update Disable"]
pub use MUDIS_W as TCUDIS_W;
#[doc = "Field `TDUDIS` writer - Timer D Update Disable"]
pub use MUDIS_W as TDUDIS_W;
#[doc = "Field `TEUDIS` writer - Timer E Update Disable"]
pub use MUDIS_W as TEUDIS_W;
#[doc = "ADC Trigger 1 Update Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AD1USRC {
    #[doc = "0: ADC trigger update from master timer"]
    Master = 0,
    #[doc = "1: ADC trigger update from timer A"]
    TimerA = 1,
    #[doc = "2: ADC trigger update from timer B"]
    TimerB = 2,
    #[doc = "3: ADC trigger update from timer C"]
    TimerC = 3,
    #[doc = "4: ADC trigger update from timer D"]
    TimerD = 4,
    #[doc = "5: ADC trigger update from timer E"]
    TimerE = 5,
}
impl From<AD1USRC> for u8 {
    #[inline(always)]
    fn from(variant: AD1USRC) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for AD1USRC {
    type Ux = u8;
}
#[doc = "Field `AD1USRC` reader - ADC Trigger 1 Update Source"]
pub type AD1USRC_R = crate::FieldReader<AD1USRC>;
impl AD1USRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<AD1USRC> {
        match self.bits {
            0 => Some(AD1USRC::Master),
            1 => Some(AD1USRC::TimerA),
            2 => Some(AD1USRC::TimerB),
            3 => Some(AD1USRC::TimerC),
            4 => Some(AD1USRC::TimerD),
            5 => Some(AD1USRC::TimerE),
            _ => None,
        }
    }
    #[doc = "ADC trigger update from master timer"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == AD1USRC::Master
    }
    #[doc = "ADC trigger update from timer A"]
    #[inline(always)]
    pub fn is_timer_a(&self) -> bool {
        *self == AD1USRC::TimerA
    }
    #[doc = "ADC trigger update from timer B"]
    #[inline(always)]
    pub fn is_timer_b(&self) -> bool {
        *self == AD1USRC::TimerB
    }
    #[doc = "ADC trigger update from timer C"]
    #[inline(always)]
    pub fn is_timer_c(&self) -> bool {
        *self == AD1USRC::TimerC
    }
    #[doc = "ADC trigger update from timer D"]
    #[inline(always)]
    pub fn is_timer_d(&self) -> bool {
        *self == AD1USRC::TimerD
    }
    #[doc = "ADC trigger update from timer E"]
    #[inline(always)]
    pub fn is_timer_e(&self) -> bool {
        *self == AD1USRC::TimerE
    }
}
#[doc = "Field `AD1USRC` writer - ADC Trigger 1 Update Source"]
pub type AD1USRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, AD1USRC>;
impl<'a, REG> AD1USRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "ADC trigger update from master timer"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::Master)
    }
    #[doc = "ADC trigger update from timer A"]
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerA)
    }
    #[doc = "ADC trigger update from timer B"]
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerB)
    }
    #[doc = "ADC trigger update from timer C"]
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerC)
    }
    #[doc = "ADC trigger update from timer D"]
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerD)
    }
    #[doc = "ADC trigger update from timer E"]
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerE)
    }
}
#[doc = "Field `AD2USRC` reader - ADC Trigger 2 Update Source"]
pub use AD1USRC_R as AD2USRC_R;
#[doc = "Field `AD3USRC` reader - ADC Trigger 3 Update Source"]
pub use AD1USRC_R as AD3USRC_R;
#[doc = "Field `AD4USRC` reader - ADC Trigger 4 Update Source"]
pub use AD1USRC_R as AD4USRC_R;
#[doc = "Field `AD2USRC` writer - ADC Trigger 2 Update Source"]
pub use AD1USRC_W as AD2USRC_W;
#[doc = "Field `AD3USRC` writer - ADC Trigger 3 Update Source"]
pub use AD1USRC_W as AD3USRC_W;
#[doc = "Field `AD4USRC` writer - ADC Trigger 4 Update Source"]
pub use AD1USRC_W as AD4USRC_W;
impl R {
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    pub fn mudis(&self) -> MUDIS_R {
        MUDIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    pub fn taudis(&self) -> TAUDIS_R {
        TAUDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    pub fn tbudis(&self) -> TBUDIS_R {
        TBUDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    pub fn tcudis(&self) -> TCUDIS_R {
        TCUDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    pub fn tdudis(&self) -> TDUDIS_R {
        TDUDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    pub fn teudis(&self) -> TEUDIS_R {
        TEUDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    pub fn ad1usrc(&self) -> AD1USRC_R {
        AD1USRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    pub fn ad2usrc(&self) -> AD2USRC_R {
        AD2USRC_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    pub fn ad3usrc(&self) -> AD3USRC_R {
        AD3USRC_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    pub fn ad4usrc(&self) -> AD4USRC_R {
        AD4USRC_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Master Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mudis(&mut self) -> MUDIS_W<CR1rs> {
        MUDIS_W::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn taudis(&mut self) -> TAUDIS_W<CR1rs> {
        TAUDIS_W::new(self, 1)
    }
    #[doc = "Bit 2 - Timer B Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tbudis(&mut self) -> TBUDIS_W<CR1rs> {
        TBUDIS_W::new(self, 2)
    }
    #[doc = "Bit 3 - Timer C Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tcudis(&mut self) -> TCUDIS_W<CR1rs> {
        TCUDIS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Timer D Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn tdudis(&mut self) -> TDUDIS_W<CR1rs> {
        TDUDIS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Timer E Update Disable"]
    #[inline(always)]
    #[must_use]
    pub fn teudis(&mut self) -> TEUDIS_W<CR1rs> {
        TEUDIS_W::new(self, 5)
    }
    #[doc = "Bits 16:18 - ADC Trigger 1 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad1usrc(&mut self) -> AD1USRC_W<CR1rs> {
        AD1USRC_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - ADC Trigger 2 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad2usrc(&mut self) -> AD2USRC_W<CR1rs> {
        AD2USRC_W::new(self, 19)
    }
    #[doc = "Bits 22:24 - ADC Trigger 3 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad3usrc(&mut self) -> AD3USRC_W<CR1rs> {
        AD3USRC_W::new(self, 22)
    }
    #[doc = "Bits 25:27 - ADC Trigger 4 Update Source"]
    #[inline(always)]
    #[must_use]
    pub fn ad4usrc(&mut self) -> AD4USRC_W<CR1rs> {
        AD4USRC_W::new(self, 25)
    }
}
#[doc = "Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1rs {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1rs {
    const RESET_VALUE: u32 = 0;
}
