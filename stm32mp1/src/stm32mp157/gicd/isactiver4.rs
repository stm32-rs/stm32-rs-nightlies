///Register `ISACTIVER4` reader
pub type R = crate::R<ISACTIVER4rs>;
///Register `ISACTIVER4` writer
pub type W = crate::W<ISACTIVER4rs>;
///Field `ISACTIVER4` reader - ISACTIVER4
pub type ISACTIVER4_R = crate::FieldReader<u32>;
///Field `ISACTIVER4` writer - ISACTIVER4
pub type ISACTIVER4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER4
    #[inline(always)]
    pub fn isactiver4(&self) -> ISACTIVER4_R {
        ISACTIVER4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISACTIVER4")
            .field("isactiver4", &self.isactiver4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER4
    #[inline(always)]
    pub fn isactiver4(&mut self) -> ISACTIVER4_W<'_, ISACTIVER4rs> {
        ISACTIVER4_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER4)*/
pub struct ISACTIVER4rs;
impl crate::RegisterSpec for ISACTIVER4rs {
    type Ux = u32;
}
///`read()` method returns [`isactiver4::R`](R) reader structure
impl crate::Readable for ISACTIVER4rs {}
///`write(|w| ..)` method takes [`isactiver4::W`](W) writer structure
impl crate::Writable for ISACTIVER4rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISACTIVER4 to value 0
impl crate::Resettable for ISACTIVER4rs {}
