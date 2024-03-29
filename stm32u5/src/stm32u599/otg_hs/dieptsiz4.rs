#[doc = "Register `DIEPTSIZ4` reader"]
pub type R = crate::R<DIEPTSIZ4rs>;
#[doc = "Register `DIEPTSIZ4` writer"]
pub type W = crate::W<DIEPTSIZ4rs>;
#[doc = "Field `XFRSIZ` reader - XFRSIZ"]
pub type XFRSIZ_R = crate::FieldReader<u32>;
#[doc = "Field `XFRSIZ` writer - XFRSIZ"]
pub type XFRSIZ_W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
#[doc = "Field `PKTCNT` reader - PKTCNT"]
pub type PKTCNT_R = crate::FieldReader<u16>;
#[doc = "Field `PKTCNT` writer - PKTCNT"]
pub type PKTCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `MCNT` reader - MCNT"]
pub type MCNT_R = crate::FieldReader;
#[doc = "Field `MCNT` writer - MCNT"]
pub type MCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:18 - XFRSIZ"]
    #[inline(always)]
    pub fn xfrsiz(&self) -> XFRSIZ_R {
        XFRSIZ_R::new(self.bits & 0x0007_ffff)
    }
    #[doc = "Bits 19:28 - PKTCNT"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:30 - MCNT"]
    #[inline(always)]
    pub fn mcnt(&self) -> MCNT_R {
        MCNT_R::new(((self.bits >> 29) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:18 - XFRSIZ"]
    #[inline(always)]
    #[must_use]
    pub fn xfrsiz(&mut self) -> XFRSIZ_W<DIEPTSIZ4rs> {
        XFRSIZ_W::new(self, 0)
    }
    #[doc = "Bits 19:28 - PKTCNT"]
    #[inline(always)]
    #[must_use]
    pub fn pktcnt(&mut self) -> PKTCNT_W<DIEPTSIZ4rs> {
        PKTCNT_W::new(self, 19)
    }
    #[doc = "Bits 29:30 - MCNT"]
    #[inline(always)]
    #[must_use]
    pub fn mcnt(&mut self) -> MCNT_W<DIEPTSIZ4rs> {
        MCNT_W::new(self, 29)
    }
}
#[doc = "The application must modify this register before enabling the endpoint. Once the endpoint is enabled using the endpoint enable bit in the DIEPCTLx registers (EPENA bit in DIEPCTLx), the core modifies this register. The application can only read this register once the core has cleared the endpoint enable bit.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dieptsiz4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dieptsiz4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPTSIZ4rs;
impl crate::RegisterSpec for DIEPTSIZ4rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dieptsiz4::R`](R) reader structure"]
impl crate::Readable for DIEPTSIZ4rs {}
#[doc = "`write(|w| ..)` method takes [`dieptsiz4::W`](W) writer structure"]
impl crate::Writable for DIEPTSIZ4rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPTSIZ4 to value 0"]
impl crate::Resettable for DIEPTSIZ4rs {
    const RESET_VALUE: u32 = 0;
}
