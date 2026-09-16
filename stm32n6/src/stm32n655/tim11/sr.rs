///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `UIF` reader - Update interrupt flag
pub type UIF_R = crate::BitReader;
///Field `UIF` writer - Update interrupt flag
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IF` reader - Capture/compare 1 interrupt flag
pub type CC1IF_R = crate::BitReader;
///Field `CC1IF` writer - Capture/compare 1 interrupt flag
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OF` reader - Capture/Compare 1 overcapture flag
pub type CC1OF_R = crate::BitReader;
///Field `CC1OF` writer - Capture/Compare 1 overcapture flag
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc1of", &self.cc1of())
            .finish()
    }
}
impl W {
    ///Bit 0 - Update interrupt flag
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<'_, SRrs> {
        UIF_W::new(self, 0)
    }
    ///Bit 1 - Capture/compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, SRrs> {
        CC1IF_W::new(self, 1)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<'_, SRrs> {
        CC1OF_W::new(self, 9)
    }
}
/**TIM11 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#TIM11:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u16;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
