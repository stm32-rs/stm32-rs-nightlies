#[doc = "Register `R4ENDADDR` reader"]
pub type R = crate::R<R4ENDADDRrs>;
#[doc = "Register `R4ENDADDR` writer"]
pub type W = crate::W<R4ENDADDRrs>;
#[doc = "Field `REGx_END_ADDR` reader - Region AXI end address"]
pub type REGX_END_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `REGx_END_ADDR` writer - Region AXI end address"]
pub type REGX_END_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Region AXI end address"]
    #[inline(always)]
    pub fn regx_end_addr(&self) -> REGX_END_ADDR_R {
        REGX_END_ADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Region AXI end address"]
    #[inline(always)]
    #[must_use]
    pub fn regx_end_addr(&mut self) -> REGX_END_ADDR_W<R4ENDADDRrs> {
        REGX_END_ADDR_W::new(self, 0)
    }
}
#[doc = "OTFDEC region x end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r4endaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r4endaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R4ENDADDRrs;
impl crate::RegisterSpec for R4ENDADDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r4endaddr::R`](R) reader structure"]
impl crate::Readable for R4ENDADDRrs {}
#[doc = "`write(|w| ..)` method takes [`r4endaddr::W`](W) writer structure"]
impl crate::Writable for R4ENDADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R4ENDADDR to value 0x0fff"]
impl crate::Resettable for R4ENDADDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
