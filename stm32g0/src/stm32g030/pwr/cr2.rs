#[doc = "Register `CR2` reader"]
pub type R = crate::R<CR2rs>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<CR2rs>;
#[doc = "Field `PVDE` reader - Power voltage detector enable"]
pub type PVDE_R = crate::BitReader;
#[doc = "Field `PVDE` writer - Power voltage detector enable"]
pub type PVDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDFT` reader - Power voltage detector falling threshold selection"]
pub type PVDFT_R = crate::FieldReader;
#[doc = "Field `PVDFT` writer - Power voltage detector falling threshold selection"]
pub type PVDFT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PVDRT` reader - Power voltage detector rising threshold selection"]
pub type PVDRT_R = crate::FieldReader;
#[doc = "Field `PVDRT` writer - Power voltage detector rising threshold selection"]
pub type PVDRT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    pub fn pvde(&self) -> PVDE_R {
        PVDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector falling threshold selection"]
    #[inline(always)]
    pub fn pvdft(&self) -> PVDFT_R {
        PVDFT_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Power voltage detector rising threshold selection"]
    #[inline(always)]
    pub fn pvdrt(&self) -> PVDRT_R {
        PVDRT_R::new(((self.bits >> 4) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Power voltage detector enable"]
    #[inline(always)]
    #[must_use]
    pub fn pvde(&mut self) -> PVDE_W<CR2rs> {
        PVDE_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - Power voltage detector falling threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn pvdft(&mut self) -> PVDFT_W<CR2rs> {
        PVDFT_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Power voltage detector rising threshold selection"]
    #[inline(always)]
    #[must_use]
    pub fn pvdrt(&mut self) -> PVDRT_W<CR2rs> {
        PVDRT_W::new(self, 4)
    }
}
#[doc = "Power control register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
