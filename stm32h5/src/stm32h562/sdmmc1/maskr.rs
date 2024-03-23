#[doc = "Register `MASKR` reader"]
pub type R = crate::R<MASKRrs>;
#[doc = "Register `MASKR` writer"]
pub type W = crate::W<MASKRrs>;
#[doc = "Field `CCRCFAILIE` reader - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub type CCRCFAILIE_R = crate::BitReader;
#[doc = "Field `CCRCFAILIE` writer - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub type CCRCFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILIE` reader - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub type DCRCFAILIE_R = crate::BitReader;
#[doc = "Field `DCRCFAILIE` writer - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub type DCRCFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTIE` reader - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub type CTIMEOUTIE_R = crate::BitReader;
#[doc = "Field `CTIMEOUTIE` writer - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub type CTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTIE` reader - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub type DTIMEOUTIE_R = crate::BitReader;
#[doc = "Field `DTIMEOUTIE` writer - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub type DTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRIE` reader - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub type TXUNDERRIE_R = crate::BitReader;
#[doc = "Field `TXUNDERRIE` writer - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub type TXUNDERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRIE` reader - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub type RXOVERRIE_R = crate::BitReader;
#[doc = "Field `RXOVERRIE` writer - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub type RXOVERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDIE` reader - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub type CMDRENDIE_R = crate::BitReader;
#[doc = "Field `CMDRENDIE` writer - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub type CMDRENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTIE` reader - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub type CMDSENTIE_R = crate::BitReader;
#[doc = "Field `CMDSENTIE` writer - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub type CMDSENTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDIE` reader - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub type DATAENDIE_R = crate::BitReader;
#[doc = "Field `DATAENDIE` writer - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub type DATAENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHOLDIE` reader - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
pub type DHOLDIE_R = crate::BitReader;
#[doc = "Field `DHOLDIE` writer - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
pub type DHOLDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDIE` reader - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub type DBCKENDIE_R = crate::BitReader;
#[doc = "Field `DBCKENDIE` writer - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub type DBCKENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DABORTIE` reader - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub type DABORTIE_R = crate::BitReader;
#[doc = "Field `DABORTIE` writer - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub type DABORTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOHEIE` reader - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub type TXFIFOHEIE_R = crate::BitReader;
#[doc = "Field `TXFIFOHEIE` writer - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub type TXFIFOHEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOHFIE` reader - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub type RXFIFOHFIE_R = crate::BitReader;
#[doc = "Field `RXFIFOHFIE` writer - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub type RXFIFOHFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOFIE` reader - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub type RXFIFOFIE_R = crate::BitReader;
#[doc = "Field `RXFIFOFIE` writer - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub type RXFIFOFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOEIE` reader - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub type TXFIFOEIE_R = crate::BitReader;
#[doc = "Field `TXFIFOEIE` writer - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub type TXFIFOEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSYD0ENDIE` reader - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub type BUSYD0ENDIE_R = crate::BitReader;
#[doc = "Field `BUSYD0ENDIE` writer - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub type BUSYD0ENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITIE` reader - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub type SDIOITIE_R = crate::BitReader;
#[doc = "Field `SDIOITIE` writer - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub type SDIOITIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILIE` reader - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
pub type ACKFAILIE_R = crate::BitReader;
#[doc = "Field `ACKFAILIE` writer - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
pub type ACKFAILIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKTIMEOUTIE` reader - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
pub type ACKTIMEOUTIE_R = crate::BitReader;
#[doc = "Field `ACKTIMEOUTIE` writer - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
pub type ACKTIMEOUTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWENDIE` reader - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
pub type VSWENDIE_R = crate::BitReader;
#[doc = "Field `VSWENDIE` writer - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
pub type VSWENDIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSTOPIE` reader - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
pub type CKSTOPIE_R = crate::BitReader;
#[doc = "Field `CKSTOPIE` writer - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
pub type CKSTOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMABTCIE` reader - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
pub type IDMABTCIE_R = crate::BitReader;
#[doc = "Field `IDMABTCIE` writer - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
pub type IDMABTCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CCRCFAILIE_R {
        CCRCFAILIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DCRCFAILIE_R {
        DCRCFAILIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CTIMEOUTIE_R {
        CTIMEOUTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DTIMEOUTIE_R {
        DTIMEOUTIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrie(&self) -> TXUNDERRIE_R {
        TXUNDERRIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RXOVERRIE_R {
        RXOVERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CMDRENDIE_R {
        CMDRENDIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CMDSENTIE_R {
        CMDSENTIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    pub fn dataendie(&self) -> DATAENDIE_R {
        DATAENDIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
    #[inline(always)]
    pub fn dholdie(&self) -> DHOLDIE_R {
        DHOLDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    pub fn dbckendie(&self) -> DBCKENDIE_R {
        DBCKENDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    pub fn dabortie(&self) -> DABORTIE_R {
        DABORTIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TXFIFOHEIE_R {
        TXFIFOHEIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RXFIFOHFIE_R {
        RXFIFOHFIE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RXFIFOFIE_R {
        RXFIFOFIE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TXFIFOEIE_R {
        TXFIFOEIE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    pub fn busyd0endie(&self) -> BUSYD0ENDIE_R {
        BUSYD0ENDIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    pub fn sdioitie(&self) -> SDIOITIE_R {
        SDIOITIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
    #[inline(always)]
    pub fn ackfailie(&self) -> ACKFAILIE_R {
        ACKFAILIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
    #[inline(always)]
    pub fn acktimeoutie(&self) -> ACKTIMEOUTIE_R {
        ACKTIMEOUTIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
    #[inline(always)]
    pub fn vswendie(&self) -> VSWENDIE_R {
        VSWENDIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
    #[inline(always)]
    pub fn ckstopie(&self) -> CKSTOPIE_R {
        CKSTOPIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
    #[inline(always)]
    pub fn idmabtcie(&self) -> IDMABTCIE_R {
        IDMABTCIE_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    #[must_use]
    pub fn ccrcfailie(&mut self) -> CCRCFAILIE_W<MASKRrs> {
        CCRCFAILIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    #[must_use]
    pub fn dcrcfailie(&mut self) -> DCRCFAILIE_W<MASKRrs> {
        DCRCFAILIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    #[must_use]
    pub fn ctimeoutie(&mut self) -> CTIMEOUTIE_W<MASKRrs> {
        CTIMEOUTIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    #[must_use]
    pub fn dtimeoutie(&mut self) -> DTIMEOUTIE_W<MASKRrs> {
        DTIMEOUTIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    #[must_use]
    pub fn txunderrie(&mut self) -> TXUNDERRIE_W<MASKRrs> {
        TXUNDERRIE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    #[must_use]
    pub fn rxoverrie(&mut self) -> RXOVERRIE_W<MASKRrs> {
        RXOVERRIE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    #[must_use]
    pub fn cmdrendie(&mut self) -> CMDRENDIE_W<MASKRrs> {
        CMDRENDIE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    #[must_use]
    pub fn cmdsentie(&mut self) -> CMDSENTIE_W<MASKRrs> {
        CMDSENTIE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    #[must_use]
    pub fn dataendie(&mut self) -> DATAENDIE_W<MASKRrs> {
        DATAENDIE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
    #[inline(always)]
    #[must_use]
    pub fn dholdie(&mut self) -> DHOLDIE_W<MASKRrs> {
        DHOLDIE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    #[must_use]
    pub fn dbckendie(&mut self) -> DBCKENDIE_W<MASKRrs> {
        DBCKENDIE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    #[must_use]
    pub fn dabortie(&mut self) -> DABORTIE_W<MASKRrs> {
        DABORTIE_W::new(self, 11)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    #[must_use]
    pub fn txfifoheie(&mut self) -> TXFIFOHEIE_W<MASKRrs> {
        TXFIFOHEIE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifohfie(&mut self) -> RXFIFOHFIE_W<MASKRrs> {
        RXFIFOHFIE_W::new(self, 15)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifofie(&mut self) -> RXFIFOFIE_W<MASKRrs> {
        RXFIFOFIE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    #[must_use]
    pub fn txfifoeie(&mut self) -> TXFIFOEIE_W<MASKRrs> {
        TXFIFOEIE_W::new(self, 18)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    #[must_use]
    pub fn busyd0endie(&mut self) -> BUSYD0ENDIE_W<MASKRrs> {
        BUSYD0ENDIE_W::new(self, 21)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sdioitie(&mut self) -> SDIOITIE_W<MASKRrs> {
        SDIOITIE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
    #[inline(always)]
    #[must_use]
    pub fn ackfailie(&mut self) -> ACKFAILIE_W<MASKRrs> {
        ACKFAILIE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
    #[inline(always)]
    #[must_use]
    pub fn acktimeoutie(&mut self) -> ACKTIMEOUTIE_W<MASKRrs> {
        ACKTIMEOUTIE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
    #[inline(always)]
    #[must_use]
    pub fn vswendie(&mut self) -> VSWENDIE_W<MASKRrs> {
        VSWENDIE_W::new(self, 25)
    }
    #[doc = "Bit 26 - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
    #[inline(always)]
    #[must_use]
    pub fn ckstopie(&mut self) -> CKSTOPIE_W<MASKRrs> {
        CKSTOPIE_W::new(self, 26)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
    #[inline(always)]
    #[must_use]
    pub fn idmabtcie(&mut self) -> IDMABTCIE_W<MASKRrs> {
        IDMABTCIE_W::new(self, 28)
    }
}
#[doc = "SDMMC mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maskr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maskr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MASKRrs;
impl crate::RegisterSpec for MASKRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskr::R`](R) reader structure"]
impl crate::Readable for MASKRrs {}
#[doc = "`write(|w| ..)` method takes [`maskr::W`](W) writer structure"]
impl crate::Writable for MASKRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MASKR to value 0"]
impl crate::Resettable for MASKRrs {
    const RESET_VALUE: u32 = 0;
}
