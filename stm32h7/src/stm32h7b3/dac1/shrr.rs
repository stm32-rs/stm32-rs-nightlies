#[doc = "Register `SHRR` reader"]
pub type R = crate::R<SHRRrs>;
#[doc = "Register `SHRR` writer"]
pub type W = crate::W<SHRRrs>;
#[doc = "Field `TREFRESH1` reader - DAC Channel 1 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type TREFRESH1_R = crate::FieldReader;
#[doc = "Field `TREFRESH1` writer - DAC Channel 1 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type TREFRESH1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TREFRESH2` reader - DAC Channel 2 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type TREFRESH2_R = crate::FieldReader;
#[doc = "Field `TREFRESH2` writer - DAC Channel 2 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
pub type TREFRESH2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn trefresh1(&self) -> TREFRESH1_R {
        TREFRESH1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - DAC Channel 2 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    pub fn trefresh2(&self) -> TREFRESH2_R {
        TREFRESH2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - DAC Channel 1 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    #[must_use]
    pub fn trefresh1(&mut self) -> TREFRESH1_W<SHRRrs> {
        TREFRESH1_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - DAC Channel 2 refresh Time (only valid in sample &amp;amp; hold mode) Refresh time= (TREFRESH\\[7:0\\]) x T LSI"]
    #[inline(always)]
    #[must_use]
    pub fn trefresh2(&mut self) -> TREFRESH2_W<SHRRrs> {
        TREFRESH2_W::new(self, 16)
    }
}
#[doc = "DAC Sample and Hold refresh time register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shrr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shrr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHRRrs;
impl crate::RegisterSpec for SHRRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shrr::R`](R) reader structure"]
impl crate::Readable for SHRRrs {}
#[doc = "`write(|w| ..)` method takes [`shrr::W`](W) writer structure"]
impl crate::Writable for SHRRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHRR to value 0x0001_0001"]
impl crate::Resettable for SHRRrs {
    const RESET_VALUE: u32 = 0x0001_0001;
}
