///Register `PUCRF` reader
pub type R = crate::R<PUCRFrs>;
///Register `PUCRF` writer
pub type W = crate::W<PUCRFrs>;
///Field `PU0` reader - Port F pull-up bit y (y=0..15)
pub type PU0_R = crate::BitReader;
///Field `PU0` writer - Port F pull-up bit y (y=0..15)
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU1` reader - Port F pull-up bit y (y=0..15)
pub type PU1_R = crate::BitReader;
///Field `PU1` writer - Port F pull-up bit y (y=0..15)
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU2` reader - Port F pull-up bit y (y=0..15)
pub type PU2_R = crate::BitReader;
///Field `PU2` writer - Port F pull-up bit y (y=0..15)
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU3` reader - Port F pull-up bit y (y=0..15)
pub type PU3_R = crate::BitReader;
///Field `PU3` writer - Port F pull-up bit y (y=0..15)
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU4` reader - Port F pull-up bit y (y=0..15)
pub type PU4_R = crate::BitReader;
///Field `PU4` writer - Port F pull-up bit y (y=0..15)
pub type PU4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU5` reader - Port F pull-up bit y (y=0..15)
pub type PU5_R = crate::BitReader;
///Field `PU5` writer - Port F pull-up bit y (y=0..15)
pub type PU5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU6` reader - Port F pull-up bit y (y=0..15)
pub type PU6_R = crate::BitReader;
///Field `PU6` writer - Port F pull-up bit y (y=0..15)
pub type PU6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU7` reader - Port F pull-up bit y (y=0..15)
pub type PU7_R = crate::BitReader;
///Field `PU7` writer - Port F pull-up bit y (y=0..15)
pub type PU7_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU8` reader - Port F pull-up bit y (y=0..15)
pub type PU8_R = crate::BitReader;
///Field `PU8` writer - Port F pull-up bit y (y=0..15)
pub type PU8_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU9` reader - Port F pull-up bit y (y=0..15)
pub type PU9_R = crate::BitReader;
///Field `PU9` writer - Port F pull-up bit y (y=0..15)
pub type PU9_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU10` reader - Port F pull-up bit y (y=0..15)
pub type PU10_R = crate::BitReader;
///Field `PU10` writer - Port F pull-up bit y (y=0..15)
pub type PU10_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU11` reader - Port F pull-up bit y (y=0..15)
pub type PU11_R = crate::BitReader;
///Field `PU11` writer - Port F pull-up bit y (y=0..15)
pub type PU11_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU12` reader - Port F pull-up bit y (y=0..15)
pub type PU12_R = crate::BitReader;
///Field `PU12` writer - Port F pull-up bit y (y=0..15)
pub type PU12_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU13` reader - Port F pull-up bit y (y=0..15)
pub type PU13_R = crate::BitReader;
///Field `PU13` writer - Port F pull-up bit y (y=0..15)
pub type PU13_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu8(&self) -> PU8_R {
        PU8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu9(&self) -> PU9_R {
        PU9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu10(&self) -> PU10_R {
        PU10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu11(&self) -> PU11_R {
        PU11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu12(&self) -> PU12_R {
        PU12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu13(&self) -> PU13_R {
        PU13_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCRF")
            .field("pu13", &self.pu13())
            .field("pu12", &self.pu12())
            .field("pu11", &self.pu11())
            .field("pu10", &self.pu10())
            .field("pu9", &self.pu9())
            .field("pu8", &self.pu8())
            .field("pu7", &self.pu7())
            .field("pu6", &self.pu6())
            .field("pu5", &self.pu5())
            .field("pu4", &self.pu4())
            .field("pu3", &self.pu3())
            .field("pu2", &self.pu2())
            .field("pu1", &self.pu1())
            .field("pu0", &self.pu0())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu0(&mut self) -> PU0_W<'_, PUCRFrs> {
        PU0_W::new(self, 0)
    }
    ///Bit 1 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu1(&mut self) -> PU1_W<'_, PUCRFrs> {
        PU1_W::new(self, 1)
    }
    ///Bit 2 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu2(&mut self) -> PU2_W<'_, PUCRFrs> {
        PU2_W::new(self, 2)
    }
    ///Bit 3 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu3(&mut self) -> PU3_W<'_, PUCRFrs> {
        PU3_W::new(self, 3)
    }
    ///Bit 4 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu4(&mut self) -> PU4_W<'_, PUCRFrs> {
        PU4_W::new(self, 4)
    }
    ///Bit 5 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu5(&mut self) -> PU5_W<'_, PUCRFrs> {
        PU5_W::new(self, 5)
    }
    ///Bit 6 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu6(&mut self) -> PU6_W<'_, PUCRFrs> {
        PU6_W::new(self, 6)
    }
    ///Bit 7 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu7(&mut self) -> PU7_W<'_, PUCRFrs> {
        PU7_W::new(self, 7)
    }
    ///Bit 8 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu8(&mut self) -> PU8_W<'_, PUCRFrs> {
        PU8_W::new(self, 8)
    }
    ///Bit 9 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu9(&mut self) -> PU9_W<'_, PUCRFrs> {
        PU9_W::new(self, 9)
    }
    ///Bit 10 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu10(&mut self) -> PU10_W<'_, PUCRFrs> {
        PU10_W::new(self, 10)
    }
    ///Bit 11 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu11(&mut self) -> PU11_W<'_, PUCRFrs> {
        PU11_W::new(self, 11)
    }
    ///Bit 12 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu12(&mut self) -> PU12_W<'_, PUCRFrs> {
        PU12_W::new(self, 12)
    }
    ///Bit 13 - Port F pull-up bit y (y=0..15)
    #[inline(always)]
    pub fn pu13(&mut self) -> PU13_W<'_, PUCRFrs> {
        PU13_W::new(self, 13)
    }
}
/**Power Port F pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pucrf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucrf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G041.html#PWR:PUCRF)*/
pub struct PUCRFrs;
impl crate::RegisterSpec for PUCRFrs {
    type Ux = u32;
}
///`read()` method returns [`pucrf::R`](R) reader structure
impl crate::Readable for PUCRFrs {}
///`write(|w| ..)` method takes [`pucrf::W`](W) writer structure
impl crate::Writable for PUCRFrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCRF to value 0
impl crate::Resettable for PUCRFrs {}
