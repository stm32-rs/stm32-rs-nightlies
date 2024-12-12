///Register `MDF_OLD1THLR` reader
pub type R = crate::R<MDF_OLD1THLRrs>;
///Register `MDF_OLD1THLR` writer
pub type W = crate::W<MDF_OLD1THLRrs>;
///Field `OLDTHL` reader - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type OLDTHL_R = crate::FieldReader<u32>;
///Field `OLDTHL` writer - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type OLDTHL_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    ///Bits 0:25 - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn oldthl(&self) -> OLDTHL_R {
        OLDTHL_R::new(self.bits & 0x03ff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDF_OLD1THLR")
            .field("oldthl", &self.oldthl())
            .finish()
    }
}
impl W {
    ///Bits 0:25 - OLD Low Threshold Value Set and cleared by software. OLDTHL represents a 26-bit signed value. The real threshold compared to the signal provided by the filter is OLDTHL. This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn oldthl(&mut self) -> OLDTHL_W<MDF_OLD1THLRrs> {
        OLDTHL_W::new(self, 0)
    }
}
/**This register is used for the adjustment of the Out-off Limit low threshold.

You can [`read`](crate::Reg::read) this register and get [`mdf_old1thlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_old1thlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U595.html#MDF1:MDF_OLD1THLR)*/
pub struct MDF_OLD1THLRrs;
impl crate::RegisterSpec for MDF_OLD1THLRrs {
    type Ux = u32;
}
///`read()` method returns [`mdf_old1thlr::R`](R) reader structure
impl crate::Readable for MDF_OLD1THLRrs {}
///`write(|w| ..)` method takes [`mdf_old1thlr::W`](W) writer structure
impl crate::Writable for MDF_OLD1THLRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDF_OLD1THLR to value 0
impl crate::Resettable for MDF_OLD1THLRrs {
    const RESET_VALUE: u32 = 0;
}
