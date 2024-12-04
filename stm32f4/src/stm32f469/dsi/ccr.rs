///Register `CCR` reader
pub type R = crate::R<CCRrs>;
///Register `CCR` writer
pub type W = crate::W<CCRrs>;
///Field `TXECKDIV` reader - TXECKDIV
pub type TXECKDIV_R = crate::FieldReader;
///Field `TXECKDIV` writer - TXECKDIV
pub type TXECKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TOCKDIV` reader - TOCKDIV
pub type TOCKDIV_R = crate::FieldReader;
///Field `TOCKDIV` writer - TOCKDIV
pub type TOCKDIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - TXECKDIV
    #[inline(always)]
    pub fn txeckdiv(&self) -> TXECKDIV_R {
        TXECKDIV_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - TOCKDIV
    #[inline(always)]
    pub fn tockdiv(&self) -> TOCKDIV_R {
        TOCKDIV_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CCR")
            .field("tockdiv", &self.tockdiv())
            .field("txeckdiv", &self.txeckdiv())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - TXECKDIV
    #[inline(always)]
    pub fn txeckdiv(&mut self) -> TXECKDIV_W<CCRrs> {
        TXECKDIV_W::new(self, 0)
    }
    ///Bits 8:15 - TOCKDIV
    #[inline(always)]
    pub fn tockdiv(&mut self) -> TOCKDIV_W<CCRrs> {
        TOCKDIV_W::new(self, 8)
    }
}
/**DSI HOST Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F469.html#DSI:CCR)*/
pub struct CCRrs;
impl crate::RegisterSpec for CCRrs {
    type Ux = u32;
}
///`read()` method returns [`ccr::R`](R) reader structure
impl crate::Readable for CCRrs {}
///`write(|w| ..)` method takes [`ccr::W`](W) writer structure
impl crate::Writable for CCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CCR to value 0x3133_302a
impl crate::Resettable for CCRrs {
    const RESET_VALUE: u32 = 0x3133_302a;
}
