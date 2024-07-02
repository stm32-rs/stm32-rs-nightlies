///Register `EXTI_EXTICR1` reader
pub type R = crate::R<EXTI_EXTICR1rs>;
///Register `EXTI_EXTICR1` writer
pub type W = crate::W<EXTI_EXTICR1rs>;
///Field `EXTI0` reader - EXTIm GPIO port selection
pub type EXTI0_R = crate::FieldReader;
///Field `EXTI0` writer - EXTIm GPIO port selection
pub type EXTI0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI1` reader - EXTIm+1 GPIO port selection
pub type EXTI1_R = crate::FieldReader;
///Field `EXTI1` writer - EXTIm+1 GPIO port selection
pub type EXTI1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI2` reader - EXTIm+2 GPIO port selection
pub type EXTI2_R = crate::FieldReader;
///Field `EXTI2` writer - EXTIm+2 GPIO port selection
pub type EXTI2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `EXTI3` reader - EXTIm+3 GPIO port selection
pub type EXTI3_R = crate::FieldReader;
///Field `EXTI3` writer - EXTIm+3 GPIO port selection
pub type EXTI3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    pub fn exti0(&self) -> EXTI0_R {
        EXTI0_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    pub fn exti1(&self) -> EXTI1_R {
        EXTI1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    pub fn exti2(&self) -> EXTI2_R {
        EXTI2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    pub fn exti3(&self) -> EXTI3_R {
        EXTI3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTI_EXTICR1")
            .field("exti0", &self.exti0())
            .field("exti1", &self.exti1())
            .field("exti2", &self.exti2())
            .field("exti3", &self.exti3())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - EXTIm GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti0(&mut self) -> EXTI0_W<EXTI_EXTICR1rs> {
        EXTI0_W::new(self, 0)
    }
    ///Bits 8:15 - EXTIm+1 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti1(&mut self) -> EXTI1_W<EXTI_EXTICR1rs> {
        EXTI1_W::new(self, 8)
    }
    ///Bits 16:23 - EXTIm+2 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti2(&mut self) -> EXTI2_W<EXTI_EXTICR1rs> {
        EXTI2_W::new(self, 16)
    }
    ///Bits 24:31 - EXTIm+3 GPIO port selection
    #[inline(always)]
    #[must_use]
    pub fn exti3(&mut self) -> EXTI3_W<EXTI_EXTICR1rs> {
        EXTI3_W::new(self, 24)
    }
}
/**EXTI external interrupt selection register

You can [`read`](crate::Reg::read) this register and get [`exti_exticr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exti_exticr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U535.html#EXTI:EXTI_EXTICR1)*/
pub struct EXTI_EXTICR1rs;
impl crate::RegisterSpec for EXTI_EXTICR1rs {
    type Ux = u32;
}
///`read()` method returns [`exti_exticr1::R`](R) reader structure
impl crate::Readable for EXTI_EXTICR1rs {}
///`write(|w| ..)` method takes [`exti_exticr1::W`](W) writer structure
impl crate::Writable for EXTI_EXTICR1rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXTI_EXTICR1 to value 0
impl crate::Resettable for EXTI_EXTICR1rs {
    const RESET_VALUE: u32 = 0;
}
