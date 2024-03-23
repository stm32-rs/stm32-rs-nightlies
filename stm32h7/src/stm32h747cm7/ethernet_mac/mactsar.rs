#[doc = "Register `MACTSAR` reader"]
pub type R = crate::R<MACTSARrs>;
#[doc = "Register `MACTSAR` writer"]
pub type W = crate::W<MACTSARrs>;
#[doc = "Field `TSAR` reader - Timestamp Addend Register"]
pub type TSAR_R = crate::FieldReader<u32>;
#[doc = "Field `TSAR` writer - Timestamp Addend Register"]
pub type TSAR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    pub fn tsar(&self) -> TSAR_R {
        TSAR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timestamp Addend Register"]
    #[inline(always)]
    #[must_use]
    pub fn tsar(&mut self) -> TSAR_W<MACTSARrs> {
        TSAR_W::new(self, 0)
    }
}
#[doc = "Timestamp addend register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mactsar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mactsar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACTSARrs;
impl crate::RegisterSpec for MACTSARrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsar::R`](R) reader structure"]
impl crate::Readable for MACTSARrs {}
#[doc = "`write(|w| ..)` method takes [`mactsar::W`](W) writer structure"]
impl crate::Writable for MACTSARrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACTSAR to value 0"]
impl crate::Resettable for MACTSARrs {
    const RESET_VALUE: u32 = 0;
}
