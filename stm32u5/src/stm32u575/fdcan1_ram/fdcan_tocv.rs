///Register `FDCAN_TOCV` reader
pub type R = crate::R<FDCAN_TOCVrs>;
///Register `FDCAN_TOCV` writer
pub type W = crate::W<FDCAN_TOCVrs>;
///Field `TOC` reader - Timeout Counter
pub type TOC_R = crate::FieldReader<u16>;
///Field `TOC` writer - Timeout Counter
pub type TOC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Timeout Counter
    #[inline(always)]
    pub fn toc(&self) -> TOC_R {
        TOC_R::new((self.bits & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_TOCV")
            .field("toc", &self.toc())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Timeout Counter
    #[inline(always)]
    pub fn toc(&mut self) -> TOC_W<FDCAN_TOCVrs> {
        TOC_W::new(self, 0)
    }
}
/**FDCAN Timeout Counter Value Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_tocv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FDCAN1_RAM:FDCAN_TOCV)*/
pub struct FDCAN_TOCVrs;
impl crate::RegisterSpec for FDCAN_TOCVrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_tocv::R`](R) reader structure
impl crate::Readable for FDCAN_TOCVrs {}
///`write(|w| ..)` method takes [`fdcan_tocv::W`](W) writer structure
impl crate::Writable for FDCAN_TOCVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_TOCV to value 0xffff
impl crate::Resettable for FDCAN_TOCVrs {
    const RESET_VALUE: u32 = 0xffff;
}
