#[doc = "Register `CWSTRT` reader"]
pub type R = crate::R<CWSTRTrs>;
#[doc = "Register `CWSTRT` writer"]
pub type W = crate::W<CWSTRTrs>;
#[doc = "Field `HOFFCNT` reader - Horizontal offset count This value gives the number of pixel clocks to count before starting a capture."]
pub type HOFFCNT_R = crate::FieldReader<u16>;
#[doc = "Field `HOFFCNT` writer - Horizontal offset count This value gives the number of pixel clocks to count before starting a capture."]
pub type HOFFCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `VST` reader - Vertical start line count The image capture starts with this line number. Previous line data are ignored. ...."]
pub type VST_R = crate::FieldReader<u16>;
#[doc = "Field `VST` writer - Vertical start line count The image capture starts with this line number. Previous line data are ignored. ...."]
pub type VST_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:13 - Horizontal offset count This value gives the number of pixel clocks to count before starting a capture."]
    #[inline(always)]
    pub fn hoffcnt(&self) -> HOFFCNT_R {
        HOFFCNT_R::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bits 16:28 - Vertical start line count The image capture starts with this line number. Previous line data are ignored. ...."]
    #[inline(always)]
    pub fn vst(&self) -> VST_R {
        VST_R::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Horizontal offset count This value gives the number of pixel clocks to count before starting a capture."]
    #[inline(always)]
    #[must_use]
    pub fn hoffcnt(&mut self) -> HOFFCNT_W<CWSTRTrs> {
        HOFFCNT_W::new(self, 0)
    }
    #[doc = "Bits 16:28 - Vertical start line count The image capture starts with this line number. Previous line data are ignored. ...."]
    #[inline(always)]
    #[must_use]
    pub fn vst(&mut self) -> VST_W<CWSTRTrs> {
        VST_W::new(self, 16)
    }
}
#[doc = "DCMI crop window start\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cwstrt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cwstrt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CWSTRTrs;
impl crate::RegisterSpec for CWSTRTrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwstrt::R`](R) reader structure"]
impl crate::Readable for CWSTRTrs {}
#[doc = "`write(|w| ..)` method takes [`cwstrt::W`](W) writer structure"]
impl crate::Writable for CWSTRTrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CWSTRT to value 0"]
impl crate::Resettable for CWSTRTrs {
    const RESET_VALUE: u32 = 0;
}
