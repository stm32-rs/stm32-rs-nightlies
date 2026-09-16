///Register `CLRFR` reader
pub type R = crate::R<CLRFRrs>;
///Register `CLRFR` writer
pub type W = crate::W<CLRFRrs>;
///Field `PROCENDFC` reader - Clear PKA End of Operation flag - 0: No action - 1: Clear the PROCENDF flag
pub type PROCENDFC_R = crate::BitReader;
///Field `PROCENDFC` writer - Clear PKA End of Operation flag - 0: No action - 1: Clear the PROCENDF flag
pub type PROCENDFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMERRFC` reader - Clear PKA RAM error flag - 0: No action - 1: Clear the RAMERRF flag Bits 18 Reserved, must be kept at zero
pub type RAMERRFC_R = crate::BitReader;
///Field `RAMERRFC` writer - Clear PKA RAM error flag - 0: No action - 1: Clear the RAMERRF flag Bits 18 Reserved, must be kept at zero
pub type RAMERRFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDRERRFC` reader - Clear Address error flag - 0: No action - 1: Clear the ADDRERRF flag
pub type ADDRERRFC_R = crate::BitReader;
///Field `ADDRERRFC` writer - Clear Address error flag - 0: No action - 1: Clear the ADDRERRF flag
pub type ADDRERRFC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 17 - Clear PKA End of Operation flag - 0: No action - 1: Clear the PROCENDF flag
    #[inline(always)]
    pub fn procendfc(&self) -> PROCENDFC_R {
        PROCENDFC_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - Clear PKA RAM error flag - 0: No action - 1: Clear the RAMERRF flag Bits 18 Reserved, must be kept at zero
    #[inline(always)]
    pub fn ramerrfc(&self) -> RAMERRFC_R {
        RAMERRFC_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - Clear Address error flag - 0: No action - 1: Clear the ADDRERRF flag
    #[inline(always)]
    pub fn addrerrfc(&self) -> ADDRERRFC_R {
        ADDRERRFC_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLRFR")
            .field("procendfc", &self.procendfc())
            .field("ramerrfc", &self.ramerrfc())
            .field("addrerrfc", &self.addrerrfc())
            .finish()
    }
}
impl W {
    ///Bit 17 - Clear PKA End of Operation flag - 0: No action - 1: Clear the PROCENDF flag
    #[inline(always)]
    pub fn procendfc(&mut self) -> PROCENDFC_W<'_, CLRFRrs> {
        PROCENDFC_W::new(self, 17)
    }
    ///Bit 19 - Clear PKA RAM error flag - 0: No action - 1: Clear the RAMERRF flag Bits 18 Reserved, must be kept at zero
    #[inline(always)]
    pub fn ramerrfc(&mut self) -> RAMERRFC_W<'_, CLRFRrs> {
        RAMERRFC_W::new(self, 19)
    }
    ///Bit 20 - Clear Address error flag - 0: No action - 1: Clear the ADDRERRF flag
    #[inline(always)]
    pub fn addrerrfc(&mut self) -> ADDRERRFC_W<'_, CLRFRrs> {
        ADDRERRFC_W::new(self, 20)
    }
}
/**PKA_CLRFR register

You can [`read`](crate::Reg::read) this register and get [`clrfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB05.html#PKA:CLRFR)*/
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
