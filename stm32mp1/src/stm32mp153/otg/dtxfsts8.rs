///Register `DTXFSTS8` reader
pub type R = crate::R<DTXFSTS8rs>;
///Field `INEPTFSAV` reader - INEPTFSAV
pub type INEPTFSAV_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - INEPTFSAV
    #[inline(always)]
    pub fn ineptfsav(&self) -> INEPTFSAV_R {
        INEPTFSAV_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DTXFSTS8")
            .field("ineptfsav", &self.ineptfsav())
            .finish()
    }
}
/**This read-only register contains the free space information for the device IN endpoint Tx FIFO.

You can [`read`](crate::Reg::read) this register and get [`dtxfsts8::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#OTG:DTXFSTS8)*/
pub struct DTXFSTS8rs;
impl crate::RegisterSpec for DTXFSTS8rs {
    type Ux = u32;
}
///`read()` method returns [`dtxfsts8::R`](R) reader structure
impl crate::Readable for DTXFSTS8rs {}
///`reset()` method sets DTXFSTS8 to value 0x0200
impl crate::Resettable for DTXFSTS8rs {
    const RESET_VALUE: u32 = 0x0200;
}