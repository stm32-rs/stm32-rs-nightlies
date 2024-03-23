#[doc = "Register `MACLETR` reader"]
pub type R = crate::R<MACLETRrs>;
#[doc = "Register `MACLETR` writer"]
pub type W = crate::W<MACLETRrs>;
#[doc = "Field `LPIET` reader - LPI Entry Timer"]
pub type LPIET_R = crate::FieldReader<u32>;
#[doc = "Field `LPIET` writer - LPI Entry Timer"]
pub type LPIET_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 3:19 - LPI Entry Timer"]
    #[inline(always)]
    pub fn lpiet(&self) -> LPIET_R {
        LPIET_R::new((self.bits >> 3) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 3:19 - LPI Entry Timer"]
    #[inline(always)]
    #[must_use]
    pub fn lpiet(&mut self) -> LPIET_W<MACLETRrs> {
        LPIET_W::new(self, 3)
    }
}
#[doc = "LPI entry timer register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`macletr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`macletr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACLETRrs;
impl crate::RegisterSpec for MACLETRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macletr::R`](R) reader structure"]
impl crate::Readable for MACLETRrs {}
#[doc = "`write(|w| ..)` method takes [`macletr::W`](W) writer structure"]
impl crate::Writable for MACLETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACLETR to value 0"]
impl crate::Resettable for MACLETRrs {
    const RESET_VALUE: u32 = 0;
}
