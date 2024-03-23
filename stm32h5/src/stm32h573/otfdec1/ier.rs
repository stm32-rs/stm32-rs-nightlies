#[doc = "Register `IER` reader"]
pub type R = crate::R<IERrs>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IERrs>;
#[doc = "Field `SEIE` reader - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set."]
pub type SEIE_R = crate::BitReader;
#[doc = "Field `SEIE` writer - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set."]
pub type SEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XONEIE` reader - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set."]
pub type XONEIE_R = crate::BitReader;
#[doc = "Field `XONEIE` writer - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set."]
pub type XONEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KEIE` reader - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set."]
pub type KEIE_R = crate::BitReader;
#[doc = "Field `KEIE` writer - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set."]
pub type KEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set."]
    #[inline(always)]
    pub fn seie(&self) -> SEIE_R {
        SEIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set."]
    #[inline(always)]
    pub fn xoneie(&self) -> XONEIE_R {
        XONEIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set."]
    #[inline(always)]
    pub fn keie(&self) -> KEIE_R {
        KEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Security error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when SEIF flag status is set."]
    #[inline(always)]
    #[must_use]
    pub fn seie(&mut self) -> SEIE_W<IERrs> {
        SEIE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Execute-only execute-never error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when XONEIF flag status is set."]
    #[inline(always)]
    #[must_use]
    pub fn xoneie(&mut self) -> XONEIE_W<IERrs> {
        XONEIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Key error interrupt enable This bit is read and written by application. It controls the OTFDEC interrupt generation when KEIF flag status is set."]
    #[inline(always)]
    #[must_use]
    pub fn keie(&mut self) -> KEIE_W<IERrs> {
        KEIE_W::new(self, 2)
    }
}
#[doc = "OTFDEC interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IERrs;
impl crate::RegisterSpec for IERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IERrs {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IERrs {
    const RESET_VALUE: u32 = 0;
}
