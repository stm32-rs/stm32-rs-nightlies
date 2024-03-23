#[doc = "Register `CONFCHR2` reader"]
pub type R = crate::R<CONFCHR2rs>;
#[doc = "Register `CONFCHR2` writer"]
pub type W = crate::W<CONFCHR2rs>;
#[doc = "Field `CONFCH8` reader - Channel 8 configuration"]
pub type CONFCH8_R = crate::FieldReader;
#[doc = "Field `CONFCH8` writer - Channel 8 configuration"]
pub type CONFCH8_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Channel 8 configuration"]
    #[inline(always)]
    pub fn confch8(&self) -> CONFCH8_R {
        CONFCH8_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Channel 8 configuration"]
    #[inline(always)]
    #[must_use]
    pub fn confch8(&mut self) -> CONFCH8_W<CONFCHR2rs> {
        CONFCH8_W::new(self, 0)
    }
}
#[doc = "channel configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`confchr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`confchr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFCHR2rs;
impl crate::RegisterSpec for CONFCHR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confchr2::R`](R) reader structure"]
impl crate::Readable for CONFCHR2rs {}
#[doc = "`write(|w| ..)` method takes [`confchr2::W`](W) writer structure"]
impl crate::Writable for CONFCHR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFCHR2 to value 0"]
impl crate::Resettable for CONFCHR2rs {
    const RESET_VALUE: u32 = 0;
}
