///Register `CLRFR` reader
pub type R = crate::R<CLRFRrs>;
///Register `CLRFR` writer
pub type W = crate::W<CLRFRrs>;
///Field `CPERF` reader - Clear the preamble error flag
pub type CPERF_R = crate::BitReader;
///Field `CPERF` writer - Clear the preamble error flag
pub type CPERF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CSERF` reader - Clear the start error flag
pub type CSERF_R = crate::BitReader;
///Field `CSERF` writer - Clear the start error flag
pub type CSERF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CTERF` reader - Clear the turnaround error flag
pub type CTERF_R = crate::BitReader;
///Field `CTERF` writer - Clear the turnaround error flag
pub type CTERF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Clear the preamble error flag
    #[inline(always)]
    pub fn cperf(&self) -> CPERF_R {
        CPERF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Clear the start error flag
    #[inline(always)]
    pub fn cserf(&self) -> CSERF_R {
        CSERF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Clear the turnaround error flag
    #[inline(always)]
    pub fn cterf(&self) -> CTERF_R {
        CTERF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLRFR")
            .field("cperf", &self.cperf())
            .field("cserf", &self.cserf())
            .field("cterf", &self.cterf())
            .finish()
    }
}
impl W {
    ///Bit 0 - Clear the preamble error flag
    #[inline(always)]
    pub fn cperf(&mut self) -> CPERF_W<'_, CLRFRrs> {
        CPERF_W::new(self, 0)
    }
    ///Bit 1 - Clear the start error flag
    #[inline(always)]
    pub fn cserf(&mut self) -> CSERF_W<'_, CLRFRrs> {
        CSERF_W::new(self, 1)
    }
    ///Bit 2 - Clear the turnaround error flag
    #[inline(always)]
    pub fn cterf(&mut self) -> CTERF_W<'_, CLRFRrs> {
        CTERF_W::new(self, 2)
    }
}
/**MDIOS clear flag register

You can [`read`](crate::Reg::read) this register and get [`clrfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#MDIOS:CLRFR)*/
pub struct CLRFRrs;
impl crate::RegisterSpec for CLRFRrs {
    type Ux = u32;
}
///`read()` method returns [`clrfr::R`](R) reader structure
impl crate::Readable for CLRFRrs {}
///`write(|w| ..)` method takes [`clrfr::W`](W) writer structure
impl crate::Writable for CLRFRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CLRFR to value 0
impl crate::Resettable for CLRFRrs {}
