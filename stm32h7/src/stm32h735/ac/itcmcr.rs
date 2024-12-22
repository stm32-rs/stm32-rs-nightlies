///Register `ITCMCR` reader
pub type R = crate::R<ITCMCRrs>;
///Register `ITCMCR` writer
pub type W = crate::W<ITCMCRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RMW` reader - RMW
pub type RMW_R = crate::BitReader;
///Field `RMW` writer - RMW
pub type RMW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RETEN` reader - RETEN
pub type RETEN_R = crate::BitReader;
///Field `RETEN` writer - RETEN
pub type RETEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SZ` reader - SZ
pub type SZ_R = crate::FieldReader;
///Field `SZ` writer - SZ
pub type SZ_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - RMW
    #[inline(always)]
    pub fn rmw(&self) -> RMW_R {
        RMW_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RETEN
    #[inline(always)]
    pub fn reten(&self) -> RETEN_R {
        RETEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bits 3:6 - SZ
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ITCMCR")
            .field("en", &self.en())
            .field("rmw", &self.rmw())
            .field("reten", &self.reten())
            .field("sz", &self.sz())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<ITCMCRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - RMW
    #[inline(always)]
    pub fn rmw(&mut self) -> RMW_W<ITCMCRrs> {
        RMW_W::new(self, 1)
    }
    ///Bit 2 - RETEN
    #[inline(always)]
    pub fn reten(&mut self) -> RETEN_W<ITCMCRrs> {
        RETEN_W::new(self, 2)
    }
    ///Bits 3:6 - SZ
    #[inline(always)]
    pub fn sz(&mut self) -> SZ_W<ITCMCRrs> {
        SZ_W::new(self, 3)
    }
}
/**Instruction and Data Tightly-Coupled Memory Control Registers

You can [`read`](crate::Reg::read) this register and get [`itcmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`itcmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H73x.html#AC:ITCMCR)*/
pub struct ITCMCRrs;
impl crate::RegisterSpec for ITCMCRrs {
    type Ux = u32;
}
///`read()` method returns [`itcmcr::R`](R) reader structure
impl crate::Readable for ITCMCRrs {}
///`write(|w| ..)` method takes [`itcmcr::W`](W) writer structure
impl crate::Writable for ITCMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ITCMCR to value 0
impl crate::Resettable for ITCMCRrs {
    const RESET_VALUE: u32 = 0;
}
