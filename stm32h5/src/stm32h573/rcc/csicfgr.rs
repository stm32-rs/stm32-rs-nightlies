///Register `CSICFGR` reader
pub type R = crate::R<CSICFGRrs>;
///Register `CSICFGR` writer
pub type W = crate::W<CSICFGRrs>;
///Field `CSICAL` reader - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value.
pub type CSICAL_R = crate::FieldReader;
///Field `CSICAL` writer - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value.
pub type CSICAL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `CSITRIM` reader - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20.
pub type CSITRIM_R = crate::FieldReader;
///Field `CSITRIM` writer - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20.
pub type CSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 6, u8, crate::Safe>;
impl R {
    ///Bits 0:7 - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value.
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:21 - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20.
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSICFGR")
            .field("csical", &self.csical())
            .field("csitrim", &self.csitrim())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - CSI clock calibration Set by hardware by option byte loading during system reset NRESET. Adjusted by software through trimming bits CSITRIM. This field represents the sum of engineering option byte calibration value and CSITRIM bits value.
    #[inline(always)]
    pub fn csical(&mut self) -> CSICAL_W<'_, CSICFGRrs> {
        CSICAL_W::new(self, 0)
    }
    ///Bits 16:21 - CSI clock trimming Set by software to adjust calibration. CSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_CSI_OPT) in order to form the calibration trimming value. CSICAL = CSITRIM + FLASH_CSI_OPT. Note: The reset value of the field is 0x20.
    #[inline(always)]
    pub fn csitrim(&mut self) -> CSITRIM_W<'_, CSICFGRrs> {
        CSITRIM_W::new(self, 16)
    }
}
/**RCC CSI calibration register

You can [`read`](crate::Reg::read) this register and get [`csicfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csicfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:CSICFGR)*/
pub struct CSICFGRrs;
impl crate::RegisterSpec for CSICFGRrs {
    type Ux = u32;
}
///`read()` method returns [`csicfgr::R`](R) reader structure
impl crate::Readable for CSICFGRrs {}
///`write(|w| ..)` method takes [`csicfgr::W`](W) writer structure
impl crate::Writable for CSICFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSICFGR to value 0x0020_0000
impl crate::Resettable for CSICFGRrs {
    const RESET_VALUE: u32 = 0x0020_0000;
}
