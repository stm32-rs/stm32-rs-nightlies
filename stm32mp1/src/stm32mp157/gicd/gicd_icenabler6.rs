///Register `GICD_ICENABLER6` reader
pub type R = crate::R<GICD_ICENABLER6rs>;
///Register `GICD_ICENABLER6` writer
pub type W = crate::W<GICD_ICENABLER6rs>;
///Field `ICENABLER6` reader - ICENABLER6
pub type ICENABLER6_R = crate::FieldReader<u32>;
///Field `ICENABLER6` writer - ICENABLER6
pub type ICENABLER6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICENABLER6
    #[inline(always)]
    pub fn icenabler6(&self) -> ICENABLER6_R {
        ICENABLER6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICENABLER6")
            .field("icenabler6", &self.icenabler6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICENABLER6
    #[inline(always)]
    #[must_use]
    pub fn icenabler6(&mut self) -> ICENABLER6_W<GICD_ICENABLER6rs> {
        ICENABLER6_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_icenabler6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icenabler6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_ICENABLER6)*/
pub struct GICD_ICENABLER6rs;
impl crate::RegisterSpec for GICD_ICENABLER6rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_icenabler6::R`](R) reader structure
impl crate::Readable for GICD_ICENABLER6rs {}
///`write(|w| ..)` method takes [`gicd_icenabler6::W`](W) writer structure
impl crate::Writable for GICD_ICENABLER6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ICENABLER6 to value 0
impl crate::Resettable for GICD_ICENABLER6rs {
    const RESET_VALUE: u32 = 0;
}
