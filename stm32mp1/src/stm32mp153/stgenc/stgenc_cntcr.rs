///Register `STGENC_CNTCR` reader
pub type R = crate::R<STGENC_CNTCRrs>;
///Register `STGENC_CNTCR` writer
pub type W = crate::W<STGENC_CNTCRrs>;
///Field `EN` reader - EN
pub type EN_R = crate::BitReader;
///Field `EN` writer - EN
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HLTDBG` reader - HLTDBG
pub type HLTDBG_R = crate::BitReader;
///Field `HLTDBG` writer - HLTDBG
pub type HLTDBG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - HLTDBG
    #[inline(always)]
    pub fn hltdbg(&self) -> HLTDBG_R {
        HLTDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STGENC_CNTCR")
            .field("en", &self.en())
            .field("hltdbg", &self.hltdbg())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<STGENC_CNTCRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - HLTDBG
    #[inline(always)]
    #[must_use]
    pub fn hltdbg(&mut self) -> HLTDBG_W<STGENC_CNTCRrs> {
        HLTDBG_W::new(self, 1)
    }
}
/**STGENC control register

You can [`read`](crate::Reg::read) this register and get [`stgenc_cntcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stgenc_cntcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#STGENC:STGENC_CNTCR)*/
pub struct STGENC_CNTCRrs;
impl crate::RegisterSpec for STGENC_CNTCRrs {
    type Ux = u32;
}
///`read()` method returns [`stgenc_cntcr::R`](R) reader structure
impl crate::Readable for STGENC_CNTCRrs {}
///`write(|w| ..)` method takes [`stgenc_cntcr::W`](W) writer structure
impl crate::Writable for STGENC_CNTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets STGENC_CNTCR to value 0
impl crate::Resettable for STGENC_CNTCRrs {
    const RESET_VALUE: u32 = 0;
}
