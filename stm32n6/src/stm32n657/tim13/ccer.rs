///Register `CCER` reader
pub type R = crate::R<CCERrs>;
///Register `CCER` writer
pub type W = crate::W<CCERrs>;
///Field `CC1E` reader - Capture/Compare 1 output enable.
pub type CC1E_R = crate::BitReader;
///Field `CC1E` writer - Capture/Compare 1 output enable.
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1P` reader - Capture/Compare 1 output Polarity.
pub type CC1P_R = crate::BitReader;
///Field `CC1P` writer - Capture/Compare 1 output Polarity.
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NP` reader - Capture/Compare 1 complementary output Polarity.
pub type CC1NP_R = crate::BitReader;
///Field `CC1NP` writer - Capture/Compare 1 complementary output Polarity.
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/Compare 1 output enable.
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity.
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 1 complementary output Polarity.
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1np", &self.cc1np())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable.
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<'_, CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity.
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<'_, CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 3 - Capture/Compare 1 complementary output Polarity.
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<'_, CCERrs> {
        CC1NP_W::new(self, 3)
    }
}
/**TIM13 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#TIM13:CCER)*/
pub struct CCERrs;
impl crate::RegisterSpec for CCERrs {
    type Ux = u16;
}
///`read()` method returns [`ccer::R`](R) reader structure
impl crate::Readable for CCERrs {}
///`write(|w| ..)` method takes [`ccer::W`](W) writer structure
impl crate::Writable for CCERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCERrs {}
