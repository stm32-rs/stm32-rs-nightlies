///Register `CEC_CR` reader
pub type R = crate::R<CEC_CRrs>;
///Register `CEC_CR` writer
pub type W = crate::W<CEC_CRrs>;
///Field `CECEN` reader - CEC Enable
pub type CECEN_R = crate::BitReader;
///Field `CECEN` writer - CEC Enable
pub type CECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TXSOM` reader - Tx Start Of Message
pub type TXSOM_R = crate::BitReader;
///Field `TXEOM` reader - Tx End Of Message
pub type TXEOM_R = crate::BitReader;
impl R {
    ///Bit 0 - CEC Enable
    #[inline(always)]
    pub fn cecen(&self) -> CECEN_R {
        CECEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tx Start Of Message
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
        f.debug_struct("CEC_CR")
            .field("txeom", &self.txeom())
            .field("txsom", &self.txsom())
            .field("cecen", &self.cecen())
            .finish()
    }
}
impl W {
    ///Bit 0 - CEC Enable
    #[inline(always)]
    pub fn cecen(&mut self) -> CECEN_W<'_, CEC_CRrs> {
        CECEN_W::new(self, 0)
    }
}
/**CEC control register

You can [`read`](crate::Reg::read) this register and get [`cec_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cec_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F446.html#HDMI_CEC:CEC_CR)*/
pub struct CEC_CRrs;
impl crate::RegisterSpec for CEC_CRrs {
    type Ux = u32;
}
///`read()` method returns [`cec_cr::R`](R) reader structure
impl crate::Readable for CEC_CRrs {}
///`write(|w| ..)` method takes [`cec_cr::W`](W) writer structure
impl crate::Writable for CEC_CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CEC_CR to value 0
impl crate::Resettable for CEC_CRrs {}
