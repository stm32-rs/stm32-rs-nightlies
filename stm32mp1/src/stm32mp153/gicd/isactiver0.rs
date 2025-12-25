///Register `ISACTIVER0` reader
pub type R = crate::R<ISACTIVER0rs>;
///Register `ISACTIVER0` writer
pub type W = crate::W<ISACTIVER0rs>;
///Field `ISACTIVER0` reader - ISACTIVER0
pub type ISACTIVER0_R = crate::FieldReader<u32>;
///Field `ISACTIVER0` writer - ISACTIVER0
pub type ISACTIVER0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER0
    #[inline(always)]
    pub fn isactiver0(&self) -> ISACTIVER0_R {
        ISACTIVER0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISACTIVER0")
            .field("isactiver0", &self.isactiver0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER0
    #[inline(always)]
    pub fn isactiver0(&mut self) -> ISACTIVER0_W<'_, ISACTIVER0rs> {
        ISACTIVER0_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`isactiver0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isactiver0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:ISACTIVER0)*/
pub struct ISACTIVER0rs;
impl crate::RegisterSpec for ISACTIVER0rs {
    type Ux = u32;
}
///`read()` method returns [`isactiver0::R`](R) reader structure
impl crate::Readable for ISACTIVER0rs {}
///`write(|w| ..)` method takes [`isactiver0::W`](W) writer structure
impl crate::Writable for ISACTIVER0rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ISACTIVER0 to value 0
impl crate::Resettable for ISACTIVER0rs {}
