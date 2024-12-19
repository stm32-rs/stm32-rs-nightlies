///Register `LPUART_BRR` reader
pub type R = crate::R<LPUART_BRRrs>;
///Register `LPUART_BRR` writer
pub type W = crate::W<LPUART_BRRrs>;
///Field `BRR` reader - LPUART baud rate division (LPUARTDIV)
pub type BRR_R = crate::FieldReader<u32>;
///Field `BRR` writer - LPUART baud rate division (LPUARTDIV)
pub type BRR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - LPUART baud rate division (LPUARTDIV)
    #[inline(always)]
    pub fn brr(&self) -> BRR_R {
        BRR_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPUART_BRR")
            .field("brr", &self.brr())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - LPUART baud rate division (LPUARTDIV)
    #[inline(always)]
    pub fn brr(&mut self) -> BRR_W<LPUART_BRRrs> {
        BRR_W::new(self, 0)
    }
}
/**LPUART baud rate register

You can [`read`](crate::Reg::read) this register and get [`lpuart_brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpuart_brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U031.html#LPUART1:LPUART_BRR)*/
pub struct LPUART_BRRrs;
impl crate::RegisterSpec for LPUART_BRRrs {
    type Ux = u32;
}
///`read()` method returns [`lpuart_brr::R`](R) reader structure
impl crate::Readable for LPUART_BRRrs {}
///`write(|w| ..)` method takes [`lpuart_brr::W`](W) writer structure
impl crate::Writable for LPUART_BRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LPUART_BRR to value 0
impl crate::Resettable for LPUART_BRRrs {
    const RESET_VALUE: u32 = 0;
}
