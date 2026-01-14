///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `UIF` reader - UIF
pub type UIF_R = crate::BitReader;
///Field `UIF` writer - UIF
pub type UIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1IF` reader - CC1IF
pub type CC1IF_R = crate::BitReader;
///Field `CC1IF` writer - CC1IF
pub type CC1IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2IF` reader - CC2IF
pub type CC2IF_R = crate::BitReader;
///Field `CC2IF` writer - CC2IF
pub type CC2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3IF` reader - CC3IF
pub type CC3IF_R = crate::BitReader;
///Field `CC3IF` writer - CC3IF
pub type CC3IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4IF` reader - CC4IF
pub type CC4IF_R = crate::BitReader;
///Field `CC4IF` writer - CC4IF
pub type CC4IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COMIF` reader - COMIF
pub type COMIF_R = crate::BitReader;
///Field `COMIF` writer - COMIF
pub type COMIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIF` reader - TIF
pub type TIF_R = crate::BitReader;
///Field `TIF` writer - TIF
pub type TIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BIF` reader - BIF
pub type BIF_R = crate::BitReader;
///Field `BIF` writer - BIF
pub type BIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `B2IF` reader - B2IF
pub type B2IF_R = crate::BitReader;
///Field `B2IF` writer - B2IF
pub type B2IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC1OF` reader - CC1OF
pub type CC1OF_R = crate::BitReader;
///Field `CC1OF` writer - CC1OF
pub type CC1OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC2OF` reader - CC2OF
pub type CC2OF_R = crate::BitReader;
///Field `CC2OF` writer - CC2OF
pub type CC2OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC3OF` reader - CC3OF
pub type CC3OF_R = crate::BitReader;
///Field `CC3OF` writer - CC3OF
pub type CC3OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC4OF` reader - CC4OF
pub type CC4OF_R = crate::BitReader;
///Field `CC4OF` writer - CC4OF
pub type CC4OF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SBIF` reader - SBIF
pub type SBIF_R = crate::BitReader;
///Field `SBIF` writer - SBIF
pub type SBIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC5IF` reader - CC5IF
pub type CC5IF_R = crate::BitReader;
///Field `CC5IF` writer - CC5IF
pub type CC5IF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CC6IF` reader - CC6IF
pub type CC6IF_R = crate::BitReader;
///Field `CC6IF` writer - CC6IF
pub type CC6IF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - UIF
    #[inline(always)]
    pub fn uif(&self) -> UIF_R {
        UIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CC1IF
    #[inline(always)]
    pub fn cc1if(&self) -> CC1IF_R {
        CC1IF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CC2IF
    #[inline(always)]
    pub fn cc2if(&self) -> CC2IF_R {
        CC2IF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CC3IF
    #[inline(always)]
    pub fn cc3if(&self) -> CC3IF_R {
        CC3IF_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CC4IF
    #[inline(always)]
    pub fn cc4if(&self) -> CC4IF_R {
        CC4IF_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - COMIF
    #[inline(always)]
    pub fn comif(&self) -> COMIF_R {
        COMIF_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - TIF
    #[inline(always)]
    pub fn tif(&self) -> TIF_R {
        TIF_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - BIF
    #[inline(always)]
    pub fn bif(&self) -> BIF_R {
        BIF_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - B2IF
    #[inline(always)]
    pub fn b2if(&self) -> B2IF_R {
        B2IF_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CC1OF
    #[inline(always)]
    pub fn cc1of(&self) -> CC1OF_R {
        CC1OF_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CC2OF
    #[inline(always)]
    pub fn cc2of(&self) -> CC2OF_R {
        CC2OF_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CC3OF
    #[inline(always)]
    pub fn cc3of(&self) -> CC3OF_R {
        CC3OF_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CC4OF
    #[inline(always)]
    pub fn cc4of(&self) -> CC4OF_R {
        CC4OF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - SBIF
    #[inline(always)]
    pub fn sbif(&self) -> SBIF_R {
        SBIF_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 16 - CC5IF
    #[inline(always)]
    pub fn cc5if(&self) -> CC5IF_R {
        CC5IF_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - CC6IF
    #[inline(always)]
    pub fn cc6if(&self) -> CC6IF_R {
        CC6IF_R::new(((self.bits >> 17) & 1) != 0)
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
            .field("comif", &self.comif())
            .field("tif", &self.tif())
            .field("bif", &self.bif())
            .field("b2if", &self.b2if())
            .field("cc1of", &self.cc1of())
            .field("cc2of", &self.cc2of())
            .field("cc3of", &self.cc3of())
            .field("cc4of", &self.cc4of())
            .field("sbif", &self.sbif())
            .field("cc5if", &self.cc5if())
            .field("cc6if", &self.cc6if())
            .finish()
    }
}
impl W {
    ///Bit 0 - UIF
    #[inline(always)]
    pub fn uif(&mut self) -> UIF_W<'_, SRrs> {
        UIF_W::new(self, 0)
    }
    ///Bit 1 - CC1IF
    #[inline(always)]
    pub fn cc1if(&mut self) -> CC1IF_W<'_, SRrs> {
        CC1IF_W::new(self, 1)
    }
    ///Bit 2 - CC2IF
    #[inline(always)]
    pub fn cc2if(&mut self) -> CC2IF_W<'_, SRrs> {
        CC2IF_W::new(self, 2)
    }
    ///Bit 3 - CC3IF
    #[inline(always)]
    pub fn cc3if(&mut self) -> CC3IF_W<'_, SRrs> {
        CC3IF_W::new(self, 3)
    }
    ///Bit 4 - CC4IF
    #[inline(always)]
    pub fn cc4if(&mut self) -> CC4IF_W<'_, SRrs> {
        CC4IF_W::new(self, 4)
    }
    ///Bit 5 - COMIF
    #[inline(always)]
    pub fn comif(&mut self) -> COMIF_W<'_, SRrs> {
        COMIF_W::new(self, 5)
    }
    ///Bit 6 - TIF
    #[inline(always)]
    pub fn tif(&mut self) -> TIF_W<'_, SRrs> {
        TIF_W::new(self, 6)
    }
    ///Bit 7 - BIF
    #[inline(always)]
    pub fn bif(&mut self) -> BIF_W<'_, SRrs> {
        BIF_W::new(self, 7)
    }
    ///Bit 8 - B2IF
    #[inline(always)]
    pub fn b2if(&mut self) -> B2IF_W<'_, SRrs> {
        B2IF_W::new(self, 8)
    }
    ///Bit 9 - CC1OF
    #[inline(always)]
    pub fn cc1of(&mut self) -> CC1OF_W<'_, SRrs> {
        CC1OF_W::new(self, 9)
    }
    ///Bit 10 - CC2OF
    #[inline(always)]
    pub fn cc2of(&mut self) -> CC2OF_W<'_, SRrs> {
        CC2OF_W::new(self, 10)
    }
    ///Bit 11 - CC3OF
    #[inline(always)]
    pub fn cc3of(&mut self) -> CC3OF_W<'_, SRrs> {
        CC3OF_W::new(self, 11)
    }
    ///Bit 12 - CC4OF
    #[inline(always)]
    pub fn cc4of(&mut self) -> CC4OF_W<'_, SRrs> {
        CC4OF_W::new(self, 12)
    }
    ///Bit 13 - SBIF
    #[inline(always)]
    pub fn sbif(&mut self) -> SBIF_W<'_, SRrs> {
        SBIF_W::new(self, 13)
    }
    ///Bit 16 - CC5IF
    #[inline(always)]
    pub fn cc5if(&mut self) -> CC5IF_W<'_, SRrs> {
        CC5IF_W::new(self, 16)
    }
    ///Bit 17 - CC6IF
    #[inline(always)]
    pub fn cc6if(&mut self) -> CC6IF_W<'_, SRrs> {
        CC6IF_W::new(self, 17)
    }
}
/**TIM4 status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TIM4:SR)*/
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
