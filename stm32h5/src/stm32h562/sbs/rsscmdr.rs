#[doc = "Register `RSSCMDR` reader"]
pub type R = crate::R<RSSCMDRrs>;
#[doc = "Register `RSSCMDR` writer"]
pub type W = crate::W<RSSCMDRrs>;
#[doc = "Field `RSSCMD` reader - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value."]
pub type RSSCMD_R = crate::FieldReader<u16>;
#[doc = "Field `RSSCMD` writer - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value."]
pub type RSSCMD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value."]
    #[inline(always)]
    pub fn rsscmd(&self) -> RSSCMD_R {
        RSSCMD_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RSS command The application can use this bitfield to pass on a command to the RSS, executed at the next reset. When RSSCMD ≠ 0 and PRODUCT_STATE is in Open, then the system always boots on RSS whatever is the boot pin value."]
    #[inline(always)]
    #[must_use]
    pub fn rsscmd(&mut self) -> RSSCMD_W<RSSCMDRrs> {
        RSSCMD_W::new(self, 0)
    }
}
#[doc = "SBS RSS command register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsscmdr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsscmdr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSSCMDRrs;
impl crate::RegisterSpec for RSSCMDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsscmdr::R`](R) reader structure"]
impl crate::Readable for RSSCMDRrs {}
#[doc = "`write(|w| ..)` method takes [`rsscmdr::W`](W) writer structure"]
impl crate::Writable for RSSCMDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSSCMDR to value 0"]
impl crate::Resettable for RSSCMDRrs {
    const RESET_VALUE: u32 = 0;
}
