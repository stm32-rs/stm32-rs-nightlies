///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
///Field `EOTC` writer - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register
pub type EOTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTFC` writer - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register
pub type TXTFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDRC` writer - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register
pub type UDRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRC` writer - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register
pub type OVRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEC` writer - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register
pub type CRCEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIFREC` writer - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register
pub type TIFREC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODFC` writer - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register
pub type MODFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPC` writer - Suspend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register
pub type SUSPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - end of transfer flag clear Writing a 1 into this bit clears EOT flag in the SPI_SR register
    #[inline(always)]
    pub fn eotc(&mut self) -> EOTC_W<IFCRrs> {
        EOTC_W::new(self, 3)
    }
    ///Bit 4 - transmission transfer filled flag clear Writing a 1 into this bit clears TXTF flag in the SPI_SR register
    #[inline(always)]
    pub fn txtfc(&mut self) -> TXTFC_W<IFCRrs> {
        TXTFC_W::new(self, 4)
    }
    ///Bit 5 - underrun flag clear Writing a 1 into this bit clears UDR flag in the SPI_SR register
    #[inline(always)]
    pub fn udrc(&mut self) -> UDRC_W<IFCRrs> {
        UDRC_W::new(self, 5)
    }
    ///Bit 6 - overrun flag clear Writing a 1 into this bit clears OVR flag in the SPI_SR register
    #[inline(always)]
    pub fn ovrc(&mut self) -> OVRC_W<IFCRrs> {
        OVRC_W::new(self, 6)
    }
    ///Bit 7 - CRC error flag clear Writing a 1 into this bit clears CRCE flag in the SPI_SR register
    #[inline(always)]
    pub fn crcec(&mut self) -> CRCEC_W<IFCRrs> {
        CRCEC_W::new(self, 7)
    }
    ///Bit 8 - TI frame format error flag clear Writing a 1 into this bit clears TIFRE flag in the SPI_SR register
    #[inline(always)]
    pub fn tifrec(&mut self) -> TIFREC_W<IFCRrs> {
        TIFREC_W::new(self, 8)
    }
    ///Bit 9 - mode fault flag clear Writing a 1 into this bit clears MODF flag in the SPI_SR register
    #[inline(always)]
    pub fn modfc(&mut self) -> MODFC_W<IFCRrs> {
        MODFC_W::new(self, 9)
    }
    ///Bit 11 - Suspend flag clear Writing a 1 into this bit clears SUSP flag in the SPI_SR register
    #[inline(always)]
    pub fn suspc(&mut self) -> SUSPC_W<IFCRrs> {
        SUSPC_W::new(self, 11)
    }
}
/**SPI/I2S interrupt/status flags clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#SPI1:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {
    const RESET_VALUE: u32 = 0;
}