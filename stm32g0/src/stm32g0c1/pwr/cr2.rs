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
#[doc = "Field `PVMENDAC` reader - PVMENDAC"]
pub type PVMENDAC_R = crate::BitReader;
#[doc = "Field `PVMENDAC` writer - PVMENDAC"]
pub type PVMENDAC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVMENUSB` reader - PVMENUSB"]
pub type PVMENUSB_R = crate::BitReader;
#[doc = "Field `PVMENUSB` writer - PVMENUSB"]
pub type PVMENUSB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOSV` reader - IOSV"]
pub type IOSV_R = crate::BitReader;
#[doc = "Field `IOSV` writer - IOSV"]
pub type IOSV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USV` reader - USV"]
pub type USV_R = crate::BitReader;
#[doc = "Field `USV` writer - USV"]
pub type USV_W<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 7 - PVMENDAC"]
    #[inline(always)]
    pub fn pvmendac(&self) -> PVMENDAC_R {
        PVMENDAC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PVMENUSB"]
    #[inline(always)]
    pub fn pvmenusb(&self) -> PVMENUSB_R {
        PVMENUSB_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - IOSV"]
    #[inline(always)]
    pub fn iosv(&self) -> IOSV_R {
        IOSV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - USV"]
    #[inline(always)]
    pub fn usv(&self) -> USV_R {
        USV_R::new(((self.bits >> 10) & 1) != 0)
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
    #[doc = "Bit 7 - PVMENDAC"]
    #[inline(always)]
    #[must_use]
    pub fn pvmendac(&mut self) -> PVMENDAC_W<CR2rs> {
        PVMENDAC_W::new(self, 7)
    }
    #[doc = "Bit 8 - PVMENUSB"]
    #[inline(always)]
    #[must_use]
    pub fn pvmenusb(&mut self) -> PVMENUSB_W<CR2rs> {
        PVMENUSB_W::new(self, 8)
    }
    #[doc = "Bit 9 - IOSV"]
    #[inline(always)]
    #[must_use]
    pub fn iosv(&mut self) -> IOSV_W<CR2rs> {
        IOSV_W::new(self, 9)
    }
    #[doc = "Bit 10 - USV"]
    #[inline(always)]
    #[must_use]
    pub fn usv(&mut self) -> USV_W<CR2rs> {
        USV_W::new(self, 10)
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
