///Register `DAC_SR` reader
pub type R = crate::R<DAC_SRrs>;
///Register `DAC_SR` writer
pub type W = crate::W<DAC_SRrs>;
///Field `DAC1RDY` reader - DAC channel1 ready status bit
pub type DAC1RDY_R = crate::BitReader;
///Field `DORSTAT1` reader - DAC channel1 output register status bit
pub type DORSTAT1_R = crate::BitReader;
///Field `DMAUDR1` reader - DAC channel1 DMA underrun flag
pub type DMAUDR1_R = crate::BitReader;
///Field `DMAUDR1` writer - DAC channel1 DMA underrun flag
pub type DMAUDR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status
pub type CAL_FLAG1_R = crate::BitReader;
///Field `BWST1` reader - DAC Channel 1 busy writing sample time flag
pub type BWST1_R = crate::BitReader;
///Field `DAC2RDY` reader - DAC channel 2 ready status bit
pub type DAC2RDY_R = crate::BitReader;
///Field `DORSTAT2` reader - DAC channel 2 output register status bit
pub type DORSTAT2_R = crate::BitReader;
///Field `DMAUDR2` reader - DAC channel2 DMA underrun flag
pub type DMAUDR2_R = crate::BitReader;
///Field `DMAUDR2` writer - DAC channel2 DMA underrun flag
pub type DMAUDR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAL_FLAG2` reader - DAC Channel 2 calibration offset status
pub type CAL_FLAG2_R = crate::BitReader;
///Field `BWST2` reader - DAC Channel 2 busy writing sample time flag
pub type BWST2_R = crate::BitReader;
impl R {
    ///Bit 11 - DAC channel1 ready status bit
    #[inline(always)]
    pub fn dac1rdy(&self) -> DAC1RDY_R {
        DAC1RDY_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - DAC channel1 output register status bit
    #[inline(always)]
    pub fn dorstat1(&self) -> DORSTAT1_R {
        DORSTAT1_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DAC Channel 1 calibration offset status
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DAC Channel 1 busy writing sample time flag
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 27 - DAC channel 2 ready status bit
    #[inline(always)]
    pub fn dac2rdy(&self) -> DAC2RDY_R {
        DAC2RDY_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - DAC channel 2 output register status bit
    #[inline(always)]
    pub fn dorstat2(&self) -> DORSTAT2_R {
        DORSTAT2_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DAC Channel 2 calibration offset status
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DAC Channel 2 busy writing sample time flag
    #[inline(always)]
    pub fn bwst2(&self) -> BWST2_R {
        BWST2_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DAC_SR")
            .field("dac1rdy", &self.dac1rdy())
            .field("dorstat1", &self.dorstat1())
            .field("dmaudr1", &self.dmaudr1())
            .field("cal_flag1", &self.cal_flag1())
            .field("bwst1", &self.bwst1())
            .field("dac2rdy", &self.dac2rdy())
            .field("dorstat2", &self.dorstat2())
            .field("dmaudr2", &self.dmaudr2())
            .field("cal_flag2", &self.cal_flag2())
            .field("bwst2", &self.bwst2())
            .finish()
    }
}
impl W {
    ///Bit 13 - DAC channel1 DMA underrun flag
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<DAC_SRrs> {
        DMAUDR1_W::new(self, 13)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag
    #[inline(always)]
    #[must_use]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<DAC_SRrs> {
        DMAUDR2_W::new(self, 29)
    }
}
/**DAC status register

You can [`read`](crate::Reg::read) this register and get [`dac_sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dac_sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#DAC1:DAC_SR)*/
pub struct DAC_SRrs;
impl crate::RegisterSpec for DAC_SRrs {
    type Ux = u32;
}
///`read()` method returns [`dac_sr::R`](R) reader structure
impl crate::Readable for DAC_SRrs {}
///`write(|w| ..)` method takes [`dac_sr::W`](W) writer structure
impl crate::Writable for DAC_SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DAC_SR to value 0
impl crate::Resettable for DAC_SRrs {
    const RESET_VALUE: u32 = 0;
}
