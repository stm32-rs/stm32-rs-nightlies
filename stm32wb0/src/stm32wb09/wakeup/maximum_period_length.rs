///Register `MAXIMUM_PERIOD_LENGTH` reader
pub type R = crate::R<MAXIMUM_PERIOD_LENGTHrs>;
///Field `LENGTH` reader - maximum period length computed by Time Interpolator
pub type LENGTH_R = crate::FieldReader<u16>;
impl R {
    ///Bits 4:13 - maximum period length computed by Time Interpolator
    #[inline(always)]
    pub fn length(&self) -> LENGTH_R {
        LENGTH_R::new(((self.bits >> 4) & 0x03ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAXIMUM_PERIOD_LENGTH")
            .field("length", &self.length())
            .finish()
    }
}
/**MAXIMUM_PERIOD_LENGTH register

You can [`read`](crate::Reg::read) this register and get [`maximum_period_length::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB09.html#WAKEUP:MAXIMUM_PERIOD_LENGTH)*/
pub struct MAXIMUM_PERIOD_LENGTHrs;
impl crate::RegisterSpec for MAXIMUM_PERIOD_LENGTHrs {
    type Ux = u32;
}
///`read()` method returns [`maximum_period_length::R`](R) reader structure
impl crate::Readable for MAXIMUM_PERIOD_LENGTHrs {}
///`reset()` method sets MAXIMUM_PERIOD_LENGTH to value 0
impl crate::Resettable for MAXIMUM_PERIOD_LENGTHrs {}
