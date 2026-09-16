///Register `DMACR` reader
pub type R = crate::R<DMACRrs>;
///Register `DMACR` writer
pub type W = crate::W<DMACRrs>;
///Field `DIEN` reader - DMA input enable
pub type DIEN_R = crate::BitReader;
///Field `DIEN` writer - DMA input enable
pub type DIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DOEN` reader - DMA output enable
pub type DOEN_R = crate::BitReader;
///Field `DOEN` writer - DMA output enable
pub type DOEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DMA input enable
    #[inline(always)]
    pub fn dien(&self) -> DIEN_R {
        DIEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - DMA output enable
    #[inline(always)]
    pub fn doen(&self) -> DOEN_R {
        DOEN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMACR")
            .field("doen", &self.doen())
            .field("dien", &self.dien())
            .finish()
    }
}
impl W {
    ///Bit 0 - DMA input enable
    #[inline(always)]
    pub fn dien(&mut self) -> DIEN_W<'_, DMACRrs> {
        DIEN_W::new(self, 0)
    }
    ///Bit 1 - DMA output enable
    #[inline(always)]
    pub fn doen(&mut self) -> DOEN_W<'_, DMACRrs> {
        DOEN_W::new(self, 1)
    }
}
/**DMA control register

You can [`read`](crate::Reg::read) this register and get [`dmacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#CRYP:DMACR)*/
pub struct DMACRrs;
impl crate::RegisterSpec for DMACRrs {
    type Ux = u32;
}
///`read()` method returns [`dmacr::R`](R) reader structure
impl crate::Readable for DMACRrs {}
///`write(|w| ..)` method takes [`dmacr::W`](W) writer structure
impl crate::Writable for DMACRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets DMACR to value 0
impl crate::Resettable for DMACRrs {}
