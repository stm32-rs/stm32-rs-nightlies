///Register `TIMG2PRER` reader
pub type R = crate::R<TIMG2PRERrs>;
///Register `TIMG2PRER` writer
pub type W = crate::W<TIMG2PRERrs>;
///Field `TIMG2PRE` reader - TIMG2PRE
pub type TIMG2PRE_R = crate::BitReader;
///Field `TIMG2PRE` writer - TIMG2PRE
pub type TIMG2PRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMG2PRERDY` reader - TIMG2PRERDY
pub type TIMG2PRERDY_R = crate::BitReader;
impl R {
    ///Bit 0 - TIMG2PRE
    #[inline(always)]
    pub fn timg2pre(&self) -> TIMG2PRE_R {
        TIMG2PRE_R::new((self.bits & 1) != 0)
    }
    ///Bit 31 - TIMG2PRERDY
    #[inline(always)]
    pub fn timg2prerdy(&self) -> TIMG2PRERDY_R {
        TIMG2PRERDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMG2PRER")
            .field("timg2pre", &self.timg2pre())
            .field("timg2prerdy", &self.timg2prerdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIMG2PRE
    #[inline(always)]
    pub fn timg2pre(&mut self) -> TIMG2PRE_W<'_, TIMG2PRERrs> {
        TIMG2PRE_W::new(self, 0)
    }
}
/**This register is used to control the prescaler value of timers located into APB2 domain. It concerns TIM1, TIM8, TIM15, TIM16, and TIM17. Refer to Section: Sub-system clock generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`timg2prer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg2prer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:TIMG2PRER)*/
pub struct TIMG2PRERrs;
impl crate::RegisterSpec for TIMG2PRERrs {
    type Ux = u32;
}
///`read()` method returns [`timg2prer::R`](R) reader structure
impl crate::Readable for TIMG2PRERrs {}
///`write(|w| ..)` method takes [`timg2prer::W`](W) writer structure
impl crate::Writable for TIMG2PRERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMG2PRER to value 0x8000_0000
impl crate::Resettable for TIMG2PRERrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
