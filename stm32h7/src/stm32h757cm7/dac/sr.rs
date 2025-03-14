///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `DMAUDR1` reader - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
pub type DMAUDR1_R = crate::BitReader;
///Field `DMAUDR1` writer - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
pub type DMAUDR1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status This bit is set and cleared by hardware
pub type CAL_FLAG1_R = crate::BitReader;
///Field `BWST1` reader - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization).
pub type BWST1_R = crate::BitReader;
///Field `DMAUDR2` reader - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
pub type DMAUDR2_R = crate::BitReader;
///Field `DMAUDR2` writer - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
pub type DMAUDR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAL_FLAG2` reader - DAC Channel 2 calibration offset status This bit is set and cleared by hardware
pub type CAL_FLAG2_R = crate::BitReader;
///Field `BWST2` reader - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization).
pub type BWST2_R = crate::BitReader;
impl R {
    ///Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - DAC Channel 1 calibration offset status This bit is set and cleared by hardware
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization).
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr2(&self) -> DMAUDR2_R {
        DMAUDR2_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - DAC Channel 2 calibration offset status This bit is set and cleared by hardware
    #[inline(always)]
    pub fn cal_flag2(&self) -> CAL_FLAG2_R {
        CAL_FLAG2_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization).
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
    ///Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<SRrs> {
        DMAUDR1_W::new(self, 13)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<SRrs> {
        DMAUDR2_W::new(self, 29)
    }
}
/**DAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM7.html#DAC:SR)*/
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
