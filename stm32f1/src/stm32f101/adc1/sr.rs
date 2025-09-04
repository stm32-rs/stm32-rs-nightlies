///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
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
    ///0: Clear the analog watchdog event flag
    Clear = 0,
}
impl From<AWDW> for bool {
    #[inline(always)]
    fn from(variant: AWDW) -> Self {
        variant as u8 != 0
    }
}
///Field `AWD` writer - Analog watchdog flag
pub type AWD_W<'a, REG> = crate::BitWriter0C<'a, REG, AWDW>;
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
/**Regular channel end of conversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCR {
    ///0: Conversion is not complete
    NotComplete = 0,
    ///1: Conversion complete
    Complete = 1,
}
impl From<EOCR> for bool {
    #[inline(always)]
    fn from(variant: EOCR) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` reader - Regular channel end of conversion
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
    ///Conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == EOCR::NotComplete
    }
    ///Conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == EOCR::Complete
    }
}
/**Regular channel end of conversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOCW {
    ///0: Clear End of conversion flag
    Clear = 0,
}
impl From<EOCW> for bool {
    #[inline(always)]
    fn from(variant: EOCW) -> Self {
        variant as u8 != 0
    }
}
///Field `EOC` writer - Regular channel end of conversion
pub type EOC_W<'a, REG> = crate::BitWriter0C<'a, REG, EOCW>;
impl<'a, REG> EOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear End of conversion flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOCW::Clear)
    }
}
/**Injected channel end of conversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCR {
    ///0: Conversion is not complete
    NotComplete = 0,
    ///1: Conversion complete
    Complete = 1,
}
impl From<JEOCR> for bool {
    #[inline(always)]
    fn from(variant: JEOCR) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC` reader - Injected channel end of conversion
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
    ///Conversion is not complete
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == JEOCR::NotComplete
    }
    ///Conversion complete
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == JEOCR::Complete
    }
}
/**Injected channel end of conversion

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JEOCW {
    ///0: Clear Injected channel end of conversion flag
    Clear = 0,
}
impl From<JEOCW> for bool {
    #[inline(always)]
    fn from(variant: JEOCW) -> Self {
        variant as u8 != 0
    }
}
///Field `JEOC` writer - Injected channel end of conversion
pub type JEOC_W<'a, REG> = crate::BitWriter0C<'a, REG, JEOCW>;
impl<'a, REG> JEOC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear Injected channel end of conversion flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JEOCW::Clear)
    }
}
/**Injected channel start flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRTR {
    ///0: No injected group conversion started
    NotStarted = 0,
    ///1: Injected group conversion has started
    Started = 1,
}
impl From<JSTRTR> for bool {
    #[inline(always)]
    fn from(variant: JSTRTR) -> Self {
        variant as u8 != 0
    }
}
///Field `JSTRT` reader - Injected channel start flag
pub type JSTRT_R = crate::BitReader<JSTRTR>;
impl JSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> JSTRTR {
        match self.bits {
            false => JSTRTR::NotStarted,
            true => JSTRTR::Started,
        }
    }
    ///No injected group conversion started
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == JSTRTR::NotStarted
    }
    ///Injected group conversion has started
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == JSTRTR::Started
    }
}
/**Injected channel start flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JSTRTW {
    ///0: Clear Injected channel Start flag
    Clear = 0,
}
impl From<JSTRTW> for bool {
    #[inline(always)]
    fn from(variant: JSTRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `JSTRT` writer - Injected channel start flag
pub type JSTRT_W<'a, REG> = crate::BitWriter0C<'a, REG, JSTRTW>;
impl<'a, REG> JSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear Injected channel Start flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(JSTRTW::Clear)
    }
}
/**Regular channel start flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTR {
    ///0: No regular channel conversion started
    NotStarted = 0,
    ///1: Regular channel conversion has started
    Started = 1,
}
impl From<STRTR> for bool {
    #[inline(always)]
    fn from(variant: STRTR) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` reader - Regular channel start flag
pub type STRT_R = crate::BitReader<STRTR>;
impl STRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> STRTR {
        match self.bits {
            false => STRTR::NotStarted,
            true => STRTR::Started,
        }
    }
    ///No regular channel conversion started
    #[inline(always)]
    pub fn is_not_started(&self) -> bool {
        *self == STRTR::NotStarted
    }
    ///Regular channel conversion has started
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == STRTR::Started
    }
}
/**Regular channel start flag

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRTW {
    ///0: Clear the Regular channel Start flag
    Clear = 0,
}
impl From<STRTW> for bool {
    #[inline(always)]
    fn from(variant: STRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `STRT` writer - Regular channel start flag
pub type STRT_W<'a, REG> = crate::BitWriter0C<'a, REG, STRTW>;
impl<'a, REG> STRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Clear the Regular channel Start flag
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(STRTW::Clear)
    }
}
impl R {
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&self) -> AWD_R {
        AWD_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    pub fn eoc(&self) -> EOC_R {
        EOC_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    pub fn jeoc(&self) -> JEOC_R {
        JEOC_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    pub fn jstrt(&self) -> JSTRT_R {
        JSTRT_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    pub fn strt(&self) -> STRT_R {
        STRT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("strt", &self.strt())
            .field("jstrt", &self.jstrt())
            .field("jeoc", &self.jeoc())
            .field("eoc", &self.eoc())
            .field("awd", &self.awd())
            .finish()
    }
}
impl W {
    ///Bit 0 - Analog watchdog flag
    #[inline(always)]
    pub fn awd(&mut self) -> AWD_W<SRrs> {
        AWD_W::new(self, 0)
    }
    ///Bit 1 - Regular channel end of conversion
    #[inline(always)]
    pub fn eoc(&mut self) -> EOC_W<SRrs> {
        EOC_W::new(self, 1)
    }
    ///Bit 2 - Injected channel end of conversion
    #[inline(always)]
    pub fn jeoc(&mut self) -> JEOC_W<SRrs> {
        JEOC_W::new(self, 2)
    }
    ///Bit 3 - Injected channel start flag
    #[inline(always)]
    pub fn jstrt(&mut self) -> JSTRT_W<SRrs> {
        JSTRT_W::new(self, 3)
    }
    ///Bit 4 - Regular channel start flag
    #[inline(always)]
    pub fn strt(&mut self) -> STRT_W<SRrs> {
        STRT_W::new(self, 4)
    }
}
/**status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#ADC1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0x1f;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
