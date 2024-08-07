///Register `HDP_VAL` reader
pub type R = crate::R<HDP_VALrs>;
///Field `HDPVAL` reader - HDPVAL
pub type HDPVAL_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - HDPVAL
    #[inline(always)]
    pub fn hdpval(&self) -> HDPVAL_R {
        HDPVAL_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HDP_VAL")
            .field("hdpval", &self.hdpval())
            .finish()
    }
}
/**HDP value

You can [`read`](crate::Reg::read) this register and get [`hdp_val::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HDP:HDP_VAL)*/
pub struct HDP_VALrs;
impl crate::RegisterSpec for HDP_VALrs {
    type Ux = u32;
}
///`read()` method returns [`hdp_val::R`](R) reader structure
impl crate::Readable for HDP_VALrs {}
///`reset()` method sets HDP_VAL to value 0
impl crate::Resettable for HDP_VALrs {
    const RESET_VALUE: u32 = 0;
}
