///Register `MPCWM1_UPWWMR` reader
pub type R = crate::R<MPCWM1_UPWWMRrs>;
///Register `MPCWM1_UPWWMR` writer
pub type W = crate::W<MPCWM1_UPWWMRrs>;
///Field `LGTH` reader - Define the length of Flash Unprivileged Writable area, in 2
pub type LGTH_R = crate::FieldReader<u16>;
///Field `LGTH` writer - Define the length of Flash Unprivileged Writable area, in 2
pub type LGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 16:27 - Define the length of Flash Unprivileged Writable area, in 2
    #[inline(always)]
    pub fn lgth(&self) -> LGTH_R {
        LGTH_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MPCWM1_UPWWMR")
            .field("lgth", &self.lgth())
            .finish()
    }
}
impl W {
    ///Bits 16:27 - Define the length of Flash Unprivileged Writable area, in 2
    #[inline(always)]
    pub fn lgth(&mut self) -> LGTH_W<'_, MPCWM1_UPWWMRrs> {
        LGTH_W::new(self, 16)
    }
}
/**Unprivileged Writable Water Mark 1 register

You can [`read`](crate::Reg::read) this register and get [`mpcwm1_upwwmr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpcwm1_upwwmr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL5X_CM4.html#TZSC:MPCWM1_UPWWMR)*/
pub struct MPCWM1_UPWWMRrs;
impl crate::RegisterSpec for MPCWM1_UPWWMRrs {
    type Ux = u32;
}
///`read()` method returns [`mpcwm1_upwwmr::R`](R) reader structure
impl crate::Readable for MPCWM1_UPWWMRrs {}
///`write(|w| ..)` method takes [`mpcwm1_upwwmr::W`](W) writer structure
impl crate::Writable for MPCWM1_UPWWMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MPCWM1_UPWWMR to value 0x0fff_0000
impl crate::Resettable for MPCWM1_UPWWMRrs {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
