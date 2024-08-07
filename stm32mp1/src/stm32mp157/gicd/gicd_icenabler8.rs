///Register `GICD_ICENABLER8` reader
pub type R = crate::R<GICD_ICENABLER8rs>;
///Register `GICD_ICENABLER8` writer
pub type W = crate::W<GICD_ICENABLER8rs>;
///Field `ICENABLER8` reader - ICENABLER8
pub type ICENABLER8_R = crate::FieldReader<u32>;
///Field `ICENABLER8` writer - ICENABLER8
pub type ICENABLER8_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER8
    #[inline(always)]
    pub fn icenabler8(&self) -> ICENABLER8_R {
        ICENABLER8_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICENABLER8")
            .field("icenabler8", &self.icenabler8())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER8
    #[inline(always)]
    #[must_use]
    pub fn icenabler8(&mut self) -> ICENABLER8_W<GICD_ICENABLER8rs> {
        ICENABLER8_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_icenabler8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_ICENABLER8)*/
pub struct GICD_ICENABLER8rs;
impl crate::RegisterSpec for GICD_ICENABLER8rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_icenabler8::R`](R) reader structure
impl crate::Readable for GICD_ICENABLER8rs {}
///`write(|w| ..)` method takes [`gicd_icenabler8::W`](W) writer structure
impl crate::Writable for GICD_ICENABLER8rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ICENABLER8 to value 0
impl crate::Resettable for GICD_ICENABLER8rs {
    const RESET_VALUE: u32 = 0;
}
