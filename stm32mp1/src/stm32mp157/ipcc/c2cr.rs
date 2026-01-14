///Register `C2CR` reader
pub type R = crate::R<C2CRrs>;
///Register `C2CR` writer
pub type W = crate::W<C2CRrs>;
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
        f.debug_struct("C2CR")
            .field("rxoie", &self.rxoie())
            .field("txfie", &self.txfie())
            .finish()
    }
}
impl W {
    ///Bit 0 - RXOIE
    #[inline(always)]
    pub fn rxoie(&mut self) -> RXOIE_W<'_, C2CRrs> {
        RXOIE_W::new(self, 0)
    }
    ///Bit 16 - TXFIE
    #[inline(always)]
    pub fn txfie(&mut self) -> TXFIE_W<'_, C2CRrs> {
        TXFIE_W::new(self, 16)
    }
}
/**IPCC Processor 2 control register

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#IPCC:C2CR)*/
pub struct C2CRrs;
impl crate::RegisterSpec for C2CRrs {
    type Ux = u32;
}
///`read()` method returns [`c2cr::R`](R) reader structure
impl crate::Readable for C2CRrs {}
///`write(|w| ..)` method takes [`c2cr::W`](W) writer structure
impl crate::Writable for C2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets C2CR to value 0
impl crate::Resettable for C2CRrs {}
