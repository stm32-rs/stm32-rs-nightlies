///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `DMAUDR1` reader - DMAUDR1
pub type DMAUDR1_R = crate::BitReader;
///Field `DMAUDR1` writer - DMAUDR1
pub type DMAUDR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAL_FLAG1` reader - CAL_FLAG1
pub type CAL_FLAG1_R = crate::BitReader;
///Field `BWST1` reader - BWST1
pub type BWST1_R = crate::BitReader;
///Field `DMAUDR2` reader - DMAUDR2
pub type DMAUDR2_R = crate::BitReader;
///Field `DMAUDR2` writer - DMAUDR2
pub type DMAUDR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAL_FLAG2` reader - CAL_FLAG2
pub type CAL_FLAG2_R = crate::BitReader;
///Field `BWST2` reader - BWST2
pub type BWST2_R = crate::BitReader;
impl R {
    ///Bit 13 - DMAUDR1
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CAL_FLAG1
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - BWST1
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 29 - DMAUDR2
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - CAL_FLAG2
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - BWST2
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SR")
            .field("dmaudr1", &self.dmaudr1())
            .field("cal_flag1", &self.cal_flag1())
            .field("bwst1", &self.bwst1())
            .field("dmaudr2", &self.dmaudr2())
            .field("cal_flag2", &self.cal_flag2())
            .field("bwst2", &self.bwst2())
            .finish()
    }
}
impl W {
    ///Bit 13 - DMAUDR1
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<'_, SRrs> {
        DMAUDR1_W::new(self, 13)
    }
    ///Bit 29 - DMAUDR2
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<'_, SRrs> {
        DMAUDR2_W::new(self, 29)
    }
}
/**DAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#DAC1:SR)*/
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
