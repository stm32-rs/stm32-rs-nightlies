///Register `LTDC_LIPCR` reader
pub type R = crate::R<LTDC_LIPCRrs>;
///Register `LTDC_LIPCR` writer
pub type W = crate::W<LTDC_LIPCRrs>;
///Field `LIPOS` reader - line interrupt position These bits configure the line interrupt position.
pub type LIPOS_R = crate::FieldReader<u16>;
///Field `LIPOS` writer - line interrupt position These bits configure the line interrupt position.
pub type LIPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    ///Bits 0:10 - line interrupt position These bits configure the line interrupt position.
    #[inline(always)]
    pub fn lipos(&self) -> LIPOS_R {
        LIPOS_R::new((self.bits & 0x07ff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_LIPCR")
            .field("lipos", &self.lipos())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - line interrupt position These bits configure the line interrupt position.
    #[inline(always)]
    #[must_use]
    pub fn lipos(&mut self) -> LIPOS_W<LTDC_LIPCRrs> {
        LIPOS_W::new(self, 0)
    }
}
/**LTDC line interrupt position configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_lipcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_lipcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:LTDC_LIPCR)*/
pub struct LTDC_LIPCRrs;
impl crate::RegisterSpec for LTDC_LIPCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_lipcr::R`](R) reader structure
impl crate::Readable for LTDC_LIPCRrs {}
///`write(|w| ..)` method takes [`ltdc_lipcr::W`](W) writer structure
impl crate::Writable for LTDC_LIPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_LIPCR to value 0
impl crate::Resettable for LTDC_LIPCRrs {
    const RESET_VALUE: u32 = 0;
}
