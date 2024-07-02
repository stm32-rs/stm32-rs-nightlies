///Register `FLASH_SECHDPCR` reader
pub type R = crate::R<FLASH_SECHDPCRrs>;
///Register `FLASH_SECHDPCR` writer
pub type W = crate::W<FLASH_SECHDPCRrs>;
///Field `HDP1_ACCDIS` reader - HDP1 area access disable When set, this bit is only cleared by a system reset.
pub type HDP1_ACCDIS_R = crate::BitReader;
///Field `HDP1_ACCDIS` writer - HDP1 area access disable When set, this bit is only cleared by a system reset.
pub type HDP1_ACCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HDP2_ACCDIS` reader - HDP2 area access disable When set, this bit is only cleared by a system reset.
pub type HDP2_ACCDIS_R = crate::BitReader;
///Field `HDP2_ACCDIS` writer - HDP2 area access disable When set, this bit is only cleared by a system reset.
pub type HDP2_ACCDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - HDP1 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    pub fn hdp1_accdis(&self) -> HDP1_ACCDIS_R {
        HDP1_ACCDIS_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HDP2 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    pub fn hdp2_accdis(&self) -> HDP2_ACCDIS_R {
        HDP2_ACCDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLASH_SECHDPCR")
            .field("hdp1_accdis", &self.hdp1_accdis())
            .field("hdp2_accdis", &self.hdp2_accdis())
            .finish()
    }
}
impl W {
    ///Bit 0 - HDP1 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    #[must_use]
    pub fn hdp1_accdis(&mut self) -> HDP1_ACCDIS_W<FLASH_SECHDPCRrs> {
        HDP1_ACCDIS_W::new(self, 0)
    }
    ///Bit 1 - HDP2 area access disable When set, this bit is only cleared by a system reset.
    #[inline(always)]
    #[must_use]
    pub fn hdp2_accdis(&mut self) -> HDP2_ACCDIS_W<FLASH_SECHDPCRrs> {
        HDP2_ACCDIS_W::new(self, 1)
    }
}
/**FLASH secure HDP control register

You can [`read`](crate::Reg::read) this register and get [`flash_sechdpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flash_sechdpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FLASH:FLASH_SECHDPCR)*/
pub struct FLASH_SECHDPCRrs;
impl crate::RegisterSpec for FLASH_SECHDPCRrs {
    type Ux = u32;
}
///`read()` method returns [`flash_sechdpcr::R`](R) reader structure
impl crate::Readable for FLASH_SECHDPCRrs {}
///`write(|w| ..)` method takes [`flash_sechdpcr::W`](W) writer structure
impl crate::Writable for FLASH_SECHDPCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FLASH_SECHDPCR to value 0
impl crate::Resettable for FLASH_SECHDPCRrs {
    const RESET_VALUE: u32 = 0;
}
