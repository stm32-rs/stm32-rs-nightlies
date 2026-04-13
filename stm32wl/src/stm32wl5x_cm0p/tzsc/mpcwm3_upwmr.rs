///Register `MPCWM3_UPWMR` reader
pub type R = crate::R<MPCWM3_UPWMRrs>;
///Register `MPCWM3_UPWMR` writer
pub type W = crate::W<MPCWM3_UPWMRrs>;
///Field `LGTH` reader - LGTH
pub type LGTH_R = crate::FieldReader<u16>;
///Field `LGTH` writer - LGTH
pub type LGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 16:27 - LGTH
    #[inline(always)]
    pub fn lgth(&self) -> LGTH_R {
        LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCWM3_UPWMR")
            .field("lgth", &self.lgth())
            .finish()
    }
}
impl W {
    ///Bits 16:27 - LGTH
    #[inline(always)]
    pub fn lgth(&mut self) -> LGTH_W<'_, MPCWM3_UPWMRrs> {
        LGTH_W::new(self, 16)
    }
}
/**Unprivileged Water Mark 3 register

You can [`read`](crate::Reg::read) this register and get [`mpcwm3_upwmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm3_upwmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM0P.html#TZSC:MPCWM3_UPWMR)*/
pub struct MPCWM3_UPWMRrs;
impl crate::RegisterSpec for MPCWM3_UPWMRrs {
    type Ux = u32;
}
///`read()` method returns [`mpcwm3_upwmr::R`](R) reader structure
impl crate::Readable for MPCWM3_UPWMRrs {}
///`write(|w| ..)` method takes [`mpcwm3_upwmr::W`](W) writer structure
impl crate::Writable for MPCWM3_UPWMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCWM3_UPWMR to value 0x0fff_0000
impl crate::Resettable for MPCWM3_UPWMRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
