#[doc = "Register `TRISE` reader"]
pub type R = crate::R<TRISErs>;
#[doc = "Register `TRISE` writer"]
pub type W = crate::W<TRISErs>;
#[doc = "Field `TRISE` reader - Maximum rise time in Fast/Standard mode (Master mode)"]
pub type TRISE_R = crate::FieldReader;
#[doc = "Field `TRISE` writer - Maximum rise time in Fast/Standard mode (Master mode)"]
pub type TRISE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    pub fn trise(&self) -> TRISE_R {
        TRISE_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Maximum rise time in Fast/Standard mode (Master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn trise(&mut self) -> TRISE_W<TRISErs> {
        TRISE_W::new(self, 0)
    }
}
#[doc = "TRISE register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trise::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trise::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRISErs;
impl crate::RegisterSpec for TRISErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trise::R`](R) reader structure"]
impl crate::Readable for TRISErs {}
#[doc = "`write(|w| ..)` method takes [`trise::W`](W) writer structure"]
impl crate::Writable for TRISErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TRISE to value 0x02"]
impl crate::Resettable for TRISErs {
    const RESET_VALUE: u32 = 0x02;
}
