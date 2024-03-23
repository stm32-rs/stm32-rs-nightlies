#[doc = "Register `C2CR1` reader"]
pub type R = crate::R<C2CR1rs>;
#[doc = "Register `C2CR1` writer"]
pub type W = crate::W<C2CR1rs>;
#[doc = "Low-power mode selection for CPU2\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LPMS {
    #[doc = "0: Stop 0 mode"]
    Stop0 = 0,
    #[doc = "1: Stop 1 mode"]
    Stop1 = 1,
    #[doc = "2: Stop 2 mode"]
    Stop2 = 2,
    #[doc = "3: Standby mode"]
    Standby = 3,
    #[doc = "4: Shutdown mode"]
    Shutdown = 4,
}
impl From<LPMS> for u8 {
    #[inline(always)]
    fn from(variant: LPMS) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LPMS {
    type Ux = u8;
}
#[doc = "Field `LPMS` reader - Low-power mode selection for CPU2"]
pub type LPMS_R = crate::FieldReader<LPMS>;
impl LPMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<LPMS> {
        match self.bits {
            0 => Some(LPMS::Stop0),
            1 => Some(LPMS::Stop1),
            2 => Some(LPMS::Stop2),
            3 => Some(LPMS::Standby),
            4 => Some(LPMS::Shutdown),
            _ => None,
        }
    }
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn is_stop0(&self) -> bool {
        *self == LPMS::Stop0
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn is_stop1(&self) -> bool {
        *self == LPMS::Stop1
    }
    #[doc = "Stop 2 mode"]
    #[inline(always)]
    pub fn is_stop2(&self) -> bool {
        *self == LPMS::Stop2
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn is_standby(&self) -> bool {
        *self == LPMS::Standby
    }
    #[doc = "Shutdown mode"]
    #[inline(always)]
    pub fn is_shutdown(&self) -> bool {
        *self == LPMS::Shutdown
    }
}
#[doc = "Field `LPMS` writer - Low-power mode selection for CPU2"]
pub type LPMS_W<'a, REG> = crate::FieldWriter<'a, REG, 3, LPMS>;
impl<'a, REG> LPMS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Stop 0 mode"]
    #[inline(always)]
    pub fn stop0(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop0)
    }
    #[doc = "Stop 1 mode"]
    #[inline(always)]
    pub fn stop1(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop1)
    }
    #[doc = "Stop 2 mode"]
    #[inline(always)]
    pub fn stop2(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Stop2)
    }
    #[doc = "Standby mode"]
    #[inline(always)]
    pub fn standby(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Standby)
    }
    #[doc = "Shutdown mode"]
    #[inline(always)]
    pub fn shutdown(self) -> &'a mut crate::W<REG> {
        self.variant(LPMS::Shutdown)
    }
}
#[doc = "Flash memory power down mode during LPRun for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDR {
    #[doc = "0: Flash memory in Idle mode when system is in LPRun mode"]
    Idle = 0,
    #[doc = "1: Flash memory in Power-down mode when system is in LPRun mode"]
    PowerDown = 1,
}
impl From<FPDR> for bool {
    #[inline(always)]
    fn from(variant: FPDR) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDR` reader - Flash memory power down mode during LPRun for CPU2"]
pub type FPDR_R = crate::BitReader<FPDR>;
impl FPDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPDR {
        match self.bits {
            false => FPDR::Idle,
            true => FPDR::PowerDown,
        }
    }
    #[doc = "Flash memory in Idle mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDR::Idle
    }
    #[doc = "Flash memory in Power-down mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDR::PowerDown
    }
}
#[doc = "Field `FPDR` writer - Flash memory power down mode during LPRun for CPU2"]
pub type FPDR_W<'a, REG> = crate::BitWriter<'a, REG, FPDR>;
impl<'a, REG> FPDR_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory in Idle mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(FPDR::Idle)
    }
    #[doc = "Flash memory in Power-down mode when system is in LPRun mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FPDR::PowerDown)
    }
}
#[doc = "Flash memory power down mode during LPSleep for CPU2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPDS {
    #[doc = "0: Flash memory in Idle mode when system is in LPSleep mode"]
    Idle = 0,
    #[doc = "1: Flash memory in Power-down mode when system is in LPSleep mode"]
    PowerDown = 1,
}
impl From<FPDS> for bool {
    #[inline(always)]
    fn from(variant: FPDS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPDS` reader - Flash memory power down mode during LPSleep for CPU2"]
pub type FPDS_R = crate::BitReader<FPDS>;
impl FPDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FPDS {
        match self.bits {
            false => FPDS::Idle,
            true => FPDS::PowerDown,
        }
    }
    #[doc = "Flash memory in Idle mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == FPDS::Idle
    }
    #[doc = "Flash memory in Power-down mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == FPDS::PowerDown
    }
}
#[doc = "Field `FPDS` writer - Flash memory power down mode during LPSleep for CPU2"]
pub type FPDS_W<'a, REG> = crate::BitWriter<'a, REG, FPDS>;
impl<'a, REG> FPDS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Flash memory in Idle mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::Idle)
    }
    #[doc = "Flash memory in Power-down mode when system is in LPSleep mode"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(FPDS::PowerDown)
    }
}
impl R {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    pub fn lpms(&self) -> LPMS_R {
        LPMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU2"]
    #[inline(always)]
    pub fn fpdr(&self) -> FPDR_R {
        FPDR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU2"]
    #[inline(always)]
    pub fn fpds(&self) -> FPDS_R {
        FPDS_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Low-power mode selection for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn lpms(&mut self) -> LPMS_W<C2CR1rs> {
        LPMS_W::new(self, 0)
    }
    #[doc = "Bit 4 - Flash memory power down mode during LPRun for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn fpdr(&mut self) -> FPDR_W<C2CR1rs> {
        FPDR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Flash memory power down mode during LPSleep for CPU2"]
    #[inline(always)]
    #[must_use]
    pub fn fpds(&mut self) -> FPDS_W<C2CR1rs> {
        FPDS_W::new(self, 5)
    }
}
#[doc = "Power CPU2 control register 1 \\[dual core device only\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2cr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2cr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C2CR1rs;
impl crate::RegisterSpec for C2CR1rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c2cr1::R`](R) reader structure"]
impl crate::Readable for C2CR1rs {}
#[doc = "`write(|w| ..)` method takes [`c2cr1::W`](W) writer structure"]
impl crate::Writable for C2CR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets C2CR1 to value 0x07"]
impl crate::Resettable for C2CR1rs {
    const RESET_VALUE: u32 = 0x07;
}
