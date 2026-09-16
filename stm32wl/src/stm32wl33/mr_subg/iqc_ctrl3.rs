///Register `IQC_CTRL3` reader
pub type R = crate::R<IQC_CTRL3rs>;
///Register `IQC_CTRL3` writer
pub type W = crate::W<IQC_CTRL3rs>;
///Field `FAST_TIME` reader - Duration of the fast mode.
pub type FAST_TIME_R = crate::FieldReader;
///Field `FAST_TIME` writer - Duration of the fast mode.
pub type FAST_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - Duration of the fast mode.
    #[inline(always)]
    pub fn fast_time(&self) -> FAST_TIME_R {
        FAST_TIME_R::new((self.bits & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IQC_CTRL3")
            .field("fast_time", &self.fast_time())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - Duration of the fast mode.
    #[inline(always)]
    pub fn fast_time(&mut self) -> FAST_TIME_W<'_, IQC_CTRL3rs> {
        FAST_TIME_W::new(self, 0)
    }
}
/**IQC_CTRL3 register

You can [`read`](crate::Reg::read) this register and get [`iqc_ctrl3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iqc_ctrl3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#MR_SUBG:IQC_CTRL3)*/
pub struct IQC_CTRL3rs;
impl crate::RegisterSpec for IQC_CTRL3rs {
    type Ux = u32;
}
///`read()` method returns [`iqc_ctrl3::R`](R) reader structure
impl crate::Readable for IQC_CTRL3rs {}
///`write(|w| ..)` method takes [`iqc_ctrl3::W`](W) writer structure
impl crate::Writable for IQC_CTRL3rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IQC_CTRL3 to value 0x07
impl crate::Resettable for IQC_CTRL3rs {
    const RESET_VALUE: u32 = 0x07;
}
