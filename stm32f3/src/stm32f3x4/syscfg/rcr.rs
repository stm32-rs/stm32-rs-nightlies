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
///Field `PAGE1_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE1_WP_W;
///Field `PAGE2_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE2_WP_W;
///Field `PAGE3_WP` writer - CCM SRAM page write protection bit
pub use PAGE0_WP_W as PAGE3_WP_W;
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
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCR")
            .field("page0_wp", &self.page0_wp())
            .field("page1_wp", &self.page1_wp())
            .field("page2_wp", &self.page2_wp())
            .field("page3_wp", &self.page3_wp())
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
}
/**CCM SRAM protection register

You can [`read`](crate::Reg::read) this register and get [`rcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F3x4.html#SYSCFG:RCR)*/
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
