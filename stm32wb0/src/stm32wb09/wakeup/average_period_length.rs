///Register `AVERAGE_PERIOD_LENGTH` reader
pub type R = crate::R<AVERAGE_PERIOD_LENGTHrs>;
///Field `LENGTH_FRACT` reader - additional information/precision on slow clock frequency.
pub type LENGTH_FRACT_R = crate::FieldReader;
///Field `LENGTH_INT` reader - average period length computed by Time Interpolator.
pub type LENGTH_INT_R = crate::FieldReader<u16>;
///Field `AVERAGE_COUNT` reader - Number of slow clock cycles.
pub type AVERAGE_COUNT_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - additional information/precision on slow clock frequency.
    #[inline(always)]
    pub fn length_fract(&self) -> LENGTH_FRACT_R {
        LENGTH_FRACT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:13 - average period length computed by Time Interpolator.
    #[inline(always)]
    pub fn length_int(&self) -> LENGTH_INT_R {
        LENGTH_INT_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
    ///Bits 24:31 - Number of slow clock cycles.
    #[inline(always)]
    pub fn average_count(&self) -> AVERAGE_COUNT_R {
        AVERAGE_COUNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AVERAGE_PERIOD_LENGTH")
            .field("length_fract", &self.length_fract())
            .field("length_int", &self.length_int())
            .field("average_count", &self.average_count())
            .finish()
    }
}
/**AVERAGE_PERIOD_LENGTH register

You can [`read`](crate::Reg::read) this register and get [`average_period_length::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#WAKEUP:AVERAGE_PERIOD_LENGTH)*/
pub struct AVERAGE_PERIOD_LENGTHrs;
impl crate::RegisterSpec for AVERAGE_PERIOD_LENGTHrs {
    type Ux = u32;
}
///`read()` method returns [`average_period_length::R`](R) reader structure
impl crate::Readable for AVERAGE_PERIOD_LENGTHrs {}
///`reset()` method sets AVERAGE_PERIOD_LENGTH to value 0
impl crate::Resettable for AVERAGE_PERIOD_LENGTHrs {}
