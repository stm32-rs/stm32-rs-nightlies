#[doc = "Register `TX_ORDSET` reader"]
pub type R = crate::R<TX_ORDSETrs>;
#[doc = "Register `TX_ORDSET` writer"]
pub type W = crate::W<TX_ORDSETrs>;
#[doc = "Field `TXORDSET` reader - TXORDSET"]
pub type TXORDSET_R = crate::FieldReader<u32>;
#[doc = "Field `TXORDSET` writer - TXORDSET"]
pub type TXORDSET_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    pub fn txordset(&self) -> TXORDSET_R {
        TXORDSET_R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - TXORDSET"]
    #[inline(always)]
    #[must_use]
    pub fn txordset(&mut self) -> TXORDSET_W<TX_ORDSETrs> {
        TXORDSET_W::new(self, 0)
    }
}
#[doc = "UCPD Tx Ordered Set Type Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_ordset::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_ordset::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_ORDSETrs;
impl crate::RegisterSpec for TX_ORDSETrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_ordset::R`](R) reader structure"]
impl crate::Readable for TX_ORDSETrs {}
#[doc = "`write(|w| ..)` method takes [`tx_ordset::W`](W) writer structure"]
impl crate::Writable for TX_ORDSETrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_ORDSET to value 0"]
impl crate::Resettable for TX_ORDSETrs {
    const RESET_VALUE: u32 = 0;
}
