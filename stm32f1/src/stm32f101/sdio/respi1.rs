///Register `RESPI1` reader
pub type R = crate::R<RESPI1rs>;
///Field `CARDSTATUS1` reader - CARDSTATUS1
pub type CARDSTATUS1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - CARDSTATUS1
    #[inline(always)]
    pub fn cardstatus1(&self) -> CARDSTATUS1_R {
        CARDSTATUS1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESPI1")
            .field("cardstatus1", &self.cardstatus1())
            .finish()
    }
}
/**Bits 31:0 = CARDSTATUS1

You can [`read`](crate::Reg::read) this register and get [`respi1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F101.html#SDIO:RESPI1)*/
pub struct RESPI1rs;
impl crate::RegisterSpec for RESPI1rs {
    type Ux = u32;
}
///`read()` method returns [`respi1::R`](R) reader structure
impl crate::Readable for RESPI1rs {}
///`reset()` method sets RESPI1 to value 0
impl crate::Resettable for RESPI1rs {}
