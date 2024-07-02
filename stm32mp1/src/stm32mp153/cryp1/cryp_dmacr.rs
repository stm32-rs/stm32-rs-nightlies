///Register `CRYP_DMACR` reader
pub type R = crate::R<CRYP_DMACRrs>;
///Register `CRYP_DMACR` writer
pub type W = crate::W<CRYP_DMACRrs>;
///Field `DIEN` reader - DIEN
pub type DIEN_R = crate::BitReader;
///Field `DIEN` writer - DIEN
pub type DIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOEN` reader - DOEN
pub type DOEN_R = crate::BitReader;
///Field `DOEN` writer - DOEN
pub type DOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DIEN
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DOEN
    #[inline(always)]
    pub fn doen(&self) -> DOEN_R {
        DOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYP_DMACR")
            .field("dien", &self.dien())
            .field("doen", &self.doen())
            .finish()
    }
}
impl W {
    ///Bit 0 - DIEN
    #[inline(always)]
    #[must_use]
    pub fn dien(&mut self) -> DIEN_W<CRYP_DMACRrs> {
        DIEN_W::new(self, 0)
    }
    ///Bit 1 - DOEN
    #[inline(always)]
    #[must_use]
    pub fn doen(&mut self) -> DOEN_W<CRYP_DMACRrs> {
        DOEN_W::new(self, 1)
    }
}
/**CRYP DMA control register

You can [`read`](crate::Reg::read) this register and get [`cryp_dmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cryp_dmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#CRYP1:CRYP_DMACR)*/
pub struct CRYP_DMACRrs;
impl crate::RegisterSpec for CRYP_DMACRrs {
    type Ux = u32;
}
///`read()` method returns [`cryp_dmacr::R`](R) reader structure
impl crate::Readable for CRYP_DMACRrs {}
///`write(|w| ..)` method takes [`cryp_dmacr::W`](W) writer structure
impl crate::Writable for CRYP_DMACRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CRYP_DMACR to value 0
impl crate::Resettable for CRYP_DMACRrs {
    const RESET_VALUE: u32 = 0;
}
