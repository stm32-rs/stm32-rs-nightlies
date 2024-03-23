#[doc = "Register `WTCR` reader"]
pub type R = crate::R<WTCRrs>;
#[doc = "Register `WTCR` writer"]
pub type W = crate::W<WTCRrs>;
#[doc = "Field `IMODE` reader - IMODE"]
pub type IMODE_R = crate::FieldReader;
#[doc = "Field `IMODE` writer - IMODE"]
pub type IMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IDTR` reader - IDTR"]
pub type IDTR_R = crate::BitReader;
#[doc = "Field `IDTR` writer - IDTR"]
pub type IDTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISIZE` reader - ISIZE"]
pub type ISIZE_R = crate::FieldReader;
#[doc = "Field `ISIZE` writer - ISIZE"]
pub type ISIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ADMODE` reader - ADMODE"]
pub type ADMODE_R = crate::FieldReader;
#[doc = "Field `ADMODE` writer - ADMODE"]
pub type ADMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADDTR` reader - ADDTR"]
pub type ADDTR_R = crate::BitReader;
#[doc = "Field `ADDTR` writer - ADDTR"]
pub type ADDTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADSIZE` reader - ADSIZE"]
pub type ADSIZE_R = crate::FieldReader;
#[doc = "Field `ADSIZE` writer - ADSIZE"]
pub type ADSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ABMODE` reader - ABMODE"]
pub type ABMODE_R = crate::FieldReader;
#[doc = "Field `ABMODE` writer - ABMODE"]
pub type ABMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ABDTR` reader - ABDTR"]
pub type ABDTR_R = crate::BitReader;
#[doc = "Field `ABDTR` writer - ABDTR"]
pub type ABDTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABSIZE` reader - ABSIZE"]
pub type ABSIZE_R = crate::FieldReader;
#[doc = "Field `ABSIZE` writer - ABSIZE"]
pub type ABSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DMODE` reader - DMODE"]
pub type DMODE_R = crate::FieldReader;
#[doc = "Field `DMODE` writer - DMODE"]
pub type DMODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DDTR` reader - DDTR"]
pub type DDTR_R = crate::BitReader;
#[doc = "Field `DDTR` writer - DDTR"]
pub type DDTR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DQSE` reader - DQSE"]
pub type DQSE_R = crate::BitReader;
#[doc = "Field `DQSE` writer - DQSE"]
pub type DQSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - IMODE"]
    #[inline(always)]
    pub fn imode(&self) -> IMODE_R {
        IMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - IDTR"]
    #[inline(always)]
    pub fn idtr(&self) -> IDTR_R {
        IDTR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ISIZE"]
    #[inline(always)]
    pub fn isize(&self) -> ISIZE_R {
        ISIZE_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:10 - ADMODE"]
    #[inline(always)]
    pub fn admode(&self) -> ADMODE_R {
        ADMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ADDTR"]
    #[inline(always)]
    pub fn addtr(&self) -> ADDTR_R {
        ADDTR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - ADSIZE"]
    #[inline(always)]
    pub fn adsize(&self) -> ADSIZE_R {
        ADSIZE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - ABMODE"]
    #[inline(always)]
    pub fn abmode(&self) -> ABMODE_R {
        ABMODE_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - ABDTR"]
    #[inline(always)]
    pub fn abdtr(&self) -> ABDTR_R {
        ABDTR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - ABSIZE"]
    #[inline(always)]
    pub fn absize(&self) -> ABSIZE_R {
        ABSIZE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 24:26 - DMODE"]
    #[inline(always)]
    pub fn dmode(&self) -> DMODE_R {
        DMODE_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - DDTR"]
    #[inline(always)]
    pub fn ddtr(&self) -> DDTR_R {
        DDTR_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DQSE"]
    #[inline(always)]
    pub fn dqse(&self) -> DQSE_R {
        DQSE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - IMODE"]
    #[inline(always)]
    #[must_use]
    pub fn imode(&mut self) -> IMODE_W<WTCRrs> {
        IMODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - IDTR"]
    #[inline(always)]
    #[must_use]
    pub fn idtr(&mut self) -> IDTR_W<WTCRrs> {
        IDTR_W::new(self, 3)
    }
    #[doc = "Bits 4:5 - ISIZE"]
    #[inline(always)]
    #[must_use]
    pub fn isize(&mut self) -> ISIZE_W<WTCRrs> {
        ISIZE_W::new(self, 4)
    }
    #[doc = "Bits 8:10 - ADMODE"]
    #[inline(always)]
    #[must_use]
    pub fn admode(&mut self) -> ADMODE_W<WTCRrs> {
        ADMODE_W::new(self, 8)
    }
    #[doc = "Bit 11 - ADDTR"]
    #[inline(always)]
    #[must_use]
    pub fn addtr(&mut self) -> ADDTR_W<WTCRrs> {
        ADDTR_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - ADSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn adsize(&mut self) -> ADSIZE_W<WTCRrs> {
        ADSIZE_W::new(self, 12)
    }
    #[doc = "Bits 16:18 - ABMODE"]
    #[inline(always)]
    #[must_use]
    pub fn abmode(&mut self) -> ABMODE_W<WTCRrs> {
        ABMODE_W::new(self, 16)
    }
    #[doc = "Bit 19 - ABDTR"]
    #[inline(always)]
    #[must_use]
    pub fn abdtr(&mut self) -> ABDTR_W<WTCRrs> {
        ABDTR_W::new(self, 19)
    }
    #[doc = "Bits 20:21 - ABSIZE"]
    #[inline(always)]
    #[must_use]
    pub fn absize(&mut self) -> ABSIZE_W<WTCRrs> {
        ABSIZE_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - DMODE"]
    #[inline(always)]
    #[must_use]
    pub fn dmode(&mut self) -> DMODE_W<WTCRrs> {
        DMODE_W::new(self, 24)
    }
    #[doc = "Bit 27 - DDTR"]
    #[inline(always)]
    #[must_use]
    pub fn ddtr(&mut self) -> DDTR_W<WTCRrs> {
        DDTR_W::new(self, 27)
    }
    #[doc = "Bit 29 - DQSE"]
    #[inline(always)]
    #[must_use]
    pub fn dqse(&mut self) -> DQSE_W<WTCRrs> {
        DQSE_W::new(self, 29)
    }
}
#[doc = "WTCR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wtcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wtcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WTCRrs;
impl crate::RegisterSpec for WTCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wtcr::R`](R) reader structure"]
impl crate::Readable for WTCRrs {}
#[doc = "`write(|w| ..)` method takes [`wtcr::W`](W) writer structure"]
impl crate::Writable for WTCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WTCR to value 0"]
impl crate::Resettable for WTCRrs {
    const RESET_VALUE: u32 = 0;
}
