///Register `ICR` writer
pub type W = crate::W<ICRrs>;
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
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 3 - ADDRCF
    #[inline(always)]
    pub fn addrcf(&mut self) -> ADDRCF_W<'_, ICRrs> {
        ADDRCF_W::new(self, 3)
    }
    ///Bit 4 - NACKCF
    #[inline(always)]
    pub fn nackcf(&mut self) -> NACKCF_W<'_, ICRrs> {
        NACKCF_W::new(self, 4)
    }
    ///Bit 5 - STOPCF
    #[inline(always)]
    pub fn stopcf(&mut self) -> STOPCF_W<'_, ICRrs> {
        STOPCF_W::new(self, 5)
    }
    ///Bit 8 - BERRCF
    #[inline(always)]
    pub fn berrcf(&mut self) -> BERRCF_W<'_, ICRrs> {
        BERRCF_W::new(self, 8)
    }
    ///Bit 9 - ARLOCF
    #[inline(always)]
    pub fn arlocf(&mut self) -> ARLOCF_W<'_, ICRrs> {
        ARLOCF_W::new(self, 9)
    }
    ///Bit 10 - OVRCF
    #[inline(always)]
    pub fn ovrcf(&mut self) -> OVRCF_W<'_, ICRrs> {
        OVRCF_W::new(self, 10)
    }
    ///Bit 11 - PECCF
    #[inline(always)]
    pub fn peccf(&mut self) -> PECCF_W<'_, ICRrs> {
        PECCF_W::new(self, 11)
    }
    ///Bit 12 - TIMOUTCF
    #[inline(always)]
    pub fn timoutcf(&mut self) -> TIMOUTCF_W<'_, ICRrs> {
        TIMOUTCF_W::new(self, 12)
    }
    ///Bit 13 - ALERTCF
    #[inline(always)]
    pub fn alertcf(&mut self) -> ALERTCF_W<'_, ICRrs> {
        ALERTCF_W::new(self, 13)
    }
}
/**Access: No wait states

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#I2C1:ICR)*/
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
