///Register `EXTICR2` reader
pub type R = crate::R<EXTICR2rs>;
///Register `EXTICR2` writer
pub type W = crate::W<EXTICR2rs>;
///Field `EXTI4` reader - EXTI 4 configuration bits
pub use super::exticr1::EXTI3_R as EXTI4_R;
///Field `EXTI5` reader - EXTI 5 configuration bits
pub use super::exticr1::EXTI3_R as EXTI5_R;
///Field `EXTI6` reader - EXTI 6 configuration bits
pub use super::exticr1::EXTI3_R as EXTI6_R;
///Field `EXTI7` reader - EXTI 7 configuration bits
pub use super::exticr1::EXTI3_R as EXTI7_R;
///Field `EXTI4` writer - EXTI 4 configuration bits
pub use super::exticr1::EXTI3_W as EXTI4_W;
///Field `EXTI5` writer - EXTI 5 configuration bits
pub use super::exticr1::EXTI3_W as EXTI5_W;
///Field `EXTI6` writer - EXTI 6 configuration bits
pub use super::exticr1::EXTI3_W as EXTI6_W;
///Field `EXTI7` writer - EXTI 7 configuration bits
pub use super::exticr1::EXTI3_W as EXTI7_W;
///EXTI 4 configuration bits
pub use super::exticr1::EXTI_ABC;
impl R {
    ///Bits 0:3 - EXTI 4 configuration bits
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI 5 configuration bits
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI 6 configuration bits
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI 7 configuration bits
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR2")
            .field("exti7", &self.exti7())
            .field("exti6", &self.exti6())
            .field("exti5", &self.exti5())
            .field("exti4", &self.exti4())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - EXTI 4 configuration bits
    #[inline(always)]
    pub fn exti4(&mut self) -> EXTI4_W<EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    ///Bits 4:7 - EXTI 5 configuration bits
    #[inline(always)]
    pub fn exti5(&mut self) -> EXTI5_W<EXTICR2rs> {
        EXTI5_W::new(self, 4)
    }
    ///Bits 8:11 - EXTI 6 configuration bits
    #[inline(always)]
    pub fn exti6(&mut self) -> EXTI6_W<EXTICR2rs> {
        EXTI6_W::new(self, 8)
    }
    ///Bits 12:15 - EXTI 7 configuration bits
    #[inline(always)]
    pub fn exti7(&mut self) -> EXTI7_W<EXTICR2rs> {
        EXTI7_W::new(self, 12)
    }
}
/**external interrupt configuration register 2

You can [`read`](crate::Reg::read) this register and get [`exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F301.html#SYSCFG:EXTICR2)*/
pub struct EXTICR2rs;
impl crate::RegisterSpec for EXTICR2rs {
    type Ux = u32;
}
///`read()` method returns [`exticr2::R`](R) reader structure
impl crate::Readable for EXTICR2rs {}
///`write(|w| ..)` method takes [`exticr2::W`](W) writer structure
impl crate::Writable for EXTICR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR2 to value 0
impl crate::Resettable for EXTICR2rs {}
