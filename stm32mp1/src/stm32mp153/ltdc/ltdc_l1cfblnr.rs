///Register `LTDC_L1CFBLNR` reader
pub type R = crate::R<LTDC_L1CFBLNRrs>;
///Register `LTDC_L1CFBLNR` writer
pub type W = crate::W<LTDC_L1CFBLNRrs>;
///Field `CFBLNBR` reader - CFBLNBR
pub type CFBLNBR_R = crate::FieldReader<u16>;
///Field `CFBLNBR` writer - CFBLNBR
pub type CFBLNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    ///Bits 0:11 - CFBLNBR
    #[inline(always)]
    pub fn cfblnbr(&self) -> CFBLNBR_R {
        CFBLNBR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LTDC_L1CFBLNR")
            .field("cfblnbr", &self.cfblnbr())
            .finish()
    }
}
impl W {
    ///Bits 0:11 - CFBLNBR
    #[inline(always)]
    #[must_use]
    pub fn cfblnbr(&mut self) -> CFBLNBR_W<LTDC_L1CFBLNRrs> {
        CFBLNBR_W::new(self, 0)
    }
}
/**This register defines the number of lines in the color frame buffer.

You can [`read`](crate::Reg::read) this register and get [`ltdc_l1cfblnr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l1cfblnr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:LTDC_L1CFBLNR)*/
pub struct LTDC_L1CFBLNRrs;
impl crate::RegisterSpec for LTDC_L1CFBLNRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_l1cfblnr::R`](R) reader structure
impl crate::Readable for LTDC_L1CFBLNRrs {}
///`write(|w| ..)` method takes [`ltdc_l1cfblnr::W`](W) writer structure
impl crate::Writable for LTDC_L1CFBLNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_L1CFBLNR to value 0
impl crate::Resettable for LTDC_L1CFBLNRrs {
    const RESET_VALUE: u32 = 0;
}
