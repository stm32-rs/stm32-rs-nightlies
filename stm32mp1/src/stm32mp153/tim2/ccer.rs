///Register `CCER` reader
pub type R = crate::R<CCERrs>;
///Register `CCER` writer
pub type W = crate::W<CCERrs>;
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
///Field `CC2E` reader - CC2E
pub type CC2E_R = crate::BitReader;
///Field `CC2E` writer - CC2E
pub type CC2E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2P` reader - CC2P
pub type CC2P_R = crate::BitReader;
///Field `CC2P` writer - CC2P
pub type CC2P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2NE` reader - CC2NE
pub type CC2NE_R = crate::BitReader;
///Field `CC2NE` writer - CC2NE
pub type CC2NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2NP` reader - CC2NP
pub type CC2NP_R = crate::BitReader;
///Field `CC2NP` writer - CC2NP
pub type CC2NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3E` reader - CC3E
pub type CC3E_R = crate::BitReader;
///Field `CC3E` writer - CC3E
pub type CC3E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3P` reader - CC3P
pub type CC3P_R = crate::BitReader;
///Field `CC3P` writer - CC3P
pub type CC3P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3NE` reader - CC3NE
pub type CC3NE_R = crate::BitReader;
///Field `CC3NE` writer - CC3NE
pub type CC3NE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3NP` reader - CC3NP
pub type CC3NP_R = crate::BitReader;
///Field `CC3NP` writer - CC3NP
pub type CC3NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4E` reader - CC4E
pub type CC4E_R = crate::BitReader;
///Field `CC4E` writer - CC4E
pub type CC4E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4P` reader - CC4P
pub type CC4P_R = crate::BitReader;
///Field `CC4P` writer - CC4P
pub type CC4P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4NP` reader - CC4NP
pub type CC4NP_R = crate::BitReader;
///Field `CC4NP` writer - CC4NP
pub type CC4NP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC5E` reader - CC5E
pub type CC5E_R = crate::BitReader;
///Field `CC5E` writer - CC5E
pub type CC5E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC5P` reader - CC5P
pub type CC5P_R = crate::BitReader;
///Field `CC5P` writer - CC5P
pub type CC5P_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC6E` reader - CC6E
pub type CC6E_R = crate::BitReader;
///Field `CC6E` writer - CC6E
pub type CC6E_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC6P` reader - CC6P
pub type CC6P_R = crate::BitReader;
///Field `CC6P` writer - CC6P
pub type CC6P_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 4 - CC2E
    #[inline(always)]
    pub fn cc2e(&self) -> CC2E_R {
        CC2E_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CC2P
    #[inline(always)]
    pub fn cc2p(&self) -> CC2P_R {
        CC2P_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CC2NE
    #[inline(always)]
    pub fn cc2ne(&self) -> CC2NE_R {
        CC2NE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CC2NP
    #[inline(always)]
    pub fn cc2np(&self) -> CC2NP_R {
        CC2NP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CC3E
    #[inline(always)]
    pub fn cc3e(&self) -> CC3E_R {
        CC3E_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CC3P
    #[inline(always)]
    pub fn cc3p(&self) -> CC3P_R {
        CC3P_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CC3NE
    #[inline(always)]
    pub fn cc3ne(&self) -> CC3NE_R {
        CC3NE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CC3NP
    #[inline(always)]
    pub fn cc3np(&self) -> CC3NP_R {
        CC3NP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CC4E
    #[inline(always)]
    pub fn cc4e(&self) -> CC4E_R {
        CC4E_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CC4P
    #[inline(always)]
    pub fn cc4p(&self) -> CC4P_R {
        CC4P_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 15 - CC4NP
    #[inline(always)]
    pub fn cc4np(&self) -> CC4NP_R {
        CC4NP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - CC5E
    #[inline(always)]
    pub fn cc5e(&self) -> CC5E_R {
        CC5E_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CC5P
    #[inline(always)]
    pub fn cc5p(&self) -> CC5P_R {
        CC5P_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 20 - CC6E
    #[inline(always)]
    pub fn cc6e(&self) -> CC6E_R {
        CC6E_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - CC6P
    #[inline(always)]
    pub fn cc6p(&self) -> CC6P_R {
        CC6P_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCER")
            .field("cc1e", &self.cc1e())
            .field("cc1p", &self.cc1p())
            .field("cc1ne", &self.cc1ne())
            .field("cc1np", &self.cc1np())
            .field("cc2e", &self.cc2e())
            .field("cc2p", &self.cc2p())
            .field("cc2ne", &self.cc2ne())
            .field("cc2np", &self.cc2np())
            .field("cc3e", &self.cc3e())
            .field("cc3p", &self.cc3p())
            .field("cc3ne", &self.cc3ne())
            .field("cc3np", &self.cc3np())
            .field("cc4e", &self.cc4e())
            .field("cc4p", &self.cc4p())
            .field("cc4np", &self.cc4np())
            .field("cc5e", &self.cc5e())
            .field("cc5p", &self.cc5p())
            .field("cc6e", &self.cc6e())
            .field("cc6p", &self.cc6p())
            .finish()
    }
}
impl W {
    ///Bit 0 - CC1E
    #[inline(always)]
    pub fn cc1e(&mut self) -> CC1E_W<'_, CCERrs> {
        CC1E_W::new(self, 0)
    }
    ///Bit 1 - CC1P
    #[inline(always)]
    pub fn cc1p(&mut self) -> CC1P_W<'_, CCERrs> {
        CC1P_W::new(self, 1)
    }
    ///Bit 2 - CC1NE
    #[inline(always)]
    pub fn cc1ne(&mut self) -> CC1NE_W<'_, CCERrs> {
        CC1NE_W::new(self, 2)
    }
    ///Bit 3 - CC1NP
    #[inline(always)]
    pub fn cc1np(&mut self) -> CC1NP_W<'_, CCERrs> {
        CC1NP_W::new(self, 3)
    }
    ///Bit 4 - CC2E
    #[inline(always)]
    pub fn cc2e(&mut self) -> CC2E_W<'_, CCERrs> {
        CC2E_W::new(self, 4)
    }
    ///Bit 5 - CC2P
    #[inline(always)]
    pub fn cc2p(&mut self) -> CC2P_W<'_, CCERrs> {
        CC2P_W::new(self, 5)
    }
    ///Bit 6 - CC2NE
    #[inline(always)]
    pub fn cc2ne(&mut self) -> CC2NE_W<'_, CCERrs> {
        CC2NE_W::new(self, 6)
    }
    ///Bit 7 - CC2NP
    #[inline(always)]
    pub fn cc2np(&mut self) -> CC2NP_W<'_, CCERrs> {
        CC2NP_W::new(self, 7)
    }
    ///Bit 8 - CC3E
    #[inline(always)]
    pub fn cc3e(&mut self) -> CC3E_W<'_, CCERrs> {
        CC3E_W::new(self, 8)
    }
    ///Bit 9 - CC3P
    #[inline(always)]
    pub fn cc3p(&mut self) -> CC3P_W<'_, CCERrs> {
        CC3P_W::new(self, 9)
    }
    ///Bit 10 - CC3NE
    #[inline(always)]
    pub fn cc3ne(&mut self) -> CC3NE_W<'_, CCERrs> {
        CC3NE_W::new(self, 10)
    }
    ///Bit 11 - CC3NP
    #[inline(always)]
    pub fn cc3np(&mut self) -> CC3NP_W<'_, CCERrs> {
        CC3NP_W::new(self, 11)
    }
    ///Bit 12 - CC4E
    #[inline(always)]
    pub fn cc4e(&mut self) -> CC4E_W<'_, CCERrs> {
        CC4E_W::new(self, 12)
    }
    ///Bit 13 - CC4P
    #[inline(always)]
    pub fn cc4p(&mut self) -> CC4P_W<'_, CCERrs> {
        CC4P_W::new(self, 13)
    }
    ///Bit 15 - CC4NP
    #[inline(always)]
    pub fn cc4np(&mut self) -> CC4NP_W<'_, CCERrs> {
        CC4NP_W::new(self, 15)
    }
    ///Bit 16 - CC5E
    #[inline(always)]
    pub fn cc5e(&mut self) -> CC5E_W<'_, CCERrs> {
        CC5E_W::new(self, 16)
    }
    ///Bit 17 - CC5P
    #[inline(always)]
    pub fn cc5p(&mut self) -> CC5P_W<'_, CCERrs> {
        CC5P_W::new(self, 17)
    }
    ///Bit 20 - CC6E
    #[inline(always)]
    pub fn cc6e(&mut self) -> CC6E_W<'_, CCERrs> {
        CC6E_W::new(self, 20)
    }
    ///Bit 21 - CC6P
    #[inline(always)]
    pub fn cc6p(&mut self) -> CC6P_W<'_, CCERrs> {
        CC6P_W::new(self, 21)
    }
}
/**TIM2 capture/compare enable register

You can [`read`](crate::Reg::read) this register and get [`ccer::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccer::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM2:CCER)*/
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
