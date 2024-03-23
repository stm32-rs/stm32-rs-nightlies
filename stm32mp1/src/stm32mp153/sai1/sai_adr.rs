#[doc = "Register `SAI_ADR` reader"]
pub type R = crate::R<SAI_ADRrs>;
#[doc = "Register `SAI_ADR` writer"]
pub type W = crate::W<SAI_ADRrs>;
#[doc = "Field `DATA` reader - DATA"]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - DATA"]
pub type DATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DATA"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<SAI_ADRrs> {
        DATA_W::new(self, 0)
    }
}
#[doc = "Data register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sai_adr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sai_adr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAI_ADRrs;
impl crate::RegisterSpec for SAI_ADRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_adr::R`](R) reader structure"]
impl crate::Readable for SAI_ADRrs {}
#[doc = "`write(|w| ..)` method takes [`sai_adr::W`](W) writer structure"]
impl crate::Writable for SAI_ADRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAI_ADR to value 0"]
impl crate::Resettable for SAI_ADRrs {
    const RESET_VALUE: u32 = 0;
}
