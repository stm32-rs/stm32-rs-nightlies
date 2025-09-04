///Register `VLCCR` reader
pub type R = crate::R<VLCCRrs>;
///Field `HLINE` reader - Horizontal Line duration
pub type HLINE_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:14 - Horizontal Line duration
    #[inline(always)]
    pub fn hline(&self) -> HLINE_R {
        HLINE_R::new((self.bits & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VLCCR")
            .field("hline", &self.hline())
            .finish()
    }
}
/**DSI Host Video Line Current Configuration Register

You can [`read`](crate::Reg::read) this register and get [`vlccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#DSI:VLCCR)*/
pub struct VLCCRrs;
impl crate::RegisterSpec for VLCCRrs {
    type Ux = u32;
}
///`read()` method returns [`vlccr::R`](R) reader structure
impl crate::Readable for VLCCRrs {}
///`reset()` method sets VLCCR to value 0
impl crate::Resettable for VLCCRrs {}
