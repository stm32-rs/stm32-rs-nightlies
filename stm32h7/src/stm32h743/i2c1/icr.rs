#[doc = "Register `ICR` writer"]
pub type W = crate::W<ICRrs>;
#[doc = "Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRCF {
    #[doc = "1: Clears the ADDR flag in ISR register"]
    Clear = 1,
}
impl From<ADDRCF> for bool {
    #[inline(always)]
    fn from(variant: ADDRCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADDRCF` writer - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
pub type ADDRCF_W<'a, REG> = crate::BitWriter<'a, REG, ADDRCF>;
impl<'a, REG> ADDRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ADDR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ADDRCF::Clear)
    }
}
#[doc = "Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKCF {
    #[doc = "1: Clears the NACK flag in ISR register"]
    Clear = 1,
}
impl From<NACKCF> for bool {
    #[inline(always)]
    fn from(variant: NACKCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKCF` writer - Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register."]
pub type NACKCF_W<'a, REG> = crate::BitWriter<'a, REG, NACKCF>;
impl<'a, REG> NACKCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the NACK flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(NACKCF::Clear)
    }
}
#[doc = "Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPCF {
    #[doc = "1: Clears the STOP flag in ISR register"]
    Clear = 1,
}
impl From<STOPCF> for bool {
    #[inline(always)]
    fn from(variant: STOPCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPCF` writer - Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
pub type STOPCF_W<'a, REG> = crate::BitWriter<'a, REG, STOPCF>;
impl<'a, REG> STOPCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the STOP flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(STOPCF::Clear)
    }
}
#[doc = "Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERRCF {
    #[doc = "1: Clears the BERR flag in ISR register"]
    Clear = 1,
}
impl From<BERRCF> for bool {
    #[inline(always)]
    fn from(variant: BERRCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BERRCF` writer - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
pub type BERRCF_W<'a, REG> = crate::BitWriter<'a, REG, BERRCF>;
impl<'a, REG> BERRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the BERR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(BERRCF::Clear)
    }
}
#[doc = "Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLOCF {
    #[doc = "1: Clears the ARLO flag in ISR register"]
    Clear = 1,
}
impl From<ARLOCF> for bool {
    #[inline(always)]
    fn from(variant: ARLOCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ARLOCF` writer - Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
pub type ARLOCF_W<'a, REG> = crate::BitWriter<'a, REG, ARLOCF>;
impl<'a, REG> ARLOCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ARLO flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ARLOCF::Clear)
    }
}
#[doc = "Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRCF {
    #[doc = "1: Clears the OVR flag in ISR register"]
    Clear = 1,
}
impl From<OVRCF> for bool {
    #[inline(always)]
    fn from(variant: OVRCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OVRCF` writer - Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG, OVRCF>;
impl<'a, REG> OVRCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the OVR flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(OVRCF::Clear)
    }
}
#[doc = "PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECCF {
    #[doc = "1: Clears the PEC flag in ISR register"]
    Clear = 1,
}
impl From<PECCF> for bool {
    #[inline(always)]
    fn from(variant: PECCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PECCF` writer - PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
pub type PECCF_W<'a, REG> = crate::BitWriter<'a, REG, PECCF>;
impl<'a, REG> PECCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the PEC flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(PECCF::Clear)
    }
}
#[doc = "Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMOUTCF {
    #[doc = "1: Clears the TIMOUT flag in ISR register"]
    Clear = 1,
}
impl From<TIMOUTCF> for bool {
    #[inline(always)]
    fn from(variant: TIMOUTCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TIMOUTCF` writer - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
pub type TIMOUTCF_W<'a, REG> = crate::BitWriter<'a, REG, TIMOUTCF>;
impl<'a, REG> TIMOUTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the TIMOUT flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(TIMOUTCF::Clear)
    }
}
#[doc = "Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTCF {
    #[doc = "1: Clears the ALERT flag in ISR register"]
    Clear = 1,
}
impl From<ALERTCF> for bool {
    #[inline(always)]
    fn from(variant: ALERTCF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALERTCF` writer - Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
pub type ALERTCF_W<'a, REG> = crate::BitWriter<'a, REG, ALERTCF>;
impl<'a, REG> ALERTCF_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clears the ALERT flag in ISR register"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ALERTCF::Clear)
    }
}
impl W {
    #[doc = "Bit 3 - Address matched flag clear Writing 1 to this bit clears the ADDR flag in the I2C_ISR register. Writing 1 to this bit also clears the START bit in the I2C_CR2 register."]
    #[inline(always)]
    #[must_use]
    pub fn addrcf(&mut self) -> ADDRCF_W<ICRrs> {
        ADDRCF_W::new(self, 3)
    }
    #[doc = "Bit 4 - Not Acknowledge flag clear Writing 1 to this bit clears the ACKF flag in I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn nackcf(&mut self) -> NACKCF_W<ICRrs> {
        NACKCF_W::new(self, 4)
    }
    #[doc = "Bit 5 - Stop detection flag clear Writing 1 to this bit clears the STOPF flag in the I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn stopcf(&mut self) -> STOPCF_W<ICRrs> {
        STOPCF_W::new(self, 5)
    }
    #[doc = "Bit 8 - Bus error flag clear Writing 1 to this bit clears the BERRF flag in the I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn berrcf(&mut self) -> BERRCF_W<ICRrs> {
        BERRCF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Arbitration Lost flag clear Writing 1 to this bit clears the ARLO flag in the I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn arlocf(&mut self) -> ARLOCF_W<ICRrs> {
        ARLOCF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Overrun/Underrun flag clear Writing 1 to this bit clears the OVR flag in the I2C_ISR register."]
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<ICRrs> {
        OVRCF_W::new(self, 10)
    }
    #[doc = "Bit 11 - PEC Error flag clear Writing 1 to this bit clears the PECERR flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
    #[inline(always)]
    #[must_use]
    pub fn peccf(&mut self) -> PECCF_W<ICRrs> {
        PECCF_W::new(self, 11)
    }
    #[doc = "Bit 12 - Timeout detection flag clear Writing 1 to this bit clears the TIMEOUT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
    #[inline(always)]
    #[must_use]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<ICRrs> {
        TIMOUTCF_W::new(self, 12)
    }
    #[doc = "Bit 13 - Alert flag clear Writing 1 to this bit clears the ALERT flag in the I2C_ISR register. Note: If the SMBus feature is not supported, this bit is reserved and forced by hardware to 0. Please refer to Section25.3: I2C implementation."]
    #[inline(always)]
    #[must_use]
    pub fn alertcf(&mut self) -> ALERTCF_W<ICRrs> {
        ALERTCF_W::new(self, 13)
    }
}
#[doc = "Access: No wait states\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for ICRrs {
    const RESET_VALUE: u32 = 0;
}
