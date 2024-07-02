///Register `GICV_APR0` reader
pub type R = crate::R<GICV_APR0rs>;
///Register `GICV_APR0` writer
pub type W = crate::W<GICV_APR0rs>;
///Field `APR0` reader - APR0
pub type APR0_R = crate::FieldReader<u32>;
///Field `APR0` writer - APR0
pub type APR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - APR0
    #[inline(always)]
    pub fn apr0(&self) -> APR0_R {
        APR0_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GICV_APR0")
            .field("apr0", &self.apr0())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - APR0
    #[inline(always)]
    #[must_use]
    pub fn apr0(&mut self) -> APR0_W<GICV_APR0rs> {
        APR0_W::new(self, 0)
    }
}
/**The GICV_APR0 is an alias of GICH_APR.

You can [`read`](crate::Reg::read) this register and get [`gicv_apr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gicv_apr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#GICV:GICV_APR0)*/
pub struct GICV_APR0rs;
impl crate::RegisterSpec for GICV_APR0rs {
    type Ux = u32;
}
///`read()` method returns [`gicv_apr0::R`](R) reader structure
impl crate::Readable for GICV_APR0rs {}
///`write(|w| ..)` method takes [`gicv_apr0::W`](W) writer structure
impl crate::Writable for GICV_APR0rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets GICV_APR0 to value 0
impl crate::Resettable for GICV_APR0rs {
    const RESET_VALUE: u32 = 0;
}
