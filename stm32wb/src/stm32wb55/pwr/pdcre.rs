///Register `PDCRE` reader
pub type R = crate::R<PDCRErs>;
///Register `PDCRE` writer
pub type W = crate::W<PDCRErs>;
///Field `PD0` reader - Port E pull-down bit y (y=0..15)
pub type PD0_R = crate::BitReader;
///Field `PD0` writer - Port E pull-down bit y (y=0..15)
pub type PD0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD1` reader - Port E pull-down bit y (y=0..15)
pub type PD1_R = crate::BitReader;
///Field `PD1` writer - Port E pull-down bit y (y=0..15)
pub type PD1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD2` reader - Port E pull-down bit y (y=0..15)
pub type PD2_R = crate::BitReader;
///Field `PD2` writer - Port E pull-down bit y (y=0..15)
pub type PD2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD3` reader - Port E pull-down bit y (y=0..15)
pub type PD3_R = crate::BitReader;
///Field `PD3` writer - Port E pull-down bit y (y=0..15)
pub type PD3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PD4` reader - Port E pull-down bit y (y=0..15)
pub type PD4_R = crate::BitReader;
///Field `PD4` writer - Port E pull-down bit y (y=0..15)
pub type PD4_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd0(&self) -> PD0_R {
        PD0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd1(&self) -> PD1_R {
        PD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd2(&self) -> PD2_R {
        PD2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd3(&self) -> PD3_R {
        PD3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd4(&self) -> PD4_R {
        PD4_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PDCRE")
            .field("pd4", &self.pd4())
            .field("pd3", &self.pd3())
            .field("pd2", &self.pd2())
            .field("pd1", &self.pd1())
            .field("pd0", &self.pd0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd0(&mut self) -> PD0_W<'_, PDCRErs> {
        PD0_W::new(self, 0)
    }
    ///Bit 1 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd1(&mut self) -> PD1_W<'_, PDCRErs> {
        PD1_W::new(self, 1)
    }
    ///Bit 2 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd2(&mut self) -> PD2_W<'_, PDCRErs> {
        PD2_W::new(self, 2)
    }
    ///Bit 3 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd3(&mut self) -> PD3_W<'_, PDCRErs> {
        PD3_W::new(self, 3)
    }
    ///Bit 4 - Port E pull-down bit y (y=0..15)
    #[inline(always)]
    pub fn pd4(&mut self) -> PD4_W<'_, PDCRErs> {
        PD4_W::new(self, 4)
    }
}
/**Power Port E pull-down control register

You can [`read`](crate::Reg::read) this register and get [`pdcre::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdcre::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#PWR:PDCRE)*/
pub struct PDCRErs;
impl crate::RegisterSpec for PDCRErs {
    type Ux = u32;
}
///`read()` method returns [`pdcre::R`](R) reader structure
impl crate::Readable for PDCRErs {}
///`write(|w| ..)` method takes [`pdcre::W`](W) writer structure
impl crate::Writable for PDCRErs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PDCRE to value 0
impl crate::Resettable for PDCRErs {}
