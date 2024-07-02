///Register `LTDC_L2PFCR` reader
pub type R = crate::R<LTDC_L2PFCRrs>;
///Register `LTDC_L2PFCR` writer
pub type W = crate::W<LTDC_L2PFCRrs>;
///Field `PF` reader - pixel format These bits configure the pixel format
pub type PF_R = crate::FieldReader;
///Field `PF` writer - pixel format These bits configure the pixel format
pub type PF_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    ///Bits 0:2 - pixel format These bits configure the pixel format
    #[inline(always)]
    pub fn pf(&self) -> PF_R {
        PF_R::new((self.bits & 7) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_L2PFCR")
            .field("pf", &self.pf())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - pixel format These bits configure the pixel format
    #[inline(always)]
    #[must_use]
    pub fn pf(&mut self) -> PF_W<LTDC_L2PFCRrs> {
        PF_W::new(self, 0)
    }
}
/**LTDC layer 2 pixel format configuration register

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2pfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2pfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U599.html#LTDC:LTDC_L2PFCR)*/
pub struct LTDC_L2PFCRrs;
impl crate::RegisterSpec for LTDC_L2PFCRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_l2pfcr::R`](R) reader structure
impl crate::Readable for LTDC_L2PFCRrs {}
///`write(|w| ..)` method takes [`ltdc_l2pfcr::W`](W) writer structure
impl crate::Writable for LTDC_L2PFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_L2PFCR to value 0
impl crate::Resettable for LTDC_L2PFCRrs {
    const RESET_VALUE: u32 = 0;
}
