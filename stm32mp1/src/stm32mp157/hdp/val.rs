///Register `VAL` reader
pub type R = crate::R<VALrs>;
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
        f.debug_struct("VAL")
            .field("hdpval", &self.hdpval())
            .finish()
    }
}
/**HDP value

You can [`read`](crate::Reg::read) this register and get [`val::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#HDP:VAL)*/
pub struct VALrs;
impl crate::RegisterSpec for VALrs {
    type Ux = u32;
}
///`read()` method returns [`val::R`](R) reader structure
impl crate::Readable for VALrs {}
///`reset()` method sets VAL to value 0
impl crate::Resettable for VALrs {
    const RESET_VALUE: u32 = 0;
}