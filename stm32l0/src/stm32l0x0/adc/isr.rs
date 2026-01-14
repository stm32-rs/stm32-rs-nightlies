///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Register `ISR` writer
pub type W = crate::W<ISRrs>;
/**ADC ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYR {
    ///0: ADC not yet ready to start conversion
    NotReady = 0,
    ///1: ADC ready to start conversion
    Ready = 1,
}
impl From<ADRDYR> for bool {
    #[inline(always)]
    fn from(variant: ADRDYR) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` reader - ADC ready
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
    ///ADC not yet ready to start conversion
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == ADRDYR::NotReady
    }
    ///ADC ready to start conversion
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == ADRDYR::Ready
    }
}
/**ADC ready

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADRDYW {
    ///1: Clear the ADC ready flag
    Clear = 1,
}
impl From<ADRDYW> for bool {
    #[inline(always)]
    fn from(variant: ADRDYW) -> Self {
        variant as u8 != 0
    }
}
///Field `ADRDY` writer - ADC ready
pub type ADRDY_W<'a, REG> = crate::BitWriter1C<'a, REG, ADRDYW>;
impl<'a, REG> ADRDY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the ADC ready flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADRDYW::Clear)
    }
}
/**End of sampling flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPR {
    ///0: Not at the end of the samplings phase
    NotAtEnd = 0,
    ///1: End of sampling phase reached
    AtEnd = 1,
}
impl From<EOSMPR> for bool {
    #[inline(always)]
    fn from(variant: EOSMPR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` reader - End of sampling flag
pub type EOSMP_R = crate::BitReader<EOSMPR>;
impl EOSMP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOSMPR {
        match self.bits {
            false => EOSMPR::NotAtEnd,
            true => EOSMPR::AtEnd,
        }
    }
    ///Not at the end of the samplings phase
    #[inline(always)]
    pub fn is_not_at_end(&self) -> bool {
        *self == EOSMPR::NotAtEnd
    }
    ///End of sampling phase reached
    #[inline(always)]
    pub fn is_at_end(&self) -> bool {
        *self == EOSMPR::AtEnd
    }
}
/**End of sampling flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSMPW {
    ///1: Clear the sampling phase flag
    Clear = 1,
}
impl From<EOSMPW> for bool {
    #[inline(always)]
    fn from(variant: EOSMPW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOSMP` writer - End of sampling flag
pub type EOSMP_W<'a, REG> = crate::BitWriter1C<'a, REG, EOSMPW>;
impl<'a, REG> EOSMP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the sampling phase flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSMPW::Clear)
    }
}
/**End of conversion flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR {
    ///0: Channel conversion is not complete
    NotComplete = 0,
    ///1: Channel conversion complete
    Complete = 1,
}
impl From<EOCR> for bool {
    #[inline(always)]
    fn from(variant: EOCR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` reader - End of conversion flag
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
    ///Channel conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR::NotComplete
    }
    ///Channel conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR::Complete
    }
}
/**End of conversion flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW {
    ///1: Clear the channel conversion flag
    Clear = 1,
}
impl From<EOCW> for bool {
    #[inline(always)]
    fn from(variant: EOCW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` writer - End of conversion flag
pub type EOC_W<'a, REG> = crate::BitWriter1C<'a, REG, EOCW>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the channel conversion flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCW::Clear)
    }
}
/**End of sequence flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSR {
    ///0: Conversion sequence is not complete
    NotComplete = 0,
    ///1: Conversion sequence complete
    Complete = 1,
}
impl From<EOSR> for bool {
    #[inline(always)]
    fn from(variant: EOSR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` reader - End of sequence flag
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
    ///Conversion sequence is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOSR::NotComplete
    }
    ///Conversion sequence complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOSR::Complete
    }
}
/**End of sequence flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOSW {
    ///1: Clear the conversion sequence flag
    Clear = 1,
}
impl From<EOSW> for bool {
    #[inline(always)]
    fn from(variant: EOSW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOS` writer - End of sequence flag
pub type EOS_W<'a, REG> = crate::BitWriter1C<'a, REG, EOSW>;
impl<'a, REG> EOS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the conversion sequence flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOSW::Clear)
    }
}
/**ADC overrun

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
///Field `OVR` reader - ADC overrun
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
/**ADC overrun

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRW {
    ///1: Clear the overrun flag
    Clear = 1,
}
impl From<OVRW> for bool {
    #[inline(always)]
    fn from(variant: OVRW) -> Self {
        variant as u8 != 0
    }
}
///Field `OVR` writer - ADC overrun
pub type OVR_W<'a, REG> = crate::BitWriter1C<'a, REG, OVRW>;
impl<'a, REG> OVR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the overrun flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRW::Clear)
    }
}
/**Analog watchdog flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDR {
    ///0: No analog watchdog event occurred
    NoEvent = 0,
    ///1: Analog watchdog event occurred
    Event = 1,
}
impl From<AWDR> for bool {
    #[inline(always)]
    fn from(variant: AWDR) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD` reader - Analog watchdog flag
pub type AWD_R = crate::BitReader<AWDR>;
impl AWD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> AWDR {
        match self.bits {
            false => AWDR::NoEvent,
            true => AWDR::Event,
        }
    }
    ///No analog watchdog event occurred
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == AWDR::NoEvent
    }
    ///Analog watchdog event occurred
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == AWDR::Event
    }
}
/**Analog watchdog flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWDW {
    ///1: Clear the analog watchdog event flag
    Clear = 1,
}
impl From<AWDW> for bool {
    #[inline(always)]
    fn from(variant: AWDW) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD` writer - Analog watchdog flag
pub type AWD_W<'a, REG> = crate::BitWriter1C<'a, REG, AWDW>;
impl<'a, REG> AWD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the analog watchdog event flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(AWDW::Clear)
    }
}
/**End Of Calibration flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALR {
    ///0: Calibration is not complete
    NotComplete = 0,
    ///1: Calibration complete
    Complete = 1,
}
impl From<EOCALR> for bool {
    #[inline(always)]
    fn from(variant: EOCALR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCAL` reader - End Of Calibration flag
pub type EOCAL_R = crate::BitReader<EOCALR>;
impl EOCAL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> EOCALR {
        match self.bits {
            false => EOCALR::NotComplete,
            true => EOCALR::Complete,
        }
    }
    ///Calibration is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCALR::NotComplete
    }
    ///Calibration complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCALR::Complete
    }
}
/**End Of Calibration flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCALW {
    ///1: Clear the calibration flag
    Clear = 1,
}
impl From<EOCALW> for bool {
    #[inline(always)]
    fn from(variant: EOCALW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOCAL` writer - End Of Calibration flag
pub type EOCAL_W<'a, REG> = crate::BitWriter1C<'a, REG, EOCALW>;
impl<'a, REG> EOCAL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the calibration flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCALW::Clear)
    }
}
impl R {
    ///Bit 0 - ADC ready
    #[inline(always)]
    pub fn adrdy(&self) -> ADRDY_R {
        ADRDY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    pub fn eosmp(&self) -> EOSMP_R {
        EOSMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - End of sequence flag
    #[inline(always)]
    pub fn eos(&self) -> EOS_R {
        EOS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 11 - End Of Calibration flag
    #[inline(always)]
    pub fn eocal(&self) -> EOCAL_R {
        EOCAL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("adrdy", &self.adrdy())
            .field("eosmp", &self.eosmp())
            .field("eoc", &self.eoc())
            .field("eos", &self.eos())
            .field("ovr", &self.ovr())
            .field("awd", &self.awd())
            .field("eocal", &self.eocal())
            .finish()
    }
}
impl W {
    ///Bit 0 - ADC ready
    #[inline(always)]
    pub fn adrdy(&mut self) -> ADRDY_W<'_, ISRrs> {
        ADRDY_W::new(self, 0)
    }
    ///Bit 1 - End of sampling flag
    #[inline(always)]
    pub fn eosmp(&mut self) -> EOSMP_W<'_, ISRrs> {
        EOSMP_W::new(self, 1)
    }
    ///Bit 2 - End of conversion flag
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<'_, ISRrs> {
        EOC_W::new(self, 2)
    }
    ///Bit 3 - End of sequence flag
    #[inline(always)]
    pub fn eos(&mut self) -> EOS_W<'_, ISRrs> {
        EOS_W::new(self, 3)
    }
    ///Bit 4 - ADC overrun
    #[inline(always)]
    pub fn ovr(&mut self) -> OVR_W<'_, ISRrs> {
        OVR_W::new(self, 4)
    }
    ///Bit 7 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W<'_, ISRrs> {
        AWD_W::new(self, 7)
    }
    ///Bit 11 - End Of Calibration flag
    #[inline(always)]
    pub fn eocal(&mut self) -> EOCAL_W<'_, ISRrs> {
        EOCAL_W::new(self, 11)
    }
}
/**interrupt and status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x0.html#ADC:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`write(|w| ..)` method takes [`isr::W`](W) writer structure
impl crate::Writable for ISRrs {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x089f;
}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
