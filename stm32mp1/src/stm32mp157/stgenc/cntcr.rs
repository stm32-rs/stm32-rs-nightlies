///Register `CNTCR` reader
pub type R = crate::R<CNTCRrs>;
///Register `CNTCR` writer
pub type W = crate::W<CNTCRrs>;
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
        f.debug_struct("CNTCR")
            .field("en", &self.en())
            .field("hltdbg", &self.hltdbg())
            .finish()
    }
}
impl W {
    ///Bit 0 - EN
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CNTCRrs> {
        EN_W::new(self, 0)
    }
    ///Bit 1 - HLTDBG
    #[inline(always)]
    pub fn hltdbg(&mut self) -> HLTDBG_W<'_, CNTCRrs> {
        HLTDBG_W::new(self, 1)
    }
}
/**STGENC control register

You can [`read`](crate::Reg::read) this register and get [`cntcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#STGENC:CNTCR)*/
pub struct CNTCRrs;
impl crate::RegisterSpec for CNTCRrs {
    type Ux = u32;
}
///`read()` method returns [`cntcr::R`](R) reader structure
impl crate::Readable for CNTCRrs {}
///`write(|w| ..)` method takes [`cntcr::W`](W) writer structure
impl crate::Writable for CNTCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CNTCR to value 0
impl crate::Resettable for CNTCRrs {}
