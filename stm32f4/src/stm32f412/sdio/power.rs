///Register `POWER` reader
pub type R = crate::R<POWERrs>;
///Register `POWER` writer
pub type W = crate::W<POWERrs>;
/**PWRCTRL

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PWRCTRL {
    ///0: Power off
    PowerOff = 0,
    ///3: Power on
    PowerOn = 3,
}
impl From<PWRCTRL> for u8 {
    #[inline(always)]
    fn from(variant: PWRCTRL) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PWRCTRL {
    type Ux = u8;
}
impl crate::IsEnum for PWRCTRL {}
///Field `PWRCTRL` reader - PWRCTRL
pub type PWRCTRL_R = crate::FieldReader<PWRCTRL>;
impl PWRCTRL_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<PWRCTRL> {
        match self.bits {
            0 => Some(PWRCTRL::PowerOff),
            3 => Some(PWRCTRL::PowerOn),
            _ => None,
        }
    }
    ///Power off
    #[inline(always)]
    pub fn is_power_off(&self) -> bool {
        *self == PWRCTRL::PowerOff
    }
    ///Power on
    #[inline(always)]
    pub fn is_power_on(&self) -> bool {
        *self == PWRCTRL::PowerOn
    }
}
///Field `PWRCTRL` writer - PWRCTRL
pub type PWRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2, PWRCTRL>;
impl<'a, REG> PWRCTRL_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    ///Power off
    #[inline(always)]
    pub fn power_off(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCTRL::PowerOff)
    }
    ///Power on
    #[inline(always)]
    pub fn power_on(self) -> &'a mut crate::W<REG> {
        self.variant(PWRCTRL::PowerOn)
    }
}
impl R {
    ///Bits 0:1 - PWRCTRL
    #[inline(always)]
    pub fn pwrctrl(&self) -> PWRCTRL_R {
        PWRCTRL_R::new((self.bits & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER")
            .field("pwrctrl", &self.pwrctrl())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - PWRCTRL
    #[inline(always)]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<'_, POWERrs> {
        PWRCTRL_W::new(self, 0)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F412.html#SDIO:POWER)*/
pub struct POWERrs;
impl crate::RegisterSpec for POWERrs {
    type Ux = u32;
}
///`read()` method returns [`power::R`](R) reader structure
impl crate::Readable for POWERrs {}
///`write(|w| ..)` method takes [`power::W`](W) writer structure
impl crate::Writable for POWERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets POWER to value 0
impl crate::Resettable for POWERrs {}
