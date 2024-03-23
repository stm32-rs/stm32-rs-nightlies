#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCRrs>;
#[doc = "Field `PERRCF` writer - Clears the Parity error flag"]
pub type PERRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRCF` writer - Clears the Overrun error flag"]
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBDCF` writer - Clears the Synchronization Block Detected flag"]
pub type SBDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDCF` writer - Clears the Synchronization Done flag"]
pub type SYNCDCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Clears the Parity error flag"]
    #[inline(always)]
    #[must_use]
    pub fn perrcf(&mut self) -> PERRCF_W<IFCRrs> {
        PERRCF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clears the Overrun error flag"]
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<IFCRrs> {
        OVRCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clears the Synchronization Block Detected flag"]
    #[inline(always)]
    #[must_use]
    pub fn sbdcf(&mut self) -> SBDCF_W<IFCRrs> {
        SBDCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clears the Synchronization Done flag"]
    #[inline(always)]
    #[must_use]
    pub fn syncdcf(&mut self) -> SYNCDCF_W<IFCRrs> {
        SYNCDCF_W::new(self, 5)
    }
}
#[doc = "Interrupt Flag Clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ifcr::W`](W) writer structure"]
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IFCR to value 0"]
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}
