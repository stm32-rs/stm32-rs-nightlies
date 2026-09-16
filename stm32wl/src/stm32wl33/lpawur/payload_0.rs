///Register `PAYLOAD_0` reader
pub type R = crate::R<PAYLOAD_0rs>;
///Field `PAYLOAD_0` reader - First part of the payload (Least significant Byte First)
pub type PAYLOAD_0_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - First part of the payload (Least significant Byte First)
    #[inline(always)]
    pub fn payload_0(&self) -> PAYLOAD_0_R {
        PAYLOAD_0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAYLOAD_0")
            .field("payload_0", &self.payload_0())
            .finish()
    }
}
/**PAYLOAD_0 register

You can [`read`](crate::Reg::read) this register and get [`payload_0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#LPAWUR:PAYLOAD_0)*/
pub struct PAYLOAD_0rs;
impl crate::RegisterSpec for PAYLOAD_0rs {
    type Ux = u32;
}
///`read()` method returns [`payload_0::R`](R) reader structure
impl crate::Readable for PAYLOAD_0rs {}
///`reset()` method sets PAYLOAD_0 to value 0
impl crate::Resettable for PAYLOAD_0rs {}
