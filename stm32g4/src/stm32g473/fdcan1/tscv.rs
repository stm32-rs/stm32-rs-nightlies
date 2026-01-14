///Register `TSCV` reader
pub type R = crate::R<TSCVrs>;
///Field `TSC` reader - TSC
pub type TSC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - TSC
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSCV").field("tsc", &self.tsc()).finish()
    }
}
/**FDCAN Timestamp Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`tscv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#FDCAN1:TSCV)*/
pub struct TSCVrs;
impl crate::RegisterSpec for TSCVrs {
    type Ux = u32;
}
///`read()` method returns [`tscv::R`](R) reader structure
impl crate::Readable for TSCVrs {}
///`reset()` method sets TSCV to value 0
impl crate::Resettable for TSCVrs {}
