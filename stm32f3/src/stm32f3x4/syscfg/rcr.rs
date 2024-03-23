#[doc = "Register `RCR` reader"]
pub type R = crate::R<RCRrs>;
#[doc = "Register `RCR` writer"]
pub type W = crate::W<RCRrs>;
#[doc = "CCM SRAM page write protection bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAGE0_WP {
    #[doc = "0: Write protection of pagex is disabled"]
    Disabled = 0,
    #[doc = "1: Write protection of pagex is enabled"]
    Enabled = 1,
}
impl From<PAGE0_WP> for bool {
    #[inline(always)]
    fn from(variant: PAGE0_WP) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PAGE0_WP` reader - CCM SRAM page write protection bit"]
pub type PAGE0_WP_R = crate::BitReader<PAGE0_WP>;
impl PAGE0_WP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PAGE0_WP {
        match self.bits {
            false => PAGE0_WP::Disabled,
            true => PAGE0_WP::Enabled,
        }
    }
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAGE0_WP::Disabled
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAGE0_WP::Enabled
    }
}
#[doc = "Field `PAGE0_WP` writer - CCM SRAM page write protection bit"]
pub type PAGE0_WP_W<'a, REG> = crate::BitWriter<'a, REG, PAGE0_WP>;
impl<'a, REG> PAGE0_WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Write protection of pagex is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PAGE0_WP::Disabled)
    }
    #[doc = "Write protection of pagex is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PAGE0_WP::Enabled)
    }
}
#[doc = "Field `PAGE1_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE1_WP_R;
#[doc = "Field `PAGE2_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE2_WP_R;
#[doc = "Field `PAGE3_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE3_WP_R;
#[doc = "Field `PAGE1_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE1_WP_W;
#[doc = "Field `PAGE2_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE2_WP_W;
#[doc = "Field `PAGE3_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE3_WP_W;
impl R {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page0_wp(&self) -> PAGE0_WP_R {
        PAGE0_WP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page1_wp(&self) -> PAGE1_WP_R {
        PAGE1_WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page2_wp(&self) -> PAGE2_WP_R {
        PAGE2_WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page3_wp(&self) -> PAGE3_WP_R {
        PAGE3_WP_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page0_wp(&mut self) -> PAGE0_WP_W<RCRrs> {
        PAGE0_WP_W::new(self, 0)
    }
    #[doc = "Bit 1 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page1_wp(&mut self) -> PAGE1_WP_W<RCRrs> {
        PAGE1_WP_W::new(self, 1)
    }
    #[doc = "Bit 2 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page2_wp(&mut self) -> PAGE2_WP_W<RCRrs> {
        PAGE2_WP_W::new(self, 2)
    }
    #[doc = "Bit 3 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page3_wp(&mut self) -> PAGE3_WP_W<RCRrs> {
        PAGE3_WP_W::new(self, 3)
    }
}
#[doc = "CCM SRAM protection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RCRrs;
impl crate::RegisterSpec for RCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rcr::R`](R) reader structure"]
impl crate::Readable for RCRrs {}
#[doc = "`write(|w| ..)` method takes [`rcr::W`](W) writer structure"]
impl crate::Writable for RCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RCR to value 0"]
impl crate::Resettable for RCRrs {
    const RESET_VALUE: u32 = 0;
}
