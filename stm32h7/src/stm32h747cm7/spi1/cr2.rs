#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `TSIZE` reader - Number of data at current transfer"]
pub type TSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `TSIZE` writer - Number of data at current transfer"]
pub type TSIZE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
#[doc = "Field `TSER` reader - Number of data transfer extension to be reload into TSIZE just when a previous"]
pub type TSER_R = crate::FieldReader<u16>;
#[doc = "Field `TSER` writer - Number of data transfer extension to be reload into TSIZE just when a previous"]
pub type TSER_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data at current transfer"]
    #[inline(always)]
    pub fn tsize(&self) -> TSIZE_R {
        TSIZE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of data transfer extension to be reload into TSIZE just when a previous"]
    #[inline(always)]
    pub fn tser(&self) -> TSER_R {
        TSER_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data at current transfer"]
    #[inline(always)]
    #[must_use]
    pub fn tsize(&mut self) -> TSIZE_W<CR2rs> {
        TSIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Number of data transfer extension to be reload into TSIZE just when a previous"]
    #[inline(always)]
    #[must_use]
    pub fn tser(&mut self) -> TSER_W<CR2rs> {
        TSER_W::new(self, 16)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for CR2rs {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2rs {
    const RESET_VALUE: u32 = 0;
}
