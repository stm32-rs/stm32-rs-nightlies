///Register `TIM16_CCER` reader
pub type R = crate::R<TIM16_CCERrs>;
///Register `TIM16_CCER` writer
pub type W = crate::W<TIM16_CCERrs>;
///Field `CC1E` reader - Capture/Compare 1 output enable
pub type CC1E_R = crate::BitReader;
///Field `CC1E` writer - Capture/Compare 1 output enable
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1P` reader - Capture/Compare 1 output polarity
pub type CC1P_R = crate::BitReader;
///Field `CC1P` writer - Capture/Compare 1 output polarity
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NE` reader - Capture/Compare 1 complementary output enable
pub type CC1NE_R = crate::BitReader;
///Field `CC1NE` writer - Capture/Compare 1 complementary output enable
pub type CC1NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NP` reader - Capture/Compare 1 complementary output polarity
pub type CC1NP_R = crate::BitReader;
///Field `CC1NP` writer - Capture/Compare 1 complementary output polarity
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&self) -> CC1NE_R {
        CC1NE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 1 complementary output polarity
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIM16_CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1ne", &self.cc1ne())
            .field("cc1np", &self.cc1np())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<TIM16_CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 output polarity
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<TIM16_CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 2 - Capture/Compare 1 complementary output enable
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W<TIM16_CCERrs> {
        CC1NE_W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 1 complementary output polarity
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<TIM16_CCERrs> {
        CC1NP_W::new(self, 3)
    }
}
/**TIM16 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`tim16_ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U083.html#TIM16:TIM16_CCER)*/
pub struct TIM16_CCERrs;
impl crate::RegisterSpec for TIM16_CCERrs {
    type Ux = u16;
}
///`read()` method returns [`tim16_ccer::R`](R) reader structure
impl crate::Readable for TIM16_CCERrs {}
///`write(|w| ..)` method takes [`tim16_ccer::W`](W) writer structure
impl crate::Writable for TIM16_CCERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
///`reset()` method sets TIM16_CCER to value 0
impl crate::Resettable for TIM16_CCERrs {
    const RESET_VALUE: u16 = 0;
}