///Register `RESP%sR` reader
pub type R = crate::R<RESPRrs>;
///Field `CARDSTATUS` reader - Status of a card, which is part of the received response
pub type CARDSTATUS_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Status of a card, which is part of the received response
    #[inline(always)]
    pub fn cardstatus(&self) -> CARDSTATUS_R {
        CARDSTATUS_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESPR")
            .field("cardstatus", &self.cardstatus())
            .finish()
    }
}
/**SDMMC response %s register

You can [`read`](crate::Reg::read) this register and get [`respr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#SDMMC1:RESP[1]R)*/
pub struct RESPRrs;
impl crate::RegisterSpec for RESPRrs {
    type Ux = u32;
}
///`read()` method returns [`respr::R`](R) reader structure
impl crate::Readable for RESPRrs {}
///`reset()` method sets RESP%sR to value 0
impl crate::Resettable for RESPRrs {}
