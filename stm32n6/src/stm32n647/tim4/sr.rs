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
///Field `CC2IF` reader - Capture/Compare 2 interrupt flag
pub type CC2IF_R = crate::BitReader;
///Field `CC2IF` writer - Capture/Compare 2 interrupt flag
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IF` reader - Capture/Compare 3 interrupt flag
pub type CC3IF_R = crate::BitReader;
///Field `CC3IF` writer - Capture/Compare 3 interrupt flag
pub type CC3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IF` reader - Capture/Compare 4 interrupt flag
pub type CC4IF_R = crate::BitReader;
///Field `CC4IF` writer - Capture/Compare 4 interrupt flag
pub type CC4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIF` reader - Trigger interrupt flag
pub type TIF_R = crate::BitReader;
///Field `TIF` writer - Trigger interrupt flag
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OF` reader - Capture/Compare 1 overcapture flag
pub type CC1OF_R = crate::BitReader;
///Field `CC1OF` writer - Capture/Compare 1 overcapture flag
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2OF` reader - Capture/compare 2 overcapture flag
pub type CC2OF_R = crate::BitReader;
///Field `CC2OF` writer - Capture/compare 2 overcapture flag
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3OF` reader - Capture/Compare 3 overcapture flag
pub type CC3OF_R = crate::BitReader;
///Field `CC3OF` writer - Capture/Compare 3 overcapture flag
pub type CC3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4OF` reader - Capture/Compare 4 overcapture flag
pub type CC4OF_R = crate::BitReader;
///Field `CC4OF` writer - Capture/Compare 4 overcapture flag
pub type CC4OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IDXF` reader - Index interrupt flag
pub type IDXF_R = crate::BitReader;
///Field `IDXF` writer - Index interrupt flag
pub type IDXF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DIRF` reader - Direction change interrupt flag
pub type DIRF_R = crate::BitReader;
///Field `DIRF` writer - Direction change interrupt flag
pub type DIRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IERRF` reader - Index error interrupt flag
pub type IERRF_R = crate::BitReader;
///Field `IERRF` writer - Index error interrupt flag
pub type IERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TERRF` reader - Transition error interrupt flag
pub type TERRF_R = crate::BitReader;
///Field `TERRF` writer - Transition error interrupt flag
pub type TERRF_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    ///Bit 2 - Capture/Compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Capture/compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 20 - Index interrupt flag
    #[inline(always)]
    pub fn idxf(&self) -> IDXF_R {
        IDXF_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Direction change interrupt flag
    #[inline(always)]
    pub fn dirf(&self) -> DIRF_R {
        DIRF_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Index error interrupt flag
    #[inline(always)]
    pub fn ierrf(&self) -> IERRF_R {
        IERRF_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Transition error interrupt flag
    #[inline(always)]
    pub fn terrf(&self) -> TERRF_R {
        TERRF_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("uif", &self.uif())
            .field("cc1if", &self.cc1if())
            .field("cc2if", &self.cc2if())
            .field("cc3if", &self.cc3if())
            .field("cc4if", &self.cc4if())
            .field("tif", &self.tif())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .field("cc3of", &self.cc3of())
            .field("cc4of", &self.cc4of())
            .field("idxf", &self.idxf())
            .field("dirf", &self.dirf())
            .field("ierrf", &self.ierrf())
            .field("terrf", &self.terrf())
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
    ///Bit 2 - Capture/Compare 2 interrupt flag
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W<'_, SRrs> {
        CC2IF_W::new(self, 2)
    }
    ///Bit 3 - Capture/Compare 3 interrupt flag
    #[inline(always)]
    pub fn cc3if(&mut self) -> CC3IF_W<'_, SRrs> {
        CC3IF_W::new(self, 3)
    }
    ///Bit 4 - Capture/Compare 4 interrupt flag
    #[inline(always)]
    pub fn cc4if(&mut self) -> CC4IF_W<'_, SRrs> {
        CC4IF_W::new(self, 4)
    }
    ///Bit 6 - Trigger interrupt flag
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<'_, SRrs> {
        TIF_W::new(self, 6)
    }
    ///Bit 9 - Capture/Compare 1 overcapture flag
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<'_, SRrs> {
        CC1OF_W::new(self, 9)
    }
    ///Bit 10 - Capture/compare 2 overcapture flag
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W<'_, SRrs> {
        CC2OF_W::new(self, 10)
    }
    ///Bit 11 - Capture/Compare 3 overcapture flag
    #[inline(always)]
    pub fn cc3of(&mut self) -> CC3OF_W<'_, SRrs> {
        CC3OF_W::new(self, 11)
    }
    ///Bit 12 - Capture/Compare 4 overcapture flag
    #[inline(always)]
    pub fn cc4of(&mut self) -> CC4OF_W<'_, SRrs> {
        CC4OF_W::new(self, 12)
    }
    ///Bit 20 - Index interrupt flag
    #[inline(always)]
    pub fn idxf(&mut self) -> IDXF_W<'_, SRrs> {
        IDXF_W::new(self, 20)
    }
    ///Bit 21 - Direction change interrupt flag
    #[inline(always)]
    pub fn dirf(&mut self) -> DIRF_W<'_, SRrs> {
        DIRF_W::new(self, 21)
    }
    ///Bit 22 - Index error interrupt flag
    #[inline(always)]
    pub fn ierrf(&mut self) -> IERRF_W<'_, SRrs> {
        IERRF_W::new(self, 22)
    }
    ///Bit 23 - Transition error interrupt flag
    #[inline(always)]
    pub fn terrf(&mut self) -> TERRF_W<'_, SRrs> {
        TERRF_W::new(self, 23)
    }
}
/**TIM4 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#TIM4:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {}
