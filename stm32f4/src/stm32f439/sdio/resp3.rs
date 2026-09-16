///Register `RESP3` reader
pub type R = crate::R<RESP3rs>;
///Field `CARDSTATUS3` reader - see Table 132.
pub type CARDSTATUS3_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - see Table 132.
    #[inline(always)]
    pub fn cardstatus3(&self) -> CARDSTATUS3_R {
        CARDSTATUS3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP3")
            .field("cardstatus3", &self.cardstatus3())
            .finish()
    }
}
/**response 1..4 register

You can [`read`](crate::Reg::read) this register and get [`resp3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F439.html#SDIO:RESP3)*/
pub struct RESP3rs;
impl crate::RegisterSpec for RESP3rs {
    type Ux = u32;
}
///`read()` method returns [`resp3::R`](R) reader structure
impl crate::Readable for RESP3rs {}
///`reset()` method sets RESP3 to value 0
impl crate::Resettable for RESP3rs {}
