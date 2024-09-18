///Register `IPCC_C2CR` reader
pub type R = crate::R<IPCC_C2CRrs>;
///Register `IPCC_C2CR` writer
pub type W = crate::W<IPCC_C2CRrs>;
///Field `RXOIE` reader - RXOIE
pub type RXOIE_R = crate::BitReader;
///Field `RXOIE` writer - RXOIE
pub type RXOIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXFIE` reader - TXFIE
pub type TXFIE_R = crate::BitReader;
///Field `TXFIE` writer - TXFIE
pub type TXFIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - RXOIE
    #[inline(always)]
    pub fn rxoie(&self) -> RXOIE_R {
        RXOIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - TXFIE
    #[inline(always)]
    pub fn txfie(&self) -> TXFIE_R {
        TXFIE_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IPCC_C2CR")
            .field("rxoie", &self.rxoie())
            .field("txfie", &self.txfie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXOIE
    #[inline(always)]
    #[must_use]
    pub fn rxoie(&mut self) -> RXOIE_W<IPCC_C2CRrs> {
        RXOIE_W::new(self, 0)
    }
    ///Bit 16 - TXFIE
    #[inline(always)]
    #[must_use]
    pub fn txfie(&mut self) -> TXFIE_W<IPCC_C2CRrs> {
        TXFIE_W::new(self, 16)
    }
}
/**IPCC Processor 2 control register

You can [`read`](crate::Reg::read) this register and get [`ipcc_c2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipcc_c2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:IPCC_C2CR)*/
pub struct IPCC_C2CRrs;
impl crate::RegisterSpec for IPCC_C2CRrs {
    type Ux = u32;
}
///`read()` method returns [`ipcc_c2cr::R`](R) reader structure
impl crate::Readable for IPCC_C2CRrs {}
///`write(|w| ..)` method takes [`ipcc_c2cr::W`](W) writer structure
impl crate::Writable for IPCC_C2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets IPCC_C2CR to value 0
impl crate::Resettable for IPCC_C2CRrs {
    const RESET_VALUE: u32 = 0;
}
