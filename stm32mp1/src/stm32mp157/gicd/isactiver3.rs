///Register `ISACTIVER3` reader
pub type R = crate::R<ISACTIVER3rs>;
///Register `ISACTIVER3` writer
pub type W = crate::W<ISACTIVER3rs>;
///Field `ISACTIVER3` reader - ISACTIVER3
pub type ISACTIVER3_R = crate::FieldReader<u32>;
///Field `ISACTIVER3` writer - ISACTIVER3
pub type ISACTIVER3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER3
    #[inline(always)]
    pub fn isactiver3(&self) -> ISACTIVER3_R {
        ISACTIVER3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISACTIVER3")
            .field("isactiver3", &self.isactiver3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER3
    #[inline(always)]
    pub fn isactiver3(&mut self) -> ISACTIVER3_W<'_, ISACTIVER3rs> {
        ISACTIVER3_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:ISACTIVER3)*/
pub struct ISACTIVER3rs;
impl crate::RegisterSpec for ISACTIVER3rs {
    type Ux = u32;
}
///`read()` method returns [`isactiver3::R`](R) reader structure
impl crate::Readable for ISACTIVER3rs {}
///`write(|w| ..)` method takes [`isactiver3::W`](W) writer structure
impl crate::Writable for ISACTIVER3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISACTIVER3 to value 0
impl crate::Resettable for ISACTIVER3rs {}
