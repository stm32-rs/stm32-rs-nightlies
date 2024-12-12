///Register `SR` reader
pub type R = crate::R<SRrs>;
///Register `SR` writer
pub type W = crate::W<SRrs>;
///Field `DAC1RDY` reader - DAC channel1 ready status bit
pub type DAC1RDY_R = crate::BitReader;
///Field `DAC1RDY` writer - DAC channel1 ready status bit
pub type DAC1RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DORSTAT1` reader - DAC channel1 output register status bit
pub type DORSTAT1_R = crate::BitReader;
///Field `DORSTAT1` writer - DAC channel1 output register status bit
pub type DORSTAT1_W<'a, REG> = crate::BitWriter<'a, REG>;
/**DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDR1 {
    ///0: No DMA underrun error condition occurred for DAC channel x
    NoError = 0,
    ///1: DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    Error = 1,
}
impl From<DMAUDR1> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1) -> Self {
        variant as u8 != 0
    }
}
///Field `DMAUDR1` reader - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
pub type DMAUDR1_R = crate::BitReader<DMAUDR1>;
impl DMAUDR1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDR1 {
        match self.bits {
            false => DMAUDR1::NoError,
            true => DMAUDR1::Error,
        }
    }
    ///No DMA underrun error condition occurred for DAC channel x
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMAUDR1::NoError
    }
    ///DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMAUDR1::Error
    }
}
///Field `DMAUDR1` writer - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
pub type DMAUDR1_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDR1>;
impl<'a, REG> DMAUDR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///No DMA underrun error condition occurred for DAC channel x
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1::NoError)
    }
    ///DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1::Error)
    }
}
/**DAC Channel 1 calibration offset status This bit is set and cleared by hardware

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL_FLAG1 {
    ///0: Calibration trimming value is lower than the offset correction value
    Lower = 0,
    ///1: Calibration trimming value is equal or greater than the offset correction value
    EqualHigher = 1,
}
impl From<CAL_FLAG1> for bool {
    #[inline(always)]
    fn from(variant: CAL_FLAG1) -> Self {
        variant as u8 != 0
    }
}
///Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status This bit is set and cleared by hardware
pub type CAL_FLAG1_R = crate::BitReader<CAL_FLAG1>;
impl CAL_FLAG1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> CAL_FLAG1 {
        match self.bits {
            false => CAL_FLAG1::Lower,
            true => CAL_FLAG1::EqualHigher,
        }
    }
    ///Calibration trimming value is lower than the offset correction value
    #[inline(always)]
    pub fn is_lower(&self) -> bool {
        *self == CAL_FLAG1::Lower
    }
    ///Calibration trimming value is equal or greater than the offset correction value
    #[inline(always)]
    pub fn is_equal_higher(&self) -> bool {
        *self == CAL_FLAG1::EqualHigher
    }
}
/**DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization).

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWST1 {
    ///0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written
    Idle = 0,
    ///1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written
    Busy = 1,
}
impl From<BWST1> for bool {
    #[inline(always)]
    fn from(variant: BWST1) -> Self {
        variant as u8 != 0
    }
}
///Field `BWST1` reader - DAC Channel 1 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of DAC_SHSR1 is complete. (It takes about 3LSI periods of synchronization).
pub type BWST1_R = crate::BitReader<BWST1>;
impl BWST1_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> BWST1 {
        match self.bits {
            false => BWST1::Idle,
            true => BWST1::Busy,
        }
    }
    ///There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BWST1::Idle
    }
    ///There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BWST1::Busy
    }
}
///Field `DAC2RDY` reader - DAC channel 2 ready status bit
pub type DAC2RDY_R = crate::BitReader;
///Field `DAC2RDY` writer - DAC channel 2 ready status bit
pub type DAC2RDY_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DORSTAT2` reader - DAC channel 2 output register status bit
pub type DORSTAT2_R = crate::BitReader;
///Field `DORSTAT2` writer - DAC channel 2 output register status bit
pub type DORSTAT2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BWST2` reader - DAC Channel 2 busy writing sample time flag This bit is systematically set just after Sample & Hold mode enable and is set each time the software writes the register DAC_SHSR2, It is cleared by hardware when the write operation of DAC_SHSR2 is complete. (It takes about 3 LSI periods of synchronization).
pub use BWST1_R as BWST2_R;
///Field `CAL_FLAG2` reader - DAC Channel 2 calibration offset status This bit is set and cleared by hardware
pub use CAL_FLAG1_R as CAL_FLAG2_R;
///Field `DMAUDR2` reader - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
pub use DMAUDR1_R as DMAUDR2_R;
///Field `DMAUDR2` writer - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
pub use DMAUDR1_W as DMAUDR2_W;
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
    ///Bit 11 - DAC channel1 ready status bit
    #[inline(always)]
    pub fn dac1rdy(&mut self) -> DAC1RDY_W<SRrs> {
        DAC1RDY_W::new(self, 11)
    }
    ///Bit 12 - DAC channel1 output register status bit
    #[inline(always)]
    pub fn dorstat1(&mut self) -> DORSTAT1_W<SRrs> {
        DORSTAT1_W::new(self, 12)
    }
    ///Bit 13 - DAC channel1 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<SRrs> {
        DMAUDR1_W::new(self, 13)
    }
    ///Bit 27 - DAC channel 2 ready status bit
    #[inline(always)]
    pub fn dac2rdy(&mut self) -> DAC2RDY_W<SRrs> {
        DAC2RDY_W::new(self, 27)
    }
    ///Bit 28 - DAC channel 2 output register status bit
    #[inline(always)]
    pub fn dorstat2(&mut self) -> DORSTAT2_W<SRrs> {
        DORSTAT2_W::new(self, 28)
    }
    ///Bit 29 - DAC channel2 DMA underrun flag This bit is set by hardware and cleared by software (by writing it to 1).
    #[inline(always)]
    pub fn dmaudr2(&mut self) -> DMAUDR2_W<SRrs> {
        DMAUDR2_W::new(self, 29)
    }
}
/**DAC status register

You can [`read`](crate::Reg::read) this register and get [`sr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G431xx.html#DAC1:SR)*/
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
///`read()` method returns [`sr::R`](R) reader structure
impl crate::Readable for SRrs {}
///`write(|w| ..)` method takes [`sr::W`](W) writer structure
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SR to value 0
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
