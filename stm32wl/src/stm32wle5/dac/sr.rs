#[doc = "Register `SR` reader"]
pub type R = crate::R<SRrs>;
#[doc = "Register `SR` writer"]
pub type W = crate::W<SRrs>;
#[doc = "DAC channel1 DMA underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAUDR1 {
    #[doc = "0: No DMA underrun error condition occurred for DAC channel x"]
    NoError = 0,
    #[doc = "1: DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    Error = 1,
}
impl From<DMAUDR1> for bool {
    #[inline(always)]
    fn from(variant: DMAUDR1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAUDR1` reader - DAC channel1 DMA underrun flag"]
pub type DMAUDR1_R = crate::BitReader<DMAUDR1>;
impl DMAUDR1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DMAUDR1 {
        match self.bits {
            false => DMAUDR1::NoError,
            true => DMAUDR1::Error,
        }
    }
    #[doc = "No DMA underrun error condition occurred for DAC channel x"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == DMAUDR1::NoError
    }
    #[doc = "DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == DMAUDR1::Error
    }
}
#[doc = "Field `DMAUDR1` writer - DAC channel1 DMA underrun flag"]
pub type DMAUDR1_W<'a, REG> = crate::BitWriter<'a, REG, DMAUDR1>;
impl<'a, REG> DMAUDR1_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No DMA underrun error condition occurred for DAC channel x"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1::NoError)
    }
    #[doc = "DMA underrun error condition occurred for DAC channel x (the currently selected trigger is driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)"]
    #[inline(always)]
    pub fn error(self) -> &'a mut crate::W<REG> {
        self.variant(DMAUDR1::Error)
    }
}
#[doc = "DAC Channel 1 calibration offset status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAL_FLAG1 {
    #[doc = "0: Calibration trimming value is lower than the offset correction value"]
    Lower = 0,
    #[doc = "1: Calibration trimming value is equal or greater than the offset correction value"]
    EqualHigher = 1,
}
impl From<CAL_FLAG1> for bool {
    #[inline(always)]
    fn from(variant: CAL_FLAG1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAL_FLAG1` reader - DAC Channel 1 calibration offset status"]
pub type CAL_FLAG1_R = crate::BitReader<CAL_FLAG1>;
impl CAL_FLAG1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAL_FLAG1 {
        match self.bits {
            false => CAL_FLAG1::Lower,
            true => CAL_FLAG1::EqualHigher,
        }
    }
    #[doc = "Calibration trimming value is lower than the offset correction value"]
    #[inline(always)]
    pub fn is_lower(&self) -> bool {
        *self == CAL_FLAG1::Lower
    }
    #[doc = "Calibration trimming value is equal or greater than the offset correction value"]
    #[inline(always)]
    pub fn is_equal_higher(&self) -> bool {
        *self == CAL_FLAG1::EqualHigher
    }
}
#[doc = "DAC Channel 1 busy writing sample time flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BWST1 {
    #[doc = "0: There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written"]
    Idle = 0,
    #[doc = "1: There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written"]
    Busy = 1,
}
impl From<BWST1> for bool {
    #[inline(always)]
    fn from(variant: BWST1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BWST1` reader - DAC Channel 1 busy writing sample time flag"]
pub type BWST1_R = crate::BitReader<BWST1>;
impl BWST1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BWST1 {
        match self.bits {
            false => BWST1::Idle,
            true => BWST1::Busy,
        }
    }
    #[doc = "There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == BWST1::Idle
    }
    #[doc = "There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BWST1::Busy
    }
}
impl R {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    pub fn dmaudr1(&self) -> DMAUDR1_R {
        DMAUDR1_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DAC Channel 1 calibration offset status"]
    #[inline(always)]
    pub fn cal_flag1(&self) -> CAL_FLAG1_R {
        CAL_FLAG1_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DAC Channel 1 busy writing sample time flag"]
    #[inline(always)]
    pub fn bwst1(&self) -> BWST1_R {
        BWST1_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - DAC channel1 DMA underrun flag"]
    #[inline(always)]
    #[must_use]
    pub fn dmaudr1(&mut self) -> DMAUDR1_W<SRrs> {
        DMAUDR1_W::new(self, 13)
    }
}
#[doc = "status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SRrs;
impl crate::RegisterSpec for SRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SRrs {}
#[doc = "`write(|w| ..)` method takes [`sr::W`](W) writer structure"]
impl crate::Writable for SRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SRrs {
    const RESET_VALUE: u32 = 0;
}
