///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
/**ADRDY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYR {
    ///0: ADC is not ready to start conversion
    NotReady = 0,
    ///1: ADC is ready to start conversion
    Ready = 1,
}
impl From<ADRDYR> for bool {
    #[inline(always)]
    fn from(variant: ADRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` reader - ADRDY
pub type ADRDY_R = crate::BitReader<ADRDYR>;
impl ADRDY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ADRDYR {
        match self.bits {
            false => ADRDYR::NotReady,
            true => ADRDYR::Ready,
        }
    }
    ///ADC is not ready to start conversion
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDYR::NotReady
    }
    ///ADC is ready to start conversion
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDYR::Ready
    }
}
/**ADRDY

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYW {
    ///1: Clear ADC is ready to start conversion flag
    Clear = 1,
}
impl From<ADRDYW> for bool {
    #[inline(always)]
    fn from(variant: ADRDYW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` writer - ADRDY
pub type ADRDY_W<'a, REG> = crate::BitWriter1C<'a, REG, ADRDYW>;
impl<'a, REG> ADRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear ADC is ready to start conversion flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYW::Clear)
    }
}
/**EOSMP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPR {
    ///0: End of sampling phase no yet reached
    NotEnded = 0,
    ///1: End of sampling phase reached
    Ended = 1,
}
impl From<EOSMPR> for bool {
    #[inline(always)]
    fn from(variant: EOSMPR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` reader - EOSMP
pub type EOSMP_R = crate::BitReader<EOSMPR>;
impl EOSMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPR {
        match self.bits {
            false => EOSMPR::NotEnded,
            true => EOSMPR::Ended,
        }
    }
    ///End of sampling phase no yet reached
    #[inline(always)]
    pub fn is_not_ended(&self) -> bool {
        *self == EOSMPR::NotEnded
    }
    ///End of sampling phase reached
    #[inline(always)]
    pub fn is_ended(&self) -> bool {
        *self == EOSMPR::Ended
    }
}
/**EOSMP

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPW {
    ///1: Clear end of sampling phase reached flag
    Clear = 1,
}
impl From<EOSMPW> for bool {
    #[inline(always)]
    fn from(variant: EOSMPW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` writer - EOSMP
pub type EOSMP_W<'a, REG> = crate::BitWriter1C<'a, REG, EOSMPW>;
impl<'a, REG> EOSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear end of sampling phase reached flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPW::Clear)
    }
}
/**EOC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR {
    ///0: Regular conversion is not complete
    NotComplete = 0,
    ///1: Regular conversion complete
    Complete = 1,
}
impl From<EOCR> for bool {
    #[inline(always)]
    fn from(variant: EOCR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` reader - EOC
pub type EOC_R = crate::BitReader<EOCR>;
impl EOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCR {
        match self.bits {
            false => EOCR::NotComplete,
            true => EOCR::Complete,
        }
    }
    ///Regular conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR::NotComplete
    }
    ///Regular conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR::Complete
    }
}
/**EOC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW {
    ///1: Clear regular conversion complete flag
    Clear = 1,
}
impl From<EOCW> for bool {
    #[inline(always)]
    fn from(variant: EOCW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` writer - EOC
pub type EOC_W<'a, REG> = crate::BitWriter1C<'a, REG, EOCW>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear regular conversion complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCW::Clear)
    }
}
/**EOS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR {
    ///0: Regular sequence is not complete
    NotComplete = 0,
    ///1: Regular sequence complete
    Complete = 1,
}
impl From<EOSR> for bool {
    #[inline(always)]
    fn from(variant: EOSR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` reader - EOS
pub type EOS_R = crate::BitReader<EOSR>;
impl EOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSR {
        match self.bits {
            false => EOSR::NotComplete,
            true => EOSR::Complete,
        }
    }
    ///Regular sequence is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOSR::NotComplete
    }
    ///Regular sequence complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOSR::Complete
    }
}
/**EOS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSW {
    ///1: Clear regular sequence complete flag
    Clear = 1,
}
impl From<EOSW> for bool {
    #[inline(always)]
    fn from(variant: EOSW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` writer - EOS
pub type EOS_W<'a, REG> = crate::BitWriter1C<'a, REG, EOSW>;
impl<'a, REG> EOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear regular sequence complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSW::Clear)
    }
}
/**OVR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR {
    ///0: No overrun occurred
    NoOverrun = 0,
    ///1: Overrun occurred
    Overrun = 1,
}
impl From<OVRR> for bool {
    #[inline(always)]
    fn from(variant: OVRR) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` reader - OVR
pub type OVR_R = crate::BitReader<OVRR>;
impl OVR_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OVRR {
        match self.bits {
            false => OVRR::NoOverrun,
            true => OVRR::Overrun,
        }
    }
    ///No overrun occurred
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR::NoOverrun
    }
    ///Overrun occurred
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR::Overrun
    }
}
/**OVR

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW {
    ///1: Clear overrun occurred flag
    Clear = 1,
}
impl From<OVRW> for bool {
    #[inline(always)]
    fn from(variant: OVRW) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` writer - OVR
pub type OVR_W<'a, REG> = crate::BitWriter1C<'a, REG, OVRW>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear overrun occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRW::Clear)
    }
}
/**JEOC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCR {
    ///0: Injected conversion is not complete
    NotComplete = 0,
    ///1: Injected conversion complete
    Complete = 1,
}
impl From<JEOCR> for bool {
    #[inline(always)]
    fn from(variant: JEOCR) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC` reader - JEOC
pub type JEOC_R = crate::BitReader<JEOCR>;
impl JEOC_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOCR {
        match self.bits {
            false => JEOCR::NotComplete,
            true => JEOCR::Complete,
        }
    }
    ///Injected conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOCR::NotComplete
    }
    ///Injected conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOCR::Complete
    }
}
/**JEOC

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCW {
    ///1: Clear injected conversion complete flag
    Clear = 1,
}
impl From<JEOCW> for bool {
    #[inline(always)]
    fn from(variant: JEOCW) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC` writer - JEOC
pub type JEOC_W<'a, REG> = crate::BitWriter1C<'a, REG, JEOCW>;
impl<'a, REG> JEOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear injected conversion complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCW::Clear)
    }
}
/**JEOS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSR {
    ///0: Injected sequence is not complete
    NotComplete = 0,
    ///1: Injected sequence complete
    Complete = 1,
}
impl From<JEOSR> for bool {
    #[inline(always)]
    fn from(variant: JEOSR) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOS` reader - JEOS
pub type JEOS_R = crate::BitReader<JEOSR>;
impl JEOS_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JEOSR {
        match self.bits {
            false => JEOSR::NotComplete,
            true => JEOSR::Complete,
        }
    }
    ///Injected sequence is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOSR::NotComplete
    }
    ///Injected sequence complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOSR::Complete
    }
}
/**JEOS

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOSW {
    ///1: Clear Injected sequence complete flag
    Clear = 1,
}
impl From<JEOSW> for bool {
    #[inline(always)]
    fn from(variant: JEOSW) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOS` writer - JEOS
pub type JEOS_W<'a, REG> = crate::BitWriter1C<'a, REG, JEOSW>;
impl<'a, REG> JEOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear Injected sequence complete flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JEOSW::Clear)
    }
}
/**Analog watchdog %s flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1R {
    ///0: No analog watchdog event occurred
    NoEvent = 0,
    ///1: Analog watchdog event occurred
    Event = 1,
}
impl From<AWD1R> for bool {
    #[inline(always)]
    fn from(variant: AWD1R) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD(1-3)` reader - Analog watchdog %s flag
pub type AWD_R = crate::BitReader<AWD1R>;
impl AWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWD1R {
        match self.bits {
            false => AWD1R::NoEvent,
            true => AWD1R::Event,
        }
    }
    ///No analog watchdog event occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWD1R::NoEvent
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWD1R::Event
    }
}
/**Analog watchdog %s flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWD1W {
    ///1: Clear analog watchdog event occurred flag
    Clear = 1,
}
impl From<AWD1W> for bool {
    #[inline(always)]
    fn from(variant: AWD1W) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD(1-3)` writer - Analog watchdog %s flag
pub type AWD_W<'a, REG> = crate::BitWriter1C<'a, REG, AWD1W>;
impl<'a, REG> AWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear analog watchdog event occurred flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AWD1W::Clear)
    }
}
/**JQOVF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFR {
    ///0: No injected context queue overflow has occurred
    NoOverflow = 0,
    ///1: Injected context queue overflow has occurred
    Overflow = 1,
}
impl From<JQOVFR> for bool {
    #[inline(always)]
    fn from(variant: JQOVFR) -> Self {
        variant as u8 != 0
    }
}
///Field `JQOVF` reader - JQOVF
pub type JQOVF_R = crate::BitReader<JQOVFR>;
impl JQOVF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JQOVFR {
        match self.bits {
            false => JQOVFR::NoOverflow,
            true => JQOVFR::Overflow,
        }
    }
    ///No injected context queue overflow has occurred
    #[inline(always)]
    pub fn is_no_overflow(&self) -> bool {
        *self == JQOVFR::NoOverflow
    }
    ///Injected context queue overflow has occurred
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == JQOVFR::Overflow
    }
}
/**JQOVF

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JQOVFW {
    ///1: Clear injected context queue overflow flag
    Clear = 1,
}
impl From<JQOVFW> for bool {
    #[inline(always)]
    fn from(variant: JQOVFW) -> Self {
        variant as u8 != 0
    }
}
///Field `JQOVF` writer - JQOVF
pub type JQOVF_W<'a, REG> = crate::BitWriter1C<'a, REG, JQOVFW>;
impl<'a, REG> JQOVF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear injected context queue overflow flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JQOVFW::Clear)
    }
}
impl R {
    ///Bit 0 - ADRDY
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - EOSMP
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - EOC
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - EOS
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - OVR
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - JEOC
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - JEOS
    #[inline(always)]
    pub fn jeos(&self) -> JEOS_R {
        JEOS_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Analog watchdog (1-3) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1` field.</div>
    #[inline(always)]
    pub fn awd(&self, n: u8) -> AWD_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWD_R::new(((self.bits >> (n + 7)) & 1) != 0)
    }
    ///Iterator for array of:
    ///Analog watchdog (1-3) flag
    #[inline(always)]
    pub fn awd_iter(&self) -> impl Iterator<Item = AWD_R> + '_ {
        (0..3).map(move |n| AWD_R::new(((self.bits >> (n + 7)) & 1) != 0))
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - JQOVF
    #[inline(always)]
    pub fn jqovf(&self) -> JQOVF_R {
        JQOVF_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("jqovf", &self.jqovf())
            .field("awd1", &self.awd1())
            .field("awd2", &self.awd2())
            .field("awd3", &self.awd3())
            .field("jeos", &self.jeos())
            .field("jeoc", &self.jeoc())
            .field("ovr", &self.ovr())
            .field("eos", &self.eos())
            .field("eoc", &self.eoc())
            .field("eosmp", &self.eosmp())
            .field("adrdy", &self.adrdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADRDY
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W<'_, ISRrs> {
        ADRDY_W::new(self, 0)
    }
    ///Bit 1 - EOSMP
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W<'_, ISRrs> {
        EOSMP_W::new(self, 1)
    }
    ///Bit 2 - EOC
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<'_, ISRrs> {
        EOC_W::new(self, 2)
    }
    ///Bit 3 - EOS
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W<'_, ISRrs> {
        EOS_W::new(self, 3)
    }
    ///Bit 4 - OVR
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<'_, ISRrs> {
        OVR_W::new(self, 4)
    }
    ///Bit 5 - JEOC
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W<'_, ISRrs> {
        JEOC_W::new(self, 5)
    }
    ///Bit 6 - JEOS
    #[inline(always)]
    pub fn jeos(&mut self) -> JEOS_W<'_, ISRrs> {
        JEOS_W::new(self, 6)
    }
    ///Analog watchdog (1-3) flag
    ///
    ///<div class="warning">`n` is number of field in register. `n == 0` corresponds to `AWD1` field.</div>
    #[inline(always)]
    pub fn awd(&mut self, n: u8) -> AWD_W<'_, ISRrs> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        AWD_W::new(self, n + 7)
    }
    ///Bit 7 - Analog watchdog 1 flag
    #[inline(always)]
    pub fn awd1(&mut self) -> AWD_W<'_, ISRrs> {
        AWD_W::new(self, 7)
    }
    ///Bit 8 - Analog watchdog 2 flag
    #[inline(always)]
    pub fn awd2(&mut self) -> AWD_W<'_, ISRrs> {
        AWD_W::new(self, 8)
    }
    ///Bit 9 - Analog watchdog 3 flag
    #[inline(always)]
    pub fn awd3(&mut self) -> AWD_W<'_, ISRrs> {
        AWD_W::new(self, 9)
    }
    ///Bit 10 - JQOVF
    #[inline(always)]
    pub fn jqovf(&mut self) -> JQOVF_W<'_, ISRrs> {
        JQOVF_W::new(self, 10)
    }
}
/**interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#ADC1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07ff;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
