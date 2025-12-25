///Register `RCR` reader
pub type R = crate::R<RCRrs>;
///Register `RCR` writer
pub type W = crate::W<RCRrs>;
/**CCM SRAM page write protection bit

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PAGE0_WP {
    ///0: Write protection of pagex is disabled
    Disabled = 0,
    ///1: Write protection of pagex is enabled
    Enabled = 1,
}
impl From<PAGE0_WP> for bool {
    #[inline(always)]
    fn from(variant: PAGE0_WP) -> Self {
        variant as u8 != 0
    }
}
///Field `PAGE0_WP` reader - CCM SRAM page write protection bit
pub type PAGE0_WP_R = crate::BitReader<PAGE0_WP>;
impl PAGE0_WP_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> PAGE0_WP {
        match self.bits {
            false => PAGE0_WP::Disabled,
            true => PAGE0_WP::Enabled,
        }
    }
    ///Write protection of pagex is disabled
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PAGE0_WP::Disabled
    }
    ///Write protection of pagex is enabled
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PAGE0_WP::Enabled
    }
}
///Field `PAGE0_WP` writer - CCM SRAM page write protection bit
pub type PAGE0_WP_W<'a, REG> = crate::BitWriter<'a, REG, PAGE0_WP>;
impl<'a, REG> PAGE0_WP_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Write protection of pagex is disabled
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PAGE0_WP::Disabled)
    }
    ///Write protection of pagex is enabled
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(PAGE0_WP::Enabled)
    }
}
///Field `PAGE1_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE1_WP_R;
///Field `PAGE2_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE2_WP_R;
///Field `PAGE3_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE3_WP_R;
///Field `PAGE4_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE4_WP_R;
///Field `PAGE5_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE5_WP_R;
///Field `PAGE6_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE6_WP_R;
///Field `PAGE7_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE7_WP_R;
///Field `PAGE8_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE8_WP_R;
///Field `PAGE9_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE9_WP_R;
///Field `PAGE10_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE10_WP_R;
///Field `PAGE11_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE11_WP_R;
///Field `PAGE12_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE12_WP_R;
///Field `PAGE13_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE13_WP_R;
///Field `PAGE14_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE14_WP_R;
///Field `PAGE15_WP` reader - CCM SRAM page write protection bit
pub use PAGE0_WP_R as PAGE15_WP_R;
///Field `PAGE1_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE1_WP_W;
///Field `PAGE2_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE2_WP_W;
///Field `PAGE3_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE3_WP_W;
///Field `PAGE4_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE4_WP_W;
///Field `PAGE5_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE5_WP_W;
///Field `PAGE6_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE6_WP_W;
///Field `PAGE7_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE7_WP_W;
///Field `PAGE8_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE8_WP_W;
///Field `PAGE9_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE9_WP_W;
///Field `PAGE10_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE10_WP_W;
///Field `PAGE11_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE11_WP_W;
///Field `PAGE12_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE12_WP_W;
///Field `PAGE13_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE13_WP_W;
///Field `PAGE14_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE14_WP_W;
///Field `PAGE15_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE15_WP_W;
impl R {
    ///Bit 0 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page0_wp(&self) -> PAGE0_WP_R {
        PAGE0_WP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page1_wp(&self) -> PAGE1_WP_R {
        PAGE1_WP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page2_wp(&self) -> PAGE2_WP_R {
        PAGE2_WP_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page3_wp(&self) -> PAGE3_WP_R {
        PAGE3_WP_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page4_wp(&self) -> PAGE4_WP_R {
        PAGE4_WP_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page5_wp(&self) -> PAGE5_WP_R {
        PAGE5_WP_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page6_wp(&self) -> PAGE6_WP_R {
        PAGE6_WP_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page7_wp(&self) -> PAGE7_WP_R {
        PAGE7_WP_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page8_wp(&self) -> PAGE8_WP_R {
        PAGE8_WP_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page9_wp(&self) -> PAGE9_WP_R {
        PAGE9_WP_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page10_wp(&self) -> PAGE10_WP_R {
        PAGE10_WP_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page11_wp(&self) -> PAGE11_WP_R {
        PAGE11_WP_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page12_wp(&self) -> PAGE12_WP_R {
        PAGE12_WP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page13_wp(&self) -> PAGE13_WP_R {
        PAGE13_WP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page14_wp(&self) -> PAGE14_WP_R {
        PAGE14_WP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page15_wp(&self) -> PAGE15_WP_R {
        PAGE15_WP_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR")
            .field("page0_wp", &self.page0_wp())
            .field("page1_wp", &self.page1_wp())
            .field("page2_wp", &self.page2_wp())
            .field("page3_wp", &self.page3_wp())
            .field("page4_wp", &self.page4_wp())
            .field("page5_wp", &self.page5_wp())
            .field("page6_wp", &self.page6_wp())
            .field("page7_wp", &self.page7_wp())
            .field("page8_wp", &self.page8_wp())
            .field("page9_wp", &self.page9_wp())
            .field("page10_wp", &self.page10_wp())
            .field("page11_wp", &self.page11_wp())
            .field("page12_wp", &self.page12_wp())
            .field("page13_wp", &self.page13_wp())
            .field("page14_wp", &self.page14_wp())
            .field("page15_wp", &self.page15_wp())
            .finish()
    }
}
impl W {
    ///Bit 0 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page0_wp(&mut self) -> PAGE0_WP_W<'_, RCRrs> {
        PAGE0_WP_W::new(self, 0)
    }
    ///Bit 1 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page1_wp(&mut self) -> PAGE1_WP_W<'_, RCRrs> {
        PAGE1_WP_W::new(self, 1)
    }
    ///Bit 2 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page2_wp(&mut self) -> PAGE2_WP_W<'_, RCRrs> {
        PAGE2_WP_W::new(self, 2)
    }
    ///Bit 3 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page3_wp(&mut self) -> PAGE3_WP_W<'_, RCRrs> {
        PAGE3_WP_W::new(self, 3)
    }
    ///Bit 4 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page4_wp(&mut self) -> PAGE4_WP_W<'_, RCRrs> {
        PAGE4_WP_W::new(self, 4)
    }
    ///Bit 5 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page5_wp(&mut self) -> PAGE5_WP_W<'_, RCRrs> {
        PAGE5_WP_W::new(self, 5)
    }
    ///Bit 6 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page6_wp(&mut self) -> PAGE6_WP_W<'_, RCRrs> {
        PAGE6_WP_W::new(self, 6)
    }
    ///Bit 7 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page7_wp(&mut self) -> PAGE7_WP_W<'_, RCRrs> {
        PAGE7_WP_W::new(self, 7)
    }
    ///Bit 8 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page8_wp(&mut self) -> PAGE8_WP_W<'_, RCRrs> {
        PAGE8_WP_W::new(self, 8)
    }
    ///Bit 9 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page9_wp(&mut self) -> PAGE9_WP_W<'_, RCRrs> {
        PAGE9_WP_W::new(self, 9)
    }
    ///Bit 10 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page10_wp(&mut self) -> PAGE10_WP_W<'_, RCRrs> {
        PAGE10_WP_W::new(self, 10)
    }
    ///Bit 11 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page11_wp(&mut self) -> PAGE11_WP_W<'_, RCRrs> {
        PAGE11_WP_W::new(self, 11)
    }
    ///Bit 12 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page12_wp(&mut self) -> PAGE12_WP_W<'_, RCRrs> {
        PAGE12_WP_W::new(self, 12)
    }
    ///Bit 13 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page13_wp(&mut self) -> PAGE13_WP_W<'_, RCRrs> {
        PAGE13_WP_W::new(self, 13)
    }
    ///Bit 14 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page14_wp(&mut self) -> PAGE14_WP_W<'_, RCRrs> {
        PAGE14_WP_W::new(self, 14)
    }
    ///Bit 15 - CCM SRAM page write protection bit
    #[inline(always)]
    pub fn page15_wp(&mut self) -> PAGE15_WP_W<'_, RCRrs> {
        PAGE15_WP_W::new(self, 15)
    }
}
/**CCM SRAM protection register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F303.html#SYSCFG:RCR)*/
pub struct RCRrs;
impl crate::RegisterSpec for RCRrs {
    type Ux = u32;
}
///`read()` method returns [`rcr::R`](R) reader structure
impl crate::Readable for RCRrs {}
///`write(|w| ..)` method takes [`rcr::W`](W) writer structure
impl crate::Writable for RCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets RCR to value 0
impl crate::Resettable for RCRrs {}
