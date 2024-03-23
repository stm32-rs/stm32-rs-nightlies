#[doc = "Register `HSPI_PIR` reader"]
pub type R = crate::R<HSPI_PIRrs>;
#[doc = "Register `HSPI_PIR` writer"]
pub type W = crate::W<HSPI_PIRrs>;
#[doc = "Field `INTERVAL` reader - 15: 0\\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases"]
pub type INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `INTERVAL` writer - 15: 0\\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases"]
pub type INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 15: 0\\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases"]
    #[inline(always)]
    pub fn interval(&self) -> INTERVAL_R {
        INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 15: 0\\]: Polling interval Number of CLK cycle between a read during the automatic-polling phases"]
    #[inline(always)]
    #[must_use]
    pub fn interval(&mut self) -> INTERVAL_W<HSPI_PIRrs> {
        INTERVAL_W::new(self, 0)
    }
}
#[doc = "HSPI polling interval register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hspi_pir::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hspi_pir::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSPI_PIRrs;
impl crate::RegisterSpec for HSPI_PIRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hspi_pir::R`](R) reader structure"]
impl crate::Readable for HSPI_PIRrs {}
#[doc = "`write(|w| ..)` method takes [`hspi_pir::W`](W) writer structure"]
impl crate::Writable for HSPI_PIRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HSPI_PIR to value 0"]
impl crate::Resettable for HSPI_PIRrs {
    const RESET_VALUE: u32 = 0;
}
