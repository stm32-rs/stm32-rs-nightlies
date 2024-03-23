#[doc = "Register `GVCIDR` reader"]
pub type R = crate::R<GVCIDRrs>;
#[doc = "Register `GVCIDR` writer"]
pub type W = crate::W<GVCIDRrs>;
#[doc = "Field `VCID` reader - Virtual Channel ID"]
pub type VCID_R = crate::FieldReader;
#[doc = "Field `VCID` writer - Virtual Channel ID"]
pub type VCID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Virtual Channel ID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Virtual Channel ID"]
    #[inline(always)]
    #[must_use]
    pub fn vcid(&mut self) -> VCID_W<GVCIDRrs> {
        VCID_W::new(self, 0)
    }
}
#[doc = "DSI Host Generic VCID Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gvcidr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gvcidr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GVCIDRrs;
impl crate::RegisterSpec for GVCIDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gvcidr::R`](R) reader structure"]
impl crate::Readable for GVCIDRrs {}
#[doc = "`write(|w| ..)` method takes [`gvcidr::W`](W) writer structure"]
impl crate::Writable for GVCIDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GVCIDR to value 0"]
impl crate::Resettable for GVCIDRrs {
    const RESET_VALUE: u32 = 0;
}
