#[doc = "Register `CSL` reader"]
pub type R = crate::R<CSLrs>;
#[doc = "Register `CSL` writer"]
pub type W = crate::W<CSLrs>;
#[doc = "Field `LENG` reader - code segment length"]
pub type LENG_R = crate::FieldReader<u16>;
#[doc = "Field `LENG` writer - code segment length"]
pub type LENG_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 8:21 - code segment length"]
    #[inline(always)]
    pub fn leng(&self) -> LENG_R {
        LENG_R::new(((self.bits >> 8) & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:21 - code segment length"]
    #[inline(always)]
    #[must_use]
    pub fn leng(&mut self) -> LENG_W<CSLrs> {
        LENG_W::new(self, 8)
    }
}
#[doc = "Code segment length\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CSLrs;
impl crate::RegisterSpec for CSLrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csl::R`](R) reader structure"]
impl crate::Readable for CSLrs {}
#[doc = "`write(|w| ..)` method takes [`csl::W`](W) writer structure"]
impl crate::Writable for CSLrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CSL to value 0"]
impl crate::Resettable for CSLrs {
    const RESET_VALUE: u32 = 0;
}
