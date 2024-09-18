///Register `GICD_ISENABLER3` reader
pub type R = crate::R<GICD_ISENABLER3rs>;
///Register `GICD_ISENABLER3` writer
pub type W = crate::W<GICD_ISENABLER3rs>;
///Field `ISENABLER3` reader - ISENABLER3
pub type ISENABLER3_R = crate::FieldReader<u32>;
///Field `ISENABLER3` writer - ISENABLER3
pub type ISENABLER3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISENABLER3
    #[inline(always)]
    pub fn isenabler3(&self) -> ISENABLER3_R {
        ISENABLER3_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ISENABLER3")
            .field("isenabler3", &self.isenabler3())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISENABLER3
    #[inline(always)]
    #[must_use]
    pub fn isenabler3(&mut self) -> ISENABLER3_W<GICD_ISENABLER3rs> {
        ISENABLER3_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_isenabler3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_isenabler3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_ISENABLER3)*/
pub struct GICD_ISENABLER3rs;
impl crate::RegisterSpec for GICD_ISENABLER3rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_isenabler3::R`](R) reader structure
impl crate::Readable for GICD_ISENABLER3rs {}
///`write(|w| ..)` method takes [`gicd_isenabler3::W`](W) writer structure
impl crate::Writable for GICD_ISENABLER3rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ISENABLER3 to value 0
impl crate::Resettable for GICD_ISENABLER3rs {
    const RESET_VALUE: u32 = 0;
}
