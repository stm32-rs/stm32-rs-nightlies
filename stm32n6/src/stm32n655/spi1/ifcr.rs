///Register `IFCR` writer
pub type W = crate::W<IFCRrs>;
///Field `EOTC` writer - end of transfer flag clear
pub type EOTC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXTFC` writer - transmission transfer filled flag clear
pub type TXTFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UDRC` writer - underrun flag clear
pub type UDRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRC` writer - overrun flag clear
pub type OVRC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRCEC` writer - CRC error flag clear
pub type CRCEC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIFREC` writer - TI frame format error flag clear
pub type TIFREC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODFC` writer - mode fault flag clear
pub type MODFC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SUSPC` writer - Suspend flag clear
pub type SUSPC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<IFCRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - end of transfer flag clear
    #[inline(always)]
    pub fn eotc(&mut self) -> EOTC_W<'_, IFCRrs> {
        EOTC_W::new(self, 3)
    }
    ///Bit 4 - transmission transfer filled flag clear
    #[inline(always)]
    pub fn txtfc(&mut self) -> TXTFC_W<'_, IFCRrs> {
        TXTFC_W::new(self, 4)
    }
    ///Bit 5 - underrun flag clear
    #[inline(always)]
    pub fn udrc(&mut self) -> UDRC_W<'_, IFCRrs> {
        UDRC_W::new(self, 5)
    }
    ///Bit 6 - overrun flag clear
    #[inline(always)]
    pub fn ovrc(&mut self) -> OVRC_W<'_, IFCRrs> {
        OVRC_W::new(self, 6)
    }
    ///Bit 7 - CRC error flag clear
    #[inline(always)]
    pub fn crcec(&mut self) -> CRCEC_W<'_, IFCRrs> {
        CRCEC_W::new(self, 7)
    }
    ///Bit 8 - TI frame format error flag clear
    #[inline(always)]
    pub fn tifrec(&mut self) -> TIFREC_W<'_, IFCRrs> {
        TIFREC_W::new(self, 8)
    }
    ///Bit 9 - mode fault flag clear
    #[inline(always)]
    pub fn modfc(&mut self) -> MODFC_W<'_, IFCRrs> {
        MODFC_W::new(self, 9)
    }
    ///Bit 11 - Suspend flag clear
    #[inline(always)]
    pub fn suspc(&mut self) -> SUSPC_W<'_, IFCRrs> {
        SUSPC_W::new(self, 11)
    }
}
/**SPI/I2S interrupt/status flags clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#SPI1:IFCR)*/
pub struct IFCRrs;
impl crate::RegisterSpec for IFCRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`ifcr::W`](W) writer structure
impl crate::Writable for IFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IFCR to value 0
impl crate::Resettable for IFCRrs {}
