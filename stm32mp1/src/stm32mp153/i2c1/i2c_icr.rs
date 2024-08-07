///Register `I2C_ICR` writer
pub type W = crate::W<I2C_ICRrs>;
///Field `ADDRCF` writer - ADDRCF
pub type ADDRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACKCF` writer - NACKCF
pub type NACKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPCF` writer - STOPCF
pub type STOPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERRCF` writer - BERRCF
pub type BERRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARLOCF` writer - ARLOCF
pub type ARLOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRCF` writer - OVRCF
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PECCF` writer - PECCF
pub type PECCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMOUTCF` writer - TIMOUTCF
pub type TIMOUTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALERTCF` writer - ALERTCF
pub type ALERTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<I2C_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - ADDRCF
    #[inline(always)]
    #[must_use]
    pub fn addrcf(&mut self) -> ADDRCF_W<I2C_ICRrs> {
        ADDRCF_W::new(self, 3)
    }
    ///Bit 4 - NACKCF
    #[inline(always)]
    #[must_use]
    pub fn nackcf(&mut self) -> NACKCF_W<I2C_ICRrs> {
        NACKCF_W::new(self, 4)
    }
    ///Bit 5 - STOPCF
    #[inline(always)]
    #[must_use]
    pub fn stopcf(&mut self) -> STOPCF_W<I2C_ICRrs> {
        STOPCF_W::new(self, 5)
    }
    ///Bit 8 - BERRCF
    #[inline(always)]
    #[must_use]
    pub fn berrcf(&mut self) -> BERRCF_W<I2C_ICRrs> {
        BERRCF_W::new(self, 8)
    }
    ///Bit 9 - ARLOCF
    #[inline(always)]
    #[must_use]
    pub fn arlocf(&mut self) -> ARLOCF_W<I2C_ICRrs> {
        ARLOCF_W::new(self, 9)
    }
    ///Bit 10 - OVRCF
    #[inline(always)]
    #[must_use]
    pub fn ovrcf(&mut self) -> OVRCF_W<I2C_ICRrs> {
        OVRCF_W::new(self, 10)
    }
    ///Bit 11 - PECCF
    #[inline(always)]
    #[must_use]
    pub fn peccf(&mut self) -> PECCF_W<I2C_ICRrs> {
        PECCF_W::new(self, 11)
    }
    ///Bit 12 - TIMOUTCF
    #[inline(always)]
    #[must_use]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<I2C_ICRrs> {
        TIMOUTCF_W::new(self, 12)
    }
    ///Bit 13 - ALERTCF
    #[inline(always)]
    #[must_use]
    pub fn alertcf(&mut self) -> ALERTCF_W<I2C_ICRrs> {
        ALERTCF_W::new(self, 13)
    }
}
/**Access: No wait states

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#I2C1:I2C_ICR)*/
pub struct I2C_ICRrs;
impl crate::RegisterSpec for I2C_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`i2c_icr::W`](W) writer structure
impl crate::Writable for I2C_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I2C_ICR to value 0
impl crate::Resettable for I2C_ICRrs {
    const RESET_VALUE: u32 = 0;
}
