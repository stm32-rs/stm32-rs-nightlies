///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
/**LPTIM Enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE {
    ///0: LPTIM is disabled
    Disabled = 0,
    ///1: LPTIM is enabled
    Enabled = 1,
}
impl From<ENABLE> for bool {
    #[inline(always)]
    fn from(variant: ENABLE) -> Self {
        variant as u8 != 0
    }
}
///Field `ENABLE` reader - LPTIM Enable
pub type ENABLE_R = crate::BitReader<ENABLE>;
impl ENABLE_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE {
        match self.bits {
            false => ENABLE::Disabled,
            true => ENABLE::Enabled,
        }
    }
    ///LPTIM is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE::Disabled
    }
    ///LPTIM is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE::Enabled
    }
}
///Field `ENABLE` writer - LPTIM Enable
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Disabled)
    }
    ///LPTIM is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Enabled)
    }
}
/**LPTIM start in single mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNGSTRTW {
    ///1: LPTIM start in Single mode
    Start = 1,
}
impl From<SNGSTRTW> for bool {
    #[inline(always)]
    fn from(variant: SNGSTRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `SNGSTRT` reader - LPTIM start in single mode
pub type SNGSTRT_R = crate::BitReader<SNGSTRTW>;
impl SNGSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SNGSTRTW> {
        match self.bits {
            true => Some(SNGSTRTW::Start),
            _ => None,
        }
    }
    ///LPTIM start in Single mode
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SNGSTRTW::Start
    }
}
///Field `SNGSTRT` writer - LPTIM start in single mode
pub type SNGSTRT_W<'a, REG> = crate::BitWriter<'a, REG, SNGSTRTW>;
impl<'a, REG> SNGSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///LPTIM start in Single mode
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SNGSTRTW::Start)
    }
}
/**Timer start in continuous mode

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTSTRTW {
    ///1: Timer start in Continuous mode
    Start = 1,
}
impl From<CNTSTRTW> for bool {
    #[inline(always)]
    fn from(variant: CNTSTRTW) -> Self {
        variant as u8 != 0
    }
}
///Field `CNTSTRT` reader - Timer start in continuous mode
pub type CNTSTRT_R = crate::BitReader<CNTSTRTW>;
impl CNTSTRT_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<CNTSTRTW> {
        match self.bits {
            true => Some(CNTSTRTW::Start),
            _ => None,
        }
    }
    ///Timer start in Continuous mode
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CNTSTRTW::Start
    }
}
///Field `CNTSTRT` writer - Timer start in continuous mode
pub type CNTSTRT_W<'a, REG> = crate::BitWriter<'a, REG, CNTSTRTW>;
impl<'a, REG> CNTSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Timer start in Continuous mode
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CNTSTRTW::Start)
    }
}
impl R {
    ///Bit 0 - LPTIM Enable
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - LPTIM start in single mode
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Timer start in continuous mode
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("cntstrt", &self.cntstrt())
            .field("sngstrt", &self.sngstrt())
            .field("enable", &self.enable())
            .finish()
    }
}
impl W {
    ///Bit 0 - LPTIM Enable
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W<'_, CRrs> {
        ENABLE_W::new(self, 0)
    }
    ///Bit 1 - LPTIM start in single mode
    #[inline(always)]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<'_, CRrs> {
        SNGSTRT_W::new(self, 1)
    }
    ///Bit 2 - Timer start in continuous mode
    #[inline(always)]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<'_, CRrs> {
        CNTSTRT_W::new(self, 2)
    }
}
/**Control Register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F777.html#LPTIM1:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
