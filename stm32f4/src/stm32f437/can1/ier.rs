///Register `IER` reader
pub type R = crate::R<IERrs>;
///Register `IER` writer
pub type W = crate::W<IERrs>;
///Field `TMEIE` reader - TMEIE
pub type TMEIE_R = crate::BitReader;
///Field `TMEIE` writer - TMEIE
pub type TMEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMPIE0` reader - FMPIE0
pub type FMPIE0_R = crate::BitReader;
///Field `FMPIE0` writer - FMPIE0
pub type FMPIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FFIE0` reader - FFIE0
pub type FFIE0_R = crate::BitReader;
///Field `FFIE0` writer - FFIE0
pub type FFIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FOVIE0` reader - FOVIE0
pub type FOVIE0_R = crate::BitReader;
///Field `FOVIE0` writer - FOVIE0
pub type FOVIE0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FMPIE1` reader - FMPIE1
pub type FMPIE1_R = crate::BitReader;
///Field `FMPIE1` writer - FMPIE1
pub type FMPIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FFIE1` reader - FFIE1
pub type FFIE1_R = crate::BitReader;
///Field `FFIE1` writer - FFIE1
pub type FFIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FOVIE1` reader - FOVIE1
pub type FOVIE1_R = crate::BitReader;
///Field `FOVIE1` writer - FOVIE1
pub type FOVIE1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EWGIE` reader - EWGIE
pub type EWGIE_R = crate::BitReader;
///Field `EWGIE` writer - EWGIE
pub type EWGIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `EPVIE` reader - EPVIE
pub type EPVIE_R = crate::BitReader;
///Field `EPVIE` writer - EPVIE
pub type EPVIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BOFIE` reader - BOFIE
pub type BOFIE_R = crate::BitReader;
///Field `BOFIE` writer - BOFIE
pub type BOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LECIE` reader - LECIE
pub type LECIE_R = crate::BitReader;
///Field `LECIE` writer - LECIE
pub type LECIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - ERRIE
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - ERRIE
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `WKUIE` reader - WKUIE
pub type WKUIE_R = crate::BitReader;
///Field `WKUIE` writer - WKUIE
pub type WKUIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SLKIE` reader - SLKIE
pub type SLKIE_R = crate::BitReader;
///Field `SLKIE` writer - SLKIE
pub type SLKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - TMEIE
    #[inline(always)]
    pub fn tmeie(&self) -> TMEIE_R {
        TMEIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - FMPIE0
    #[inline(always)]
    pub fn fmpie0(&self) -> FMPIE0_R {
        FMPIE0_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - FFIE0
    #[inline(always)]
    pub fn ffie0(&self) -> FFIE0_R {
        FFIE0_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - FOVIE0
    #[inline(always)]
    pub fn fovie0(&self) -> FOVIE0_R {
        FOVIE0_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - FMPIE1
    #[inline(always)]
    pub fn fmpie1(&self) -> FMPIE1_R {
        FMPIE1_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - FFIE1
    #[inline(always)]
    pub fn ffie1(&self) -> FFIE1_R {
        FFIE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - FOVIE1
    #[inline(always)]
    pub fn fovie1(&self) -> FOVIE1_R {
        FOVIE1_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 8 - EWGIE
    #[inline(always)]
    pub fn ewgie(&self) -> EWGIE_R {
        EWGIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - EPVIE
    #[inline(always)]
    pub fn epvie(&self) -> EPVIE_R {
        EPVIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - BOFIE
    #[inline(always)]
    pub fn bofie(&self) -> BOFIE_R {
        BOFIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - LECIE
    #[inline(always)]
    pub fn lecie(&self) -> LECIE_R {
        LECIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 15 - ERRIE
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - WKUIE
    #[inline(always)]
    pub fn wkuie(&self) -> WKUIE_R {
        WKUIE_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - SLKIE
    #[inline(always)]
    pub fn slkie(&self) -> SLKIE_R {
        SLKIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IER")
            .field("slkie", &self.slkie())
            .field("wkuie", &self.wkuie())
            .field("errie", &self.errie())
            .field("lecie", &self.lecie())
            .field("bofie", &self.bofie())
            .field("epvie", &self.epvie())
            .field("ewgie", &self.ewgie())
            .field("fovie1", &self.fovie1())
            .field("ffie1", &self.ffie1())
            .field("fmpie1", &self.fmpie1())
            .field("fovie0", &self.fovie0())
            .field("ffie0", &self.ffie0())
            .field("fmpie0", &self.fmpie0())
            .field("tmeie", &self.tmeie())
            .finish()
    }
}
impl W {
    ///Bit 0 - TMEIE
    #[inline(always)]
    pub fn tmeie(&mut self) -> TMEIE_W<'_, IERrs> {
        TMEIE_W::new(self, 0)
    }
    ///Bit 1 - FMPIE0
    #[inline(always)]
    pub fn fmpie0(&mut self) -> FMPIE0_W<'_, IERrs> {
        FMPIE0_W::new(self, 1)
    }
    ///Bit 2 - FFIE0
    #[inline(always)]
    pub fn ffie0(&mut self) -> FFIE0_W<'_, IERrs> {
        FFIE0_W::new(self, 2)
    }
    ///Bit 3 - FOVIE0
    #[inline(always)]
    pub fn fovie0(&mut self) -> FOVIE0_W<'_, IERrs> {
        FOVIE0_W::new(self, 3)
    }
    ///Bit 4 - FMPIE1
    #[inline(always)]
    pub fn fmpie1(&mut self) -> FMPIE1_W<'_, IERrs> {
        FMPIE1_W::new(self, 4)
    }
    ///Bit 5 - FFIE1
    #[inline(always)]
    pub fn ffie1(&mut self) -> FFIE1_W<'_, IERrs> {
        FFIE1_W::new(self, 5)
    }
    ///Bit 6 - FOVIE1
    #[inline(always)]
    pub fn fovie1(&mut self) -> FOVIE1_W<'_, IERrs> {
        FOVIE1_W::new(self, 6)
    }
    ///Bit 8 - EWGIE
    #[inline(always)]
    pub fn ewgie(&mut self) -> EWGIE_W<'_, IERrs> {
        EWGIE_W::new(self, 8)
    }
    ///Bit 9 - EPVIE
    #[inline(always)]
    pub fn epvie(&mut self) -> EPVIE_W<'_, IERrs> {
        EPVIE_W::new(self, 9)
    }
    ///Bit 10 - BOFIE
    #[inline(always)]
    pub fn bofie(&mut self) -> BOFIE_W<'_, IERrs> {
        BOFIE_W::new(self, 10)
    }
    ///Bit 11 - LECIE
    #[inline(always)]
    pub fn lecie(&mut self) -> LECIE_W<'_, IERrs> {
        LECIE_W::new(self, 11)
    }
    ///Bit 15 - ERRIE
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, IERrs> {
        ERRIE_W::new(self, 15)
    }
    ///Bit 16 - WKUIE
    #[inline(always)]
    pub fn wkuie(&mut self) -> WKUIE_W<'_, IERrs> {
        WKUIE_W::new(self, 16)
    }
    ///Bit 17 - SLKIE
    #[inline(always)]
    pub fn slkie(&mut self) -> SLKIE_W<'_, IERrs> {
        SLKIE_W::new(self, 17)
    }
}
/**interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#CAN1:IER)*/
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
///`read()` method returns [`ier::R`](R) reader structure
impl crate::Readable for IERrs {}
///`write(|w| ..)` method takes [`ier::W`](W) writer structure
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IER to value 0
impl crate::Resettable for IERrs {}
