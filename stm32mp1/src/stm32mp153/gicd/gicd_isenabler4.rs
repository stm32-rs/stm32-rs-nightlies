///Register `GICD_ISENABLER4` reader
pub type R = crate::R<GICD_ISENABLER4rs>;
///Register `GICD_ISENABLER4` writer
pub type W = crate::W<GICD_ISENABLER4rs>;
///Field `ISENABLER4` reader - ISENABLER4
pub type ISENABLER4_R = crate::FieldReader<u32>;
///Field `ISENABLER4` writer - ISENABLER4
pub type ISENABLER4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER4
    #[inline(always)]
    pub fn isenabler4(&self) -> ISENABLER4_R {
        ISENABLER4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ISENABLER4")
            .field("isenabler4", &self.isenabler4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER4
    #[inline(always)]
    #[must_use]
    pub fn isenabler4(&mut self) -> ISENABLER4_W<GICD_ISENABLER4rs> {
        ISENABLER4_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_isenabler4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_ISENABLER4)*/
pub struct GICD_ISENABLER4rs;
impl crate::RegisterSpec for GICD_ISENABLER4rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_isenabler4::R`](R) reader structure
impl crate::Readable for GICD_ISENABLER4rs {}
///`write(|w| ..)` method takes [`gicd_isenabler4::W`](W) writer structure
impl crate::Writable for GICD_ISENABLER4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ISENABLER4 to value 0
impl crate::Resettable for GICD_ISENABLER4rs {
    const RESET_VALUE: u32 = 0;
}
