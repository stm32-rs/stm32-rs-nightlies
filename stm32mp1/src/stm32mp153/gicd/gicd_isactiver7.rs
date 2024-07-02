///Register `GICD_ISACTIVER7` reader
pub type R = crate::R<GICD_ISACTIVER7rs>;
///Register `GICD_ISACTIVER7` writer
pub type W = crate::W<GICD_ISACTIVER7rs>;
///Field `ISACTIVER7` reader - ISACTIVER7
pub type ISACTIVER7_R = crate::FieldReader<u32>;
///Field `ISACTIVER7` writer - ISACTIVER7
pub type ISACTIVER7_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISACTIVER7
    #[inline(always)]
    pub fn isactiver7(&self) -> ISACTIVER7_R {
        ISACTIVER7_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ISACTIVER7")
            .field("isactiver7", &self.isactiver7())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISACTIVER7
    #[inline(always)]
    #[must_use]
    pub fn isactiver7(&mut self) -> ISACTIVER7_W<GICD_ISACTIVER7rs> {
        ISACTIVER7_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_isactiver7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isactiver7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_ISACTIVER7)*/
pub struct GICD_ISACTIVER7rs;
impl crate::RegisterSpec for GICD_ISACTIVER7rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_isactiver7::R`](R) reader structure
impl crate::Readable for GICD_ISACTIVER7rs {}
///`write(|w| ..)` method takes [`gicd_isactiver7::W`](W) writer structure
impl crate::Writable for GICD_ISACTIVER7rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ISACTIVER7 to value 0
impl crate::Resettable for GICD_ISACTIVER7rs {
    const RESET_VALUE: u32 = 0;
}
