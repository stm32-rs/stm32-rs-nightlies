///Register `FS_GNPTXFSIZ_Host` reader
pub type R = crate::R<FS_GNPTXFSIZ_HOSTrs>;
///Register `FS_GNPTXFSIZ_Host` writer
pub type W = crate::W<FS_GNPTXFSIZ_HOSTrs>;
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
        f.debug_struct("FS_GNPTXFSIZ_Host")
            .field("nptxfsa", &self.nptxfsa())
            .field("nptxfd", &self.nptxfd())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Non-periodic transmit RAM start address
    #[inline(always)]
    pub fn nptxfsa(&mut self) -> NPTXFSA_W<'_, FS_GNPTXFSIZ_HOSTrs> {
        NPTXFSA_W::new(self, 0)
    }
    ///Bits 16:31 - Non-periodic TxFIFO depth
    #[inline(always)]
    pub fn nptxfd(&mut self) -> NPTXFD_W<'_, FS_GNPTXFSIZ_HOSTrs> {
        NPTXFD_W::new(self, 16)
    }
}
/**OTG_FS non-periodic transmit FIFO size register (Host mode)

You can [`read`](crate::Reg::read) this register and get [`fs_gnptxfsiz_host::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fs_gnptxfsiz_host::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F417.html#OTG_FS_GLOBAL:FS_GNPTXFSIZ_Host)*/
pub struct FS_GNPTXFSIZ_HOSTrs;
impl crate::RegisterSpec for FS_GNPTXFSIZ_HOSTrs {
    type Ux = u32;
}
///`read()` method returns [`fs_gnptxfsiz_host::R`](R) reader structure
impl crate::Readable for FS_GNPTXFSIZ_HOSTrs {}
///`write(|w| ..)` method takes [`fs_gnptxfsiz_host::W`](W) writer structure
impl crate::Writable for FS_GNPTXFSIZ_HOSTrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FS_GNPTXFSIZ_Host to value 0x0200
impl crate::Resettable for FS_GNPTXFSIZ_HOSTrs {
    const RESET_VALUE: u32 = 0x0200;
}
