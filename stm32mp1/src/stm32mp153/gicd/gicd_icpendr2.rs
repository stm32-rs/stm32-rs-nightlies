///Register `GICD_ICPENDR2` reader
pub type R = crate::R<GICD_ICPENDR2rs>;
///Register `GICD_ICPENDR2` writer
pub type W = crate::W<GICD_ICPENDR2rs>;
///Field `ICPENDR2` reader - ICPENDR2
pub type ICPENDR2_R = crate::FieldReader<u32>;
///Field `ICPENDR2` writer - ICPENDR2
pub type ICPENDR2_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ICPENDR2
    #[inline(always)]
    pub fn icpendr2(&self) -> ICPENDR2_R {
        ICPENDR2_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ICPENDR2")
            .field("icpendr2", &self.icpendr2())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ICPENDR2
    #[inline(always)]
    #[must_use]
    pub fn icpendr2(&mut self) -> ICPENDR2_W<GICD_ICPENDR2rs> {
        ICPENDR2_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_icpendr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_icpendr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_ICPENDR2)*/
pub struct GICD_ICPENDR2rs;
impl crate::RegisterSpec for GICD_ICPENDR2rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_icpendr2::R`](R) reader structure
impl crate::Readable for GICD_ICPENDR2rs {}
///`write(|w| ..)` method takes [`gicd_icpendr2::W`](W) writer structure
impl crate::Writable for GICD_ICPENDR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ICPENDR2 to value 0
impl crate::Resettable for GICD_ICPENDR2rs {
    const RESET_VALUE: u32 = 0;
}
