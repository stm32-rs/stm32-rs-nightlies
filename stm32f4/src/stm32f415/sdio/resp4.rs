///Register `RESP4` reader
pub type R = crate::R<RESP4rs>;
///Field `CARDSTATUS4` reader - see Table 132.
pub type CARDSTATUS4_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - see Table 132.
    #[inline(always)]
    pub fn cardstatus4(&self) -> CARDSTATUS4_R {
        CARDSTATUS4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP4")
            .field("cardstatus4", &self.cardstatus4())
            .finish()
    }
}
/**response 1..4 register

You can [`read`](crate::Reg::read) this register and get [`resp4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F415.html#SDIO:RESP4)*/
pub struct RESP4rs;
impl crate::RegisterSpec for RESP4rs {
    type Ux = u32;
}
///`read()` method returns [`resp4::R`](R) reader structure
impl crate::Readable for RESP4rs {}
///`reset()` method sets RESP4 to value 0
impl crate::Resettable for RESP4rs {}
