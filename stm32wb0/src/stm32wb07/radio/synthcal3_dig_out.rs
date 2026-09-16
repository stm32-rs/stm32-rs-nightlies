///Register `SYNTHCAL3_DIG_OUT` reader
pub type R = crate::R<SYNTHCAL3_DIG_OUTrs>;
///Field `SYNTHCAL_DEBUG_BUS` reader - Calibration debug bus.
pub type SYNTHCAL_DEBUG_BUS_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - Calibration debug bus.
    #[inline(always)]
    pub fn synthcal_debug_bus(&self) -> SYNTHCAL_DEBUG_BUS_R {
        SYNTHCAL_DEBUG_BUS_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNTHCAL3_DIG_OUT")
            .field("synthcal_debug_bus", &self.synthcal_debug_bus())
            .finish()
    }
}
/**SYNTHCAL3_DIG_OUT register

You can [`read`](crate::Reg::read) this register and get [`synthcal3_dig_out::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB07.html#RADIO:SYNTHCAL3_DIG_OUT)*/
pub struct SYNTHCAL3_DIG_OUTrs;
impl crate::RegisterSpec for SYNTHCAL3_DIG_OUTrs {
    type Ux = u32;
}
///`read()` method returns [`synthcal3_dig_out::R`](R) reader structure
impl crate::Readable for SYNTHCAL3_DIG_OUTrs {}
///`reset()` method sets SYNTHCAL3_DIG_OUT to value 0
impl crate::Resettable for SYNTHCAL3_DIG_OUTrs {}
