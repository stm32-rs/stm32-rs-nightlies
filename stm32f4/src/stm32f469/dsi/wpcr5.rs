#[doc = "Register `WPCR5` reader"]
pub type R = crate::R<WPCR5rs>;
#[doc = "Register `WPCR5` writer"]
pub type W = crate::W<WPCR5rs>;
#[doc = "Field `THSZERO` reader - tCLK-POST"]
pub type THSZERO_R = crate::FieldReader;
#[doc = "Field `THSZERO` writer - tCLK-POST"]
pub type THSZERO_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    pub fn thszero(&self) -> THSZERO_R {
        THSZERO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - tCLK-POST"]
    #[inline(always)]
    #[must_use]
    pub fn thszero(&mut self) -> THSZERO_W<WPCR5rs> {
        THSZERO_W::new(self, 0)
    }
}
#[doc = "DSI Wrapper PHY Configuration Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wpcr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpcr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WPCR5rs;
impl crate::RegisterSpec for WPCR5rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wpcr5::R`](R) reader structure"]
impl crate::Readable for WPCR5rs {}
#[doc = "`write(|w| ..)` method takes [`wpcr5::W`](W) writer structure"]
impl crate::Writable for WPCR5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WPCR5 to value 0"]
impl crate::Resettable for WPCR5rs {
    const RESET_VALUE: u32 = 0;
}
