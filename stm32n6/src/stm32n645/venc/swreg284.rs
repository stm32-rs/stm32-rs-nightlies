///Register `SWREG284` reader
pub type R = crate::R<SWREG284rs>;
///Register `SWREG284` writer
pub type W = crate::W<SWREG284rs>;
///Field `SWREG_FIELD` reader - segment 3: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 (all format mode)
pub type SWREG_FIELD_R = crate::FieldReader<u32>;
///Field `SWREG_FIELD` writer - segment 3: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 (all format mode)
pub type SWREG_FIELD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - segment 3: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&self) -> SWREG_FIELD_R {
        SWREG_FIELD_R::new(self.bits)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SWREG284")
            .field("swreg_field", &self.swreg_field())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - segment 3: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 (all format mode)
    #[inline(always)]
    pub fn swreg_field(&mut self) -> SWREG_FIELD_W<'_, SWREG284rs> {
        SWREG_FIELD_W::new(self, 0)
    }
}
/**VENC segment 3: intra 4x4 mode 8-9 penalty, intra 4x4 previous mode favor for H.264 register

You can [`read`](crate::Reg::read) this register and get [`swreg284::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swreg284::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#VENC:SWREG284)*/
pub struct SWREG284rs;
impl crate::RegisterSpec for SWREG284rs {
    type Ux = u32;
}
///`read()` method returns [`swreg284::R`](R) reader structure
impl crate::Readable for SWREG284rs {}
///`write(|w| ..)` method takes [`swreg284::W`](W) writer structure
impl crate::Writable for SWREG284rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SWREG284 to value 0
impl crate::Resettable for SWREG284rs {}
