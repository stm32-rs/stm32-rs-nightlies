#[doc = "Register `HASH_STR` reader"]
pub type R = crate::R<HASH_STRrs>;
#[doc = "Register `HASH_STR` writer"]
pub type W = crate::W<HASH_STRrs>;
#[doc = "Field `NBLW` reader - NBLW"]
pub type NBLW_R = crate::FieldReader;
#[doc = "Field `NBLW` writer - NBLW"]
pub type NBLW_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DCAL` writer - DCAL"]
pub type DCAL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - NBLW"]
    #[inline(always)]
    pub fn nblw(&self) -> NBLW_R {
        NBLW_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - NBLW"]
    #[inline(always)]
    #[must_use]
    pub fn nblw(&mut self) -> NBLW_W<HASH_STRrs> {
        NBLW_W::new(self, 0)
    }
    #[doc = "Bit 8 - DCAL"]
    #[inline(always)]
    #[must_use]
    pub fn dcal(&mut self) -> DCAL_W<HASH_STRrs> {
        DCAL_W::new(self, 8)
    }
}
#[doc = "The HASH_STR register has two functions: It is used to define the number of valid bits in the last word of the message entered in the hash processor (that is the number of valid least significant bits in the last data written to the HASH_DIN register) It is used to start the processing of the last block in the message by writing the DCAL bit to 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_str::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_str::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HASH_STRrs;
impl crate::RegisterSpec for HASH_STRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hash_str::R`](R) reader structure"]
impl crate::Readable for HASH_STRrs {}
#[doc = "`write(|w| ..)` method takes [`hash_str::W`](W) writer structure"]
impl crate::Writable for HASH_STRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HASH_STR to value 0"]
impl crate::Resettable for HASH_STRrs {
    const RESET_VALUE: u32 = 0;
}
