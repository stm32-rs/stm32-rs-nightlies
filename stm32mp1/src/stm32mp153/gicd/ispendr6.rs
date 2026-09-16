///Register `ISPENDR6` reader
pub type R = crate::R<ISPENDR6rs>;
///Register `ISPENDR6` writer
pub type W = crate::W<ISPENDR6rs>;
///Field `ISPENDR6` reader - ISPENDR6
pub type ISPENDR6_R = crate::FieldReader<u32>;
///Field `ISPENDR6` writer - ISPENDR6
pub type ISPENDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR6
    #[inline(always)]
    pub fn ispendr6(&self) -> ISPENDR6_R {
        ISPENDR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISPENDR6")
            .field("ispendr6", &self.ispendr6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR6
    #[inline(always)]
    pub fn ispendr6(&mut self) -> ISPENDR6_W<'_, ISPENDR6rs> {
        ISPENDR6_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`ispendr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ispendr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISPENDR6)*/
pub struct ISPENDR6rs;
impl crate::RegisterSpec for ISPENDR6rs {
    type Ux = u32;
}
///`read()` method returns [`ispendr6::R`](R) reader structure
impl crate::Readable for ISPENDR6rs {}
///`write(|w| ..)` method takes [`ispendr6::W`](W) writer structure
impl crate::Writable for ISPENDR6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISPENDR6 to value 0
impl crate::Resettable for ISPENDR6rs {}
