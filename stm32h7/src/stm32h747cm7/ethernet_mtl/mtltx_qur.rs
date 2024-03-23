#[doc = "Register `MTLTxQUR` reader"]
pub type R = crate::R<MTLTX_QURrs>;
#[doc = "Register `MTLTxQUR` writer"]
pub type W = crate::W<MTLTX_QURrs>;
#[doc = "Field `UFFRMCNT` reader - Underflow Packet Counter"]
pub type UFFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `UFFRMCNT` writer - Underflow Packet Counter"]
pub type UFFRMCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `UFCNTOVF` reader - Overflow Bit for Underflow Packet Counter"]
pub type UFCNTOVF_R = crate::BitReader;
#[doc = "Field `UFCNTOVF` writer - Overflow Bit for Underflow Packet Counter"]
pub type UFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UFFRMCNT_R {
        UFFRMCNT_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UFCNTOVF_R {
        UFCNTOVF_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    #[must_use]
    pub fn uffrmcnt(&mut self) -> UFFRMCNT_W<MTLTX_QURrs> {
        UFFRMCNT_W::new(self, 0)
    }
    #[doc = "Bit 11 - Overflow Bit for Underflow Packet Counter"]
    #[inline(always)]
    #[must_use]
    pub fn ufcntovf(&mut self) -> UFCNTOVF_W<MTLTX_QURrs> {
        UFCNTOVF_W::new(self, 11)
    }
}
#[doc = "Tx queue underflow register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtltx_qur::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtltx_qur::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTLTX_QURrs;
impl crate::RegisterSpec for MTLTX_QURrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtltx_qur::R`](R) reader structure"]
impl crate::Readable for MTLTX_QURrs {}
#[doc = "`write(|w| ..)` method takes [`mtltx_qur::W`](W) writer structure"]
impl crate::Writable for MTLTX_QURrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MTLTxQUR to value 0"]
impl crate::Resettable for MTLTX_QURrs {
    const RESET_VALUE: u32 = 0;
}
