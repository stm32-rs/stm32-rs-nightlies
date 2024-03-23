#[doc = "Register `INI1_FN_MOD2` reader"]
pub type R = crate::R<INI1_FN_MOD2rs>;
#[doc = "Register `INI1_FN_MOD2` writer"]
pub type W = crate::W<INI1_FN_MOD2rs>;
#[doc = "Field `BYPASS_MERGE` reader - Disables alteration of transactions by the up-sizer unless required by the protocol"]
pub type BYPASS_MERGE_R = crate::BitReader;
#[doc = "Field `BYPASS_MERGE` writer - Disables alteration of transactions by the up-sizer unless required by the protocol"]
pub type BYPASS_MERGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disables alteration of transactions by the up-sizer unless required by the protocol"]
    #[inline(always)]
    pub fn bypass_merge(&self) -> BYPASS_MERGE_R {
        BYPASS_MERGE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disables alteration of transactions by the up-sizer unless required by the protocol"]
    #[inline(always)]
    #[must_use]
    pub fn bypass_merge(&mut self) -> BYPASS_MERGE_W<INI1_FN_MOD2rs> {
        BYPASS_MERGE_W::new(self, 0)
    }
}
#[doc = "AXI interconnect - INI x functionality modification 2 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ini1_fn_mod2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ini1_fn_mod2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INI1_FN_MOD2rs;
impl crate::RegisterSpec for INI1_FN_MOD2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ini1_fn_mod2::R`](R) reader structure"]
impl crate::Readable for INI1_FN_MOD2rs {}
#[doc = "`write(|w| ..)` method takes [`ini1_fn_mod2::W`](W) writer structure"]
impl crate::Writable for INI1_FN_MOD2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INI1_FN_MOD2 to value 0x04"]
impl crate::Resettable for INI1_FN_MOD2rs {
    const RESET_VALUE: u32 = 0x04;
}
