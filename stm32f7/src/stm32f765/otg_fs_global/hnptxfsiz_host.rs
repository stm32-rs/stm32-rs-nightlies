///Register `HNPTXFSIZ_Host` reader
pub type R = crate::R<HNPTXFSIZ_HOSTrs>;
///Register `HNPTXFSIZ_Host` writer
pub type W = crate::W<HNPTXFSIZ_HOSTrs>;
///Field `NPTXFSA` reader - Non-periodic transmit RAM start address
pub type NPTXFSA_R = crate::FieldReader<u16>;
///Field `NPTXFSA` writer - Non-periodic transmit RAM start address
pub type NPTXFSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `NPTXFD` reader - Non-periodic TxFIFO depth
pub type NPTXFD_R = crate::FieldReader<u16>;
///Field `NPTXFD` writer - Non-periodic TxFIFO depth
pub type NPTXFD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Non-periodic transmit RAM start address
    #[inline(always)]
    pub fn nptxfsa(&self) -> NPTXFSA_R {
        NPTXFSA_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - Non-periodic TxFIFO depth
    #[inline(always)]
    pub fn nptxfd(&self) -> NPTXFD_R {
        NPTXFD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HNPTXFSIZ_Host")
            .field("nptxfsa", &self.nptxfsa())
            .field("nptxfd", &self.nptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Non-periodic transmit RAM start address
    #[inline(always)]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<'_, HNPTXFSIZ_HOSTrs> {
        NPTXFSA_W::new(self, 0)
    }
    ///Bits 16:31 - Non-periodic TxFIFO depth
    #[inline(always)]
    pub fn nptxfd(&mut self) -> NPTXFD_W<'_, HNPTXFSIZ_HOSTrs> {
        NPTXFD_W::new(self, 16)
    }
}
/**OTG_FS Host non-periodic transmit FIFO size register

You can [`read`](crate::Reg::read) this register and get [`hnptxfsiz_host::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hnptxfsiz_host::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F765.html#OTG_FS_GLOBAL:HNPTXFSIZ_Host)*/
pub struct HNPTXFSIZ_HOSTrs;
impl crate::RegisterSpec for HNPTXFSIZ_HOSTrs {
    type Ux = u32;
}
///`read()` method returns [`hnptxfsiz_host::R`](R) reader structure
impl crate::Readable for HNPTXFSIZ_HOSTrs {}
///`write(|w| ..)` method takes [`hnptxfsiz_host::W`](W) writer structure
impl crate::Writable for HNPTXFSIZ_HOSTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HNPTXFSIZ_Host to value 0x0200
impl crate::Resettable for HNPTXFSIZ_HOSTrs {
    const RESET_VALUE: u32 = 0x0200;
}
