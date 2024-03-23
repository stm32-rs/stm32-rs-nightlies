#[doc = "Register `ACR` reader"]
pub type R = crate::R<ACRrs>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<ACRrs>;
#[doc = "Latency\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LATENCY {
    #[doc = "0: Zero wait state is used to read a word in the NVM"]
    Ws0 = 0,
    #[doc = "1: One wait state is used to read a word in the NVM"]
    Ws1 = 1,
}
impl From<LATENCY> for bool {
    #[inline(always)]
    fn from(variant: LATENCY) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LATENCY` reader - Latency"]
pub type LATENCY_R = crate::BitReader<LATENCY>;
impl LATENCY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> LATENCY {
        match self.bits {
            false => LATENCY::Ws0,
            true => LATENCY::Ws1,
        }
    }
    #[doc = "Zero wait state is used to read a word in the NVM"]
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY::Ws0
    }
    #[doc = "One wait state is used to read a word in the NVM"]
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY::Ws1
    }
}
#[doc = "Field `LATENCY` writer - Latency"]
pub type LATENCY_W<'a, REG> = crate::BitWriter<'a, REG, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Zero wait state is used to read a word in the NVM"]
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws0)
    }
    #[doc = "One wait state is used to read a word in the NVM"]
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws1)
    }
}
#[doc = "Prefetch enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    #[doc = "0: Prefetch is disabled"]
    Disabled = 0,
    #[doc = "1: Prefetch is enabled"]
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRFTEN` reader - Prefetch enable"]
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::Disabled,
            true => PRFTEN::Enabled,
        }
    }
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
#[doc = "Field `PRFTEN` writer - Prefetch enable"]
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Prefetch is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    #[doc = "Prefetch is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
#[doc = "Flash mode during Sleep\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_PD {
    #[doc = "0: When the device is in Sleep mode, the NVM is in Idle mode"]
    NvmidleMode = 0,
    #[doc = "1: When the device is in Sleep mode, the NVM is in power-down mode"]
    NvmpwrDownMode = 1,
}
impl From<SLEEP_PD> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_PD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEEP_PD` reader - Flash mode during Sleep"]
pub type SLEEP_PD_R = crate::BitReader<SLEEP_PD>;
impl SLEEP_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SLEEP_PD {
        match self.bits {
            false => SLEEP_PD::NvmidleMode,
            true => SLEEP_PD::NvmpwrDownMode,
        }
    }
    #[doc = "When the device is in Sleep mode, the NVM is in Idle mode"]
    #[inline(always)]
    pub fn is_nvmidle_mode(&self) -> bool {
        *self == SLEEP_PD::NvmidleMode
    }
    #[doc = "When the device is in Sleep mode, the NVM is in power-down mode"]
    #[inline(always)]
    pub fn is_nvmpwr_down_mode(&self) -> bool {
        *self == SLEEP_PD::NvmpwrDownMode
    }
}
#[doc = "Field `SLEEP_PD` writer - Flash mode during Sleep"]
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG, SLEEP_PD>;
impl<'a, REG> SLEEP_PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When the device is in Sleep mode, the NVM is in Idle mode"]
    #[inline(always)]
    pub fn nvmidle_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_PD::NvmidleMode)
    }
    #[doc = "When the device is in Sleep mode, the NVM is in power-down mode"]
    #[inline(always)]
    pub fn nvmpwr_down_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_PD::NvmpwrDownMode)
    }
}
#[doc = "Flash mode during Run\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_PD {
    #[doc = "0: When the device is in Run mode, the NVM is in Idle mode"]
    NvmidleMode = 0,
    #[doc = "1: When the device is in Run mode, the NVM is in power-down mode"]
    NvmpwrDownMode = 1,
}
impl From<RUN_PD> for bool {
    #[inline(always)]
    fn from(variant: RUN_PD) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN_PD` reader - Flash mode during Run"]
pub type RUN_PD_R = crate::BitReader<RUN_PD>;
impl RUN_PD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RUN_PD {
        match self.bits {
            false => RUN_PD::NvmidleMode,
            true => RUN_PD::NvmpwrDownMode,
        }
    }
    #[doc = "When the device is in Run mode, the NVM is in Idle mode"]
    #[inline(always)]
    pub fn is_nvmidle_mode(&self) -> bool {
        *self == RUN_PD::NvmidleMode
    }
    #[doc = "When the device is in Run mode, the NVM is in power-down mode"]
    #[inline(always)]
    pub fn is_nvmpwr_down_mode(&self) -> bool {
        *self == RUN_PD::NvmpwrDownMode
    }
}
#[doc = "Field `RUN_PD` writer - Flash mode during Run"]
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG, RUN_PD>;
impl<'a, REG> RUN_PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When the device is in Run mode, the NVM is in Idle mode"]
    #[inline(always)]
    pub fn nvmidle_mode(self) -> &'a mut crate::W<REG> {
        self.variant(RUN_PD::NvmidleMode)
    }
    #[doc = "When the device is in Run mode, the NVM is in power-down mode"]
    #[inline(always)]
    pub fn nvmpwr_down_mode(self) -> &'a mut crate::W<REG> {
        self.variant(RUN_PD::NvmpwrDownMode)
    }
}
#[doc = "Disable Buffer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISAB_BUF {
    #[doc = "0: The buffers are enabled"]
    Enabled = 0,
    #[doc = "1: The buffers are disabled"]
    Disabled = 1,
}
impl From<DISAB_BUF> for bool {
    #[inline(always)]
    fn from(variant: DISAB_BUF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISAB_BUF` reader - Disable Buffer"]
pub type DISAB_BUF_R = crate::BitReader<DISAB_BUF>;
impl DISAB_BUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DISAB_BUF {
        match self.bits {
            false => DISAB_BUF::Enabled,
            true => DISAB_BUF::Disabled,
        }
    }
    #[doc = "The buffers are enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISAB_BUF::Enabled
    }
    #[doc = "The buffers are disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISAB_BUF::Disabled
    }
}
#[doc = "Field `DISAB_BUF` writer - Disable Buffer"]
pub type DISAB_BUF_W<'a, REG> = crate::BitWriter<'a, REG, DISAB_BUF>;
impl<'a, REG> DISAB_BUF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The buffers are enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISAB_BUF::Enabled)
    }
    #[doc = "The buffers are disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISAB_BUF::Disabled)
    }
}
#[doc = "Pre-read data address\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRE_READ {
    #[doc = "0: The pre-read is disabled"]
    Disabled = 0,
    #[doc = "1: The pre-read is enabled"]
    Enabled = 1,
}
impl From<PRE_READ> for bool {
    #[inline(always)]
    fn from(variant: PRE_READ) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRE_READ` reader - Pre-read data address"]
pub type PRE_READ_R = crate::BitReader<PRE_READ>;
impl PRE_READ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PRE_READ {
        match self.bits {
            false => PRE_READ::Disabled,
            true => PRE_READ::Enabled,
        }
    }
    #[doc = "The pre-read is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRE_READ::Disabled
    }
    #[doc = "The pre-read is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRE_READ::Enabled
    }
}
#[doc = "Field `PRE_READ` writer - Pre-read data address"]
pub type PRE_READ_W<'a, REG> = crate::BitWriter<'a, REG, PRE_READ>;
impl<'a, REG> PRE_READ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The pre-read is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRE_READ::Disabled)
    }
    #[doc = "The pre-read is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRE_READ::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Latency"]
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Prefetch enable"]
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Flash mode during Sleep"]
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Flash mode during Run"]
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable Buffer"]
    #[inline(always)]
    pub fn disab_buf(&self) -> DISAB_BUF_R {
        DISAB_BUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Pre-read data address"]
    #[inline(always)]
    pub fn pre_read(&self) -> PRE_READ_R {
        PRE_READ_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Latency"]
    #[inline(always)]
    #[must_use]
    pub fn latency(&mut self) -> LATENCY_W<ACRrs> {
        LATENCY_W::new(self, 0)
    }
    #[doc = "Bit 1 - Prefetch enable"]
    #[inline(always)]
    #[must_use]
    pub fn prften(&mut self) -> PRFTEN_W<ACRrs> {
        PRFTEN_W::new(self, 1)
    }
    #[doc = "Bit 3 - Flash mode during Sleep"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<ACRrs> {
        SLEEP_PD_W::new(self, 3)
    }
    #[doc = "Bit 4 - Flash mode during Run"]
    #[inline(always)]
    #[must_use]
    pub fn run_pd(&mut self) -> RUN_PD_W<ACRrs> {
        RUN_PD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Disable Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn disab_buf(&mut self) -> DISAB_BUF_W<ACRrs> {
        DISAB_BUF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Pre-read data address"]
    #[inline(always)]
    #[must_use]
    pub fn pre_read(&mut self) -> PRE_READ_W<ACRrs> {
        PRE_READ_W::new(self, 6)
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`acr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for ACRrs {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for ACRrs {
    const RESET_VALUE: u32 = 0;
}
