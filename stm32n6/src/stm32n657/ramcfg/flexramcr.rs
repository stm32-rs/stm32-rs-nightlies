///Register `FLEXRAMCR` reader
pub type R = crate::R<FLEXRAMCRrs>;
///Register `FLEXRAMCR` writer
pub type W = crate::W<FLEXRAMCRrs>;
///Field `SRAMER` reader - SRAM erase
pub type SRAMER_R = crate::BitReader;
///Field `SRAMER` writer - SRAM erase
pub type SRAMER_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SRAMHWERDIS` reader - SRAM hardware erase disable
pub type SRAMHWERDIS_R = crate::BitReader;
///Field `SRAMHWERDIS` writer - SRAM hardware erase disable
pub type SRAMHWERDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ITCMCFG` reader - Configuration of the FLEXMEM I-TCM extension
pub type ITCMCFG_R = crate::FieldReader;
///Field `ITCMCFG` writer - Configuration of the FLEXMEM I-TCM extension
pub type ITCMCFG_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `DTCMCFG` reader - Configuration of the FLEXMEM D-TCM extension
pub type DTCMCFG_R = crate::BitReader;
///Field `DTCMCFG` writer - Configuration of the FLEXMEM D-TCM extension
pub type DTCMCFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&self) -> SRAMER_R {
        SRAMER_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 12 - SRAM hardware erase disable
    #[inline(always)]
    pub fn sramhwerdis(&self) -> SRAMHWERDIS_R {
        SRAMHWERDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bits 16:17 - Configuration of the FLEXMEM I-TCM extension
    #[inline(always)]
    pub fn itcmcfg(&self) -> ITCMCFG_R {
        ITCMCFG_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bit 24 - Configuration of the FLEXMEM D-TCM extension
    #[inline(always)]
    pub fn dtcmcfg(&self) -> DTCMCFG_R {
        DTCMCFG_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FLEXRAMCR")
            .field("sramer", &self.sramer())
            .field("sramhwerdis", &self.sramhwerdis())
            .field("itcmcfg", &self.itcmcfg())
            .field("dtcmcfg", &self.dtcmcfg())
            .finish()
    }
}
impl W {
    ///Bit 8 - SRAM erase
    #[inline(always)]
    pub fn sramer(&mut self) -> SRAMER_W<'_, FLEXRAMCRrs> {
        SRAMER_W::new(self, 8)
    }
    ///Bit 12 - SRAM hardware erase disable
    #[inline(always)]
    pub fn sramhwerdis(&mut self) -> SRAMHWERDIS_W<'_, FLEXRAMCRrs> {
        SRAMHWERDIS_W::new(self, 12)
    }
    ///Bits 16:17 - Configuration of the FLEXMEM I-TCM extension
    #[inline(always)]
    pub fn itcmcfg(&mut self) -> ITCMCFG_W<'_, FLEXRAMCRrs> {
        ITCMCFG_W::new(self, 16)
    }
    ///Bit 24 - Configuration of the FLEXMEM D-TCM extension
    #[inline(always)]
    pub fn dtcmcfg(&mut self) -> DTCMCFG_W<'_, FLEXRAMCRrs> {
        DTCMCFG_W::new(self, 24)
    }
}
/**RAMCFG FLEXRAM control register

You can [`read`](crate::Reg::read) this register and get [`flexramcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flexramcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#RAMCFG:FLEXRAMCR)*/
pub struct FLEXRAMCRrs;
impl crate::RegisterSpec for FLEXRAMCRrs {
    type Ux = u32;
}
///`read()` method returns [`flexramcr::R`](R) reader structure
impl crate::Readable for FLEXRAMCRrs {}
///`write(|w| ..)` method takes [`flexramcr::W`](W) writer structure
impl crate::Writable for FLEXRAMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets FLEXRAMCR to value 0
impl crate::Resettable for FLEXRAMCRrs {}
