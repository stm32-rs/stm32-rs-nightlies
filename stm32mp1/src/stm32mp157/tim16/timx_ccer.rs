///Register `TIMx_CCER` reader
pub type R = crate::R<TIMX_CCERrs>;
///Register `TIMx_CCER` writer
pub type W = crate::W<TIMX_CCERrs>;
///Field `CC1E` reader - CC1E
pub type CC1E_R = crate::BitReader;
///Field `CC1E` writer - CC1E
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1P` reader - CC1P
pub type CC1P_R = crate::BitReader;
///Field `CC1P` writer - CC1P
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NE` reader - CC1NE
pub type CC1NE_R = crate::BitReader;
///Field `CC1NE` writer - CC1NE
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NP` reader - CC1NP
pub type CC1NP_R = crate::BitReader;
///Field `CC1NP` writer - CC1NP
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CC1E
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1P
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CC1NE
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CC1NP
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMx_CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1ne", &self.cc1ne())
            .field("cc1np", &self.cc1np())
            .finish()
    }
}
impl W {
    ///Bit 0 - CC1E
    #[inline(always)]
    #[must_use]
    pub fn cc1e(&mut self) -> CC1E_W<TIMX_CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - CC1P
    #[inline(always)]
    #[must_use]
    pub fn cc1p(&mut self) -> CC1P_W<TIMX_CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 2 - CC1NE
    #[inline(always)]
    #[must_use]
    pub fn cc1ne(&mut self) -> CC1NE_W<TIMX_CCERrs> {
        CC1NE_W::new(self, 2)
    }
    ///Bit 3 - CC1NP
    #[inline(always)]
    #[must_use]
    pub fn cc1np(&mut self) -> CC1NP_W<TIMX_CCERrs> {
        CC1NP_W::new(self, 3)
    }
}
/**TIM16/TIM17 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`timx_ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timx_ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#TIM16:TIMx_CCER)*/
pub struct TIMX_CCERrs;
impl crate::RegisterSpec for TIMX_CCERrs {
    type Ux = u16;
}
///`read()` method returns [`timx_ccer::R`](R) reader structure
impl crate::Readable for TIMX_CCERrs {}
///`write(|w| ..)` method takes [`timx_ccer::W`](W) writer structure
impl crate::Writable for TIMX_CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIMx_CCER to value 0
impl crate::Resettable for TIMX_CCERrs {
    const RESET_VALUE: u16 = 0;
}
