///Register `GICD_ISACTIVER5` reader
pub type R = crate::R<GICD_ISACTIVER5rs>;
///Register `GICD_ISACTIVER5` writer
pub type W = crate::W<GICD_ISACTIVER5rs>;
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
        f.debug_struct("GICD_ISACTIVER5")
            .field("isactiver5", &self.isactiver5())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER5
    #[inline(always)]
    #[must_use]
    pub fn isactiver5(&mut self) -> ISACTIVER5_W<GICD_ISACTIVER5rs> {
        ISACTIVER5_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_isactiver5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_ISACTIVER5)*/
pub struct GICD_ISACTIVER5rs;
impl crate::RegisterSpec for GICD_ISACTIVER5rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_isactiver5::R`](R) reader structure
impl crate::Readable for GICD_ISACTIVER5rs {}
///`write(|w| ..)` method takes [`gicd_isactiver5::W`](W) writer structure
impl crate::Writable for GICD_ISACTIVER5rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ISACTIVER5 to value 0
impl crate::Resettable for GICD_ISACTIVER5rs {
    const RESET_VALUE: u32 = 0;
}
