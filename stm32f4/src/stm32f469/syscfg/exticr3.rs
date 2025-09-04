///Register `EXTICR3` reader
pub type R = crate::R<EXTICR3rs>;
///Register `EXTICR3` writer
pub type W = crate::W<EXTICR3rs>;
///EXTI x configuration (x = 8 to 11)
pub use super::exticr1::EXTI0;
///Field `EXTI8` reader - EXTI x configuration (x = 8 to 11)
pub use super::exticr1::EXTI0_R as EXTI8_R;
///Field `EXTI9` reader - EXTI x configuration (x = 8 to 11)
pub use super::exticr1::EXTI0_R as EXTI9_R;
///Field `EXTI10` reader - EXTI10
pub use super::exticr1::EXTI0_R as EXTI10_R;
///Field `EXTI11` reader - EXTI x configuration (x = 8 to 11)
pub use super::exticr1::EXTI0_R as EXTI11_R;
///Field `EXTI8` writer - EXTI x configuration (x = 8 to 11)
pub use super::exticr1::EXTI0_W as EXTI8_W;
///Field `EXTI9` writer - EXTI x configuration (x = 8 to 11)
pub use super::exticr1::EXTI0_W as EXTI9_W;
///Field `EXTI10` writer - EXTI10
pub use super::exticr1::EXTI0_W as EXTI10_W;
///Field `EXTI11` writer - EXTI x configuration (x = 8 to 11)
pub use super::exticr1::EXTI0_W as EXTI11_W;
impl R {
    ///Bits 0:3 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - EXTI10
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR3")
            .field("exti11", &self.exti11())
            .field("exti10", &self.exti10())
            .field("exti9", &self.exti9())
            .field("exti8", &self.exti8())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W<EXTICR3rs> {
        EXTI8_W::new(self, 0)
    }
    ///Bits 4:7 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W<EXTICR3rs> {
        EXTI9_W::new(self, 4)
    }
    ///Bits 8:11 - EXTI10
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W<EXTICR3rs> {
        EXTI10_W::new(self, 8)
    }
    ///Bits 12:15 - EXTI x configuration (x = 8 to 11)
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W<EXTICR3rs> {
        EXTI11_W::new(self, 12)
    }
}
/**external interrupt configuration register 3

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#SYSCFG:EXTICR3)*/
pub struct EXTICR3rs;
impl crate::RegisterSpec for EXTICR3rs {
    type Ux = u32;
}
///`read()` method returns [`exticr3::R`](R) reader structure
impl crate::Readable for EXTICR3rs {}
///`write(|w| ..)` method takes [`exticr3::W`](W) writer structure
impl crate::Writable for EXTICR3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EXTICR3 to value 0
impl crate::Resettable for EXTICR3rs {}
