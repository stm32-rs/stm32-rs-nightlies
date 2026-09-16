///Register `CR` reader
pub type R = crate::R<CRrs>;
///Register `CR` writer
pub type W = crate::W<CRrs>;
///Field `CECEN` reader - CEC Enable
pub type CECEN_R = crate::BitReader;
///Field `CECEN` writer - CEC Enable
pub type CECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSOM` reader - Tx start of message
pub type TXSOM_R = crate::BitReader;
///Field `TXSOM` writer - Tx start of message
pub type TXSOM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXEOM` reader - Tx End Of Message
pub type TXEOM_R = crate::BitReader;
///Field `TXEOM` writer - Tx End Of Message
pub type TXEOM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - CEC Enable
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx start of message
    #[inline(always)]
    pub fn txsom(&self) -> TXSOM_R {
        TXSOM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tx End Of Message
    #[inline(always)]
    pub fn txeom(&self) -> TXEOM_R {
        TXEOM_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR")
            .field("txeom", &self.txeom())
            .field("txsom", &self.txsom())
            .field("cecen", &self.cecen())
            .finish()
    }
}
impl W {
    ///Bit 0 - CEC Enable
    #[inline(always)]
    pub fn cecen(&mut self) -> CECEN_W<'_, CRrs> {
        CECEN_W::new(self, 0)
    }
    ///Bit 1 - Tx start of message
    #[inline(always)]
    pub fn txsom(&mut self) -> TXSOM_W<'_, CRrs> {
        TXSOM_W::new(self, 1)
    }
    ///Bit 2 - Tx End Of Message
    #[inline(always)]
    pub fn txeom(&mut self) -> TXEOM_W<'_, CRrs> {
        TXEOM_W::new(self, 2)
    }
}
/**control register

You can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F779.html#CEC:CR)*/
pub struct CRrs;
impl crate::RegisterSpec for CRrs {
    type Ux = u32;
}
///`read()` method returns [`cr::R`](R) reader structure
impl crate::Readable for CRrs {}
///`write(|w| ..)` method takes [`cr::W`](W) writer structure
impl crate::Writable for CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR to value 0
impl crate::Resettable for CRrs {}
