///Register `STATUS2REG` reader
pub type R = crate::R<STATUS2REGrs>;
///Field `IQSAMPLESREADY` reader - indicates if IQ samples have been received on the last reception.
pub type IQSAMPLESREADY_R = crate::BitReader;
///Field `IQSAMPLESNUMBER` reader - indicate the number of IQ samples stored in the RAM buffer addressed by StatMach.
pub type IQSAMPLESNUMBER_R = crate::FieldReader;
///Field `IQSAMPLESMISSINGERROR` reader - IQ sample internal buffer overflow error flag.
pub type IQSAMPLESMISSINGERROR_R = crate::BitReader;
///Field `ANTENNASWITCHINGPATTERNACCESSERROR` reader - timing error flag related to Antenna Pattern not read on-time.
pub type ANTENNASWITCHINGPATTERNACCESSERROR_R = crate::BitReader;
///Field `ANTENNA_SWITCHING_PATTERN_ADDRESS_ERROR` reader - AHB access error flag.
pub type ANTENNA_SWITCHING_PATTERN_ADDRESS_ERROR_R = crate::BitReader;
impl R {
    ///Bit 0 - indicates if IQ samples have been received on the last reception.
    #[inline(always)]
    pub fn iqsamplesready(&self) -> IQSAMPLESREADY_R {
        IQSAMPLESREADY_R::new((self.bits & 1) != 0)
    }
    ///Bits 1:7 - indicate the number of IQ samples stored in the RAM buffer addressed by StatMach.
    #[inline(always)]
    pub fn iqsamplesnumber(&self) -> IQSAMPLESNUMBER_R {
        IQSAMPLESNUMBER_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    ///Bit 29 - IQ sample internal buffer overflow error flag.
    #[inline(always)]
    pub fn iqsamplesmissingerror(&self) -> IQSAMPLESMISSINGERROR_R {
        IQSAMPLESMISSINGERROR_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - timing error flag related to Antenna Pattern not read on-time.
    #[inline(always)]
    pub fn antennaswitchingpatternaccesserror(&self) -> ANTENNASWITCHINGPATTERNACCESSERROR_R {
        ANTENNASWITCHINGPATTERNACCESSERROR_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - AHB access error flag.
    #[inline(always)]
    pub fn antenna_switching_pattern_address_error(
        &self,
    ) -> ANTENNA_SWITCHING_PATTERN_ADDRESS_ERROR_R {
        ANTENNA_SWITCHING_PATTERN_ADDRESS_ERROR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS2REG")
            .field("iqsamplesready", &self.iqsamplesready())
            .field("iqsamplesnumber", &self.iqsamplesnumber())
            .field("iqsamplesmissingerror", &self.iqsamplesmissingerror())
            .field(
                "antennaswitchingpatternaccesserror",
                &self.antennaswitchingpatternaccesserror(),
            )
            .field(
                "antenna_switching_pattern_address_error",
                &self.antenna_switching_pattern_address_error(),
            )
            .finish()
    }
}
/**STATUS2REG register

You can [`read`](crate::Reg::read) this register and get [`status2reg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#BLUE:STATUS2REG)*/
pub struct STATUS2REGrs;
impl crate::RegisterSpec for STATUS2REGrs {
    type Ux = u32;
}
///`read()` method returns [`status2reg::R`](R) reader structure
impl crate::Readable for STATUS2REGrs {}
///`reset()` method sets STATUS2REG to value 0
impl crate::Resettable for STATUS2REGrs {}
