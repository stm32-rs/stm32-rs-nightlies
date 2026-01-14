///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `ADDRCF` writer - Address matched flag clear
pub type ADDRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NACKCF` writer - Not Acknowledge flag clear
pub type NACKCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `STOPCF` writer - STOP detection flag clear
pub type STOPCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BERRCF` writer - Bus error flag clear
pub type BERRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ARLOCF` writer - Arbitration lost flag clear
pub type ARLOCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `OVRCF` writer - Overrun/Underrun flag clear
pub type OVRCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PECCF` writer - PEC Error flag clear
pub type PECCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TIMOUTCF` writer - Timeout detection flag clear
pub type TIMOUTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ALERTCF` writer - Alert flag clear
pub type ALERTCF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - Address matched flag clear
    #[inline(always)]
    pub fn addrcf(&mut self) -> ADDRCF_W<'_, ICRrs> {
        ADDRCF_W::new(self, 3)
    }
    ///Bit 4 - Not Acknowledge flag clear
    #[inline(always)]
    pub fn nackcf(&mut self) -> NACKCF_W<'_, ICRrs> {
        NACKCF_W::new(self, 4)
    }
    ///Bit 5 - STOP detection flag clear
    #[inline(always)]
    pub fn stopcf(&mut self) -> STOPCF_W<'_, ICRrs> {
        STOPCF_W::new(self, 5)
    }
    ///Bit 8 - Bus error flag clear
    #[inline(always)]
    pub fn berrcf(&mut self) -> BERRCF_W<'_, ICRrs> {
        BERRCF_W::new(self, 8)
    }
    ///Bit 9 - Arbitration lost flag clear
    #[inline(always)]
    pub fn arlocf(&mut self) -> ARLOCF_W<'_, ICRrs> {
        ARLOCF_W::new(self, 9)
    }
    ///Bit 10 - Overrun/Underrun flag clear
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W<'_, ICRrs> {
        OVRCF_W::new(self, 10)
    }
    ///Bit 11 - PEC Error flag clear
    #[inline(always)]
    pub fn peccf(&mut self) -> PECCF_W<'_, ICRrs> {
        PECCF_W::new(self, 11)
    }
    ///Bit 12 - Timeout detection flag clear
    #[inline(always)]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<'_, ICRrs> {
        TIMOUTCF_W::new(self, 12)
    }
    ///Bit 13 - Alert flag clear
    #[inline(always)]
    pub fn alertcf(&mut self) -> ALERTCF_W<'_, ICRrs> {
        ALERTCF_W::new(self, 13)
    }
}
/**I2C interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#I2C1:ICR)*/
pub struct ICRrs;
impl crate::RegisterSpec for ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`icr::W`](W) writer structure
impl crate::Writable for ICRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets ICR to value 0
impl crate::Resettable for ICRrs {}
