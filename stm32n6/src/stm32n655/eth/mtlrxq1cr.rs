///Register `MTLRXQ1CR` reader
pub type R = crate::R<MTLRXQ1CRrs>;
///Register `MTLRXQ1CR` writer
pub type W = crate::W<MTLRXQ1CRrs>;
///Field `RXQ_WEGT` reader - Receive Queue Weight
pub type RXQ_WEGT_R = crate::FieldReader;
///Field `RXQ_WEGT` writer - Receive Queue Weight
pub type RXQ_WEGT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `RXQ_FRM_ARBIT` reader - Receive Queue Packet Arbitration
pub type RXQ_FRM_ARBIT_R = crate::BitReader;
///Field `RXQ_FRM_ARBIT` writer - Receive Queue Packet Arbitration
pub type RXQ_FRM_ARBIT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - Receive Queue Weight
    #[inline(always)]
    pub fn rxq_wegt(&self) -> RXQ_WEGT_R {
        RXQ_WEGT_R::new((self.bits & 7) as u8)
    }
    ///Bit 3 - Receive Queue Packet Arbitration
    #[inline(always)]
    pub fn rxq_frm_arbit(&self) -> RXQ_FRM_ARBIT_R {
        RXQ_FRM_ARBIT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLRXQ1CR")
            .field("rxq_wegt", &self.rxq_wegt())
            .field("rxq_frm_arbit", &self.rxq_frm_arbit())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - Receive Queue Weight
    #[inline(always)]
    pub fn rxq_wegt(&mut self) -> RXQ_WEGT_W<'_, MTLRXQ1CRrs> {
        RXQ_WEGT_W::new(self, 0)
    }
    ///Bit 3 - Receive Queue Packet Arbitration
    #[inline(always)]
    pub fn rxq_frm_arbit(&mut self) -> RXQ_FRM_ARBIT_W<'_, MTLRXQ1CRrs> {
        RXQ_FRM_ARBIT_W::new(self, 3)
    }
}
/**R1 queue 1 control register

You can [`read`](crate::Reg::read) this register and get [`mtlrxq1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlrxq1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#ETH:MTLRXQ1CR)*/
pub struct MTLRXQ1CRrs;
impl crate::RegisterSpec for MTLRXQ1CRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlrxq1cr::R`](R) reader structure
impl crate::Readable for MTLRXQ1CRrs {}
///`write(|w| ..)` method takes [`mtlrxq1cr::W`](W) writer structure
impl crate::Writable for MTLRXQ1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLRXQ1CR to value 0
impl crate::Resettable for MTLRXQ1CRrs {}
