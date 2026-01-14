///Register `ISACTIVER5` reader
pub type R = crate::R<ISACTIVER5rs>;
///Register `ISACTIVER5` writer
pub type W = crate::W<ISACTIVER5rs>;
///Field `ISACTIVER5` reader - ISACTIVER5
pub type ISACTIVER5_R = crate::FieldReader<u32>;
///Field `ISACTIVER5` writer - ISACTIVER5
pub type ISACTIVER5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER5
    #[inline(always)]
    pub fn isactiver5(&self) -> ISACTIVER5_R {
        ISACTIVER5_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISACTIVER5")
            .field("isactiver5", &self.isactiver5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER5
    #[inline(always)]
    pub fn isactiver5(&mut self) -> ISACTIVER5_W<'_, ISACTIVER5rs> {
        ISACTIVER5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER5)*/
pub struct ISACTIVER5rs;
impl crate::RegisterSpec for ISACTIVER5rs {
    type Ux = u32;
}
///`read()` method returns [`isactiver5::R`](R) reader structure
impl crate::Readable for ISACTIVER5rs {}
///`write(|w| ..)` method takes [`isactiver5::W`](W) writer structure
impl crate::Writable for ISACTIVER5rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISACTIVER5 to value 0
impl crate::Resettable for ISACTIVER5rs {}
