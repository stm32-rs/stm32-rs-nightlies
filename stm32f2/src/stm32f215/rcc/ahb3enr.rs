///Register `AHB3ENR` reader
pub type R = crate::R<AHB3ENRrs>;
///Register `AHB3ENR` writer
pub type W = crate::W<AHB3ENRrs>;
/**Flexible static memory controller module clock enable

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSMCEN {
    ///0: The selected clock is disabled
    Disabled = 0,
    ///1: The selected clock is enabled
    Enabled = 1,
}
impl From<FSMCEN> for bool {
    #[inline(always)]
    fn from(variant: FSMCEN) -> Self {
        variant as u8 != 0
    }
}
///Field `FSMCEN` reader - Flexible static memory controller module clock enable
pub type FSMCEN_R = crate::BitReader<FSMCEN>;
impl FSMCEN_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> FSMCEN {
        match self.bits {
            false => FSMCEN::Disabled,
            true => FSMCEN::Enabled,
        }
    }
    ///The selected clock is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FSMCEN::Disabled
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FSMCEN::Enabled
    }
}
///Field `FSMCEN` writer - Flexible static memory controller module clock enable
pub type FSMCEN_W<'a, REG> = crate::BitWriter<'a, REG, FSMCEN>;
impl<'a, REG> FSMCEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///The selected clock is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCEN::Disabled)
    }
    ///The selected clock is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(FSMCEN::Enabled)
    }
}
impl R {
    ///Bit 0 - Flexible static memory controller module clock enable
    #[inline(always)]
    pub fn fsmcen(&self) -> FSMCEN_R {
        FSMCEN_R::new((self.bits & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB3ENR")
            .field("fsmcen", &self.fsmcen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Flexible static memory controller module clock enable
    #[inline(always)]
    pub fn fsmcen(&mut self) -> FSMCEN_W<'_, AHB3ENRrs> {
        FSMCEN_W::new(self, 0)
    }
}
/**AHB3 peripheral clock enable register

You can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F215.html#RCC:AHB3ENR)*/
pub struct AHB3ENRrs;
impl crate::RegisterSpec for AHB3ENRrs {
    type Ux = u32;
}
///`read()` method returns [`ahb3enr::R`](R) reader structure
impl crate::Readable for AHB3ENRrs {}
///`write(|w| ..)` method takes [`ahb3enr::W`](W) writer structure
impl crate::Writable for AHB3ENRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets AHB3ENR to value 0
impl crate::Resettable for AHB3ENRrs {}
