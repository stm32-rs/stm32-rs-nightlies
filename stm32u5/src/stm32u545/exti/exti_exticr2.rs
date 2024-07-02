///Register `EXTI_EXTICR2` reader
pub type R = crate::R<EXTI_EXTICR2rs>;
///Register `EXTI_EXTICR2` writer
pub type W = crate::W<EXTI_EXTICR2rs>;
///Field `EXTI4` reader - EXTIm GPIO port selection
pub type EXTI4_R = crate::FieldReader;
///Field `EXTI4` writer - EXTIm GPIO port selection
pub type EXTI4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI5` reader - EXTIm+1 GPIO port selection
pub type EXTI5_R = crate::FieldReader;
///Field `EXTI5` writer - EXTIm+1 GPIO port selection
pub type EXTI5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI6` reader - EXTIm+2 GPIO port selection
pub type EXTI6_R = crate::FieldReader;
///Field `EXTI6` writer - EXTIm+2 GPIO port selection
pub type EXTI6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI7` reader - EXTIm+3 GPIO port selection
pub type EXTI7_R = crate::FieldReader;
///Field `EXTI7` writer - EXTIm+3 GPIO port selection
pub type EXTI7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti4(&self) -> EXTI4_R {
        EXTI4_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti5(&self) -> EXTI5_R {
        EXTI5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti6(&self) -> EXTI6_R {
        EXTI6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti7(&self) -> EXTI7_R {
        EXTI7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_EXTICR2")
            .field("exti4", &self.exti4())
            .field("exti5", &self.exti5())
            .field("exti6", &self.exti6())
            .field("exti7", &self.exti7())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti4(&mut self) -> EXTI4_W<EXTI_EXTICR2rs> {
        EXTI4_W::new(self, 0)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti5(&mut self) -> EXTI5_W<EXTI_EXTICR2rs> {
        EXTI5_W::new(self, 8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti6(&mut self) -> EXTI6_W<EXTI_EXTICR2rs> {
        EXTI6_W::new(self, 16)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti7(&mut self) -> EXTI7_W<EXTI_EXTICR2rs> {
        EXTI7_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exti_exticr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#EXTI:EXTI_EXTICR2)*/
pub struct EXTI_EXTICR2rs;
impl crate::RegisterSpec for EXTI_EXTICR2rs {
    type Ux = u32;
}
///`read()` method returns [`exti_exticr2::R`](R) reader structure
impl crate::Readable for EXTI_EXTICR2rs {}
///`write(|w| ..)` method takes [`exti_exticr2::W`](W) writer structure
impl crate::Writable for EXTI_EXTICR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTI_EXTICR2 to value 0
impl crate::Resettable for EXTI_EXTICR2rs {
    const RESET_VALUE: u32 = 0;
}
