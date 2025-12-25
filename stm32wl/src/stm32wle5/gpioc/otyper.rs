///Register `OTYPER` reader
pub type R = crate::R<OTYPERrs>;
///Register `OTYPER` writer
pub type W = crate::W<OTYPERrs>;
/**Port x configuration bits (y = 0..15)

Value on reset: 0*/
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTPUT_TYPE {
    ///0: Output push-pull (reset state)
    PushPull = 0,
    ///1: Output open-drain
    OpenDrain = 1,
}
impl From<OUTPUT_TYPE> for bool {
    #[inline(always)]
    fn from(variant: OUTPUT_TYPE) -> Self {
        variant as u8 != 0
    }
}
///Field `OT0` reader - Port x configuration bits (y = 0..15)
pub type OT0_R = crate::BitReader<OUTPUT_TYPE>;
impl OT0_R {
    ///Get enumerated values variant
    #[inline(always)]
    pub const fn variant(&self) -> OUTPUT_TYPE {
        match self.bits {
            false => OUTPUT_TYPE::PushPull,
            true => OUTPUT_TYPE::OpenDrain,
        }
    }
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn is_push_pull(&self) -> bool {
        *self == OUTPUT_TYPE::PushPull
    }
    ///Output open-drain
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == OUTPUT_TYPE::OpenDrain
    }
}
///Field `OT0` writer - Port x configuration bits (y = 0..15)
pub type OT0_W<'a, REG> = crate::BitWriter<'a, REG, OUTPUT_TYPE>;
impl<'a, REG> OT0_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    ///Output push-pull (reset state)
    #[inline(always)]
    pub fn push_pull(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_TYPE::PushPull)
    }
    ///Output open-drain
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut crate::W<REG> {
        self.variant(OUTPUT_TYPE::OpenDrain)
    }
}
///Field `OT1` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT1_R;
///Field `OT2` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT2_R;
///Field `OT3` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT3_R;
///Field `OT4` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT4_R;
///Field `OT5` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT5_R;
///Field `OT6` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT6_R;
///Field `OT13` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT13_R;
///Field `OT14` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT14_R;
///Field `OT15` reader - Port x configuration bits (y = 0..15)
pub use OT0_R as OT15_R;
///Field `OT1` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT1_W;
///Field `OT2` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT2_W;
///Field `OT3` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT3_W;
///Field `OT4` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT4_W;
///Field `OT5` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT5_W;
///Field `OT6` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT6_W;
///Field `OT13` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT13_W;
///Field `OT14` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT14_W;
///Field `OT15` writer - Port x configuration bits (y = 0..15)
pub use OT0_W as OT15_W;
impl R {
    ///Bit 0 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot0(&self) -> OT0_R {
        OT0_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot1(&self) -> OT1_R {
        OT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot2(&self) -> OT2_R {
        OT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&self) -> OT3_R {
        OT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot4(&self) -> OT4_R {
        OT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot5(&self) -> OT5_R {
        OT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot6(&self) -> OT6_R {
        OT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot13(&self) -> OT13_R {
        OT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot14(&self) -> OT14_R {
        OT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot15(&self) -> OT15_R {
        OT15_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OTYPER")
            .field("ot0", &self.ot0())
            .field("ot15", &self.ot15())
            .field("ot14", &self.ot14())
            .field("ot13", &self.ot13())
            .field("ot6", &self.ot6())
            .field("ot5", &self.ot5())
            .field("ot4", &self.ot4())
            .field("ot3", &self.ot3())
            .field("ot2", &self.ot2())
            .field("ot1", &self.ot1())
            .finish()
    }
}
impl W {
    ///Bit 0 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot0(&mut self) -> OT0_W<'_, OTYPERrs> {
        OT0_W::new(self, 0)
    }
    ///Bit 1 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot1(&mut self) -> OT1_W<'_, OTYPERrs> {
        OT1_W::new(self, 1)
    }
    ///Bit 2 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot2(&mut self) -> OT2_W<'_, OTYPERrs> {
        OT2_W::new(self, 2)
    }
    ///Bit 3 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot3(&mut self) -> OT3_W<'_, OTYPERrs> {
        OT3_W::new(self, 3)
    }
    ///Bit 4 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot4(&mut self) -> OT4_W<'_, OTYPERrs> {
        OT4_W::new(self, 4)
    }
    ///Bit 5 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot5(&mut self) -> OT5_W<'_, OTYPERrs> {
        OT5_W::new(self, 5)
    }
    ///Bit 6 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot6(&mut self) -> OT6_W<'_, OTYPERrs> {
        OT6_W::new(self, 6)
    }
    ///Bit 13 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot13(&mut self) -> OT13_W<'_, OTYPERrs> {
        OT13_W::new(self, 13)
    }
    ///Bit 14 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot14(&mut self) -> OT14_W<'_, OTYPERrs> {
        OT14_W::new(self, 14)
    }
    ///Bit 15 - Port x configuration bits (y = 0..15)
    #[inline(always)]
    pub fn ot15(&mut self) -> OT15_W<'_, OTYPERrs> {
        OT15_W::new(self, 15)
    }
}
/**GPIO port output type register

You can [`read`](crate::Reg::read) this register and get [`otyper::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otyper::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WLE5.html#GPIOC:OTYPER)*/
pub struct OTYPERrs;
impl crate::RegisterSpec for OTYPERrs {
    type Ux = u32;
}
///`read()` method returns [`otyper::R`](R) reader structure
impl crate::Readable for OTYPERrs {}
///`write(|w| ..)` method takes [`otyper::W`](W) writer structure
impl crate::Writable for OTYPERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets OTYPER to value 0
impl crate::Resettable for OTYPERrs {}
