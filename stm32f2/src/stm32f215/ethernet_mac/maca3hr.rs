#[doc = "Register `MACA3HR` reader"]
pub type R = crate::R<MACA3HRrs>;
#[doc = "Register `MACA3HR` writer"]
pub type W = crate::W<MACA3HRrs>;
#[doc = "Field `MACA3H` reader - MAC address3 high"]
pub type MACA3H_R = crate::FieldReader<u16>;
#[doc = "Field `MACA3H` writer - MAC address3 high"]
pub type MACA3H_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MBC` reader - Mask byte control"]
pub type MBC_R = crate::FieldReader;
#[doc = "Field `MBC` writer - Mask byte control"]
pub type MBC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SA` reader - Source address"]
pub type SA_R = crate::BitReader;
#[doc = "Field `SA` writer - Source address"]
pub type SA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AE` reader - Address enable"]
pub type AE_R = crate::BitReader;
#[doc = "Field `AE` writer - Address enable"]
pub type AE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - MAC address3 high"]
    #[inline(always)]
    pub fn maca3h(&self) -> MACA3H_R {
        MACA3H_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    pub fn mbc(&self) -> MBC_R {
        MBC_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    pub fn ae(&self) -> AE_R {
        AE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAC address3 high"]
    #[inline(always)]
    #[must_use]
    pub fn maca3h(&mut self) -> MACA3H_W<MACA3HRrs> {
        MACA3H_W::new(self, 0)
    }
    #[doc = "Bits 24:29 - Mask byte control"]
    #[inline(always)]
    #[must_use]
    pub fn mbc(&mut self) -> MBC_W<MACA3HRrs> {
        MBC_W::new(self, 24)
    }
    #[doc = "Bit 30 - Source address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<MACA3HRrs> {
        SA_W::new(self, 30)
    }
    #[doc = "Bit 31 - Address enable"]
    #[inline(always)]
    #[must_use]
    pub fn ae(&mut self) -> AE_W<MACA3HRrs> {
        AE_W::new(self, 31)
    }
}
#[doc = "Ethernet MAC address 3 high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`maca3hr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`maca3hr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MACA3HRrs;
impl crate::RegisterSpec for MACA3HRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maca3hr::R`](R) reader structure"]
impl crate::Readable for MACA3HRrs {}
#[doc = "`write(|w| ..)` method takes [`maca3hr::W`](W) writer structure"]
impl crate::Writable for MACA3HRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MACA3HR to value 0xffff"]
impl crate::Resettable for MACA3HRrs {
    const RESET_VALUE: u32 = 0xffff;
}
