///Register `OTFDEC_ICR` writer
pub type W = crate::W<OTFDEC_ICRrs>;
///Field `SEIF` writer - Security Error Interrupt Flag clear This bit is written by application, and always reads as 0.
pub type SEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XONEIF` writer - Execute-only execute-Never Error Interrupt Flag clear This bit is written by application, and always reads as 0.
pub type XONEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEIF` writer - Key Error Interrupt Flag clear This bit is written by application, and always reads as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to read or execute again any encrypted region, OTFDEC key registers must properly initialized, again.
pub type KEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<OTFDEC_ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Security Error Interrupt Flag clear This bit is written by application, and always reads as 0.
    #[inline(always)]
    pub fn seif(&mut self) -> SEIF_W<OTFDEC_ICRrs> {
        SEIF_W::new(self, 0)
    }
    ///Bit 1 - Execute-only execute-Never Error Interrupt Flag clear This bit is written by application, and always reads as 0.
    #[inline(always)]
    pub fn xoneif(&mut self) -> XONEIF_W<OTFDEC_ICRrs> {
        XONEIF_W::new(self, 1)
    }
    ///Bit 2 - Key Error Interrupt Flag clear This bit is written by application, and always reads as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to read or execute again any encrypted region, OTFDEC key registers must properly initialized, again.
    #[inline(always)]
    pub fn keif(&mut self) -> KEIF_W<OTFDEC_ICRrs> {
        KEIF_W::new(self, 2)
    }
}
/**OTFDEC interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otfdec_icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H735.html#OTFDEC1:OTFDEC_ICR)*/
pub struct OTFDEC_ICRrs;
impl crate::RegisterSpec for OTFDEC_ICRrs {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`otfdec_icr::W`](W) writer structure
impl crate::Writable for OTFDEC_ICRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets OTFDEC_ICR to value 0
impl crate::Resettable for OTFDEC_ICRrs {
    const RESET_VALUE: u32 = 0;
}