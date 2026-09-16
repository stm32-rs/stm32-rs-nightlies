///Register `I2C_CR1` reader
pub type R = crate::R<I2C_CR1rs>;
///Register `I2C_CR1` writer
pub type W = crate::W<I2C_CR1rs>;
///Field `PE` reader - Peripheral enable - 0: Peripheral disable - 1: Peripheral enable
pub type PE_R = crate::BitReader;
///Field `PE` writer - Peripheral enable - 0: Peripheral disable - 1: Peripheral enable
pub type PE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXIE` reader - TX Interrupt enable - 0: Transmit (TXIS) interrupt disabled - 1: Transmit (TXIS) interrupt enabled
pub type TXIE_R = crate::BitReader;
///Field `TXIE` writer - TX Interrupt enable - 0: Transmit (TXIS) interrupt disabled - 1: Transmit (TXIS) interrupt enabled
pub type TXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXIE` reader - RX Interrupt enable - 0: Receive (RXNE) interrupt disabled - 1: Receive (RXNE) interrupt enabled
pub type RXIE_R = crate::BitReader;
///Field `RXIE` writer - RX Interrupt enable - 0: Receive (RXNE) interrupt disabled - 1: Receive (RXNE) interrupt enabled
pub type RXIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDRIE` reader - Address match Interrupt enable (slave only) - 0: Address match (ADDR) interrupts disabled - 1: Address match (ADDR) interrupts enabled
pub type ADDRIE_R = crate::BitReader;
///Field `ADDRIE` writer - Address match Interrupt enable (slave only) - 0: Address match (ADDR) interrupts disabled - 1: Address match (ADDR) interrupts enabled
pub type ADDRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACKIE` reader - Not acknowledge received Interrupt enable - 0: Not acknowledge (NACKF) received interrupts disabled - 1: Not acknowledge (NACKF) received interrupts enabled
pub type NACKIE_R = crate::BitReader;
///Field `NACKIE` writer - Not acknowledge received Interrupt enable - 0: Not acknowledge (NACKF) received interrupts disabled - 1: Not acknowledge (NACKF) received interrupts enabled
pub type NACKIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPIE` reader - STOP detection Interrupt enable - 0: Stop detection (STOPF) interrupt disabled - 1: Stop detection (STOPF) interrupt enabled
pub type STOPIE_R = crate::BitReader;
///Field `STOPIE` writer - STOP detection Interrupt enable - 0: Stop detection (STOPF) interrupt disabled - 1: Stop detection (STOPF) interrupt enabled
pub type STOPIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TCIE` reader - Transfer Complete interrupt enable - 0: Transfer Complete interrupt disabled - 1: Transfer Complete interrupt enabled
pub type TCIE_R = crate::BitReader;
///Field `TCIE` writer - Transfer Complete interrupt enable - 0: Transfer Complete interrupt disabled - 1: Transfer Complete interrupt enabled
pub type TCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ERRIE` reader - Error interrupts enable - 0: Error detection interrupts disabled - 1: Error detection interrupts enabled Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
pub type ERRIE_R = crate::BitReader;
///Field `ERRIE` writer - Error interrupts enable - 0: Error detection interrupts disabled - 1: Error detection interrupts enabled Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
pub type ERRIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DNF` reader - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF\[3:0\] * tI2CCLK - 0000: Digital filter disabled - 0001: Digital filter enabled and filtering capability up to 1 tI2CCLK - 1111: digital filter enabled and filtering capability up to15 tI2CCLK
pub type DNF_R = crate::FieldReader;
///Field `DNF` writer - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF\[3:0\] * tI2CCLK - 0000: Digital filter disabled - 0001: Digital filter enabled and filtering capability up to 1 tI2CCLK - 1111: digital filter enabled and filtering capability up to15 tI2CCLK
pub type DNF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `ANFOFF` reader - Analog noise filter OFF - 0: Analog noise filter enabled - 1: Analog noise filter disabled
pub type ANFOFF_R = crate::BitReader;
///Field `ANFOFF` writer - Analog noise filter OFF - 0: Analog noise filter enabled - 1: Analog noise filter disabled
pub type ANFOFF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXDMAEN` reader - DMA transmission requests enable - 0: DMA mode disabled for transmission - 1: DMA mode enabled for transmission
pub type TXDMAEN_R = crate::BitReader;
///Field `TXDMAEN` writer - DMA transmission requests enable - 0: DMA mode disabled for transmission - 1: DMA mode enabled for transmission
pub type TXDMAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RXDMAEN` reader - DMA reception requests enable - 0: DMA mode disabled for reception - 1: DMA mode enabled for reception
pub type RXDMAEN_R = crate::BitReader;
///Field `SBC` reader - Slave byte control This bit is used to enable hardware byte control in slave mode. - 0: Slave byte control disabled - 1: Slave byte control enabled
pub type SBC_R = crate::BitReader;
///Field `SBC` writer - Slave byte control This bit is used to enable hardware byte control in slave mode. - 0: Slave byte control disabled - 1: Slave byte control enabled
pub type SBC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NOSTRETCH` reader - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. - 0: Clock stretching enabled - 1: Clock stretching disabled Note: This bit can only be programmed when the I2C is disabled (PE = 0).
pub type NOSTRETCH_R = crate::BitReader;
///Field `NOSTRETCH` writer - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. - 0: Clock stretching enabled - 1: Clock stretching disabled Note: This bit can only be programmed when the I2C is disabled (PE = 0).
pub type NOSTRETCH_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `GCEN` reader - General call enable - 0: General call disabled. Address 0b00000000 is NACKed. - 1: General call enabled. Address 0b00000000 is ACKed.
pub type GCEN_R = crate::BitReader;
///Field `GCEN` writer - General call enable - 0: General call disabled. Address 0b00000000 is NACKed. - 1: General call enabled. Address 0b00000000 is ACKed.
pub type GCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMBHEN` reader - SMBus Host address enable - 0: Host address disabled. Address 0b0001000x is NACKed. - 1: Host address enabled. Address 0b0001000x is ACKed.
pub type SMBHEN_R = crate::BitReader;
///Field `SMBHEN` writer - SMBus Host address enable - 0: Host address disabled. Address 0b0001000x is NACKed. - 1: Host address enabled. Address 0b0001000x is ACKed.
pub type SMBHEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SMBDEN` reader - SMBus Device Default address enable - 0: Device default address disabled. Address 0b1100001x is NACKed. - 1: Device default address enabled. Address 0b1100001x is ACKed.
pub type SMBDEN_R = crate::BitReader;
///Field `SMBDEN` writer - SMBus Device Default address enable - 0: Device default address disabled. Address 0b1100001x is NACKed. - 1: Device default address enabled. Address 0b1100001x is ACKed.
pub type SMBDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALERTEN` reader - SMBus alert enable Device mode (SMBHEN=0): - 0: Releases SMBA pin high and Alert Response Address Header disabled: 0001100x followed by NACK. - 1: Drives SMBA pin low and Alert Response Address Header enables: 0001100x followed by ACK. Host mode (SMBHEN=1): - 0: SMBus Alert pin (SMBA) not supported. - 1: SMBus Alert pin (SMBA) supported.
pub type ALERTEN_R = crate::BitReader;
///Field `ALERTEN` writer - SMBus alert enable Device mode (SMBHEN=0): - 0: Releases SMBA pin high and Alert Response Address Header disabled: 0001100x followed by NACK. - 1: Drives SMBA pin low and Alert Response Address Header enables: 0001100x followed by ACK. Host mode (SMBHEN=1): - 0: SMBus Alert pin (SMBA) not supported. - 1: SMBus Alert pin (SMBA) supported.
pub type ALERTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PECEN` reader - PEC enable - 0: PEC calculation disabled - 1: PEC calculation enabled
pub type PECEN_R = crate::BitReader;
///Field `PECEN` writer - PEC enable - 0: PEC calculation disabled - 1: PEC calculation enabled
pub type PECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Peripheral enable - 0: Peripheral disable - 1: Peripheral enable
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - TX Interrupt enable - 0: Transmit (TXIS) interrupt disabled - 1: Transmit (TXIS) interrupt enabled
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - RX Interrupt enable - 0: Receive (RXNE) interrupt disabled - 1: Receive (RXNE) interrupt enabled
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Address match Interrupt enable (slave only) - 0: Address match (ADDR) interrupts disabled - 1: Address match (ADDR) interrupts enabled
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Not acknowledge received Interrupt enable - 0: Not acknowledge (NACKF) received interrupts disabled - 1: Not acknowledge (NACKF) received interrupts enabled
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - STOP detection Interrupt enable - 0: Stop detection (STOPF) interrupt disabled - 1: Stop detection (STOPF) interrupt enabled
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Transfer Complete interrupt enable - 0: Transfer Complete interrupt disabled - 1: Transfer Complete interrupt enabled
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Error interrupts enable - 0: Error detection interrupts disabled - 1: Error detection interrupts enabled Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF\[3:0\] * tI2CCLK - 0000: Digital filter disabled - 0001: Digital filter enabled and filtering capability up to 1 tI2CCLK - 1111: digital filter enabled and filtering capability up to15 tI2CCLK
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bit 12 - Analog noise filter OFF - 0: Analog noise filter enabled - 1: Analog noise filter disabled
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 14 - DMA transmission requests enable - 0: DMA mode disabled for transmission - 1: DMA mode enabled for transmission
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - DMA reception requests enable - 0: DMA mode disabled for reception - 1: DMA mode enabled for reception
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode. - 0: Slave byte control disabled - 1: Slave byte control enabled
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. - 0: Clock stretching enabled - 1: Clock stretching disabled Note: This bit can only be programmed when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 19 - General call enable - 0: General call disabled. Address 0b00000000 is NACKed. - 1: General call enabled. Address 0b00000000 is ACKed.
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - SMBus Host address enable - 0: Host address disabled. Address 0b0001000x is NACKed. - 1: Host address enabled. Address 0b0001000x is ACKed.
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - SMBus Device Default address enable - 0: Device default address disabled. Address 0b1100001x is NACKed. - 1: Device default address enabled. Address 0b1100001x is ACKed.
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - SMBus alert enable Device mode (SMBHEN=0): - 0: Releases SMBA pin high and Alert Response Address Header disabled: 0001100x followed by NACK. - 1: Drives SMBA pin low and Alert Response Address Header enables: 0001100x followed by ACK. Host mode (SMBHEN=1): - 0: SMBus Alert pin (SMBA) not supported. - 1: SMBus Alert pin (SMBA) supported.
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - PEC enable - 0: PEC calculation disabled - 1: PEC calculation enabled
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2C_CR1")
            .field("pe", &self.pe())
            .field("txie", &self.txie())
            .field("rxie", &self.rxie())
            .field("addrie", &self.addrie())
            .field("nackie", &self.nackie())
            .field("stopie", &self.stopie())
            .field("tcie", &self.tcie())
            .field("errie", &self.errie())
            .field("dnf", &self.dnf())
            .field("anfoff", &self.anfoff())
            .field("txdmaen", &self.txdmaen())
            .field("rxdmaen", &self.rxdmaen())
            .field("sbc", &self.sbc())
            .field("nostretch", &self.nostretch())
            .field("gcen", &self.gcen())
            .field("smbhen", &self.smbhen())
            .field("smbden", &self.smbden())
            .field("alerten", &self.alerten())
            .field("pecen", &self.pecen())
            .finish()
    }
}
impl W {
    ///Bit 0 - Peripheral enable - 0: Peripheral disable - 1: Peripheral enable
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W<'_, I2C_CR1rs> {
        PE_W::new(self, 0)
    }
    ///Bit 1 - TX Interrupt enable - 0: Transmit (TXIS) interrupt disabled - 1: Transmit (TXIS) interrupt enabled
    #[inline(always)]
    pub fn txie(&mut self) -> TXIE_W<'_, I2C_CR1rs> {
        TXIE_W::new(self, 1)
    }
    ///Bit 2 - RX Interrupt enable - 0: Receive (RXNE) interrupt disabled - 1: Receive (RXNE) interrupt enabled
    #[inline(always)]
    pub fn rxie(&mut self) -> RXIE_W<'_, I2C_CR1rs> {
        RXIE_W::new(self, 2)
    }
    ///Bit 3 - Address match Interrupt enable (slave only) - 0: Address match (ADDR) interrupts disabled - 1: Address match (ADDR) interrupts enabled
    #[inline(always)]
    pub fn addrie(&mut self) -> ADDRIE_W<'_, I2C_CR1rs> {
        ADDRIE_W::new(self, 3)
    }
    ///Bit 4 - Not acknowledge received Interrupt enable - 0: Not acknowledge (NACKF) received interrupts disabled - 1: Not acknowledge (NACKF) received interrupts enabled
    #[inline(always)]
    pub fn nackie(&mut self) -> NACKIE_W<'_, I2C_CR1rs> {
        NACKIE_W::new(self, 4)
    }
    ///Bit 5 - STOP detection Interrupt enable - 0: Stop detection (STOPF) interrupt disabled - 1: Stop detection (STOPF) interrupt enabled
    #[inline(always)]
    pub fn stopie(&mut self) -> STOPIE_W<'_, I2C_CR1rs> {
        STOPIE_W::new(self, 5)
    }
    ///Bit 6 - Transfer Complete interrupt enable - 0: Transfer Complete interrupt disabled - 1: Transfer Complete interrupt enabled
    #[inline(always)]
    pub fn tcie(&mut self) -> TCIE_W<'_, I2C_CR1rs> {
        TCIE_W::new(self, 6)
    }
    ///Bit 7 - Error interrupts enable - 0: Error detection interrupts disabled - 1: Error detection interrupts enabled Note: Any of these errors generate an interrupt: Arbitration Loss (ARLO) Bus Error detection (BERR) Overrun/Underrun (OVR) Timeout detection (TIMEOUT) PEC error detection (PECERR) Alert pin event detection (ALERT)
    #[inline(always)]
    pub fn errie(&mut self) -> ERRIE_W<'_, I2C_CR1rs> {
        ERRIE_W::new(self, 7)
    }
    ///Bits 8:11 - Digital noise filter These bits are used to configure the digital noise filter on SDA and SCL input. The digital filter will filter spikes with a length of up to DNF\[3:0\] * tI2CCLK - 0000: Digital filter disabled - 0001: Digital filter enabled and filtering capability up to 1 tI2CCLK - 1111: digital filter enabled and filtering capability up to15 tI2CCLK
    #[inline(always)]
    pub fn dnf(&mut self) -> DNF_W<'_, I2C_CR1rs> {
        DNF_W::new(self, 8)
    }
    ///Bit 12 - Analog noise filter OFF - 0: Analog noise filter enabled - 1: Analog noise filter disabled
    #[inline(always)]
    pub fn anfoff(&mut self) -> ANFOFF_W<'_, I2C_CR1rs> {
        ANFOFF_W::new(self, 12)
    }
    ///Bit 14 - DMA transmission requests enable - 0: DMA mode disabled for transmission - 1: DMA mode enabled for transmission
    #[inline(always)]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<'_, I2C_CR1rs> {
        TXDMAEN_W::new(self, 14)
    }
    ///Bit 16 - Slave byte control This bit is used to enable hardware byte control in slave mode. - 0: Slave byte control disabled - 1: Slave byte control enabled
    #[inline(always)]
    pub fn sbc(&mut self) -> SBC_W<'_, I2C_CR1rs> {
        SBC_W::new(self, 16)
    }
    ///Bit 17 - Clock stretching disable This bit is used to disable clock stretching in slave mode. It must be kept cleared in master mode. - 0: Clock stretching enabled - 1: Clock stretching disabled Note: This bit can only be programmed when the I2C is disabled (PE = 0).
    #[inline(always)]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<'_, I2C_CR1rs> {
        NOSTRETCH_W::new(self, 17)
    }
    ///Bit 19 - General call enable - 0: General call disabled. Address 0b00000000 is NACKed. - 1: General call enabled. Address 0b00000000 is ACKed.
    #[inline(always)]
    pub fn gcen(&mut self) -> GCEN_W<'_, I2C_CR1rs> {
        GCEN_W::new(self, 19)
    }
    ///Bit 20 - SMBus Host address enable - 0: Host address disabled. Address 0b0001000x is NACKed. - 1: Host address enabled. Address 0b0001000x is ACKed.
    #[inline(always)]
    pub fn smbhen(&mut self) -> SMBHEN_W<'_, I2C_CR1rs> {
        SMBHEN_W::new(self, 20)
    }
    ///Bit 21 - SMBus Device Default address enable - 0: Device default address disabled. Address 0b1100001x is NACKed. - 1: Device default address enabled. Address 0b1100001x is ACKed.
    #[inline(always)]
    pub fn smbden(&mut self) -> SMBDEN_W<'_, I2C_CR1rs> {
        SMBDEN_W::new(self, 21)
    }
    ///Bit 22 - SMBus alert enable Device mode (SMBHEN=0): - 0: Releases SMBA pin high and Alert Response Address Header disabled: 0001100x followed by NACK. - 1: Drives SMBA pin low and Alert Response Address Header enables: 0001100x followed by ACK. Host mode (SMBHEN=1): - 0: SMBus Alert pin (SMBA) not supported. - 1: SMBus Alert pin (SMBA) supported.
    #[inline(always)]
    pub fn alerten(&mut self) -> ALERTEN_W<'_, I2C_CR1rs> {
        ALERTEN_W::new(self, 22)
    }
    ///Bit 23 - PEC enable - 0: PEC calculation disabled - 1: PEC calculation enabled
    #[inline(always)]
    pub fn pecen(&mut self) -> PECEN_W<'_, I2C_CR1rs> {
        PECEN_W::new(self, 23)
    }
}
/**I2C_CR1 register

You can [`read`](crate::Reg::read) this register and get [`i2c_cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WL33.html#I2C2:I2C_CR1)*/
pub struct I2C_CR1rs;
impl crate::RegisterSpec for I2C_CR1rs {
    type Ux = u32;
}
///`read()` method returns [`i2c_cr1::R`](R) reader structure
impl crate::Readable for I2C_CR1rs {}
///`write(|w| ..)` method takes [`i2c_cr1::W`](W) writer structure
impl crate::Writable for I2C_CR1rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_CR1 to value 0
impl crate::Resettable for I2C_CR1rs {}
