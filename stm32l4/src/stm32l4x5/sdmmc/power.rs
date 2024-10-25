///Register `POWER` reader
pub type R = crate::R<POWERrs>;
///Register `POWER` writer
pub type W = crate::W<POWERrs>;
///Field `PWRCTRL` reader - PWRCTRL
pub type PWRCTRL_R = crate::FieldReader;
///Field `PWRCTRL` writer - PWRCTRL
pub type PWRCTRL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
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
    #[must_use]
    pub fn pwrctrl(&mut self) -> PWRCTRL_W<POWERrs> {
        PWRCTRL_W::new(self, 0)
    }
}
/**power control register

You can [`read`](crate::Reg::read) this register and get [`power::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4x5.html#SDMMC:POWER)*/
pub struct POWERrs;
impl crate::RegisterSpec for POWERrs {
    type Ux = u32;
}
///`read()` method returns [`power::R`](R) reader structure
impl crate::Readable for POWERrs {}
///`write(|w| ..)` method takes [`power::W`](W) writer structure
impl crate::Writable for POWERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets POWER to value 0
impl crate::Resettable for POWERrs {
    const RESET_VALUE: u32 = 0;
}
