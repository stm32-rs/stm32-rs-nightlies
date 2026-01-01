///Register `I2C_ICR` writer
pub type W = crate::W<I2C_ICRrs>;
///Field `ADDRCF` writer - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register.
pub type ADDRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACKCF` writer - Not acknowledge flag clear Writing 1 to this bit clears the NACKF flag in I2C_ISR register.
pub type NACKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPCF` writer - STOP detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register.
pub type STOPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERRCF` writer - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register.
pub type BERRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARLOCF` writer - Arbitration lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register.
pub type ARLOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRCF` writer - Overrun/underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register.
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PECCF` writer - PEC error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type PECCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMOUTCF` writer - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type TIMOUTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALERTCF` writer - Alert flag clear Note: Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
pub type ALERTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<I2C_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register.
    #[inline(always)]
    pub fn addrcf(&mut self) -> ADDRCF_W<'_, I2C_ICRrs> {
        ADDRCF_W::new(self, 3)
    }
    ///Bit 4 - Not acknowledge flag clear Writing 1 to this bit clears the NACKF flag in I2C_ISR register.
    #[inline(always)]
    pub fn nackcf(&mut self) -> NACKCF_W<'_, I2C_ICRrs> {
        NACKCF_W::new(self, 4)
    }
    ///Bit 5 - STOP detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register.
    #[inline(always)]
    pub fn stopcf(&mut self) -> STOPCF_W<'_, I2C_ICRrs> {
        STOPCF_W::new(self, 5)
    }
    ///Bit 8 - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register.
    #[inline(always)]
    pub fn berrcf(&mut self) -> BERRCF_W<'_, I2C_ICRrs> {
        BERRCF_W::new(self, 8)
    }
    ///Bit 9 - Arbitration lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register.
    #[inline(always)]
    pub fn arlocf(&mut self) -> ARLOCF_W<'_, I2C_ICRrs> {
        ARLOCF_W::new(self, 9)
    }
    ///Bit 10 - Overrun/underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register.
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W<'_, I2C_ICRrs> {
        OVRCF_W::new(self, 10)
    }
    ///Bit 11 - PEC error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn peccf(&mut self) -> PECCF_W<'_, I2C_ICRrs> {
        PECCF_W::new(self, 11)
    }
    ///Bit 12 - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<'_, I2C_ICRrs> {
        TIMOUTCF_W::new(self, 12)
    }
    ///Bit 13 - Alert flag clear Note: Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Refer to Section 25.3.
    #[inline(always)]
    pub fn alertcf(&mut self) -> ALERTCF_W<'_, I2C_ICRrs> {
        ALERTCF_W::new(self, 13)
    }
}
/**I2C interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32C092.html#I2C1:I2C_ICR)*/
pub struct I2C_ICRrs;
impl crate::RegisterSpec for I2C_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`i2c_icr::W`](W) writer structure
impl crate::Writable for I2C_ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets I2C_ICR to value 0
impl crate::Resettable for I2C_ICRrs {}
