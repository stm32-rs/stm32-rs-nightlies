#[doc = "Register `LUTH` reader"]
pub type R = crate::R<LUTHrs>;
#[doc = "Register `LUTH` writer"]
pub type W = crate::W<LUTHrs>;
#[doc = "Field `LO` reader - Line offset"]
pub type LO_R = crate::FieldReader<u32>;
#[doc = "Field `LO` writer - Line offset"]
pub type LO_W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 4:21 - Line offset"]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new((self.bits >> 4) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 4:21 - Line offset"]
    #[inline(always)]
    #[must_use]
    pub fn lo(&mut self) -> LO_W<LUTHrs> {
        LO_W::new(self, 4)
    }
}
#[doc = "Graphic MMU LUT entry x high\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`luth::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`luth::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LUTHrs;
impl crate::RegisterSpec for LUTHrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`luth::R`](R) reader structure"]
impl crate::Readable for LUTHrs {}
#[doc = "`write(|w| ..)` method takes [`luth::W`](W) writer structure"]
impl crate::Writable for LUTHrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LUTH to value 0"]
impl crate::Resettable for LUTHrs {
    const RESET_VALUE: u32 = 0;
}
