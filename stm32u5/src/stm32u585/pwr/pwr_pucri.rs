///Register `PWR_PUCRI` reader
pub type R = crate::R<PWR_PUCRIrs>;
///Register `PWR_PUCRI` writer
pub type W = crate::W<PWR_PUCRIrs>;
///Field `PU0` reader - Port I pull-up bit
pub type PU0_R = crate::BitReader;
///Field `PU0` writer - Port I pull-up bit
pub type PU0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU1` reader - Port I pull-up bit
pub type PU1_R = crate::BitReader;
///Field `PU1` writer - Port I pull-up bit
pub type PU1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU2` reader - Port I pull-up bit
pub type PU2_R = crate::BitReader;
///Field `PU2` writer - Port I pull-up bit
pub type PU2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU3` reader - Port I pull-up bit
pub type PU3_R = crate::BitReader;
///Field `PU3` writer - Port I pull-up bit
pub type PU3_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU4` reader - Port I pull-up bit
pub type PU4_R = crate::BitReader;
///Field `PU4` writer - Port I pull-up bit
pub type PU4_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU5` reader - Port I pull-up bit
pub type PU5_R = crate::BitReader;
///Field `PU5` writer - Port I pull-up bit
pub type PU5_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU6` reader - Port I pull-up bit
pub type PU6_R = crate::BitReader;
///Field `PU6` writer - Port I pull-up bit
pub type PU6_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PU7` reader - Port I pull-up bit
pub type PU7_R = crate::BitReader;
///Field `PU7` writer - Port I pull-up bit
pub type PU7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Port I pull-up bit
    #[inline(always)]
    pub fn pu0(&self) -> PU0_R {
        PU0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port I pull-up bit
    #[inline(always)]
    pub fn pu1(&self) -> PU1_R {
        PU1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port I pull-up bit
    #[inline(always)]
    pub fn pu2(&self) -> PU2_R {
        PU2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port I pull-up bit
    #[inline(always)]
    pub fn pu3(&self) -> PU3_R {
        PU3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port I pull-up bit
    #[inline(always)]
    pub fn pu4(&self) -> PU4_R {
        PU4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port I pull-up bit
    #[inline(always)]
    pub fn pu5(&self) -> PU5_R {
        PU5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port I pull-up bit
    #[inline(always)]
    pub fn pu6(&self) -> PU6_R {
        PU6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Port I pull-up bit
    #[inline(always)]
    pub fn pu7(&self) -> PU7_R {
        PU7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWR_PUCRI")
            .field("pu0", &self.pu0())
            .field("pu1", &self.pu1())
            .field("pu2", &self.pu2())
            .field("pu3", &self.pu3())
            .field("pu4", &self.pu4())
            .field("pu5", &self.pu5())
            .field("pu6", &self.pu6())
            .field("pu7", &self.pu7())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port I pull-up bit
    #[inline(always)]
    #[must_use]
    pub fn pu0(&mut self) -> PU0_W<PWR_PUCRIrs> {
        PU0_W::new(self, 0)
    }
    ///Bit 1 - Port I pull-up bit
    #[inline(always)]
    #[must_use]
    pub fn pu1(&mut self) -> PU1_W<PWR_PUCRIrs> {
        PU1_W::new(self, 1)
    }
    ///Bit 2 - Port I pull-up bit
    #[inline(always)]
    #[must_use]
    pub fn pu2(&mut self) -> PU2_W<PWR_PUCRIrs> {
        PU2_W::new(self, 2)
    }
    ///Bit 3 - Port I pull-up bit
    #[inline(always)]
    #[must_use]
    pub fn pu3(&mut self) -> PU3_W<PWR_PUCRIrs> {
        PU3_W::new(self, 3)
    }
    ///Bit 4 - Port I pull-up bit
    #[inline(always)]
    #[must_use]
    pub fn pu4(&mut self) -> PU4_W<PWR_PUCRIrs> {
        PU4_W::new(self, 4)
    }
    ///Bit 5 - Port I pull-up bit
    #[inline(always)]
    #[must_use]
    pub fn pu5(&mut self) -> PU5_W<PWR_PUCRIrs> {
        PU5_W::new(self, 5)
    }
    ///Bit 6 - Port I pull-up bit
    #[inline(always)]
    #[must_use]
    pub fn pu6(&mut self) -> PU6_W<PWR_PUCRIrs> {
        PU6_W::new(self, 6)
    }
    ///Bit 7 - Port I pull-up bit
    #[inline(always)]
    #[must_use]
    pub fn pu7(&mut self) -> PU7_W<PWR_PUCRIrs> {
        PU7_W::new(self, 7)
    }
}
/**PWR port I pull-up control register

You can [`read`](crate::Reg::read) this register and get [`pwr_pucri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr_pucri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#PWR:PWR_PUCRI)*/
pub struct PWR_PUCRIrs;
impl crate::RegisterSpec for PWR_PUCRIrs {
    type Ux = u32;
}
///`read()` method returns [`pwr_pucri::R`](R) reader structure
impl crate::Readable for PWR_PUCRIrs {}
///`write(|w| ..)` method takes [`pwr_pucri::W`](W) writer structure
impl crate::Writable for PWR_PUCRIrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PWR_PUCRI to value 0
impl crate::Resettable for PWR_PUCRIrs {
    const RESET_VALUE: u32 = 0;
}
