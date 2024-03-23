#[doc = "Register `LVCIDR` reader"]
pub type R = crate::R<LVCIDRrs>;
#[doc = "Register `LVCIDR` writer"]
pub type W = crate::W<LVCIDRrs>;
#[doc = "Field `VCID` reader - VCID"]
pub type VCID_R = crate::FieldReader;
#[doc = "Field `VCID` writer - VCID"]
pub type VCID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    #[must_use]
    pub fn vcid(&mut self) -> VCID_W<LVCIDRrs> {
        VCID_W::new(self, 0)
    }
}
#[doc = "DSI Host LTDC VCID register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvcidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvcidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVCIDRrs;
impl crate::RegisterSpec for LVCIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lvcidr::R`](R) reader structure"]
impl crate::Readable for LVCIDRrs {}
#[doc = "`write(|w| ..)` method takes [`lvcidr::W`](W) writer structure"]
impl crate::Writable for LVCIDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LVCIDR to value 0"]
impl crate::Resettable for LVCIDRrs {
    const RESET_VALUE: u32 = 0;
}
