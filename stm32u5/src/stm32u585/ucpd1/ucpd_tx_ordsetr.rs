///Register `UCPD_TX_ORDSETR` reader
pub type R = crate::R<UCPD_TX_ORDSETRrs>;
///Register `UCPD_TX_ORDSETR` writer
pub type W = crate::W<UCPD_TX_ORDSETRrs>;
///Field `TXORDSET` reader - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
pub type TXORDSET_R = crate::FieldReader<u32>;
///Field `TXORDSET` writer - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
pub type TXORDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
    #[inline(always)]
    pub fn txordset(&self) -> TXORDSET_R {
        TXORDSET_R::new(self.bits & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UCPD_TX_ORDSETR")
            .field("txordset", &self.txordset())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - Ordered set to transmit The bitfield determines a full 20-bit sequence to transmit, consisting of four K-codes, each of five bits, defining the packet to transmit. The bit 0 (bit 0 of K-code1) is the first, the bit 19 (bit 4 of K‑code4) the last.
    #[inline(always)]
    #[must_use]
    pub fn txordset(&mut self) -> TXORDSET_W<UCPD_TX_ORDSETRrs> {
        TXORDSET_W::new(self, 0)
    }
}
/**UCPD Tx ordered set type register

You can [`read`](crate::Reg::read) this register and get [`ucpd_tx_ordsetr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucpd_tx_ordsetr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#UCPD1:UCPD_TX_ORDSETR)*/
pub struct UCPD_TX_ORDSETRrs;
impl crate::RegisterSpec for UCPD_TX_ORDSETRrs {
    type Ux = u32;
}
///`read()` method returns [`ucpd_tx_ordsetr::R`](R) reader structure
impl crate::Readable for UCPD_TX_ORDSETRrs {}
///`write(|w| ..)` method takes [`ucpd_tx_ordsetr::W`](W) writer structure
impl crate::Writable for UCPD_TX_ORDSETRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets UCPD_TX_ORDSETR to value 0
impl crate::Resettable for UCPD_TX_ORDSETRrs {
    const RESET_VALUE: u32 = 0;
}
