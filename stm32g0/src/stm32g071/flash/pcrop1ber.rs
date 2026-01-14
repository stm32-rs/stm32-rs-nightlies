///Register `PCROP1BER` reader
pub type R = crate::R<PCROP1BERrs>;
///Field `PCROP1B_END` reader - PCROP1B area end offset
pub type PCROP1B_END_R = crate::FieldReader;
impl R {
    ///Bits 0:7 - PCROP1B area end offset
    #[inline(always)]
    pub fn pcrop1b_end(&self) -> PCROP1B_END_R {
        PCROP1B_END_R::new((self.bits & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PCROP1BER")
            .field("pcrop1b_end", &self.pcrop1b_end())
            .finish()
    }
}
/**Flash PCROP zone B End address register

You can [`read`](crate::Reg::read) this register and get [`pcrop1ber::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G071.html#FLASH:PCROP1BER)*/
pub struct PCROP1BERrs;
impl crate::RegisterSpec for PCROP1BERrs {
    type Ux = u32;
}
///`read()` method returns [`pcrop1ber::R`](R) reader structure
impl crate::Readable for PCROP1BERrs {}
///`reset()` method sets PCROP1BER to value 0xf000_0000
impl crate::Resettable for PCROP1BERrs {
    const RESET_VALUE: u32 = 0xf000_0000;
}
