#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCRrs>;
#[doc = "Field `EOTC` writer - End Of Transfer flag clear"]
pub type EOTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXTFC` writer - Transmission Transfer Filled flag clear"]
pub type TXTFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UDRC` writer - Underrun flag clear"]
pub type UDRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRC` writer - Overrun flag clear"]
pub type OVRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEC` writer - CRC Error flag clear"]
pub type CRCEC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIFREC` writer - TI frame format error flag clear"]
pub type TIFREC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODFC` writer - Mode Fault flag clear"]
pub type MODFC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUSPC` writer - SUSPend flag clear"]
pub type SUSPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 3 - End Of Transfer flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn eotc(&mut self) -> EOTC_W<IFCRrs> {
        EOTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - Transmission Transfer Filled flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn txtfc(&mut self) -> TXTFC_W<IFCRrs> {
        TXTFC_W::new(self, 4)
    }
    #[doc = "Bit 5 - Underrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn udrc(&mut self) -> UDRC_W<IFCRrs> {
        UDRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - Overrun flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn ovrc(&mut self) -> OVRC_W<IFCRrs> {
        OVRC_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC Error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn crcec(&mut self) -> CRCEC_W<IFCRrs> {
        CRCEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - TI frame format error flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn tifrec(&mut self) -> TIFREC_W<IFCRrs> {
        TIFREC_W::new(self, 8)
    }
    #[doc = "Bit 9 - Mode Fault flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn modfc(&mut self) -> MODFC_W<IFCRrs> {
        MODFC_W::new(self, 9)
    }
    #[doc = "Bit 11 - SUSPend flag clear"]
    #[inline(always)]
    #[must_use]
    pub fn suspc(&mut self) -> SUSPC_W<IFCRrs> {
        SUSPC_W::new(self, 11)
    }
}
#[doc = "Interrupt/Status Flags Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
