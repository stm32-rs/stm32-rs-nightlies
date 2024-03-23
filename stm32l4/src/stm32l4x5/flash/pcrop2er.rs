#[doc = "Register `PCROP2ER` reader"]
pub type R = crate::R<PCROP2ERrs>;
#[doc = "Register `PCROP2ER` writer"]
pub type W = crate::W<PCROP2ERrs>;
#[doc = "Field `PCROP2_END` reader - Bank 2 PCROP area end offset"]
pub type PCROP2_END_R = crate::FieldReader<u16>;
#[doc = "Field `PCROP2_END` writer - Bank 2 PCROP area end offset"]
pub type PCROP2_END_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Bank 2 PCROP area end offset"]
    #[inline(always)]
    pub fn pcrop2_end(&self) -> PCROP2_END_R {
        PCROP2_END_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank 2 PCROP area end offset"]
    #[inline(always)]
    #[must_use]
    pub fn pcrop2_end(&mut self) -> PCROP2_END_W<PCROP2ERrs> {
        PCROP2_END_W::new(self, 0)
    }
}
#[doc = "Flash Bank 2 PCROP End address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcrop2er::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcrop2er::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PCROP2ERrs;
impl crate::RegisterSpec for PCROP2ERrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcrop2er::R`](R) reader structure"]
impl crate::Readable for PCROP2ERrs {}
#[doc = "`write(|w| ..)` method takes [`pcrop2er::W`](W) writer structure"]
impl crate::Writable for PCROP2ERrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCROP2ER to value 0xffff_0000"]
impl crate::Resettable for PCROP2ERrs {
    const RESET_VALUE: u32 = 0xffff_0000;
}
