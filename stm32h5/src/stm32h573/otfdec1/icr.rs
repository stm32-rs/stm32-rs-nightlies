///Register `ICR` writer
pub type W = crate::W<ICRrs>;
///Field `SEIF` writer - Security error interrupt flag clear This bit is written by application, and always read as 0.
pub type SEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `XONEIF` writer - Execute-only execute-never error interrupt flag clear This bit is written by application, and always read as 0.
pub type XONEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `KEIF` writer - Key error interrupt flag clear This bit is written by application, and always read as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to access again any encrypted region, OTFDEC key registers must be properly initialized again.
pub type KEIF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl core::fmt::Debug for crate::generic::Reg<ICRrs> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Security error interrupt flag clear This bit is written by application, and always read as 0.
    #[inline(always)]
    pub fn seif(&mut self) -> SEIF_W<'_, ICRrs> {
        SEIF_W::new(self, 0)
    }
    ///Bit 1 - Execute-only execute-never error interrupt flag clear This bit is written by application, and always read as 0.
    #[inline(always)]
    pub fn xoneif(&mut self) -> XONEIF_W<'_, ICRrs> {
        XONEIF_W::new(self, 1)
    }
    ///Bit 2 - Key error interrupt flag clear This bit is written by application, and always read as 0. Note: Clearing KEIF does not solve the source of the problem (bad key registers). To be able to access again any encrypted region, OTFDEC key registers must be properly initialized again.
    #[inline(always)]
    pub fn keif(&mut self) -> KEIF_W<'_, ICRrs> {
        KEIF_W::new(self, 2)
    }
}
/**OTFDEC interrupt clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H573.html#OTFDEC1:ICR)*/
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
