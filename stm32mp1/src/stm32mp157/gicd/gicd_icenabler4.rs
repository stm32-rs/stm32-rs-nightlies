///Register `GICD_ICENABLER4` reader
pub type R = crate::R<GICD_ICENABLER4rs>;
///Register `GICD_ICENABLER4` writer
pub type W = crate::W<GICD_ICENABLER4rs>;
///Field `ICENABLER4` reader - ICENABLER4
pub type ICENABLER4_R = crate::FieldReader<u32>;
///Field `ICENABLER4` writer - ICENABLER4
pub type ICENABLER4_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER4
    #[inline(always)]
    pub fn icenabler4(&self) -> ICENABLER4_R {
        ICENABLER4_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICENABLER4")
            .field("icenabler4", &self.icenabler4())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER4
    #[inline(always)]
    #[must_use]
    pub fn icenabler4(&mut self) -> ICENABLER4_W<GICD_ICENABLER4rs> {
        ICENABLER4_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_icenabler4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_ICENABLER4)*/
pub struct GICD_ICENABLER4rs;
impl crate::RegisterSpec for GICD_ICENABLER4rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_icenabler4::R`](R) reader structure
impl crate::Readable for GICD_ICENABLER4rs {}
///`write(|w| ..)` method takes [`gicd_icenabler4::W`](W) writer structure
impl crate::Writable for GICD_ICENABLER4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ICENABLER4 to value 0
impl crate::Resettable for GICD_ICENABLER4rs {
    const RESET_VALUE: u32 = 0;
}
