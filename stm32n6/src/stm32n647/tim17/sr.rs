///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `UIF` reader - Update interrupt flag
pub type UIF_R = crate::BitReader;
///Field `UIF` writer - Update interrupt flag
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IF` reader - Capture/Compare 1 interrupt flag
pub type CC1IF_R = crate::BitReader;
///Field `CC1IF` writer - Capture/Compare 1 interrupt flag
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMIF` reader - COM interrupt flag
pub type COMIF_R = crate::BitReader;
///Field `COMIF` writer - COM interrupt flag
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIF` reader - Break interrupt flag
pub type BIF_R = crate::BitReader;
///Field `BIF` writer - Break interrupt flag
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 1 - Capture/Compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
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
            .field("comif", &self.comif())
            .field("bif", &self.bif())
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
    ///Bit 1 - Capture/Compare 1 interrupt flag
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, SRrs> {
        CC1IF_W::new(self, 1)
    }
    ///Bit 5 - COM interrupt flag
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W<'_, SRrs> {
        COMIF_W::new(self, 5)
    }
    ///Bit 7 - Break interrupt flag
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W<'_, SRrs> {
        BIF_W::new(self, 7)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<'_, SRrs> {
        CC1OF_W::new(self, 9)
    }
}
/**TIM17 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM17:SR)*/
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
