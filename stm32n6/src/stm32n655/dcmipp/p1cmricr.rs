///Register `P1CMRICR` reader
pub type R = crate::R<P1CMRICRrs>;
///Register `P1CMRICR` writer
pub type W = crate::W<P1CMRICRrs>;
///Field `ROILSZ` reader - Region of interest line size width
pub type ROILSZ_R = crate::FieldReader;
///Field `ROILSZ` writer - Region of interest line size width
pub type ROILSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ROI1EN` reader - Region of interest 1 enable
pub type ROI1EN_R = crate::BitReader;
///Field `ROI1EN` writer - Region of interest 1 enable
pub type ROI1EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROI2EN` reader - Region of interest 2 enable
pub type ROI2EN_R = crate::BitReader;
///Field `ROI2EN` writer - Region of interest 2 enable
pub type ROI2EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROI3EN` reader - Region of interest 3 enable
pub type ROI3EN_R = crate::BitReader;
///Field `ROI3EN` writer - Region of interest 3 enable
pub type ROI3EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROI4EN` reader - Region of interest 4 enable
pub type ROI4EN_R = crate::BitReader;
///Field `ROI4EN` writer - Region of interest 4 enable
pub type ROI4EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROI5EN` reader - Region of interest 5 enable
pub type ROI5EN_R = crate::BitReader;
///Field `ROI5EN` writer - Region of interest 5 enable
pub type ROI5EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROI6EN` reader - Region of interest 6 enable
pub type ROI6EN_R = crate::BitReader;
///Field `ROI6EN` writer - Region of interest 6 enable
pub type ROI6EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROI7EN` reader - Region of interest 7 enable
pub type ROI7EN_R = crate::BitReader;
///Field `ROI7EN` writer - Region of interest 7 enable
pub type ROI7EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ROI8EN` reader - Region of interest 8 enable
pub type ROI8EN_R = crate::BitReader;
///Field `ROI8EN` writer - Region of interest 8 enable
pub type ROI8EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:1 - Region of interest line size width
    #[inline(always)]
    pub fn roilsz(&self) -> ROILSZ_R {
        ROILSZ_R::new((self.bits & 3) as u8)
    }
    ///Bit 16 - Region of interest 1 enable
    #[inline(always)]
    pub fn roi1en(&self) -> ROI1EN_R {
        ROI1EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Region of interest 2 enable
    #[inline(always)]
    pub fn roi2en(&self) -> ROI2EN_R {
        ROI2EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Region of interest 3 enable
    #[inline(always)]
    pub fn roi3en(&self) -> ROI3EN_R {
        ROI3EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Region of interest 4 enable
    #[inline(always)]
    pub fn roi4en(&self) -> ROI4EN_R {
        ROI4EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Region of interest 5 enable
    #[inline(always)]
    pub fn roi5en(&self) -> ROI5EN_R {
        ROI5EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Region of interest 6 enable
    #[inline(always)]
    pub fn roi6en(&self) -> ROI6EN_R {
        ROI6EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Region of interest 7 enable
    #[inline(always)]
    pub fn roi7en(&self) -> ROI7EN_R {
        ROI7EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Region of interest 8 enable
    #[inline(always)]
    pub fn roi8en(&self) -> ROI8EN_R {
        ROI8EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("P1CMRICR")
            .field("roilsz", &self.roilsz())
            .field("roi1en", &self.roi1en())
            .field("roi2en", &self.roi2en())
            .field("roi3en", &self.roi3en())
            .field("roi4en", &self.roi4en())
            .field("roi5en", &self.roi5en())
            .field("roi6en", &self.roi6en())
            .field("roi7en", &self.roi7en())
            .field("roi8en", &self.roi8en())
            .finish()
    }
}
impl W {
    ///Bits 0:1 - Region of interest line size width
    #[inline(always)]
    pub fn roilsz(&mut self) -> ROILSZ_W<'_, P1CMRICRrs> {
        ROILSZ_W::new(self, 0)
    }
    ///Bit 16 - Region of interest 1 enable
    #[inline(always)]
    pub fn roi1en(&mut self) -> ROI1EN_W<'_, P1CMRICRrs> {
        ROI1EN_W::new(self, 16)
    }
    ///Bit 17 - Region of interest 2 enable
    #[inline(always)]
    pub fn roi2en(&mut self) -> ROI2EN_W<'_, P1CMRICRrs> {
        ROI2EN_W::new(self, 17)
    }
    ///Bit 18 - Region of interest 3 enable
    #[inline(always)]
    pub fn roi3en(&mut self) -> ROI3EN_W<'_, P1CMRICRrs> {
        ROI3EN_W::new(self, 18)
    }
    ///Bit 19 - Region of interest 4 enable
    #[inline(always)]
    pub fn roi4en(&mut self) -> ROI4EN_W<'_, P1CMRICRrs> {
        ROI4EN_W::new(self, 19)
    }
    ///Bit 20 - Region of interest 5 enable
    #[inline(always)]
    pub fn roi5en(&mut self) -> ROI5EN_W<'_, P1CMRICRrs> {
        ROI5EN_W::new(self, 20)
    }
    ///Bit 21 - Region of interest 6 enable
    #[inline(always)]
    pub fn roi6en(&mut self) -> ROI6EN_W<'_, P1CMRICRrs> {
        ROI6EN_W::new(self, 21)
    }
    ///Bit 22 - Region of interest 7 enable
    #[inline(always)]
    pub fn roi7en(&mut self) -> ROI7EN_W<'_, P1CMRICRrs> {
        ROI7EN_W::new(self, 22)
    }
    ///Bit 23 - Region of interest 8 enable
    #[inline(always)]
    pub fn roi8en(&mut self) -> ROI8EN_W<'_, P1CMRICRrs> {
        ROI8EN_W::new(self, 23)
    }
}
/**DCMIPP Pipex common ROI configuration register

You can [`read`](crate::Reg::read) this register and get [`p1cmricr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1cmricr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#DCMIPP:P1CMRICR)*/
pub struct P1CMRICRrs;
impl crate::RegisterSpec for P1CMRICRrs {
    type Ux = u32;
}
///`read()` method returns [`p1cmricr::R`](R) reader structure
impl crate::Readable for P1CMRICRrs {}
///`write(|w| ..)` method takes [`p1cmricr::W`](W) writer structure
impl crate::Writable for P1CMRICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets P1CMRICR to value 0
impl crate::Resettable for P1CMRICRrs {}
