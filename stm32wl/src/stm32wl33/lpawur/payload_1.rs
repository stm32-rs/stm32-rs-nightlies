///Register `PAYLOAD_1` reader
pub type R = crate::R<PAYLOAD_1rs>;
///Field `PAYLOAD_1` reader - Second part of the payload (Least significant Byte First)
pub type PAYLOAD_1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Second part of the payload (Least significant Byte First)
    #[inline(always)]
    pub fn payload_1(&self) -> PAYLOAD_1_R {
        PAYLOAD_1_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAYLOAD_1")
            .field("payload_1", &self.payload_1())
            .finish()
    }
}
/**PAYLOAD_1 register

You can [`read`](crate::Reg::read) this register and get [`payload_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:PAYLOAD_1)*/
pub struct PAYLOAD_1rs;
impl crate::RegisterSpec for PAYLOAD_1rs {
    type Ux = u32;
}
///`read()` method returns [`payload_1::R`](R) reader structure
impl crate::Readable for PAYLOAD_1rs {}
///`reset()` method sets PAYLOAD_1 to value 0
impl crate::Resettable for PAYLOAD_1rs {}
