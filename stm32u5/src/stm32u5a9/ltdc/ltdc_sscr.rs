///Register `LTDC_SSCR` reader
pub type R = crate::R<LTDC_SSCRrs>;
///Register `LTDC_SSCR` writer
pub type W = crate::W<LTDC_SSCRrs>;
///Field `VSH` reader - vertical synchronization height (in units of horizontal scan line) These bits define the vertical Synchronization height minus 1. It represents the number of horizontal synchronization lines.
pub type VSH_R = crate::FieldReader<u16>;
///Field `VSH` writer - vertical synchronization height (in units of horizontal scan line) These bits define the vertical Synchronization height minus 1. It represents the number of horizontal synchronization lines.
pub type VSH_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `HSW` reader - horizontal synchronization width (in units of pixel clock period) These bits define the number of Horizontal Synchronization pixel minus 1.
pub type HSW_R = crate::FieldReader<u16>;
///Field `HSW` writer - horizontal synchronization width (in units of pixel clock period) These bits define the number of Horizontal Synchronization pixel minus 1.
pub type HSW_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:10 - vertical synchronization height (in units of horizontal scan line) These bits define the vertical Synchronization height minus 1. It represents the number of horizontal synchronization lines.
    #[inline(always)]
    pub fn vsh(&self) -> VSH_R {
        VSH_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bits 16:27 - horizontal synchronization width (in units of pixel clock period) These bits define the number of Horizontal Synchronization pixel minus 1.
    #[inline(always)]
    pub fn hsw(&self) -> HSW_R {
        HSW_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_SSCR")
            .field("vsh", &self.vsh())
            .field("hsw", &self.hsw())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - vertical synchronization height (in units of horizontal scan line) These bits define the vertical Synchronization height minus 1. It represents the number of horizontal synchronization lines.
    #[inline(always)]
    #[must_use]
    pub fn vsh(&mut self) -> VSH_W<LTDC_SSCRrs> {
        VSH_W::new(self, 0)
    }
    ///Bits 16:27 - horizontal synchronization width (in units of pixel clock period) These bits define the number of Horizontal Synchronization pixel minus 1.
    #[inline(always)]
    #[must_use]
    pub fn hsw(&mut self) -> HSW_W<LTDC_SSCRrs> {
        HSW_W::new(self, 16)
    }
}
/**LTDC synchronization size configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_sscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_sscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_SSCR)*/
pub struct LTDC_SSCRrs;
impl crate::RegisterSpec for LTDC_SSCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_sscr::R`](R) reader structure
impl crate::Readable for LTDC_SSCRrs {}
///`write(|w| ..)` method takes [`ltdc_sscr::W`](W) writer structure
impl crate::Writable for LTDC_SSCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_SSCR to value 0
impl crate::Resettable for LTDC_SSCRrs {
    const RESET_VALUE: u32 = 0;
}
