///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CECEN` reader - CEC Enable The CECEN bit is set and cleared by software. CECEN=1 starts message reception and enables the TXSOM control. CECEN=0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission.
pub type CECEN_R = crate::BitReader;
///Field `CECEN` writer - CEC Enable The CECEN bit is set and cleared by software. CECEN=1 starts message reception and enables the TXSOM control. CECEN=0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission.
pub type CECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSOM` reader - Tx Start Of Message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-Bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission will start after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND=1), in case of transmission underrun (TXUDR=1), negative acknowledge (TXACKE=1), and transmission error (TXERR=1). It is also cleared by CECEN=0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST=1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN=1 TXSOM must be set when transmission data is available into TXDR HEADERs first four bits containing own peripheral address are taken from TXDR\[7:4\], not from CEC_CFGR.OAR which is used only for reception
pub type TXSOM_R = crate::BitReader;
///Field `TXSOM` writer - Tx Start Of Message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-Bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission will start after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND=1), in case of transmission underrun (TXUDR=1), negative acknowledge (TXACKE=1), and transmission error (TXERR=1). It is also cleared by CECEN=0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST=1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN=1 TXSOM must be set when transmission data is available into TXDR HEADERs first four bits containing own peripheral address are taken from TXDR\[7:4\], not from CEC_CFGR.OAR which is used only for reception
pub type TXSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEOM` reader - Tx End Of Message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN=1 TXEOM must be set before writing transmission data to TXDR If TXEOM is set when TXSOM=0, transmitted message will consist of 1 byte (HEADER) only (PING message)
pub type TXEOM_R = crate::BitReader;
///Field `TXEOM` writer - Tx End Of Message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN=1 TXEOM must be set before writing transmission data to TXDR If TXEOM is set when TXSOM=0, transmitted message will consist of 1 byte (HEADER) only (PING message)
pub type TXEOM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CEC Enable The CECEN bit is set and cleared by software. CECEN=1 starts message reception and enables the TXSOM control. CECEN=0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission.
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx Start Of Message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-Bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission will start after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND=1), in case of transmission underrun (TXUDR=1), negative acknowledge (TXACKE=1), and transmission error (TXERR=1). It is also cleared by CECEN=0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST=1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN=1 TXSOM must be set when transmission data is available into TXDR HEADERs first four bits containing own peripheral address are taken from TXDR\[7:4\], not from CEC_CFGR.OAR which is used only for reception
    #[inline(always)]
    pub fn txsom(&self) -> TXSOM_R {
        TXSOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tx End Of Message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN=1 TXEOM must be set before writing transmission data to TXDR If TXEOM is set when TXSOM=0, transmitted message will consist of 1 byte (HEADER) only (PING message)
    #[inline(always)]
    pub fn txeom(&self) -> TXEOM_R {
        TXEOM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("cecen", &self.cecen())
            .field("txsom", &self.txsom())
            .field("txeom", &self.txeom())
            .finish()
    }
}
impl W {
    ///Bit 0 - CEC Enable The CECEN bit is set and cleared by software. CECEN=1 starts message reception and enables the TXSOM control. CECEN=0 disables the CEC peripheral, clears all bits of CEC_CR register and aborts any on-going reception or transmission.
    #[inline(always)]
    pub fn cecen(&mut self) -> CECEN_W<'_, CRrs> {
        CECEN_W::new(self, 0)
    }
    ///Bit 1 - Tx Start Of Message TXSOM is set by software to command transmission of the first byte of a CEC message. If the CEC message consists of only one byte, TXEOM must be set before of TXSOM. Start-Bit is effectively started on the CEC line after SFT is counted. If TXSOM is set while a message reception is ongoing, transmission will start after the end of reception. TXSOM is cleared by hardware after the last byte of the message is sent with a positive acknowledge (TXEND=1), in case of transmission underrun (TXUDR=1), negative acknowledge (TXACKE=1), and transmission error (TXERR=1). It is also cleared by CECEN=0. It is not cleared and transmission is automatically retried in case of arbitration lost (ARBLST=1). TXSOM can be also used as a status bit informing application whether any transmission request is pending or under execution. The application can abort a transmission request at any time by clearing the CECEN bit. Note: TXSOM must be set when CECEN=1 TXSOM must be set when transmission data is available into TXDR HEADERs first four bits containing own peripheral address are taken from TXDR\[7:4\], not from CEC_CFGR.OAR which is used only for reception
    #[inline(always)]
    pub fn txsom(&mut self) -> TXSOM_W<'_, CRrs> {
        TXSOM_W::new(self, 1)
    }
    ///Bit 2 - Tx End Of Message The TXEOM bit is set by software to command transmission of the last byte of a CEC message. TXEOM is cleared by hardware at the same time and under the same conditions as for TXSOM. Note: TXEOM must be set when CECEN=1 TXEOM must be set before writing transmission data to TXDR If TXEOM is set when TXSOM=0, transmitted message will consist of 1 byte (HEADER) only (PING message)
    #[inline(always)]
    pub fn txeom(&mut self) -> TXEOM_W<'_, CRrs> {
        TXEOM_W::new(self, 2)
    }
}
/**CEC control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H745_CM4.html#CEC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
