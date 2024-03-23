#[doc = "Register `IFCR` writer"]
pub type W = crate::W<IFCRrs>;
#[doc = "end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOTCW {
    #[doc = "1: Clear interrupt flag"]
    Clear = 1,
}
impl From<EOTCW> for bool {
    #[inline(always)]
    fn from(variant: EOTCW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOTC` writer - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register"]
pub type EOTC_W<'a, REG> = crate::BitWriter<'a, REG, EOTCW>;
impl<'a, REG> EOTC_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear interrupt flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(EOTCW::Clear)
    }
}
#[doc = "Field `TXTFC` writer - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register"]
pub use EOTC_W as TXTFC_W;
#[doc = "Field `UDRC` writer - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register"]
pub use EOTC_W as UDRC_W;
#[doc = "Field `OVRC` writer - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register"]
pub use EOTC_W as OVRC_W;
#[doc = "Field `CRCEC` writer - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register"]
pub use EOTC_W as CRCEC_W;
#[doc = "Field `TIFREC` writer - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register"]
pub use EOTC_W as TIFREC_W;
#[doc = "Field `MODFC` writer - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register"]
pub use EOTC_W as MODFC_W;
#[doc = "Field `SUSPC` writer - SUSPend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register"]
pub use EOTC_W as SUSPC_W;
impl W {
    #[doc = "Bit 3 - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn eotc(&mut self) -> EOTC_W<IFCRrs> {
        EOTC_W::new(self, 3)
    }
    #[doc = "Bit 4 - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn txtfc(&mut self) -> TXTFC_W<IFCRrs> {
        TXTFC_W::new(self, 4)
    }
    #[doc = "Bit 5 - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn udrc(&mut self) -> UDRC_W<IFCRrs> {
        UDRC_W::new(self, 5)
    }
    #[doc = "Bit 6 - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn ovrc(&mut self) -> OVRC_W<IFCRrs> {
        OVRC_W::new(self, 6)
    }
    #[doc = "Bit 7 - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn crcec(&mut self) -> CRCEC_W<IFCRrs> {
        CRCEC_W::new(self, 7)
    }
    #[doc = "Bit 8 - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn tifrec(&mut self) -> TIFREC_W<IFCRrs> {
        TIFREC_W::new(self, 8)
    }
    #[doc = "Bit 9 - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn modfc(&mut self) -> MODFC_W<IFCRrs> {
        MODFC_W::new(self, 9)
    }
    #[doc = "Bit 11 - SUSPend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register"]
    #[inline(always)]
    #[must_use]
    pub fn suspc(&mut self) -> SUSPC_W<IFCRrs> {
        SUSPC_W::new(self, 11)
    }
}
#[doc = "SPI/I2S interrupt/status flags clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
