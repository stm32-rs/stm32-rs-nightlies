///Register `ISENABLER6` reader
pub type R = crate::R<ISENABLER6rs>;
///Register `ISENABLER6` writer
pub type W = crate::W<ISENABLER6rs>;
///Field `ISENABLER6` reader - ISENABLER6
pub type ISENABLER6_R = crate::FieldReader<u32>;
///Field `ISENABLER6` writer - ISENABLER6
pub type ISENABLER6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER6
    #[inline(always)]
    pub fn isenabler6(&self) -> ISENABLER6_R {
        ISENABLER6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISENABLER6")
            .field("isenabler6", &self.isenabler6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER6
    #[inline(always)]
    pub fn isenabler6(&mut self) -> ISENABLER6_W<'_, ISENABLER6rs> {
        ISENABLER6_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isenabler6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isenabler6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISENABLER6)*/
pub struct ISENABLER6rs;
impl crate::RegisterSpec for ISENABLER6rs {
    type Ux = u32;
}
///`read()` method returns [`isenabler6::R`](R) reader structure
impl crate::Readable for ISENABLER6rs {}
///`write(|w| ..)` method takes [`isenabler6::W`](W) writer structure
impl crate::Writable for ISENABLER6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISENABLER6 to value 0
impl crate::Resettable for ISENABLER6rs {}
