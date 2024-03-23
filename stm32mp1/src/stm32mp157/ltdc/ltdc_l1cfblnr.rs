#[doc = "Register `LTDC_L1CFBLNR` reader"]
pub type R = crate::R<LTDC_L1CFBLNRrs>;
#[doc = "Register `LTDC_L1CFBLNR` writer"]
pub type W = crate::W<LTDC_L1CFBLNRrs>;
#[doc = "Field `CFBLNBR` reader - CFBLNBR"]
pub type CFBLNBR_R = crate::FieldReader<u16>;
#[doc = "Field `CFBLNBR` writer - CFBLNBR"]
pub type CFBLNBR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - CFBLNBR"]
    #[inline(always)]
    pub fn cfblnbr(&self) -> CFBLNBR_R {
        CFBLNBR_R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - CFBLNBR"]
    #[inline(always)]
    #[must_use]
    pub fn cfblnbr(&mut self) -> CFBLNBR_W<LTDC_L1CFBLNRrs> {
        CFBLNBR_W::new(self, 0)
    }
}
#[doc = "This register defines the number of lines in the color frame buffer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l1cfblnr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l1cfblnr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L1CFBLNRrs;
impl crate::RegisterSpec for LTDC_L1CFBLNRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l1cfblnr::R`](R) reader structure"]
impl crate::Readable for LTDC_L1CFBLNRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l1cfblnr::W`](W) writer structure"]
impl crate::Writable for LTDC_L1CFBLNRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L1CFBLNR to value 0"]
impl crate::Resettable for LTDC_L1CFBLNRrs {
    const RESET_VALUE: u32 = 0;
}
