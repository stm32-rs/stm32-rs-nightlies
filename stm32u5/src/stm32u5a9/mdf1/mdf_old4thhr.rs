///Register `MDF_OLD4THHR` reader
pub type R = crate::R<MDF_OLD4THHRrs>;
///Register `MDF_OLD4THHR` writer
pub type W = crate::W<MDF_OLD4THHRrs>;
///Field `OLDTHH` reader - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
pub type OLDTHH_R = crate::FieldReader<u32>;
///Field `OLDTHH` writer - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
pub type OLDTHH_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
    #[inline(always)]
    pub fn oldthh(&self) -> OLDTHH_R {
        OLDTHH_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDF_OLD4THHR")
            .field("oldthh", &self.oldthh())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLD High Threshold Value Set and cleared by software. OLDTHH represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHH. This field can be write-protected, please refer to Section 1.4.15: Register protection for details
    #[inline(always)]
    #[must_use]
    pub fn oldthh(&mut self) -> OLDTHH_W<MDF_OLD4THHRrs> {
        OLDTHH_W::new(self, 0)
    }
}
/**This register is used for the adjustment of the Out-off Limit high threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old4thhr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old4thhr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#MDF1:MDF_OLD4THHR)*/
pub struct MDF_OLD4THHRrs;
impl crate::RegisterSpec for MDF_OLD4THHRrs {
    type Ux = u32;
}
///`read()` method returns [`mdf_old4thhr::R`](R) reader structure
impl crate::Readable for MDF_OLD4THHRrs {}
///`write(|w| ..)` method takes [`mdf_old4thhr::W`](W) writer structure
impl crate::Writable for MDF_OLD4THHRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDF_OLD4THHR to value 0
impl crate::Resettable for MDF_OLD4THHRrs {
    const RESET_VALUE: u32 = 0;
}
