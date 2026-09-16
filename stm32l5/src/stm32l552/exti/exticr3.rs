///Register `EXTICR3` reader
pub type R = crate::R<EXTICR3rs>;
///Register `EXTICR3` writer
pub type W = crate::W<EXTICR3rs>;
///Field `EXTI8` reader - EXTIm GPIO port selection
pub type EXTI8_R = crate::FieldReader;
///Field `EXTI8` writer - EXTIm GPIO port selection
pub type EXTI8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI9` reader - EXTIm+1 GPIO port selection
pub type EXTI9_R = crate::FieldReader;
///Field `EXTI9` writer - EXTIm+1 GPIO port selection
pub type EXTI9_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI10` reader - EXTIm+2 GPIO port selection
pub type EXTI10_R = crate::FieldReader;
///Field `EXTI10` writer - EXTIm+2 GPIO port selection
pub type EXTI10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI11` reader - EXTIm+3 GPIO port selection
pub type EXTI11_R = crate::FieldReader;
///Field `EXTI11` writer - EXTIm+3 GPIO port selection
pub type EXTI11_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti8(&self) -> EXTI8_R {
        EXTI8_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti9(&self) -> EXTI9_R {
        EXTI9_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti10(&self) -> EXTI10_R {
        EXTI10_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti11(&self) -> EXTI11_R {
        EXTI11_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTICR3")
            .field("exti8", &self.exti8())
            .field("exti9", &self.exti9())
            .field("exti10", &self.exti10())
            .field("exti11", &self.exti11())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti8(&mut self) -> EXTI8_W<'_, EXTICR3rs> {
        EXTI8_W::new(self, 0)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti9(&mut self) -> EXTI9_W<'_, EXTICR3rs> {
        EXTI9_W::new(self, 8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti10(&mut self) -> EXTI10_W<'_, EXTICR3rs> {
        EXTI10_W::new(self, 16)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti11(&mut self) -> EXTI11_W<'_, EXTICR3rs> {
        EXTI11_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exticr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L552.html#EXTI:EXTICR3)*/
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
