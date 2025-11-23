///Register `LPCR` reader
pub type R = crate::R<LPCRrs>;
///Register `LPCR` writer
pub type W = crate::W<LPCRrs>;
///Field `DEP` reader - DEP
pub type DEP_R = crate::BitReader;
///Field `DEP` writer - DEP
pub type DEP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `VSP` reader - VSP
pub type VSP_R = crate::BitReader;
///Field `VSP` writer - VSP
pub type VSP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HSP` reader - HSP
pub type HSP_R = crate::BitReader;
///Field `HSP` writer - HSP
pub type HSP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DEP
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - VSP
    #[inline(always)]
    pub fn vsp(&self) -> VSP_R {
        VSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - HSP
    #[inline(always)]
    pub fn hsp(&self) -> HSP_R {
        HSP_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCR")
            .field("dep", &self.dep())
            .field("vsp", &self.vsp())
            .field("hsp", &self.hsp())
            .finish()
    }
}
impl W {
    ///Bit 0 - DEP
    #[inline(always)]
    pub fn dep(&mut self) -> DEP_W<'_, LPCRrs> {
        DEP_W::new(self, 0)
    }
    ///Bit 1 - VSP
    #[inline(always)]
    pub fn vsp(&mut self) -> VSP_W<'_, LPCRrs> {
        VSP_W::new(self, 1)
    }
    ///Bit 2 - HSP
    #[inline(always)]
    pub fn hsp(&mut self) -> HSP_W<'_, LPCRrs> {
        HSP_W::new(self, 2)
    }
}
/**DSI Host LTDC polarity configuration register

You can [`read`](crate::Reg::read) this register and get [`lpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DSI:LPCR)*/
pub struct LPCRrs;
impl crate::RegisterSpec for LPCRrs {
    type Ux = u32;
}
///`read()` method returns [`lpcr::R`](R) reader structure
impl crate::Readable for LPCRrs {}
///`write(|w| ..)` method takes [`lpcr::W`](W) writer structure
impl crate::Writable for LPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets LPCR to value 0
impl crate::Resettable for LPCRrs {}
