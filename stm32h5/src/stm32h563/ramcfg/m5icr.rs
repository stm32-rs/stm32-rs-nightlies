#[doc = "Register `M5ICR` reader"]
pub type R = crate::R<M5ICRrs>;
#[doc = "Register `M5ICR` writer"]
pub type W = crate::W<M5ICRrs>;
#[doc = "Field `CSEDC` reader - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
pub type CSEDC_R = crate::BitReader;
#[doc = "Field `CSEDC` writer - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
pub type CSEDC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDED` reader - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
pub type CDED_R = crate::BitReader;
#[doc = "Field `CDED` writer - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
pub type CDED_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
    #[inline(always)]
    pub fn csedc(&self) -> CSEDC_R {
        CSEDC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
    #[inline(always)]
    pub fn cded(&self) -> CDED_R {
        CDED_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear ECC single error detected and corrected Writing 1 to this flag clears the SEDC bit in the RAMCFG_MxISR register. Reading this flag returns the SEDC value."]
    #[inline(always)]
    #[must_use]
    pub fn csedc(&mut self) -> CSEDC_W<M5ICRrs> {
        CSEDC_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear ECC double error detected Writing 1 to this flag clears the DED bit in the RAMCFG_MxISR register. Reading this flag returns the DED value."]
    #[inline(always)]
    #[must_use]
    pub fn cded(&mut self) -> CDED_W<M5ICRrs> {
        CDED_W::new(self, 1)
    }
}
#[doc = "RAMCFG memory 5 interrupt clear register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m5icr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`m5icr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5ICRrs;
impl crate::RegisterSpec for M5ICRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5icr::R`](R) reader structure"]
impl crate::Readable for M5ICRrs {}
#[doc = "`write(|w| ..)` method takes [`m5icr::W`](W) writer structure"]
impl crate::Writable for M5ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets M5ICR to value 0"]
impl crate::Resettable for M5ICRrs {
    const RESET_VALUE: u32 = 0;
}
