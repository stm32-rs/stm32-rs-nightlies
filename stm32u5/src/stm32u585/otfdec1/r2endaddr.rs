#[doc = "Register `R2ENDADDR` reader"]
pub type R = crate::R<R2ENDADDRrs>;
#[doc = "Register `R2ENDADDR` writer"]
pub type W = crate::W<R2ENDADDRrs>;
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
    pub fn regx_end_addr(&mut self) -> REGX_END_ADDR_W<R2ENDADDRrs> {
        REGX_END_ADDR_W::new(self, 0)
    }
}
#[doc = "OTFDEC region x end address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r2endaddr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r2endaddr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R2ENDADDRrs;
impl crate::RegisterSpec for R2ENDADDRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r2endaddr::R`](R) reader structure"]
impl crate::Readable for R2ENDADDRrs {}
#[doc = "`write(|w| ..)` method takes [`r2endaddr::W`](W) writer structure"]
impl crate::Writable for R2ENDADDRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets R2ENDADDR to value 0x0fff"]
impl crate::Resettable for R2ENDADDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
