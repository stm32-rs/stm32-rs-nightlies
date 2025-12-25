///Register `CLCR` reader
pub type R = crate::R<CLCRrs>;
///Register `CLCR` writer
pub type W = crate::W<CLCRrs>;
///Field `DPCC` reader - DPCC
pub type DPCC_R = crate::BitReader;
///Field `DPCC` writer - DPCC
pub type DPCC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ACR` reader - ACR
pub type ACR_R = crate::BitReader;
///Field `ACR` writer - ACR
pub type ACR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - DPCC
    #[inline(always)]
    pub fn dpcc(&self) -> DPCC_R {
        DPCC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ACR
    #[inline(always)]
    pub fn acr(&self) -> ACR_R {
        ACR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLCR")
            .field("dpcc", &self.dpcc())
            .field("acr", &self.acr())
            .finish()
    }
}
impl W {
    ///Bit 0 - DPCC
    #[inline(always)]
    pub fn dpcc(&mut self) -> DPCC_W<'_, CLCRrs> {
        DPCC_W::new(self, 0)
    }
    ///Bit 1 - ACR
    #[inline(always)]
    pub fn acr(&mut self) -> ACR_W<'_, CLCRrs> {
        ACR_W::new(self, 1)
    }
}
/**DSI Host clock lane configuration register

You can [`read`](crate::Reg::read) this register and get [`clcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#DSI:CLCR)*/
pub struct CLCRrs;
impl crate::RegisterSpec for CLCRrs {
    type Ux = u32;
}
///`read()` method returns [`clcr::R`](R) reader structure
impl crate::Readable for CLCRrs {}
///`write(|w| ..)` method takes [`clcr::W`](W) writer structure
impl crate::Writable for CLCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLCR to value 0
impl crate::Resettable for CLCRrs {}
