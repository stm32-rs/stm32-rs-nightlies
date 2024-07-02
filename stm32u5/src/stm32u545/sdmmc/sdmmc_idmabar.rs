///Register `SDMMC_IDMABAR` reader
pub type R = crate::R<SDMMC_IDMABARrs>;
///Register `SDMMC_IDMABAR` writer
pub type W = crate::W<SDMMC_IDMABARrs>;
///Field `IDMABA` reader - Word aligned Linked list memory base address
pub type IDMABA_R = crate::FieldReader<u32>;
///Field `IDMABA` writer - Word aligned Linked list memory base address
pub type IDMABA_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    ///Bits 2:31 - Word aligned Linked list memory base address
    #[inline(always)]
    pub fn idmaba(&self) -> IDMABA_R {
        IDMABA_R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDMMC_IDMABAR")
            .field("idmaba", &self.idmaba())
            .finish()
    }
}
impl W {
    ///Bits 2:31 - Word aligned Linked list memory base address
    #[inline(always)]
    #[must_use]
    pub fn idmaba(&mut self) -> IDMABA_W<SDMMC_IDMABARrs> {
        IDMABA_W::new(self, 2)
    }
}
/**linked list memory base register

You can [`read`](crate::Reg::read) this register and get [`sdmmc_idmabar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmmc_idmabar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U545.html#SDMMC:SDMMC_IDMABAR)*/
pub struct SDMMC_IDMABARrs;
impl crate::RegisterSpec for SDMMC_IDMABARrs {
    type Ux = u32;
}
///`read()` method returns [`sdmmc_idmabar::R`](R) reader structure
impl crate::Readable for SDMMC_IDMABARrs {}
///`write(|w| ..)` method takes [`sdmmc_idmabar::W`](W) writer structure
impl crate::Writable for SDMMC_IDMABARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SDMMC_IDMABAR to value 0
impl crate::Resettable for SDMMC_IDMABARrs {
    const RESET_VALUE: u32 = 0;
}
