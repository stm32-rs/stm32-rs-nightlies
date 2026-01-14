///Register `CR1` reader
pub type R = crate::R<CR1rs>;
///Register `CR1` writer
pub type W = crate::W<CR1rs>;
/**Master Update Disable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MUDIS {
    ///0: Timer update enabled
    Enabled = 0,
    ///1: Timer update disabled
    Disabled = 1,
}
impl From<MUDIS> for bool {
    #[inline(always)]
    fn from(variant: MUDIS) -> Self {
        variant as u8 != 0
    }
}
///Field `MUDIS` reader - Master Update Disable
pub type MUDIS_R = crate::BitReader<MUDIS>;
impl MUDIS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> MUDIS {
        match self.bits {
            false => MUDIS::Enabled,
            true => MUDIS::Disabled,
        }
    }
    ///Timer update enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MUDIS::Enabled
    }
    ///Timer update disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MUDIS::Disabled
    }
}
///Field `MUDIS` writer - Master Update Disable
pub type MUDIS_W<'a, REG> = crate::BitWriter<'a, REG, MUDIS>;
impl<'a, REG> MUDIS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer update enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUDIS::Enabled)
    }
    ///Timer update disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(MUDIS::Disabled)
    }
}
///Field `TUDIS(A,B,C,D,E)` reader - Timer %s Update Disable
pub use MUDIS_R as TUDIS_R;
///Field `TUDIS(A,B,C,D,E)` writer - Timer %s Update Disable
pub use MUDIS_W as TUDIS_W;
/**ADC Trigger %s Update Source

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AD1USRC {
    ///0: ADC trigger update from master timer
    Master = 0,
    ///1: ADC trigger update from timer A
    TimerA = 1,
    ///2: ADC trigger update from timer B
    TimerB = 2,
    ///3: ADC trigger update from timer C
    TimerC = 3,
    ///4: ADC trigger update from timer D
    TimerD = 4,
    ///5: ADC trigger update from timer E
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
impl crate::IsEnum for AD1USRC {}
///Field `ADUSRC(1-4)` reader - ADC Trigger %s Update Source
pub type ADUSRC_R = crate::FieldReader<AD1USRC>;
impl ADUSRC_R {
    ///Get enumerated values variant
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
    ///ADC trigger update from master timer
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == AD1USRC::Master
    }
    ///ADC trigger update from timer A
    #[inline(always)]
    pub fn is_timer_a(&self) -> bool {
        *self == AD1USRC::TimerA
    }
    ///ADC trigger update from timer B
    #[inline(always)]
    pub fn is_timer_b(&self) -> bool {
        *self == AD1USRC::TimerB
    }
    ///ADC trigger update from timer C
    #[inline(always)]
    pub fn is_timer_c(&self) -> bool {
        *self == AD1USRC::TimerC
    }
    ///ADC trigger update from timer D
    #[inline(always)]
    pub fn is_timer_d(&self) -> bool {
        *self == AD1USRC::TimerD
    }
    ///ADC trigger update from timer E
    #[inline(always)]
    pub fn is_timer_e(&self) -> bool {
        *self == AD1USRC::TimerE
    }
}
///Field `ADUSRC(1-4)` writer - ADC Trigger %s Update Source
pub type ADUSRC_W<'a, REG> = crate::FieldWriter<'a, REG, 3, AD1USRC>;
impl<'a, REG> ADUSRC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///ADC trigger update from master timer
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::Master)
    }
    ///ADC trigger update from timer A
    #[inline(always)]
    pub fn timer_a(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerA)
    }
    ///ADC trigger update from timer B
    #[inline(always)]
    pub fn timer_b(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerB)
    }
    ///ADC trigger update from timer C
    #[inline(always)]
    pub fn timer_c(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerC)
    }
    ///ADC trigger update from timer D
    #[inline(always)]
    pub fn timer_d(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerD)
    }
    ///ADC trigger update from timer E
    #[inline(always)]
    pub fn timer_e(self) -> &'a mut crate::W<REG> {
        self.variant(AD1USRC::TimerE)
    }
}
impl R {
    ///Bit 0 - Master Update Disable
    #[inline(always)]
    pub fn mudis(&self) -> MUDIS_R {
        MUDIS_R::new((self.bits & 1) != 0)
    }
    ///Timer (A,B,C,D,E) Update Disable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TAUDIS` field.</div>
    #[inline(always)]
    pub fn tudis(&self, n: u8) -> TUDIS_R {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TUDIS_R::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Timer (A,B,C,D,E) Update Disable
    #[inline(always)]
    pub fn tudis_iter(&self) -> impl Iterator<Item = TUDIS_R> + '_ {
        (0..5).map(move |n| TUDIS_R::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    ///Bit 1 - Timer A Update Disable
    #[inline(always)]
    pub fn taudis(&self) -> TUDIS_R {
        TUDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer B Update Disable
    #[inline(always)]
    pub fn tbudis(&self) -> TUDIS_R {
        TUDIS_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Timer C Update Disable
    #[inline(always)]
    pub fn tcudis(&self) -> TUDIS_R {
        TUDIS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Timer D Update Disable
    #[inline(always)]
    pub fn tdudis(&self) -> TUDIS_R {
        TUDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Timer E Update Disable
    #[inline(always)]
    pub fn teudis(&self) -> TUDIS_R {
        TUDIS_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///ADC Trigger (1-4) Update Source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AD1USRC` field.</div>
    #[inline(always)]
    pub fn adusrc(&self, n: u8) -> ADUSRC_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        ADUSRC_R::new(((self.bits >> (n * 3 + 16)) & 7) as u8)
    }
    ///Iterator for array of:
    ///ADC Trigger (1-4) Update Source
    #[inline(always)]
    pub fn adusrc_iter(&self) -> impl Iterator<Item = ADUSRC_R> + '_ {
        (0..4).map(move |n| ADUSRC_R::new(((self.bits >> (n * 3 + 16)) & 7) as u8))
    }
    ///Bits 16:18 - ADC Trigger 1 Update Source
    #[inline(always)]
    pub fn ad1usrc(&self) -> ADUSRC_R {
        ADUSRC_R::new(((self.bits >> 16) & 7) as u8)
    }
    ///Bits 19:21 - ADC Trigger 2 Update Source
    #[inline(always)]
    pub fn ad2usrc(&self) -> ADUSRC_R {
        ADUSRC_R::new(((self.bits >> 19) & 7) as u8)
    }
    ///Bits 22:24 - ADC Trigger 3 Update Source
    #[inline(always)]
    pub fn ad3usrc(&self) -> ADUSRC_R {
        ADUSRC_R::new(((self.bits >> 22) & 7) as u8)
    }
    ///Bits 25:27 - ADC Trigger 4 Update Source
    #[inline(always)]
    pub fn ad4usrc(&self) -> ADUSRC_R {
        ADUSRC_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR1")
            .field("ad1usrc", &self.ad1usrc())
            .field("ad2usrc", &self.ad2usrc())
            .field("ad3usrc", &self.ad3usrc())
            .field("ad4usrc", &self.ad4usrc())
            .field("mudis", &self.mudis())
            .field("taudis", &self.taudis())
            .field("tbudis", &self.tbudis())
            .field("tcudis", &self.tcudis())
            .field("tdudis", &self.tdudis())
            .field("teudis", &self.teudis())
            .finish()
    }
}
impl W {
    ///Bit 0 - Master Update Disable
    #[inline(always)]
    pub fn mudis(&mut self) -> MUDIS_W<'_, CR1rs> {
        MUDIS_W::new(self, 0)
    }
    ///Timer (A,B,C,D,E) Update Disable
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `TAUDIS` field.</div>
    #[inline(always)]
    pub fn tudis(&mut self, n: u8) -> TUDIS_W<'_, CR1rs> {
        #[allow(clippy::no_effect)]
        [(); 5][n as usize];
        TUDIS_W::new(self, n + 1)
    }
    ///Bit 1 - Timer A Update Disable
    #[inline(always)]
    pub fn taudis(&mut self) -> TUDIS_W<'_, CR1rs> {
        TUDIS_W::new(self, 1)
    }
    ///Bit 2 - Timer B Update Disable
    #[inline(always)]
    pub fn tbudis(&mut self) -> TUDIS_W<'_, CR1rs> {
        TUDIS_W::new(self, 2)
    }
    ///Bit 3 - Timer C Update Disable
    #[inline(always)]
    pub fn tcudis(&mut self) -> TUDIS_W<'_, CR1rs> {
        TUDIS_W::new(self, 3)
    }
    ///Bit 4 - Timer D Update Disable
    #[inline(always)]
    pub fn tdudis(&mut self) -> TUDIS_W<'_, CR1rs> {
        TUDIS_W::new(self, 4)
    }
    ///Bit 5 - Timer E Update Disable
    #[inline(always)]
    pub fn teudis(&mut self) -> TUDIS_W<'_, CR1rs> {
        TUDIS_W::new(self, 5)
    }
    ///ADC Trigger (1-4) Update Source
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AD1USRC` field.</div>
    #[inline(always)]
    pub fn adusrc(&mut self, n: u8) -> ADUSRC_W<'_, CR1rs> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        ADUSRC_W::new(self, n * 3 + 16)
    }
    ///Bits 16:18 - ADC Trigger 1 Update Source
    #[inline(always)]
    pub fn ad1usrc(&mut self) -> ADUSRC_W<'_, CR1rs> {
        ADUSRC_W::new(self, 16)
    }
    ///Bits 19:21 - ADC Trigger 2 Update Source
    #[inline(always)]
    pub fn ad2usrc(&mut self) -> ADUSRC_W<'_, CR1rs> {
        ADUSRC_W::new(self, 19)
    }
    ///Bits 22:24 - ADC Trigger 3 Update Source
    #[inline(always)]
    pub fn ad3usrc(&mut self) -> ADUSRC_W<'_, CR1rs> {
        ADUSRC_W::new(self, 22)
    }
    ///Bits 25:27 - ADC Trigger 4 Update Source
    #[inline(always)]
    pub fn ad4usrc(&mut self) -> ADUSRC_W<'_, CR1rs> {
        ADUSRC_W::new(self, 25)
    }
}
/**Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H755_CM4.html#HRTIM_Common:CR1)*/
pub struct CR1rs;
impl crate::RegisterSpec for CR1rs {
    type Ux = u32;
}
///`read()` method returns [`cr1::R`](R) reader structure
impl crate::Readable for CR1rs {}
///`write(|w| ..)` method takes [`cr1::W`](W) writer structure
impl crate::Writable for CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR1 to value 0
impl crate::Resettable for CR1rs {}
