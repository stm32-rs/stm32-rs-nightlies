#[doc = "Register `DIFSEL` reader"]
pub type R = crate::R<DIFSELrs>;
#[doc = "Register `DIFSEL` writer"]
pub type W = crate::W<DIFSELrs>;
#[doc = "Field `DIFSEL_1_15` reader - Differential mode for channels 15 to 1"]
pub type DIFSEL_1_15_R = crate::FieldReader<u16>;
#[doc = "Field `DIFSEL_1_15` writer - Differential mode for channels 15 to 1"]
pub type DIFSEL_1_15_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `DIFSEL_16_18` reader - Differential mode for channels 18 to 16"]
pub type DIFSEL_16_18_R = crate::FieldReader;
impl R {
    #[doc = "Bits 1:15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    pub fn difsel_1_15(&self) -> DIFSEL_1_15_R {
        DIFSEL_1_15_R::new(((self.bits >> 1) & 0x7fff) as u16)
    }
    #[doc = "Bits 16:18 - Differential mode for channels 18 to 16"]
    #[inline(always)]
    pub fn difsel_16_18(&self) -> DIFSEL_16_18_R {
        DIFSEL_16_18_R::new(((self.bits >> 16) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 1:15 - Differential mode for channels 15 to 1"]
    #[inline(always)]
    #[must_use]
    pub fn difsel_1_15(&mut self) -> DIFSEL_1_15_W<DIFSELrs> {
        DIFSEL_1_15_W::new(self, 1)
    }
}
#[doc = "Differential Mode Selection Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`difsel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`difsel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIFSELrs;
impl crate::RegisterSpec for DIFSELrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`difsel::R`](R) reader structure"]
impl crate::Readable for DIFSELrs {}
#[doc = "`write(|w| ..)` method takes [`difsel::W`](W) writer structure"]
impl crate::Writable for DIFSELrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIFSEL to value 0"]
impl crate::Resettable for DIFSELrs {
    const RESET_VALUE: u32 = 0;
}
