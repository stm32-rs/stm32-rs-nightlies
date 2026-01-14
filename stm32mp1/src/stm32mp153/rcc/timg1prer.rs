///Register `TIMG1PRER` reader
pub type R = crate::R<TIMG1PRERrs>;
///Register `TIMG1PRER` writer
pub type W = crate::W<TIMG1PRERrs>;
///Field `TIMG1PRE` reader - TIMG1PRE
pub type TIMG1PRE_R = crate::BitReader;
///Field `TIMG1PRE` writer - TIMG1PRE
pub type TIMG1PRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMG1PRERDY` reader - TIMG1PRERDY
pub type TIMG1PRERDY_R = crate::BitReader;
impl R {
    ///Bit 0 - TIMG1PRE
    #[inline(always)]
    pub fn timg1pre(&self) -> TIMG1PRE_R {
        TIMG1PRE_R::new((self.bits & 1) != 0)
    }
    ///Bit 31 - TIMG1PRERDY
    #[inline(always)]
    pub fn timg1prerdy(&self) -> TIMG1PRERDY_R {
        TIMG1PRERDY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMG1PRER")
            .field("timg1pre", &self.timg1pre())
            .field("timg1prerdy", &self.timg1prerdy())
            .finish()
    }
}
impl W {
    ///Bit 0 - TIMG1PRE
    #[inline(always)]
    pub fn timg1pre(&mut self) -> TIMG1PRE_W<'_, TIMG1PRERrs> {
        TIMG1PRE_W::new(self, 0)
    }
}
/**This register is used to control the prescaler value of timers located into APB1 domain. It concerns TIM2, TIM3, TIM4, TIM5, TIM6, TIM7, TIM12, TIM13 and TIM14. Refer to Section: Sub-system clock generation for additional information.

You can [`read`](crate::Reg::read) this register and get [`timg1prer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timg1prer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#RCC:TIMG1PRER)*/
pub struct TIMG1PRERrs;
impl crate::RegisterSpec for TIMG1PRERrs {
    type Ux = u32;
}
///`read()` method returns [`timg1prer::R`](R) reader structure
impl crate::Readable for TIMG1PRERrs {}
///`write(|w| ..)` method takes [`timg1prer::W`](W) writer structure
impl crate::Writable for TIMG1PRERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets TIMG1PRER to value 0x8000_0000
impl crate::Resettable for TIMG1PRERrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
