///Register `GICD_ICPENDR6` reader
pub type R = crate::R<GICD_ICPENDR6rs>;
///Register `GICD_ICPENDR6` writer
pub type W = crate::W<GICD_ICPENDR6rs>;
///Field `ICPENDR6` reader - ICPENDR6
pub type ICPENDR6_R = crate::FieldReader<u32>;
///Field `ICPENDR6` writer - ICPENDR6
pub type ICPENDR6_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR6
    #[inline(always)]
    pub fn icpendr6(&self) -> ICPENDR6_R {
        ICPENDR6_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICPENDR6")
            .field("icpendr6", &self.icpendr6())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR6
    #[inline(always)]
    #[must_use]
    pub fn icpendr6(&mut self) -> ICPENDR6_W<GICD_ICPENDR6rs> {
        ICPENDR6_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_icpendr6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icpendr6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICD:GICD_ICPENDR6)*/
pub struct GICD_ICPENDR6rs;
impl crate::RegisterSpec for GICD_ICPENDR6rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_icpendr6::R`](R) reader structure
impl crate::Readable for GICD_ICPENDR6rs {}
///`write(|w| ..)` method takes [`gicd_icpendr6::W`](W) writer structure
impl crate::Writable for GICD_ICPENDR6rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ICPENDR6 to value 0
impl crate::Resettable for GICD_ICPENDR6rs {
    const RESET_VALUE: u32 = 0;
}
