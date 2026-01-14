///Register `RESP2` reader
pub type R = crate::R<RESP2rs>;
///Field `CARDSTATUS2` reader - see Table 132.
pub type CARDSTATUS2_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - see Table 132.
    #[inline(always)]
    pub fn cardstatus2(&self) -> CARDSTATUS2_R {
        CARDSTATUS2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESP2")
            .field("cardstatus2", &self.cardstatus2())
            .finish()
    }
}
/**response 1..4 register

You can [`read`](crate::Reg::read) this register and get [`resp2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F437.html#SDIO:RESP2)*/
pub struct RESP2rs;
impl crate::RegisterSpec for RESP2rs {
    type Ux = u32;
}
///`read()` method returns [`resp2::R`](R) reader structure
impl crate::Readable for RESP2rs {}
///`reset()` method sets RESP2 to value 0
impl crate::Resettable for RESP2rs {}
