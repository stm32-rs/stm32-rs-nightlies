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
#[doc = "Field `PAGE4_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE4_WP_R;
#[doc = "Field `PAGE5_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE5_WP_R;
#[doc = "Field `PAGE6_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE6_WP_R;
#[doc = "Field `PAGE7_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE7_WP_R;
#[doc = "Field `PAGE8_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE8_WP_R;
#[doc = "Field `PAGE9_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE9_WP_R;
#[doc = "Field `PAGE10_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE10_WP_R;
#[doc = "Field `PAGE11_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE11_WP_R;
#[doc = "Field `PAGE12_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE12_WP_R;
#[doc = "Field `PAGE13_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE13_WP_R;
#[doc = "Field `PAGE14_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE14_WP_R;
#[doc = "Field `PAGE15_WP` reader - CCM SRAM page write protection bit"]
pub use PAGE0_WP_R as PAGE15_WP_R;
#[doc = "Field `PAGE1_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE1_WP_W;
#[doc = "Field `PAGE2_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE2_WP_W;
#[doc = "Field `PAGE3_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE3_WP_W;
#[doc = "Field `PAGE4_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE4_WP_W;
#[doc = "Field `PAGE5_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE5_WP_W;
#[doc = "Field `PAGE6_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE6_WP_W;
#[doc = "Field `PAGE7_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE7_WP_W;
#[doc = "Field `PAGE8_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE8_WP_W;
#[doc = "Field `PAGE9_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE9_WP_W;
#[doc = "Field `PAGE10_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE10_WP_W;
#[doc = "Field `PAGE11_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE11_WP_W;
#[doc = "Field `PAGE12_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE12_WP_W;
#[doc = "Field `PAGE13_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE13_WP_W;
#[doc = "Field `PAGE14_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE14_WP_W;
#[doc = "Field `PAGE15_WP` writer - CCM SRAM page write protection bit"]
pub use PAGE0_WP_W as PAGE15_WP_W;
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
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page4_wp(&self) -> PAGE4_WP_R {
        PAGE4_WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page5_wp(&self) -> PAGE5_WP_R {
        PAGE5_WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page6_wp(&self) -> PAGE6_WP_R {
        PAGE6_WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page7_wp(&self) -> PAGE7_WP_R {
        PAGE7_WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page8_wp(&self) -> PAGE8_WP_R {
        PAGE8_WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page9_wp(&self) -> PAGE9_WP_R {
        PAGE9_WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page10_wp(&self) -> PAGE10_WP_R {
        PAGE10_WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page11_wp(&self) -> PAGE11_WP_R {
        PAGE11_WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page12_wp(&self) -> PAGE12_WP_R {
        PAGE12_WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page13_wp(&self) -> PAGE13_WP_R {
        PAGE13_WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page14_wp(&self) -> PAGE14_WP_R {
        PAGE14_WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CCM SRAM page write protection bit"]
    #[inline(always)]
    pub fn page15_wp(&self) -> PAGE15_WP_R {
        PAGE15_WP_R::new(((self.bits >> 15) & 1) != 0)
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
    #[doc = "Bit 4 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page4_wp(&mut self) -> PAGE4_WP_W<RCRrs> {
        PAGE4_WP_W::new(self, 4)
    }
    #[doc = "Bit 5 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page5_wp(&mut self) -> PAGE5_WP_W<RCRrs> {
        PAGE5_WP_W::new(self, 5)
    }
    #[doc = "Bit 6 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page6_wp(&mut self) -> PAGE6_WP_W<RCRrs> {
        PAGE6_WP_W::new(self, 6)
    }
    #[doc = "Bit 7 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page7_wp(&mut self) -> PAGE7_WP_W<RCRrs> {
        PAGE7_WP_W::new(self, 7)
    }
    #[doc = "Bit 8 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page8_wp(&mut self) -> PAGE8_WP_W<RCRrs> {
        PAGE8_WP_W::new(self, 8)
    }
    #[doc = "Bit 9 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page9_wp(&mut self) -> PAGE9_WP_W<RCRrs> {
        PAGE9_WP_W::new(self, 9)
    }
    #[doc = "Bit 10 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page10_wp(&mut self) -> PAGE10_WP_W<RCRrs> {
        PAGE10_WP_W::new(self, 10)
    }
    #[doc = "Bit 11 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page11_wp(&mut self) -> PAGE11_WP_W<RCRrs> {
        PAGE11_WP_W::new(self, 11)
    }
    #[doc = "Bit 12 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page12_wp(&mut self) -> PAGE12_WP_W<RCRrs> {
        PAGE12_WP_W::new(self, 12)
    }
    #[doc = "Bit 13 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page13_wp(&mut self) -> PAGE13_WP_W<RCRrs> {
        PAGE13_WP_W::new(self, 13)
    }
    #[doc = "Bit 14 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page14_wp(&mut self) -> PAGE14_WP_W<RCRrs> {
        PAGE14_WP_W::new(self, 14)
    }
    #[doc = "Bit 15 - CCM SRAM page write protection bit"]
    #[inline(always)]
    #[must_use]
    pub fn page15_wp(&mut self) -> PAGE15_WP_W<RCRrs> {
        PAGE15_WP_W::new(self, 15)
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
