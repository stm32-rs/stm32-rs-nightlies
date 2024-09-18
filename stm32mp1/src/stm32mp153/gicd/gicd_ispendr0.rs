///Register `GICD_ISPENDR0` reader
pub type R = crate::R<GICD_ISPENDR0rs>;
///Register `GICD_ISPENDR0` writer
pub type W = crate::W<GICD_ISPENDR0rs>;
///Field `ISPENDR0` reader - ISPENDR0
pub type ISPENDR0_R = crate::FieldReader<u32>;
///Field `ISPENDR0` writer - ISPENDR0
pub type ISPENDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - ISPENDR0
    #[inline(always)]
    pub fn ispendr0(&self) -> ISPENDR0_R {
        ISPENDR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICD_ISPENDR0")
            .field("ispendr0", &self.ispendr0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - ISPENDR0
    #[inline(always)]
    #[must_use]
    pub fn ispendr0(&mut self) -> ISPENDR0_W<GICD_ISPENDR0rs> {
        ISPENDR0_W::new(self, 0)
    }
}
/**For interrupts ID

You can [`read`](crate::Reg::read) this register and get [`gicd_ispendr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicd_ispendr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#GICD:GICD_ISPENDR0)*/
pub struct GICD_ISPENDR0rs;
impl crate::RegisterSpec for GICD_ISPENDR0rs {
    type Ux = u32;
}
///`read()` method returns [`gicd_ispendr0::R`](R) reader structure
impl crate::Readable for GICD_ISPENDR0rs {}
///`write(|w| ..)` method takes [`gicd_ispendr0::W`](W) writer structure
impl crate::Writable for GICD_ISPENDR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICD_ISPENDR0 to value 0
impl crate::Resettable for GICD_ISPENDR0rs {
    const RESET_VALUE: u32 = 0;
}
