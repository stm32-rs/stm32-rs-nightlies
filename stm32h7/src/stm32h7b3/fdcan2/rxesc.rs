#[doc = "Register `RXESC` reader"]
pub type R = crate::R<RXESCrs>;
#[doc = "Register `RXESC` writer"]
pub type W = crate::W<RXESCrs>;
#[doc = "Field `F0DS` reader - Rx FIFO 1 Data Field Size:"]
pub type F0DS_R = crate::FieldReader;
#[doc = "Field `F0DS` writer - Rx FIFO 1 Data Field Size:"]
pub type F0DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `F1DS` reader - Rx FIFO 0 Data Field Size:"]
pub type F1DS_R = crate::FieldReader;
#[doc = "Field `F1DS` writer - Rx FIFO 0 Data Field Size:"]
pub type F1DS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RBDS` reader - Rx Buffer Data Field Size:"]
pub type RBDS_R = crate::FieldReader;
#[doc = "Field `RBDS` writer - Rx Buffer Data Field Size:"]
pub type RBDS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0DS_R {
        F0DS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1DS_R {
        F1DS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn rbds(&self) -> RBDS_R {
        RBDS_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn f0ds(&mut self) -> F0DS_W<RXESCrs> {
        F0DS_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn f1ds(&mut self) -> F1DS_W<RXESCrs> {
        F1DS_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    #[must_use]
    pub fn rbds(&mut self) -> RBDS_W<RXESCrs> {
        RBDS_W::new(self, 8)
    }
}
#[doc = "FDCAN Rx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxesc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxesc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RXESCrs;
impl crate::RegisterSpec for RXESCrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxesc::R`](R) reader structure"]
impl crate::Readable for RXESCrs {}
#[doc = "`write(|w| ..)` method takes [`rxesc::W`](W) writer structure"]
impl crate::Writable for RXESCrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RXESCrs {
    const RESET_VALUE: u32 = 0;
}
