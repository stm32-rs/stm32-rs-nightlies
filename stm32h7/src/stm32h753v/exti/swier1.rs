///Register `SWIER1` reader
pub type R = crate::R<SWIER1rs>;
///Register `SWIER1` writer
pub type W = crate::W<SWIER1rs>;
/**Rising trigger event configuration bit of Configurable Event input

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFTWARE_INTERRUPT {
    ///1: Generates an interrupt request
    Pend = 1,
}
impl From<SOFTWARE_INTERRUPT> for bool {
    #[inline(always)]
    fn from(variant: SOFTWARE_INTERRUPT) -> Self {
        variant as u8 != 0
    }
}
///Field `SWIER0` reader - Rising trigger event configuration bit of Configurable Event input
pub type SWIER0_R = crate::BitReader<SOFTWARE_INTERRUPT>;
impl SWIER0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> Option<SOFTWARE_INTERRUPT> {
        match self.bits {
            true => Some(SOFTWARE_INTERRUPT::Pend),
            _ => None,
        }
    }
    ///Generates an interrupt request
    #[inline(always)]
    pub fn is_pend(&self) -> bool {
        *self == SOFTWARE_INTERRUPT::Pend
    }
}
///Field `SWIER0` writer - Rising trigger event configuration bit of Configurable Event input
pub type SWIER0_W<'a, REG> = crate::BitWriter<'a, REG, SOFTWARE_INTERRUPT>;
impl<'a, REG> SWIER0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Generates an interrupt request
    #[inline(always)]
    pub fn pend(self) -> &'a mut crate::W<REG> {
        self.variant(SOFTWARE_INTERRUPT::Pend)
    }
}
///Field `SWIER1` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER1_R;
///Field `SWIER2` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER2_R;
///Field `SWIER3` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER3_R;
///Field `SWIER4` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER4_R;
///Field `SWIER5` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER5_R;
///Field `SWIER6` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER6_R;
///Field `SWIER7` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER7_R;
///Field `SWIER8` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER8_R;
///Field `SWIER9` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER9_R;
///Field `SWIER10` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER10_R;
///Field `SWIER11` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER11_R;
///Field `SWIER12` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER12_R;
///Field `SWIER13` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER13_R;
///Field `SWIER14` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER14_R;
///Field `SWIER15` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER15_R;
///Field `SWIER16` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER16_R;
///Field `SWIER17` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER17_R;
///Field `SWIER18` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER18_R;
///Field `SWIER19` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER19_R;
///Field `SWIER20` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER20_R;
///Field `SWIER21` reader - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_R as SWIER21_R;
///Field `SWIER1` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER1_W;
///Field `SWIER2` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER2_W;
///Field `SWIER3` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER3_W;
///Field `SWIER4` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER4_W;
///Field `SWIER5` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER5_W;
///Field `SWIER6` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER6_W;
///Field `SWIER7` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER7_W;
///Field `SWIER8` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER8_W;
///Field `SWIER9` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER9_W;
///Field `SWIER10` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER10_W;
///Field `SWIER11` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER11_W;
///Field `SWIER12` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER12_W;
///Field `SWIER13` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER13_W;
///Field `SWIER14` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER14_W;
///Field `SWIER15` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER15_W;
///Field `SWIER16` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER16_W;
///Field `SWIER17` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER17_W;
///Field `SWIER18` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER18_W;
///Field `SWIER19` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER19_W;
///Field `SWIER20` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER20_W;
///Field `SWIER21` writer - Rising trigger event configuration bit of Configurable Event input
pub use SWIER0_W as SWIER21_W;
impl R {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier0(&self) -> SWIER0_R {
        SWIER0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier1(&self) -> SWIER1_R {
        SWIER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier2(&self) -> SWIER2_R {
        SWIER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier3(&self) -> SWIER3_R {
        SWIER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier4(&self) -> SWIER4_R {
        SWIER4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier5(&self) -> SWIER5_R {
        SWIER5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier6(&self) -> SWIER6_R {
        SWIER6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier7(&self) -> SWIER7_R {
        SWIER7_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier8(&self) -> SWIER8_R {
        SWIER8_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier9(&self) -> SWIER9_R {
        SWIER9_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier10(&self) -> SWIER10_R {
        SWIER10_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier11(&self) -> SWIER11_R {
        SWIER11_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier12(&self) -> SWIER12_R {
        SWIER12_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier13(&self) -> SWIER13_R {
        SWIER13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier14(&self) -> SWIER14_R {
        SWIER14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier15(&self) -> SWIER15_R {
        SWIER15_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier16(&self) -> SWIER16_R {
        SWIER16_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier17(&self) -> SWIER17_R {
        SWIER17_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier18(&self) -> SWIER18_R {
        SWIER18_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier19(&self) -> SWIER19_R {
        SWIER19_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier20(&self) -> SWIER20_R {
        SWIER20_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier21(&self) -> SWIER21_R {
        SWIER21_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWIER1")
            .field("swier0", &self.swier0())
            .field("swier1", &self.swier1())
            .field("swier2", &self.swier2())
            .field("swier3", &self.swier3())
            .field("swier4", &self.swier4())
            .field("swier5", &self.swier5())
            .field("swier6", &self.swier6())
            .field("swier7", &self.swier7())
            .field("swier8", &self.swier8())
            .field("swier9", &self.swier9())
            .field("swier10", &self.swier10())
            .field("swier11", &self.swier11())
            .field("swier12", &self.swier12())
            .field("swier13", &self.swier13())
            .field("swier14", &self.swier14())
            .field("swier15", &self.swier15())
            .field("swier16", &self.swier16())
            .field("swier17", &self.swier17())
            .field("swier18", &self.swier18())
            .field("swier19", &self.swier19())
            .field("swier20", &self.swier20())
            .field("swier21", &self.swier21())
            .finish()
    }
}
impl W {
    ///Bit 0 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier0(&mut self) -> SWIER0_W<'_, SWIER1rs> {
        SWIER0_W::new(self, 0)
    }
    ///Bit 1 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier1(&mut self) -> SWIER1_W<'_, SWIER1rs> {
        SWIER1_W::new(self, 1)
    }
    ///Bit 2 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier2(&mut self) -> SWIER2_W<'_, SWIER1rs> {
        SWIER2_W::new(self, 2)
    }
    ///Bit 3 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier3(&mut self) -> SWIER3_W<'_, SWIER1rs> {
        SWIER3_W::new(self, 3)
    }
    ///Bit 4 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier4(&mut self) -> SWIER4_W<'_, SWIER1rs> {
        SWIER4_W::new(self, 4)
    }
    ///Bit 5 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier5(&mut self) -> SWIER5_W<'_, SWIER1rs> {
        SWIER5_W::new(self, 5)
    }
    ///Bit 6 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier6(&mut self) -> SWIER6_W<'_, SWIER1rs> {
        SWIER6_W::new(self, 6)
    }
    ///Bit 7 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier7(&mut self) -> SWIER7_W<'_, SWIER1rs> {
        SWIER7_W::new(self, 7)
    }
    ///Bit 8 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier8(&mut self) -> SWIER8_W<'_, SWIER1rs> {
        SWIER8_W::new(self, 8)
    }
    ///Bit 9 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier9(&mut self) -> SWIER9_W<'_, SWIER1rs> {
        SWIER9_W::new(self, 9)
    }
    ///Bit 10 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier10(&mut self) -> SWIER10_W<'_, SWIER1rs> {
        SWIER10_W::new(self, 10)
    }
    ///Bit 11 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier11(&mut self) -> SWIER11_W<'_, SWIER1rs> {
        SWIER11_W::new(self, 11)
    }
    ///Bit 12 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier12(&mut self) -> SWIER12_W<'_, SWIER1rs> {
        SWIER12_W::new(self, 12)
    }
    ///Bit 13 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier13(&mut self) -> SWIER13_W<'_, SWIER1rs> {
        SWIER13_W::new(self, 13)
    }
    ///Bit 14 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier14(&mut self) -> SWIER14_W<'_, SWIER1rs> {
        SWIER14_W::new(self, 14)
    }
    ///Bit 15 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier15(&mut self) -> SWIER15_W<'_, SWIER1rs> {
        SWIER15_W::new(self, 15)
    }
    ///Bit 16 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier16(&mut self) -> SWIER16_W<'_, SWIER1rs> {
        SWIER16_W::new(self, 16)
    }
    ///Bit 17 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier17(&mut self) -> SWIER17_W<'_, SWIER1rs> {
        SWIER17_W::new(self, 17)
    }
    ///Bit 18 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier18(&mut self) -> SWIER18_W<'_, SWIER1rs> {
        SWIER18_W::new(self, 18)
    }
    ///Bit 19 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier19(&mut self) -> SWIER19_W<'_, SWIER1rs> {
        SWIER19_W::new(self, 19)
    }
    ///Bit 20 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier20(&mut self) -> SWIER20_W<'_, SWIER1rs> {
        SWIER20_W::new(self, 20)
    }
    ///Bit 21 - Rising trigger event configuration bit of Configurable Event input
    #[inline(always)]
    pub fn swier21(&mut self) -> SWIER21_W<'_, SWIER1rs> {
        SWIER21_W::new(self, 21)
    }
}
/**EXTI software interrupt event register

You can [`read`](crate::Reg::read) this register and get [`swier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H753V.html#EXTI:SWIER1)*/
pub struct SWIER1rs;
impl crate::RegisterSpec for SWIER1rs {
    type Ux = u32;
}
///`read()` method returns [`swier1::R`](R) reader structure
impl crate::Readable for SWIER1rs {}
///`write(|w| ..)` method takes [`swier1::W`](W) writer structure
impl crate::Writable for SWIER1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWIER1 to value 0
impl crate::Resettable for SWIER1rs {}
