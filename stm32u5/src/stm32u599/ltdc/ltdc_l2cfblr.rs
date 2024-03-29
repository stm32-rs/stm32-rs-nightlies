#[doc = "Register `LTDC_L2CFBLR` reader"]
pub type R = crate::R<LTDC_L2CFBLRrs>;
#[doc = "Register `LTDC_L2CFBLR` writer"]
pub type W = crate::W<LTDC_L2CFBLRrs>;
#[doc = "Field `CFBLL` reader - color frame buffer line length These bits define the length of one line of pixels in bytes + 3. The line length is computed as follows: active high width * number of bytes per pixel + 3."]
pub type CFBLL_R = crate::FieldReader<u16>;
#[doc = "Field `CFBLL` writer - color frame buffer line length These bits define the length of one line of pixels in bytes + 3. The line length is computed as follows: active high width * number of bytes per pixel + 3."]
pub type CFBLL_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `CFBP` reader - color frame buffer pitch in bytes These bits define the pitch that is the increment from the start of one line of pixels to the start of the next line in bytes."]
pub type CFBP_R = crate::FieldReader<u16>;
#[doc = "Field `CFBP` writer - color frame buffer pitch in bytes These bits define the pitch that is the increment from the start of one line of pixels to the start of the next line in bytes."]
pub type CFBP_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - color frame buffer line length These bits define the length of one line of pixels in bytes + 3. The line length is computed as follows: active high width * number of bytes per pixel + 3."]
    #[inline(always)]
    pub fn cfbll(&self) -> CFBLL_R {
        CFBLL_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - color frame buffer pitch in bytes These bits define the pitch that is the increment from the start of one line of pixels to the start of the next line in bytes."]
    #[inline(always)]
    pub fn cfbp(&self) -> CFBP_R {
        CFBP_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - color frame buffer line length These bits define the length of one line of pixels in bytes + 3. The line length is computed as follows: active high width * number of bytes per pixel + 3."]
    #[inline(always)]
    #[must_use]
    pub fn cfbll(&mut self) -> CFBLL_W<LTDC_L2CFBLRrs> {
        CFBLL_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - color frame buffer pitch in bytes These bits define the pitch that is the increment from the start of one line of pixels to the start of the next line in bytes."]
    #[inline(always)]
    #[must_use]
    pub fn cfbp(&mut self) -> CFBP_W<LTDC_L2CFBLRrs> {
        CFBP_W::new(self, 16)
    }
}
#[doc = "LTDC layer 2 color frame buffer length register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ltdc_l2cfblr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ltdc_l2cfblr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LTDC_L2CFBLRrs;
impl crate::RegisterSpec for LTDC_L2CFBLRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltdc_l2cfblr::R`](R) reader structure"]
impl crate::Readable for LTDC_L2CFBLRrs {}
#[doc = "`write(|w| ..)` method takes [`ltdc_l2cfblr::W`](W) writer structure"]
impl crate::Writable for LTDC_L2CFBLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LTDC_L2CFBLR to value 0"]
impl crate::Resettable for LTDC_L2CFBLRrs {
    const RESET_VALUE: u32 = 0;
}
