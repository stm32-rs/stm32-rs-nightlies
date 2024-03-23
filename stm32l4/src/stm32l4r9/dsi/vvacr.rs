#[doc = "Register `VVACR` reader"]
pub type R = crate::R<VVACRrs>;
#[doc = "Register `VVACR` writer"]
pub type W = crate::W<VVACRrs>;
#[doc = "Field `VA` reader - Vertical Active duration"]
pub type VA_R = crate::FieldReader<u16>;
#[doc = "Field `VA` writer - Vertical Active duration"]
pub type VA_W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - Vertical Active duration"]
    #[inline(always)]
    pub fn va(&self) -> VA_R {
        VA_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Vertical Active duration"]
    #[inline(always)]
    #[must_use]
    pub fn va(&mut self) -> VA_W<VVACRrs> {
        VA_W::new(self, 0)
    }
}
#[doc = "DSI Host Video VA Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vvacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vvacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VVACRrs;
impl crate::RegisterSpec for VVACRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vvacr::R`](R) reader structure"]
impl crate::Readable for VVACRrs {}
#[doc = "`write(|w| ..)` method takes [`vvacr::W`](W) writer structure"]
impl crate::Writable for VVACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VVACR to value 0"]
impl crate::Resettable for VVACRrs {
    const RESET_VALUE: u32 = 0;
}
