///Register `CCER` reader
pub type R = crate::R<CCERrs>;
///Register `CCER` writer
pub type W = crate::W<CCERrs>;
///Field `CC1E` reader - Capture/Compare 1 output enable
pub type CC1E_R = crate::BitReader;
///Field `CC1E` writer - Capture/Compare 1 output enable
pub type CC1E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1P` reader - Capture/Compare 1 output Polarity
pub type CC1P_R = crate::BitReader;
///Field `CC1P` writer - Capture/Compare 1 output Polarity
pub type CC1P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1NP` reader - Capture/Compare 1 output Polarity
pub type CC1NP_R = crate::BitReader;
///Field `CC1NP` writer - Capture/Compare 1 output Polarity
pub type CC1NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2E` reader - Capture/Compare 2 output enable
pub type CC2E_R = crate::BitReader;
///Field `CC2E` writer - Capture/Compare 2 output enable
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2P` reader - Capture/Compare 2 output Polarity
pub type CC2P_R = crate::BitReader;
///Field `CC2P` writer - Capture/Compare 2 output Polarity
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2NP` reader - Capture/Compare 2 output Polarity
pub type CC2NP_R = crate::BitReader;
///Field `CC2NP` writer - Capture/Compare 2 output Polarity
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3E` reader - Capture/Compare 3 output enable
pub type CC3E_R = crate::BitReader;
///Field `CC3E` writer - Capture/Compare 3 output enable
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3P` reader - Capture/Compare 3 output Polarity
pub type CC3P_R = crate::BitReader;
///Field `CC3P` writer - Capture/Compare 3 output Polarity
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3NP` reader - Capture/Compare 3 output Polarity
pub type CC3NP_R = crate::BitReader;
///Field `CC3NP` writer - Capture/Compare 3 output Polarity
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4E` reader - Capture/Compare 4 output enable
pub type CC4E_R = crate::BitReader;
///Field `CC4E` writer - Capture/Compare 4 output enable
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4P` reader - Capture/Compare 3 output Polarity
pub type CC4P_R = crate::BitReader;
///Field `CC4P` writer - Capture/Compare 3 output Polarity
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4NP` reader - Capture/Compare 4 output Polarity
pub type CC4NP_R = crate::BitReader;
///Field `CC4NP` writer - Capture/Compare 4 output Polarity
pub type CC4NP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&self) -> CC1E_R {
        CC1E_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&self) -> CC1P_R {
        CC1P_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&self) -> CC1NP_R {
        CC1NP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Capture/Compare 3 output enable
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 output enable
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - Capture/Compare 4 output Polarity
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("cc4np", &self.cc4np())
            .field("cc4p", &self.cc4p())
            .field("cc4e", &self.cc4e())
            .field("cc3np", &self.cc3np())
            .field("cc3p", &self.cc3p())
            .field("cc3e", &self.cc3e())
            .field("cc2np", &self.cc2np())
            .field("cc2p", &self.cc2p())
            .field("cc2e", &self.cc2e())
            .field("cc1np", &self.cc1np())
            .field("cc1p", &self.cc1p())
            .field("cc1e", &self.cc1e())
            .finish()
    }
}
impl W {
    ///Bit 0 - Capture/Compare 1 output enable
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<'_, CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<'_, CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 3 - Capture/Compare 1 output Polarity
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<'_, CCERrs> {
        CC1NP_W::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 2 output enable
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W<'_, CCERrs> {
        CC2E_W::new(self, 4)
    }
    ///Bit 5 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W<'_, CCERrs> {
        CC2P_W::new(self, 5)
    }
    ///Bit 7 - Capture/Compare 2 output Polarity
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W<'_, CCERrs> {
        CC2NP_W::new(self, 7)
    }
    ///Bit 8 - Capture/Compare 3 output enable
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W<'_, CCERrs> {
        CC3E_W::new(self, 8)
    }
    ///Bit 9 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W<'_, CCERrs> {
        CC3P_W::new(self, 9)
    }
    ///Bit 11 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W<'_, CCERrs> {
        CC3NP_W::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 output enable
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W<'_, CCERrs> {
        CC4E_W::new(self, 12)
    }
    ///Bit 13 - Capture/Compare 3 output Polarity
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W<'_, CCERrs> {
        CC4P_W::new(self, 13)
    }
    ///Bit 15 - Capture/Compare 4 output Polarity
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W<'_, CCERrs> {
        CC4NP_W::new(self, 15)
    }
}
/**capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#TIM3:CCER)*/
pub struct CCERrs;
impl crate::RegisterSpec for CCERrs {
    type Ux = u32;
}
///`read()` method returns [`ccer::R`](R) reader structure
impl crate::Readable for CCERrs {}
///`write(|w| ..)` method takes [`ccer::W`](W) writer structure
impl crate::Writable for CCERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CCER to value 0
impl crate::Resettable for CCERrs {}
