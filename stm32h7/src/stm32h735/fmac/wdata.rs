#[doc = "Register `WDATA` reader"]
pub type R = crate::R<WDATArs>;
#[doc = "Register `WDATA` writer"]
pub type W = crate::W<WDATArs>;
#[doc = "Field `WDATA` reader - Write data (write data are transferred to the address indicated by the write pointer)"]
pub type WDATA_R = crate::FieldReader<u16>;
#[doc = "Field `WDATA` writer - Write data (write data are transferred to the address indicated by the write pointer)"]
pub type WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Write data (write data are transferred to the address indicated by the write pointer)"]
    #[inline(always)]
    pub fn wdata(&self) -> WDATA_R {
        WDATA_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Write data (write data are transferred to the address indicated by the write pointer)"]
    #[inline(always)]
    #[must_use]
    pub fn wdata(&mut self) -> WDATA_W<WDATArs> {
        WDATA_W::new(self, 0)
    }
}
#[doc = "Write data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDATArs;
impl crate::RegisterSpec for WDATArs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdata::R`](R) reader structure"]
impl crate::Readable for WDATArs {}
#[doc = "`write(|w| ..)` method takes [`wdata::W`](W) writer structure"]
impl crate::Writable for WDATArs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDATA to value 0"]
impl crate::Resettable for WDATArs {
    const RESET_VALUE: u32 = 0;
}
