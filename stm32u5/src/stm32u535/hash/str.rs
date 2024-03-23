#[doc = "Register `STR` reader"]
pub type R = crate::R<STRrs>;
#[doc = "Register `STR` writer"]
pub type W = crate::W<STRrs>;
#[doc = "Field `NBLW` reader - Number of valid bits in the last word of the message"]
pub type NBLW_R = crate::FieldReader;
#[doc = "Field `NBLW` writer - Number of valid bits in the last word of the message"]
pub type NBLW_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DCAL` writer - Digest calculation"]
pub type DCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Number of valid bits in the last word of the message"]
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Number of valid bits in the last word of the message"]
    #[inline(always)]
    #[must_use]
    pub fn nblw(&mut self) -> NBLW_W<STRrs> {
        NBLW_W::new(self, 0)
    }
    #[doc = "Bit 8 - Digest calculation"]
    #[inline(always)]
    #[must_use]
    pub fn dcal(&mut self) -> DCAL_W<STRrs> {
        DCAL_W::new(self, 8)
    }
}
#[doc = "start register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`str::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`str::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STRrs;
impl crate::RegisterSpec for STRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`str::R`](R) reader structure"]
impl crate::Readable for STRrs {}
#[doc = "`write(|w| ..)` method takes [`str::W`](W) writer structure"]
impl crate::Writable for STRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for STRrs {
    const RESET_VALUE: u32 = 0;
}
