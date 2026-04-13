///Register `MTLTxQUR` reader
pub type R = crate::R<MTLTX_QURrs>;
///Register `MTLTxQUR` writer
pub type W = crate::W<MTLTX_QURrs>;
///Field `UFFRMCNT` reader - Underflow Packet Counter
pub type UFFRMCNT_R = crate::FieldReader<u16>;
///Field `UFFRMCNT` writer - Underflow Packet Counter
pub type UFFRMCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
///Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter
pub type UFCNTOVF_R = crate::BitReader;
///Field `UFCNTOVF` writer - Overflow Bit for Underflow Packet Counter
pub type UFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:10 - Underflow Packet Counter
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    ///Bit 11 - Overflow Bit for Underflow Packet Counter
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLTxQUR")
            .field("ufcntovf", &self.ufcntovf())
            .field("uffrmcnt", &self.uffrmcnt())
            .finish()
    }
}
impl W {
    ///Bits 0:10 - Underflow Packet Counter
    #[inline(always)]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W<'_, MTLTX_QURrs> {
        UFFRMCNT_W::new(self, 0)
    }
    ///Bit 11 - Overflow Bit for Underflow Packet Counter
    #[inline(always)]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W<'_, MTLTX_QURrs> {
        UFCNTOVF_W::new(self, 11)
    }
}
/**Tx queue underflow register

You can [`read`](crate::Reg::read) this register and get [`mtltx_qur::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtltx_qur::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H723.html#Ethernet_MTL:MTLTxQUR)*/
pub struct MTLTX_QURrs;
impl crate::RegisterSpec for MTLTX_QURrs {
    type Ux = u32;
}
///`read()` method returns [`mtltx_qur::R`](R) reader structure
impl crate::Readable for MTLTX_QURrs {}
///`write(|w| ..)` method takes [`mtltx_qur::W`](W) writer structure
impl crate::Writable for MTLTX_QURrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLTxQUR to value 0
impl crate::Resettable for MTLTX_QURrs {}
