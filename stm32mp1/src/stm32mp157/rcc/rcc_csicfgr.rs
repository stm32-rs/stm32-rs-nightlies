///Register `RCC_CSICFGR` reader
pub type R = crate::R<RCC_CSICFGRrs>;
///Register `RCC_CSICFGR` writer
pub type W = crate::W<RCC_CSICFGRrs>;
///Field `CSITRIM` reader - CSITRIM
pub type CSITRIM_R = crate::FieldReader;
///Field `CSITRIM` writer - CSITRIM
pub type CSITRIM_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `CSICAL` reader - CSICAL
pub type CSICAL_R = crate::FieldReader;
impl R {
    ///Bits 8:12 - CSITRIM
    #[inline(always)]
    pub fn csitrim(&self) -> CSITRIM_R {
        CSITRIM_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    ///Bits 16:23 - CSICAL
    #[inline(always)]
    pub fn csical(&self) -> CSICAL_R {
        CSICAL_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RCC_CSICFGR")
            .field("csitrim", &self.csitrim())
            .field("csical", &self.csical())
            .finish()
    }
}
impl W {
    ///Bits 8:12 - CSITRIM
    #[inline(always)]
    #[must_use]
    pub fn csitrim(&mut self) -> CSITRIM_W<RCC_CSICFGRrs> {
        CSITRIM_W::new(self, 8)
    }
}
/**This register is used to fine-tune the CSI frequency. If TZEN = MCKPROT = , this register can only be modified in secure mode. Write access to this register is not allowed during the clock restore sequence. See The clock restore sequence description for details.

You can [`read`](crate::Reg::read) this register and get [`rcc_csicfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rcc_csicfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#RCC:RCC_CSICFGR)*/
pub struct RCC_CSICFGRrs;
impl crate::RegisterSpec for RCC_CSICFGRrs {
    type Ux = u32;
}
///`read()` method returns [`rcc_csicfgr::R`](R) reader structure
impl crate::Readable for RCC_CSICFGRrs {}
///`write(|w| ..)` method takes [`rcc_csicfgr::W`](W) writer structure
impl crate::Writable for RCC_CSICFGRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets RCC_CSICFGR to value 0x1000
impl crate::Resettable for RCC_CSICFGRrs {
    const RESET_VALUE: u32 = 0x1000;
}
