///Register `EXTICR4` reader
pub type R = crate::R<EXTICR4rs>;
///Register `EXTICR4` writer
pub type W = crate::W<EXTICR4rs>;
///EXTI 12 configuration bits
pub use super::exticr1::EXTI0;
///Field `EXTI12` reader - EXTI 12 configuration bits
pub use super::exticr1::EXTI0_R as EXTI12_R;
///Field `EXTI13` reader - EXTI 13 configuration bits
pub use super::exticr1::EXTI0_R as EXTI13_R;
///Field `EXTI14` reader - EXTI 14 configuration bits
pub use super::exticr1::EXTI0_R as EXTI14_R;
///Field `EXTI15` reader - EXTI 15 configuration bits
pub use super::exticr1::EXTI0_R as EXTI15_R;
///Field `EXTI12` writer - EXTI 12 configuration bits
pub use super::exticr1::EXTI0_W as EXTI12_W;
///Field `EXTI13` writer - EXTI 13 configuration bits
pub use super::exticr1::EXTI0_W as EXTI13_W;
///Field `EXTI14` writer - EXTI 14 configuration bits
pub use super::exticr1::EXTI0_W as EXTI14_W;
///Field `EXTI15` writer - EXTI 15 configuration bits
pub use super::exticr1::EXTI0_W as EXTI15_W;
impl R {
    ///Bits 0:3 - EXTI 12 configuration bits
    #[inline(always)]
    pub fn exti12(&self) -> EXTI12_R {
        EXTI12_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI 13 configuration bits
    #[inline(always)]
    pub fn exti13(&self) -> EXTI13_R {
        EXTI13_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI 14 configuration bits
    #[inline(always)]
    pub fn exti14(&self) -> EXTI14_R {
        EXTI14_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI 15 configuration bits
    #[inline(always)]
    pub fn exti15(&self) -> EXTI15_R {
        EXTI15_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR4")
            .field("exti15", &self.exti15())
            .field("exti14", &self.exti14())
            .field("exti13", &self.exti13())
            .field("exti12", &self.exti12())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - EXTI 12 configuration bits
    #[inline(always)]
    pub fn exti12(&mut self) -> EXTI12_W<EXTICR4rs> {
        EXTI12_W::new(self, 0)
    }
    ///Bits 4:7 - EXTI 13 configuration bits
    #[inline(always)]
    pub fn exti13(&mut self) -> EXTI13_W<EXTICR4rs> {
        EXTI13_W::new(self, 4)
    }
    ///Bits 8:11 - EXTI 14 configuration bits
    #[inline(always)]
    pub fn exti14(&mut self) -> EXTI14_W<EXTICR4rs> {
        EXTI14_W::new(self, 8)
    }
    ///Bits 12:15 - EXTI 15 configuration bits
    #[inline(always)]
    pub fn exti15(&mut self) -> EXTI15_W<EXTICR4rs> {
        EXTI15_W::new(self, 12)
    }
}
/**external interrupt configuration register 4

You can [`read`](crate::Reg::read) this register and get [`exticr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F0x1.html#SYSCFG:EXTICR4)*/
pub struct EXTICR4rs;
impl crate::RegisterSpec for EXTICR4rs {
    type Ux = u32;
}
///`read()` method returns [`exticr4::R`](R) reader structure
impl crate::Readable for EXTICR4rs {}
///`write(|w| ..)` method takes [`exticr4::W`](W) writer structure
impl crate::Writable for EXTICR4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR4 to value 0
impl crate::Resettable for EXTICR4rs {}
