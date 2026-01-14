///Register `HSICFGR` reader
pub type R = crate::R<HSICFGRrs>;
///Register `HSICFGR` writer
pub type W = crate::W<HSICFGRrs>;
///Field `HSICAL` reader - HSI clock calibration Set by hardware by option byte loading during system reset nreset. Adjusted by software through trimming bits HSITRIM. This field represents the sum of engineering option byte calibration value and HSITRIM bits value.
pub type HSICAL_R = crate::FieldReader<u16>;
///Field `HSITRIM` reader - HSI clock trimming Set by software to adjust calibration. HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_OPT) in order to form the calibration trimming value. HSICAL = HSITRIM + FLASH_HSI_OPT. After a change of HSITRIM it takes one system clock cycle before the new HSITRIM value is updated Note: The reset value of the field is 0x40.
pub type HSITRIM_R = crate::FieldReader;
///Field `HSITRIM` writer - HSI clock trimming Set by software to adjust calibration. HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_OPT) in order to form the calibration trimming value. HSICAL = HSITRIM + FLASH_HSI_OPT. After a change of HSITRIM it takes one system clock cycle before the new HSITRIM value is updated Note: The reset value of the field is 0x40.
pub type HSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 7, u8, crate::Safe>;
impl R {
    ///Bits 0:11 - HSI clock calibration Set by hardware by option byte loading during system reset nreset. Adjusted by software through trimming bits HSITRIM. This field represents the sum of engineering option byte calibration value and HSITRIM bits value.
    #[inline(always)]
    pub fn hsical(&self) -> HSICAL_R {
        HSICAL_R::new((self.bits & 0x0fff) as u16)
    }
    ///Bits 16:22 - HSI clock trimming Set by software to adjust calibration. HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_OPT) in order to form the calibration trimming value. HSICAL = HSITRIM + FLASH_HSI_OPT. After a change of HSITRIM it takes one system clock cycle before the new HSITRIM value is updated Note: The reset value of the field is 0x40.
    #[inline(always)]
    pub fn hsitrim(&self) -> HSITRIM_R {
        HSITRIM_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSICFGR")
            .field("hsical", &self.hsical())
            .field("hsitrim", &self.hsitrim())
            .finish()
    }
}
impl W {
    ///Bits 16:22 - HSI clock trimming Set by software to adjust calibration. HSITRIM field is added to the engineering option bytes loaded during reset phase (FLASH_HSI_OPT) in order to form the calibration trimming value. HSICAL = HSITRIM + FLASH_HSI_OPT. After a change of HSITRIM it takes one system clock cycle before the new HSITRIM value is updated Note: The reset value of the field is 0x40.
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HSITRIM_W<'_, HSICFGRrs> {
        HSITRIM_W::new(self, 16)
    }
}
/**RCC HSI calibration register

You can [`read`](crate::Reg::read) this register and get [`hsicfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsicfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#RCC:HSICFGR)*/
pub struct HSICFGRrs;
impl crate::RegisterSpec for HSICFGRrs {
    type Ux = u32;
}
///`read()` method returns [`hsicfgr::R`](R) reader structure
impl crate::Readable for HSICFGRrs {}
///`write(|w| ..)` method takes [`hsicfgr::W`](W) writer structure
impl crate::Writable for HSICFGRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HSICFGR to value 0x0040_0000
impl crate::Resettable for HSICFGRrs {
    const RESET_VALUE: u32 = 0x0040_0000;
}
