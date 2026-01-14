///Register `TOCV` reader
pub type R = crate::R<TOCVrs>;
///Field `TOC` reader - TOC
pub type TOC_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:15 - TOC
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOCV").field("toc", &self.toc()).finish()
    }
}
/**FDCAN Timeout Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`tocv::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G473.html#FDCAN1:TOCV)*/
pub struct TOCVrs;
impl crate::RegisterSpec for TOCVrs {
    type Ux = u32;
}
///`read()` method returns [`tocv::R`](R) reader structure
impl crate::Readable for TOCVrs {}
///`reset()` method sets TOCV to value 0xffff
impl crate::Resettable for TOCVrs {
    const RESET_VALUE: u32 = 0xffff;
}
