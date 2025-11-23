///Register `ACR` reader
pub type R = crate::R<ACRrs>;
///Register `ACR` writer
pub type W = crate::W<ACRrs>;
/**Latency

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LATENCY {
    ///0: Zero wait state is used to read a word in the NVM
    Ws0 = 0,
    ///1: One wait state is used to read a word in the NVM
    Ws1 = 1,
}
impl From<LATENCY> for bool {
    #[inline(always)]
    fn from(variant: LATENCY) -> Self {
        variant as u8 != 0
    }
}
///Field `LATENCY` reader - Latency
pub type LATENCY_R = crate::BitReader<LATENCY>;
impl LATENCY_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> LATENCY {
        match self.bits {
            false => LATENCY::Ws0,
            true => LATENCY::Ws1,
        }
    }
    ///Zero wait state is used to read a word in the NVM
    #[inline(always)]
    pub fn is_ws0(&self) -> bool {
        *self == LATENCY::Ws0
    }
    ///One wait state is used to read a word in the NVM
    #[inline(always)]
    pub fn is_ws1(&self) -> bool {
        *self == LATENCY::Ws1
    }
}
///Field `LATENCY` writer - Latency
pub type LATENCY_W<'a, REG> = crate::BitWriter<'a, REG, LATENCY>;
impl<'a, REG> LATENCY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Zero wait state is used to read a word in the NVM
    #[inline(always)]
    pub fn ws0(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws0)
    }
    ///One wait state is used to read a word in the NVM
    #[inline(always)]
    pub fn ws1(self) -> &'a mut crate::W<REG> {
        self.variant(LATENCY::Ws1)
    }
}
/**Prefetch enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRFTEN {
    ///0: Prefetch is disabled
    Disabled = 0,
    ///1: Prefetch is enabled
    Enabled = 1,
}
impl From<PRFTEN> for bool {
    #[inline(always)]
    fn from(variant: PRFTEN) -> Self {
        variant as u8 != 0
    }
}
///Field `PRFTEN` reader - Prefetch enable
pub type PRFTEN_R = crate::BitReader<PRFTEN>;
impl PRFTEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRFTEN {
        match self.bits {
            false => PRFTEN::Disabled,
            true => PRFTEN::Enabled,
        }
    }
    ///Prefetch is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRFTEN::Disabled
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRFTEN::Enabled
    }
}
///Field `PRFTEN` writer - Prefetch enable
pub type PRFTEN_W<'a, REG> = crate::BitWriter<'a, REG, PRFTEN>;
impl<'a, REG> PRFTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Prefetch is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Disabled)
    }
    ///Prefetch is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRFTEN::Enabled)
    }
}
/**Flash mode during Sleep

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLEEP_PD {
    ///0: When the device is in Sleep mode, the NVM is in Idle mode
    NvmidleMode = 0,
    ///1: When the device is in Sleep mode, the NVM is in power-down mode
    NvmpwrDownMode = 1,
}
impl From<SLEEP_PD> for bool {
    #[inline(always)]
    fn from(variant: SLEEP_PD) -> Self {
        variant as u8 != 0
    }
}
///Field `SLEEP_PD` reader - Flash mode during Sleep
pub type SLEEP_PD_R = crate::BitReader<SLEEP_PD>;
impl SLEEP_PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> SLEEP_PD {
        match self.bits {
            false => SLEEP_PD::NvmidleMode,
            true => SLEEP_PD::NvmpwrDownMode,
        }
    }
    ///When the device is in Sleep mode, the NVM is in Idle mode
    #[inline(always)]
    pub fn is_nvmidle_mode(&self) -> bool {
        *self == SLEEP_PD::NvmidleMode
    }
    ///When the device is in Sleep mode, the NVM is in power-down mode
    #[inline(always)]
    pub fn is_nvmpwr_down_mode(&self) -> bool {
        *self == SLEEP_PD::NvmpwrDownMode
    }
}
///Field `SLEEP_PD` writer - Flash mode during Sleep
pub type SLEEP_PD_W<'a, REG> = crate::BitWriter<'a, REG, SLEEP_PD>;
impl<'a, REG> SLEEP_PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When the device is in Sleep mode, the NVM is in Idle mode
    #[inline(always)]
    pub fn nvmidle_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_PD::NvmidleMode)
    }
    ///When the device is in Sleep mode, the NVM is in power-down mode
    #[inline(always)]
    pub fn nvmpwr_down_mode(self) -> &'a mut crate::W<REG> {
        self.variant(SLEEP_PD::NvmpwrDownMode)
    }
}
/**Flash mode during Run

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUN_PD {
    ///0: When the device is in Run mode, the NVM is in Idle mode
    NvmidleMode = 0,
    ///1: When the device is in Run mode, the NVM is in power-down mode
    NvmpwrDownMode = 1,
}
impl From<RUN_PD> for bool {
    #[inline(always)]
    fn from(variant: RUN_PD) -> Self {
        variant as u8 != 0
    }
}
///Field `RUN_PD` reader - Flash mode during Run
pub type RUN_PD_R = crate::BitReader<RUN_PD>;
impl RUN_PD_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> RUN_PD {
        match self.bits {
            false => RUN_PD::NvmidleMode,
            true => RUN_PD::NvmpwrDownMode,
        }
    }
    ///When the device is in Run mode, the NVM is in Idle mode
    #[inline(always)]
    pub fn is_nvmidle_mode(&self) -> bool {
        *self == RUN_PD::NvmidleMode
    }
    ///When the device is in Run mode, the NVM is in power-down mode
    #[inline(always)]
    pub fn is_nvmpwr_down_mode(&self) -> bool {
        *self == RUN_PD::NvmpwrDownMode
    }
}
///Field `RUN_PD` writer - Flash mode during Run
pub type RUN_PD_W<'a, REG> = crate::BitWriter<'a, REG, RUN_PD>;
impl<'a, REG> RUN_PD_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///When the device is in Run mode, the NVM is in Idle mode
    #[inline(always)]
    pub fn nvmidle_mode(self) -> &'a mut crate::W<REG> {
        self.variant(RUN_PD::NvmidleMode)
    }
    ///When the device is in Run mode, the NVM is in power-down mode
    #[inline(always)]
    pub fn nvmpwr_down_mode(self) -> &'a mut crate::W<REG> {
        self.variant(RUN_PD::NvmpwrDownMode)
    }
}
/**Disable Buffer

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISAB_BUF {
    ///0: The buffers are enabled
    Enabled = 0,
    ///1: The buffers are disabled
    Disabled = 1,
}
impl From<DISAB_BUF> for bool {
    #[inline(always)]
    fn from(variant: DISAB_BUF) -> Self {
        variant as u8 != 0
    }
}
///Field `DISAB_BUF` reader - Disable Buffer
pub type DISAB_BUF_R = crate::BitReader<DISAB_BUF>;
impl DISAB_BUF_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DISAB_BUF {
        match self.bits {
            false => DISAB_BUF::Enabled,
            true => DISAB_BUF::Disabled,
        }
    }
    ///The buffers are enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DISAB_BUF::Enabled
    }
    ///The buffers are disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DISAB_BUF::Disabled
    }
}
///Field `DISAB_BUF` writer - Disable Buffer
pub type DISAB_BUF_W<'a, REG> = crate::BitWriter<'a, REG, DISAB_BUF>;
impl<'a, REG> DISAB_BUF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The buffers are enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISAB_BUF::Enabled)
    }
    ///The buffers are disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DISAB_BUF::Disabled)
    }
}
/**Pre-read data address

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRE_READ {
    ///0: The pre-read is disabled
    Disabled = 0,
    ///1: The pre-read is enabled
    Enabled = 1,
}
impl From<PRE_READ> for bool {
    #[inline(always)]
    fn from(variant: PRE_READ) -> Self {
        variant as u8 != 0
    }
}
///Field `PRE_READ` reader - Pre-read data address
pub type PRE_READ_R = crate::BitReader<PRE_READ>;
impl PRE_READ_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PRE_READ {
        match self.bits {
            false => PRE_READ::Disabled,
            true => PRE_READ::Enabled,
        }
    }
    ///The pre-read is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PRE_READ::Disabled
    }
    ///The pre-read is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PRE_READ::Enabled
    }
}
///Field `PRE_READ` writer - Pre-read data address
pub type PRE_READ_W<'a, REG> = crate::BitWriter<'a, REG, PRE_READ>;
impl<'a, REG> PRE_READ_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The pre-read is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRE_READ::Disabled)
    }
    ///The pre-read is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PRE_READ::Enabled)
    }
}
impl R {
    ///Bit 0 - Latency
    #[inline(always)]
    pub fn latency(&self) -> LATENCY_R {
        LATENCY_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Prefetch enable
    #[inline(always)]
    pub fn prften(&self) -> PRFTEN_R {
        PRFTEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Flash mode during Sleep
    #[inline(always)]
    pub fn sleep_pd(&self) -> SLEEP_PD_R {
        SLEEP_PD_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Flash mode during Run
    #[inline(always)]
    pub fn run_pd(&self) -> RUN_PD_R {
        RUN_PD_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Disable Buffer
    #[inline(always)]
    pub fn disab_buf(&self) -> DISAB_BUF_R {
        DISAB_BUF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Pre-read data address
    #[inline(always)]
    pub fn pre_read(&self) -> PRE_READ_R {
        PRE_READ_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ACR")
            .field("latency", &self.latency())
            .field("prften", &self.prften())
            .field("sleep_pd", &self.sleep_pd())
            .field("run_pd", &self.run_pd())
            .field("disab_buf", &self.disab_buf())
            .field("pre_read", &self.pre_read())
            .finish()
    }
}
impl W {
    ///Bit 0 - Latency
    #[inline(always)]
    pub fn latency(&mut self) -> LATENCY_W<'_, ACRrs> {
        LATENCY_W::new(self, 0)
    }
    ///Bit 1 - Prefetch enable
    #[inline(always)]
    pub fn prften(&mut self) -> PRFTEN_W<'_, ACRrs> {
        PRFTEN_W::new(self, 1)
    }
    ///Bit 3 - Flash mode during Sleep
    #[inline(always)]
    pub fn sleep_pd(&mut self) -> SLEEP_PD_W<'_, ACRrs> {
        SLEEP_PD_W::new(self, 3)
    }
    ///Bit 4 - Flash mode during Run
    #[inline(always)]
    pub fn run_pd(&mut self) -> RUN_PD_W<'_, ACRrs> {
        RUN_PD_W::new(self, 4)
    }
    ///Bit 5 - Disable Buffer
    #[inline(always)]
    pub fn disab_buf(&mut self) -> DISAB_BUF_W<'_, ACRrs> {
        DISAB_BUF_W::new(self, 5)
    }
    ///Bit 6 - Pre-read data address
    #[inline(always)]
    pub fn pre_read(&mut self) -> PRE_READ_W<'_, ACRrs> {
        PRE_READ_W::new(self, 6)
    }
}
/**Access control register

You can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L0x1.html#FLASH:ACR)*/
pub struct ACRrs;
impl crate::RegisterSpec for ACRrs {
    type Ux = u32;
}
///`read()` method returns [`acr::R`](R) reader structure
impl crate::Readable for ACRrs {}
///`write(|w| ..)` method takes [`acr::W`](W) writer structure
impl crate::Writable for ACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ACR to value 0
impl crate::Resettable for ACRrs {}
