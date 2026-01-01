///Register `ISACTIVER6` reader
pub type R = crate::R<ISACTIVER6rs>;
///Register `ISACTIVER6` writer
pub type W = crate::W<ISACTIVER6rs>;
///Field `ISACTIVER6` reader - ISACTIVER6
pub type ISACTIVER6_R = crate::FieldReader<u32>;
///Field `ISACTIVER6` writer - ISACTIVER6
pub type ISACTIVER6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER6
    #[inline(always)]
    pub fn isactiver6(&self) -> ISACTIVER6_R {
        ISACTIVER6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISACTIVER6")
            .field("isactiver6", &self.isactiver6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER6
    #[inline(always)]
    pub fn isactiver6(&mut self) -> ISACTIVER6_W<'_, ISACTIVER6rs> {
        ISACTIVER6_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISACTIVER6)*/
pub struct ISACTIVER6rs;
impl crate::RegisterSpec for ISACTIVER6rs {
    type Ux = u32;
}
///`read()` method returns [`isactiver6::R`](R) reader structure
impl crate::Readable for ISACTIVER6rs {}
///`write(|w| ..)` method takes [`isactiver6::W`](W) writer structure
impl crate::Writable for ISACTIVER6rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISACTIVER6 to value 0
impl crate::Resettable for ISACTIVER6rs {}
