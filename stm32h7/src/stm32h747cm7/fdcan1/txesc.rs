#[doc = "Register `TXESC` reader"]
pub type R = crate::R<TXESCrs>;
#[doc = "Register `TXESC` writer"]
pub type W = crate::W<TXESCrs>;
#[doc = "Field `TBDS` reader - Tx Buffer Data Field Size:"]
pub type TBDS_R = crate::FieldReader;
#[doc = "Field `TBDS` writer - Tx Buffer Data Field Size:"]
pub type TBDS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn tbds(&self) -> TBDS_R {
        TBDS_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Tx Buffer Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn tbds(&mut self) -> TBDS_W<TXESCrs> {
        TBDS_W::new(self, 0)
    }
}
#[doc = "FDCAN Tx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXESCrs;
impl crate::RegisterSpec for TXESCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txesc::R`](R) reader structure"]
impl crate::Readable for TXESCrs {}
#[doc = "`write(|w| ..)` method takes [`txesc::W`](W) writer structure"]
impl crate::Writable for TXESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TXESCrs {
    const RESET_VALUE: u32 = 0;
}
