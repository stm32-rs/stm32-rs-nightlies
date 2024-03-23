#[doc = "Register `CR` reader"]
pub type R = crate::R<CRrs>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CRrs>;
#[doc = "ENABLE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENABLE {
    #[doc = "0: LPTIM is disabled"]
    Disabled = 0,
    #[doc = "1: LPTIM is enabled"]
    Enabled = 1,
}
impl From<ENABLE> for bool {
    #[inline(always)]
    fn from(variant: ENABLE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - ENABLE"]
pub type ENABLE_R = crate::BitReader<ENABLE>;
impl ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENABLE {
        match self.bits {
            false => ENABLE::Disabled,
            true => ENABLE::Enabled,
        }
    }
    #[doc = "LPTIM is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ENABLE::Disabled
    }
    #[doc = "LPTIM is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ENABLE::Enabled
    }
}
#[doc = "Field `ENABLE` writer - ENABLE"]
pub type ENABLE_W<'a, REG> = crate::BitWriter<'a, REG, ENABLE>;
impl<'a, REG> ENABLE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Disabled)
    }
    #[doc = "LPTIM is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(ENABLE::Enabled)
    }
}
#[doc = "SNGSTRT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SNGSTRTW {
    #[doc = "1: LPTIM start in Single mode"]
    Start = 1,
}
impl From<SNGSTRTW> for bool {
    #[inline(always)]
    fn from(variant: SNGSTRTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SNGSTRT` reader - SNGSTRT"]
pub type SNGSTRT_R = crate::BitReader<SNGSTRTW>;
impl SNGSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SNGSTRTW> {
        match self.bits {
            true => Some(SNGSTRTW::Start),
            _ => None,
        }
    }
    #[doc = "LPTIM start in Single mode"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == SNGSTRTW::Start
    }
}
#[doc = "Field `SNGSTRT` writer - SNGSTRT"]
pub type SNGSTRT_W<'a, REG> = crate::BitWriter<'a, REG, SNGSTRTW>;
impl<'a, REG> SNGSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LPTIM start in Single mode"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(SNGSTRTW::Start)
    }
}
#[doc = "CNTSTRT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNTSTRTW {
    #[doc = "1: Timer start in Continuous mode"]
    Start = 1,
}
impl From<CNTSTRTW> for bool {
    #[inline(always)]
    fn from(variant: CNTSTRTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTSTRT` reader - CNTSTRT"]
pub type CNTSTRT_R = crate::BitReader<CNTSTRTW>;
impl CNTSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CNTSTRTW> {
        match self.bits {
            true => Some(CNTSTRTW::Start),
            _ => None,
        }
    }
    #[doc = "Timer start in Continuous mode"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == CNTSTRTW::Start
    }
}
#[doc = "Field `CNTSTRT` writer - CNTSTRT"]
pub type CNTSTRT_W<'a, REG> = crate::BitWriter<'a, REG, CNTSTRTW>;
impl<'a, REG> CNTSTRT_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Timer start in Continuous mode"]
    #[inline(always)]
    pub fn start(self) -> &'a mut crate::W<REG> {
        self.variant(CNTSTRTW::Start)
    }
}
#[doc = "COUNTRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTRSTR {
    #[doc = "0: Triggering of reset is possible"]
    Idle = 0,
    #[doc = "1: Reset in progress, do not write 1 to this field"]
    Busy = 1,
}
impl From<COUNTRSTR> for bool {
    #[inline(always)]
    fn from(variant: COUNTRSTR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUNTRST` reader - COUNTRST"]
pub type COUNTRST_R = crate::BitReader<COUNTRSTR>;
impl COUNTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> COUNTRSTR {
        match self.bits {
            false => COUNTRSTR::Idle,
            true => COUNTRSTR::Busy,
        }
    }
    #[doc = "Triggering of reset is possible"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == COUNTRSTR::Idle
    }
    #[doc = "Reset in progress, do not write 1 to this field"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == COUNTRSTR::Busy
    }
}
#[doc = "COUNTRST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COUNTRSTW {
    #[doc = "1: Trigger synchronous reset of CNT (3 LPTimer core clock cycles)"]
    Reset = 1,
}
impl From<COUNTRSTW> for bool {
    #[inline(always)]
    fn from(variant: COUNTRSTW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `COUNTRST` writer - COUNTRST"]
pub type COUNTRST_W<'a, REG> = crate::BitWriter<'a, REG, COUNTRSTW>;
impl<'a, REG> COUNTRST_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger synchronous reset of CNT (3 LPTimer core clock cycles)"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(COUNTRSTW::Reset)
    }
}
#[doc = "RSTARE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSTARE {
    #[doc = "0: CNT Register reads do not trigger reset"]
    Disabled = 0,
    #[doc = "1: CNT Register reads trigger reset of LPTIM"]
    Enabled = 1,
}
impl From<RSTARE> for bool {
    #[inline(always)]
    fn from(variant: RSTARE) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RSTARE` reader - RSTARE"]
pub type RSTARE_R = crate::BitReader<RSTARE>;
impl RSTARE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RSTARE {
        match self.bits {
            false => RSTARE::Disabled,
            true => RSTARE::Enabled,
        }
    }
    #[doc = "CNT Register reads do not trigger reset"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RSTARE::Disabled
    }
    #[doc = "CNT Register reads trigger reset of LPTIM"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RSTARE::Enabled
    }
}
#[doc = "Field `RSTARE` writer - RSTARE"]
pub type RSTARE_W<'a, REG> = crate::BitWriter<'a, REG, RSTARE>;
impl<'a, REG> RSTARE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CNT Register reads do not trigger reset"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTARE::Disabled)
    }
    #[doc = "CNT Register reads trigger reset of LPTIM"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RSTARE::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SNGSTRT"]
    #[inline(always)]
    pub fn sngstrt(&self) -> SNGSTRT_R {
        SNGSTRT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CNTSTRT"]
    #[inline(always)]
    pub fn cntstrt(&self) -> CNTSTRT_R {
        CNTSTRT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COUNTRST"]
    #[inline(always)]
    pub fn countrst(&self) -> COUNTRST_R {
        COUNTRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RSTARE"]
    #[inline(always)]
    pub fn rstare(&self) -> RSTARE_R {
        RSTARE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ENABLE"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> ENABLE_W<CRrs> {
        ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - SNGSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn sngstrt(&mut self) -> SNGSTRT_W<CRrs> {
        SNGSTRT_W::new(self, 1)
    }
    #[doc = "Bit 2 - CNTSTRT"]
    #[inline(always)]
    #[must_use]
    pub fn cntstrt(&mut self) -> CNTSTRT_W<CRrs> {
        CNTSTRT_W::new(self, 2)
    }
    #[doc = "Bit 3 - COUNTRST"]
    #[inline(always)]
    #[must_use]
    pub fn countrst(&mut self) -> COUNTRST_W<CRrs> {
        COUNTRST_W::new(self, 3)
    }
    #[doc = "Bit 4 - RSTARE"]
    #[inline(always)]
    #[must_use]
    pub fn rstare(&mut self) -> RSTARE_W<CRrs> {
        RSTARE_W::new(self, 4)
    }
}
#[doc = "control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CRrs {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CRrs {
    const RESET_VALUE: u32 = 0;
}
